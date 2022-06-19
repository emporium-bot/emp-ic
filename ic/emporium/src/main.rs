use crate::dip20::*;
use cap_sdk::handshake;
use compile_time_run::run_command_str;
use ic_kit::{
  candid::{candid_method, CandidType, Deserialize, Nat},
  ic,
  interfaces::{
    management::{CanisterStatus, WithCanisterId},
    Method,
  },
  macros::*,
  Principal,
};

// mod http;
mod dip20;
mod ledger;
mod token_proxy;

const ONE_HOUR: u64 = 3_600_000_000;

// #[update]
// #[candid_method]
// async fn test_get() -> String {
//   format!(
//     "{:?}",
//     http::get("https://rickandmortyapi.com/api/character/1".to_string()).await
//   )
// }

// BEGIN QUERY METHODS //

#[query]
#[candid_method]
fn get_user(discord_id: String) -> Option<ledger::User> {
  ledger::with(|ledger| ledger.users.get(&discord_id).cloned())
}

#[query]
#[candid_method]
fn get_users() -> Vec<ledger::User> {
  ledger::with(|ledger| ledger.users.values().cloned().collect())
}

// END QUERY METHODS //

// BEGIN USER METHODS //

/// Register daily submission for user, requires registration
#[update]
#[candid_method]
fn daily(discord_user: String) -> Result<String, String> {
  ledger::with_mut(|data| {
    let mut user = data
      .users
      .get_mut(&discord_user)
      .ok_or("Unregistered user")?;

    let now = ic::time();

    // Check if the user has already submitted within 18 hours
    // If so, reject the request
    if now - user.daily.last_timestamp < 18 * ONE_HOUR {
      return Err("Already submitted today".to_string());
    }

    // Update the user's daily streak
    // reset streak if last was over 24 hrs
    if now - user.daily.last_timestamp > 24 * ONE_HOUR {
      user.daily.streak = 0;
    } else {
      user.daily.streak += 1;
    }
    user.daily.last_timestamp = now;

    // Update the user's coins
    // user gets exponentially increasing amounts the longer the streak
    // TODO: Define a more gradual increase
    user.coins += user.daily.streak.pow(2);

    // TODO: initiate transfer OR mint for user

    Ok("Daily submission successful".to_string())
  })
}

/// Register a new user
#[update]
#[candid_method]
fn register(discord_user: String) -> Result<(), String> {
  ledger::with_mut(|data| {
    if data.users.contains_key(&discord_user.clone()) {
      return Err("User already registered".to_string());
    }

    data.users.insert(
      discord_user.clone(),
      ledger::User {
        discord_id: discord_user.clone(),
        principal: ic::caller(),
        daily: ledger::StreakData {
          last_timestamp: 0,
          streak: 0,
        },
        work: ledger::StreakData {
          last_timestamp: 0,
          streak: 0,
        },
        coins: 0,
      },
    );
    data.total_users += 1;

    Ok(())
  })
}

// Modify registered users principal
#[update]
#[candid_method]
fn set_principal(discord_user: String, principal: Principal) -> Result<(), String> {
  ledger::with_mut(|data| {
    let mut user = data
      .users
      .get_mut(&discord_user)
      .ok_or("Unregistered user")?;

    // Check if the user is the caller
    if user.principal != ic::caller() {
      return Err("Not authorized".to_string());
    }

    user.principal = principal;

    Ok(())
  })
}

// END USER METHODS //

// BEGIN CANISTER SETUP //

#[derive(Clone, Deserialize, Debug, CandidType)]
pub struct InitArgs {
  pub cap_canister: Option<Principal>,
  pub nft_canister: Option<Principal>,
}

#[init]
#[candid_method(init)]
fn init(args: Option<InitArgs>) {
  ic::print(format!("{:?}", args));
  let args = args.unwrap();
  ledger::with_mut(|ledger| {
    ledger.nft_canister = args.nft_canister;
    handshake(1_000_000_000_000, args.cap_canister);
  });
}

#[pre_upgrade]
fn pre_upgrade() {
  let stats = STATS.with(|s| s.borrow().clone());
  let balances = BALANCES.with(|b| b.borrow().clone());
  let allows = ALLOWS.with(|a| a.borrow().clone());
  let tx_log = TXLOG.with(|t| t.borrow().clone());
  ic::stable_store((stats, balances, allows, tx_log)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
  let (metadata_stored, balances_stored, allowances_stored, tx_log_stored): (
    StatsData,
    Balances,
    Allowances,
    TxLog,
  ) = ic::stable_restore().unwrap();
  STATS.with(|s| {
    let mut stats = s.borrow_mut();
    *stats = metadata_stored;
  });
  BALANCES.with(|b| {
    let mut balances = b.borrow_mut();
    *balances = balances_stored;
  });
  ALLOWS.with(|a| {
    let mut allowances = a.borrow_mut();
    *allowances = allowances_stored;
  });
  TXLOG.with(|t| {
    let mut tx_log = t.borrow_mut();
    *tx_log = tx_log_stored;
  });
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

// END CANISTER SETUP //
