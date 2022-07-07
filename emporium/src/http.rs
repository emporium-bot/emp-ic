use candid::{candid_method, CandidType, Nat, Principal};
use ic_cdk;
use ic_cdk_macros::{self, heartbeat, query, update};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

type Timestamp = u64;
type Rate = f32;

#[derive(CandidType, Clone, Deserialize, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct HttpHeader {
  pub name: String,
  pub value: String,
}

#[derive(Clone, Debug, PartialEq, CandidType, Eq, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
  GET,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct CanisterHttpRequestArgs {
  pub url: String,
  pub headers: Vec<HttpHeader>,
  pub body: Option<Vec<u8>>,
  pub http_method: HttpMethod,
  pub transform_method_name: Option<String>,
}

#[derive(CandidType, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CanisterHttpResponsePayload {
  pub status: u64,
  pub headers: Vec<HttpHeader>,
  pub body: Vec<u8>,
}

thread_local! {
    pub static RESPONSE_HEADERS_SANTIZATION: Vec<&'static str> = vec![
        "x-mbx-uuid",
        "x-mbx-used-weight",
        "x-mbx-used-weight-1m",
        "Date",
        "Via",
        "X-Amz-Cf-Id",
    ];
}

#[derive(CandidType, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CaracterResponse {
  pub id: u64,
  pub name: String,
  pub status: String,
  pub species: String,
  pub r#type: String,
  pub gender: String,
  pub image: String,
}
/*
A function to call IC http_request function with a single minute range.
This function is to be triggered by timer as jobs move to the tip of the queue.
 */
pub async fn get(url: String) -> Result<CaracterResponse, String> {
  // prepare system http_request call
  let mut request_headers = vec![];
  request_headers.insert(
    0,
    HttpHeader {
      name: "Connection".to_string(),
      value: "keep-alive".to_string(),
    },
  );

  let request = CanisterHttpRequestArgs {
    url,
    http_method: HttpMethod::GET,
    body: None,
    transform_method_name: Some("sanitize_response".to_string()),
    headers: request_headers,
  };

  let body = candid::utils::encode_one(&request).unwrap();

  match ic_cdk::api::call::call_raw(
    Principal::management_canister(),
    "http_request",
    &body[..],
    0,
  )
  .await
  {
    Ok(result) => {
      // decode the result
      let decoded_result = candid::utils::decode_one(&result).unwrap();

      // put the result to hashmap
      ic_cdk::println!("{:?}", decoded_result);
      return Ok(decoded_result);
    }
    Err((r, m)) => {
      return Err(format!(
        "The http_request resulted into error. RejectionCode: {r:?}, Error: {m}"
      ));
    }
  }
}

#[query]
#[candid_method(query)]
#[export_name = "transform"]
async fn sanitize_response(raw: CanisterHttpResponsePayload) -> CanisterHttpResponsePayload {
  let mut sanitized = raw.clone();
  RESPONSE_HEADERS_SANTIZATION.with(|response_headers_blacklist| {
    let mut processed_headers = vec![];
    for header in raw.headers.iter() {
      if !response_headers_blacklist.contains(&header.name.as_str()) {
        processed_headers.insert(0, header.clone());
      }
    }
    sanitized.headers = processed_headers;
  });
  return sanitized;
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}
