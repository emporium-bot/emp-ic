use ic_kit::candid::{CandidType, Deserialize, Int, Nat, Principal};

// BEGIN DIP721v2 //

#[derive(CandidType, Clone, Deserialize, Debug)]
pub enum GenericValue {
    BoolContent(bool),
    TextContent(String),
    BlobContent(Vec<u8>),
    Principal(Principal),
    Nat8Content(u8),
    Nat16Content(u16),
    Nat32Content(u32),
    Nat64Content(u64),
    NatContent(Nat),
    Int8Content(i8),
    Int16Content(i16),
    Int32Content(i32),
    Int64Content(i64),
    IntContent(Int),
    FloatContent(f64), // motoko only support f64
    NestedContent(Vec<(String, GenericValue)>),
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct TokenMetadata {
    pub token_identifier: Nat,
    pub owner: Option<Principal>,
    pub operator: Option<Principal>,
    pub is_burned: bool,
    pub properties: Vec<(String, GenericValue)>,
    pub minted_at: u64,
    pub minted_by: Principal,
    pub transferred_at: Option<u64>,
    pub transferred_by: Option<Principal>,
    pub approved_at: Option<u64>,
    pub approved_by: Option<Principal>,
    pub burned_at: Option<u64>,
    pub burned_by: Option<Principal>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum NftError {
    UnauthorizedOwner,
    UnauthorizedOperator,
    OwnerNotFound,
    OperatorNotFound,
    TokenNotFound,
    ExistedNFT,
    SelfApprove,
    SelfTransfer,
    TxNotFound,
    Other(String),
}

// END DIP721v2 //

// BEGIN DIP20 //

#[derive(CandidType, Debug, Deserialize, Eq, PartialEq)]
pub enum TxError {
    InsufficientAllowance,
    InsufficientBalance,
    ErrorOperationStyle,
    Unauthorized,
    LedgerTrap,
    ErrorTo,
    Other,
    BlockUsed,
    AmountTooSmall,
}
pub type TxReceipt = Result<Nat, TxError>;

// END DIP20 //
