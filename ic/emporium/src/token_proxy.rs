use ic_kit::{
    candid::{CandidType, Deserialize, Int, Nat, Principal},
    ic, RejectionCode,
};

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

pub(crate) struct _DIP721v2Proxy {}

impl _DIP721v2Proxy {
    // Update Methods

    pub async fn _transfer_from(
        from: &Principal,
        to: &Principal,
        token_id: &Nat,
        contract: &Principal,
    ) -> Result<Nat, String> {
        let call_res: Result<(Result<Nat, NftError>,), (RejectionCode, String)> = ic::call(
            *contract,
            "transferFrom",
            (*from, *to, Nat::from(token_id.clone())),
        )
        .await;

        call_res
            .map_err(|err| format!("{:?}", err))?
            .0
            .map_err(|err| format!("{:?}", err))
    }

    pub async fn _transfer(
        contract: &Principal,
        to: &Principal,
        token_id: &Nat,
    ) -> Result<Nat, String> {
        let call_res: Result<(Result<Nat, NftError>,), (RejectionCode, String)> =
            ic::call(*contract, "transfer", (*to, Nat::from(token_id.clone()))).await;

        let res = call_res
            .map_err(|err| format!("{:?}", err))?
            .0
            .map_err(|err| format!("{:?}", err));

        match &res {
            Ok(val) => Ok(val.clone()),
            _ => Err("transfer error".to_string()),
        }
    }

    // Query Methods

    pub async fn _token_metadata(
        token_id: &Nat,
        contract: &Principal,
    ) -> Result<TokenMetadata, String> {
        let call_res: Result<(Result<TokenMetadata, NftError>,), (RejectionCode, String)> =
            ic::call(*contract, "tokenMetadata", (Nat::from(token_id.clone()),)).await;

        call_res
            .map_err(|err| format!("{:?}", err))?
            .0
            .map_err(|err| format!("{:?}", err))
    }

    pub async fn _owner_of(
        contract: &Principal,
        token_id: &Nat,
    ) -> Result<Option<Principal>, String> {
        let call_res: Result<(Result<Option<Principal>, NftError>,), (RejectionCode, String)> =
            ic::call(*contract, "ownerOf", (Nat::from(token_id.clone()),)).await;

        call_res
            .map_err(|err| format!("{:?}", err))?
            .0
            .map_err(|err| format!("{:?}", err))
    }

    pub async fn _operator_of(
        contract: &Principal,
        token_id: &Nat,
    ) -> Result<Option<Principal>, String> {
        let call_res: Result<(Result<Option<Principal>, NftError>,), (RejectionCode, String)> =
            ic::call(*contract, "operatorOf", (Nat::from(token_id.clone()),)).await;

        call_res
            .map_err(|err| format!("{:?}", err))?
            .0
            .map_err(|err| format!("{:?}", err))
    }
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

pub(crate) struct _Dip20Proxy {}

impl _Dip20Proxy {
    // Update Methods

    pub async fn _transfer_from(
        from: &Principal,
        to: &Principal,
        amount: &Nat,
        contract: &Principal,
    ) -> Result<Nat, String> {
        let call_res: Result<(TxReceipt,), (RejectionCode, String)> =
            ic::call(*contract, "transferFrom", (*from, *to, amount.clone())).await;

        call_res
            .map_err(|err| format!("{:?}", err))?
            .0
            .map_err(|err| format!("{:?}", err))
    }

    pub async fn _transfer(
        to: &Principal,
        amount: &Nat,
        contract: &Principal,
    ) -> Result<Nat, String> {
        let call_res: Result<(TxReceipt,), (RejectionCode, String)> =
            ic::call(*contract, "transfer", (*to, amount.clone())).await;
        call_res
            .map_err(|err| format!("{:?}", err))?
            .0
            .map_err(|err| format!("{:?}", err))
    }

    // Query Methods

    pub async fn _balance_of(contract: &Principal, owner: &Principal) -> Result<Nat, String> {
        let call_res: Result<(Nat,), (RejectionCode, String)> =
            ic::call(*contract, "balanceOf", (*owner,)).await;
        call_res
            .map_err(|err| format!("{:?}", err))
            .map(|res| res.0)
    }

    pub async fn _allowance(
        contract: &Principal,
        owner: &Principal,
        spender: &Principal,
    ) -> Result<Nat, String> {
        let call_res: Result<(Nat,), (RejectionCode, String)> =
            ic::call(*contract, "allowance", (*owner, *spender)).await;
        call_res
            .map_err(|err| format!("{:?}", err))
            .map(|res| res.0)
    }
}

// END DIP20 //
