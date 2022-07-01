use crate::dip20::*;
use cap_sdk::{archive, from_archive, Archive};
use chrono::{Duration, TimeZone, Utc};
use compile_time_run::run_command_str;
use ic_cdk::export::Principal;
use ic_kit::{
    candid::{candid_method, CandidType, Deserialize, Nat},
    ic,
    macros::*,
};
use regex::Regex;
use std::convert::TryInto;

// mod http;
mod dip20;
mod ledger;
mod token_proxy;

const ONE_HOUR: u64 = 3_600_000_000_000;
const ONE_MINUTE: u64 = 60_000_000_000;
const BASE_REWARD: u64 = 100;

// Discord emojis
const FIRE_EMOJI: &str = "<a:Fire:940749752549650432>";
const AYY_EMOJI: &str = "<:ayy:305818615712579584>";

// #[update]
// #[candid_method]
// async fn test_get() -> String {
//   format!(
//     "{:?}",
//     http::get("https://rickandmortyapi.com/api/character/1".to_string()).await
//   )
// }

// BEGIN QUERY METHODS //

#[derive(Clone, Deserialize, CandidType)]
struct BalanceResponse {
    discord_id: String,
    balance: Nat,
    total_rewards: Nat,
    daily_streak: Nat,
    work_streak: Nat,
}

#[query]
#[candid_method]
fn user_balance(discord_id: String) -> Result<BalanceResponse, String> {
    let user: ledger::User =
        ledger::with(|ledger| ledger.users.get(&discord_id).cloned()).ok_or("User not found :(")?;
    Ok(BalanceResponse {
        discord_id: format!("<@{}>", user.discord_id.clone()),
        balance: balance_of(user.principal),
        total_rewards: Nat::from(user.total_rewards),
        daily_streak: Nat::from(user.daily.streak),
        work_streak: Nat::from(user.work.streak),
    })
}

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
///
/// Users can only submit once per day, minumum 20 hours after previous submission,
/// with the streak running out after 28 hrs.
#[update]
#[candid_method]
async fn daily(discord_user: String) -> Result<String, String> {
    let res = ledger::with_mut(|data| {
        let mut user = data
            .users
            .get_mut(&discord_user)
            .ok_or("Unregistered user")?;

        let time = ic::time();

        let now = Utc.timestamp_nanos(time.try_into().unwrap());
        let last = Utc
            .timestamp_nanos(user.daily.last_timestamp.try_into().unwrap())
            .date()
            .and_hms(0, 0, 0);

        ic::print(format!("{}\n{}\n", last, now));

        let duration = now - last;

        // if were within the last day (UTC), reject the user
        if duration.num_days() == 0 {
            // calc offset until 00:00 tomorrow
            let offset = (now.date().and_hms(0, 0, 0) + Duration::days(1)) - now;

            return Err(format!(
                "<@{}>, daily rewards already claimed! Try again in {} hours, {} minutes, {}s",
                discord_user,
                offset.num_hours(),
                offset.num_minutes() % 60,
                offset.num_seconds() % 60
            ));
        }

        // reset streak if last is more than a day old (this is super lenient for the streak)
        if duration.num_days() > 1 {
            user.daily.streak = 0;
        }

        // // user gets exponentially increasing amounts the longer the streak
        // // TODO: Define a more gradual increase, and taper off at a price
        let bonus = user.daily.streak.pow(2);
        user.total_rewards += BASE_REWARD + bonus;
        user.daily.streak += 1;
        user.daily.last_timestamp = time;

        Ok((user.principal, bonus, user.daily.streak))
    });

    match res {
        Ok((principal, bonus, streak)) => {
            dip20::mint(principal, Nat::from(BASE_REWARD + bonus))
                .await
                .map_err(|e| format!("{:?}", e))?;
            Ok(format!(
                "<@{}>, claimed `{} EMP` daily rewards{}",
                discord_user,
                BASE_REWARD,
                if bonus > 0 {
                    format!(
                        ", plus `{} EMP` streak bonus for {} days {}",
                        bonus, streak, FIRE_EMOJI
                    )
                } else {
                    format!(" {}", AYY_EMOJI)
                },
            ))
        }
        Err(e) => Err(e),
    }
}

