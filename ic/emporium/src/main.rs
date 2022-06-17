use compile_time_run::run_command_str;
use ic_kit::{
  candid::{candid_method, CandidType, Deserialize},
  ic,
  interfaces::{
    management::{CanisterStatus, WithCanisterId},
    Method,
  },
  macros::*,
  Principal,
};

mod http;
mod ledger;
mod token_proxy;

use crate::http::*;

#[derive(Clone, Deserialize, CandidType)]
pub struct InitArgs {
  pub ft_canister: Principal,
  pub nft_canister: Principal,
}

#[init]
fn init(args: Option<InitArgs>) {
  // let args = args.unwrap();
  // ledger::with_mut(|ledger| {
  //   ledger.ft_canister = Some(args.ft_canister);
  //   ledger.nft_canister = Some(args.nft_canister);
  // });
}

#[update]
#[candid_method]
async fn test_get() -> String {
  format!(
    "{:?}",
    http::get("https://rickandmortyapi.com/api/character/1".to_string()).await
  )
}

/// Register daily submission for user, requires registration
#[update]
#[candid_method]
fn daily(discord_user: String) -> Result<(), String> {
  ledger::with_mut(|data| {
    let mut user = data
      .users
      .get_mut(&discord_user)
      .ok_or("Unregistered user")?;

    let now = ic::time();

    // Check if the user has already submitted within 18 hours
    // If so, reject the request
    if now - user.daily.last_timestamp < 18 * 60 * 60 {
      return Err("Already submitted today".to_string());
    }

    // reset streak if last was over 24 hrs
    if now - user.daily.last_timestamp > 24 * 60 * 60 {
      user.daily.streak = 0;
    }

    // Update the user's daily data
    user.daily.last_timestamp = now;
    user.daily.streak += 1;

    // Update the user's coins
    // user gets exponentially increasing amounts the longer the streak
    // TODO: Define a more gradual increase
    user.coins += user.daily.streak.pow(2);

    // TODO: initiate transfer OR mint for user

    Ok(())
  })
}

// Canister Stuff

/// Check if a given principal is included in the current canister controller list
///
/// To let the canister call the `aaaaa-aa` Management API `canister_status`,
/// the canister needs to be a controller of itself.
async fn _is_controller() -> Result<(), String> {
  let caller = ic::caller();
  let self_id = ic::id();

  let status = CanisterStatus::perform(
    Principal::management_canister(),
    (WithCanisterId {
      canister_id: ic::id(),
    },),
  )
  .await
  .map(|(status,)| Ok(status))
  .unwrap_or_else(|(code, message)| Err(format!("Code: {:?}, Message: {}", code, message)))?;

  match status.settings.controllers.contains(&caller) {
    true => Ok(()),
    false => Err(format!("{} is not a controller of {}", caller, self_id)),
  }
}

#[query(name = "gitCommitHash")]
#[candid_method(query, rename = "gitCommitHash")]
fn git_commit_hash() -> &'static str {
  run_command_str!("git", "rev-parse", "HEAD")
}

#[query(name = "rustToolchainInfo")]
#[candid_method(query, rename = "rustToolchainInfo")]
fn rust_toolchain_info() -> &'static str {
  run_command_str!("rustup", "show")
}

#[query(name = "dfxInfo")]
#[candid_method(query, rename = "dfxInfo")]
fn dfx_info() -> &'static str {
  run_command_str!("dfx", "--version")
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
  std::print!("{}", export_candid());
}

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
  candid::export_service!();
  __export_service()
}
