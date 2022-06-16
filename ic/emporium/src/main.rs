use compile_time_run::{run_command, run_command_str};
use ic_kit::{
  candid::{candid_method, encode_args, CandidType, Deserialize, Nat},
  ic,
  interfaces::{
    management::{self, CanisterStatus, CanisterStatusResponse, WithCanisterId},
    Method,
  },
  macros::*,
  Principal, RejectionCode,
};

// cover metadata
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