/// Register work submission for user, requires registration
///
/// Users can only submit once every hour, with the streak running out after 2 hours
#[update]
#[candid_method]
async fn work(discord_user: String) -> Result<String, String> {
    let res = ledger::with_mut(|data| {
        let mut user = data
            .users
            .get_mut(&discord_user)
            .ok_or("Unregistered user")?;

        let now = ic::time();
        let difference = now - user.work.last_timestamp;

        // check if user has submitted in the last ONE_HOUR
        if difference < ONE_HOUR {
            return Err(format!(
                "<@{}>, you are already working! try again in {} minutes.",
                discord_user,
                (ONE_HOUR - difference) / ONE_MINUTE
            ));
        }

        // update the user's work streak
        // reset streak if last was over 2 hrs
        if now - user.work.last_timestamp > 2 * ONE_HOUR {
            user.work.streak = 0;
        }
        // user gets exponentially increasing amounts the longer the streak
        // TODO: Define a more gradual increase, and taper off at a price
        let amount = 100 + user.work.streak.pow(2);
        user.total_rewards += amount;
        user.work.streak += 1;
        user.work.last_timestamp = now;

        Ok((user.principal, Nat::from(amount)))
    });

    match res {
        Ok((principal, amount)) => {
            dip20::mint(principal, amount.clone())
                .await
                .map_err(|e| format!("{:?}", e))?;
            Ok(format!(
                "<@{}>, is working! Rewards: `{} EMP`",
                discord_user, amount
            ))
        }
        Err(e) => Err(e),
    }
}

/// Register a new user
#[update]
#[candid_method]
fn register(discord_user: String) -> Result<String, String> {
    // regex check for valid discord username
    let re = Regex::new(r"^\d{17,18}$").unwrap();
    if !re.is_match(&discord_user) {
        return Err("Invalid discord unique id".to_string());
    }

    ledger::with_mut(|data| {
        let caller = ic::caller();
        if data.users.contains_key(&discord_user.clone()) {
            return Err("User already registered".to_string());
        }

        data.users.insert(
            discord_user.clone(),
            ledger::User::new(discord_user.clone(), caller),
        );
        data.total_users += 1;

        Ok(format!(
            "<@{}>, registered principal id: `{:}`",
            discord_user, caller
        ))
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
            .ok_or("User not registered")?;

        // Check if the user is the caller
        if user.principal != ic::caller() {
            return Err("Not authorized to change this principal".to_string());
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
    pub custodians: Option<Vec<Principal>>,
}

#[init]
#[candid_method(init)]
fn init(args: Option<InitArgs>) {
    let args = args.unwrap();
    ledger::with_mut(|ledger| {
        ledger.nft_canister = args.nft_canister;
        cap_sdk::handshake(1_000_000_000_000, args.cap_canister);
    });

    ledger::custodians_mut(|custodians| match args.custodians {
        Some(c) => {
            custodians.extend(c);
        }
        None => {
            custodians.push(ic::caller());
        }
    })
}

#[pre_upgrade]
fn pre_upgrade() {
    let ledger_clone = ledger::with(|ledger| ledger.clone());
    let custodians = ledger::custodians_mut(|custodians| custodians.clone());
    let stats = STATS.with(|s| s.borrow().clone());
    let balances = BALANCES.with(|b| b.borrow().clone());
    let allows = ALLOWS.with(|a| a.borrow().clone());
    let tx_log = TXLOG.with(|t| t.borrow().clone());
    let cap = archive();
    ic::stable_store((
        ledger_clone,
        custodians,
        stats,
        balances,
        allows,
        tx_log,
        cap,
    ))
    .unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (
        ledger_stored,
        custodians_stored,
        metadata_stored,
        balances_stored,
        allowances_stored,
        tx_log_stored,
        cap,
    ): (
        ledger::Ledger,
        Vec<Principal>,
        StatsData,
        Balances,
        Allowances,
        TxLog,
        Archive,
    ) = ic::stable_restore().unwrap();
    ledger::with_mut(|ledger| {
        *ledger = ledger_stored;
    });
    ledger::custodians_mut(|custodians| {
        *custodians = custodians_stored.clone();
    });
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
    from_archive(cap);
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
