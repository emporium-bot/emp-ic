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

#[derive(Clone, Deserialize, CandidType)]
pub struct User {
    pub discord_id: String,
    pub principal: Principal,
    pub daily: StreakData,
    pub work: StreakData,
    pub coins: u64,
}

#[derive(Clone, Deserialize, CandidType)]
pub struct Ledger {
    pub total_users: u64,
    pub ft_canister: Option<Principal>,
    pub nft_canister: Option<Principal>,
    pub users: HashMap<String, User>,
}

thread_local! {
  static LEDGER: RefCell<Ledger> = RefCell::new(Ledger {
    total_users: 0,
    ft_canister: None,
    nft_canister: None,
    users: HashMap::new(),
  });
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
