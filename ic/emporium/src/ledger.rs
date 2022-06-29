use derive_new::new;
use ic_kit::{
    candid::{CandidType, Deserialize},
    ic, Principal,
};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Deserialize, CandidType)]
pub struct StreakData {
    pub last_timestamp: u64,
    pub streak: u64,
}

impl StreakData {
    pub fn new() -> Self {
        Self {
            last_timestamp: 0,
            streak: 1,
        }
    }
}

#[derive(Clone, Deserialize, CandidType)]
pub struct User {
    pub discord_id: String,
    pub principal: Principal,
    pub daily: StreakData,
    pub work: StreakData,
    pub total_rewards: u64,
}

impl User {
    pub fn new(discord_id: String, principal: Principal) -> Self {
        Self {
            discord_id,
            principal,
            daily: StreakData::new(),
            work: StreakData::new(),
            total_rewards: 0,
        }
    }
}

#[derive(Clone, Deserialize, CandidType, new)]
pub struct Ledger {
    pub total_users: u64,
    pub nft_canister: Option<Principal>,
    pub users: HashMap<String, User>,
}

thread_local! {
  static LEDGER: RefCell<Ledger> = RefCell::new(Ledger::new(
    0,
    None,
    HashMap::new(),
  ));
  static CUSTODIANS: RefCell<Vec<Principal>> = RefCell::new(Vec::new());
}

// TODO: use controllers for ownership
// this will require the canister to be a controller of itself (like dip721)
pub fn _is_auth() -> Result<(), String> {
    CUSTODIANS.with(|c| {
        let custodians = c.borrow();
        if custodians.contains(&ic::caller()) {
            Ok(())
        } else {
            Err("Error: Unauthorized principal ID".to_string())
        }
    })
}

pub fn custodians_mut<T, F: FnOnce(&mut Vec<Principal>) -> T>(f: F) -> T {
    CUSTODIANS.with(|custodians| f(&mut custodians.borrow_mut()))
}

pub fn with<T, F: FnOnce(&Ledger) -> T>(f: F) -> T {
    LEDGER.with(|ledger| f(&ledger.borrow()))
}

pub fn with_mut<T, F: FnOnce(&mut Ledger) -> T>(f: F) -> T {
    LEDGER.with(|ledger| f(&mut ledger.borrow_mut()))
}
