use ic_kit::{
  candid::{CandidType, Deserialize},
  Principal,
};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Deserialize, CandidType)]
pub struct DailyData {
  pub last_timestamp: u64,
  pub streak: u64,
}

#[derive(Clone, Deserialize, CandidType)]
pub struct WorkData {
  pub last_timestamp: u64,
  pub streak: u64,
}

#[derive(Clone, Deserialize, CandidType)]
pub struct User {
  pub discord_id: String,
  pub principal: Principal,
  pub daily: DailyData,
  pub work: WorkData,
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
}

pub fn _with<T, F: FnOnce(&Ledger) -> T>(f: F) -> T {
  LEDGER.with(|ledger| f(&ledger.borrow()))
}

pub fn with_mut<T, F: FnOnce(&mut Ledger) -> T>(f: F) -> T {
  LEDGER.with(|ledger| f(&mut ledger.borrow_mut()))
}
