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

#[allow(dead_code)]
pub type NftResult = Result<(), NftError>;
#[allow(dead_code)]
pub type NftNatResult = Result<Nat, NftError>;
#[allow(dead_code)]
pub type NftPrincipalResult = Result<Option<Principal>, NftError>;
#[allow(dead_code)]
pub type NftMetadataResult = Result<TokenMetadata, NftError>;

pub(crate) struct _DIP721v2Proxy {}

impl _DIP721v2Proxy {
    // Update Methods

    pub async fn _transfer_from(
        from: &Principal,
        to: &Principal,
        token_id: &Nat,
        contract: &Principal,
    ) -> Result<Nat, String> {
        let call_res: Result<(NftNatResult,), (RejectionCode, String)> =
            ic::call(*contract, "transferFrom", (*from, *to, token_id.clone())).await;

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
        let call_res: Result<(NftNatResult,), (RejectionCode, String)> =
            ic::call(*contract, "transfer", (*to, token_id.clone())).await;

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
        let call_res: Result<(NftMetadataResult,), (RejectionCode, String)> =
            ic::call(*contract, "tokenMetadata", (token_id.clone(),)).await;

        call_res
            .map_err(|err| format!("{:?}", err))?
            .0
            .map_err(|err| format!("{:?}", err))
    }

    pub async fn _owner_of(
        contract: &Principal,
        token_id: &Nat,
    ) -> Result<Option<Principal>, String> {
        let call_res: Result<(NftPrincipalResult,), (RejectionCode, String)> =
            ic::call(*contract, "ownerOf", (token_id.clone(),)).await;

        call_res
            .map_err(|err| format!("{:?}", err))?
            .0
            .map_err(|err| format!("{:?}", err))
    }

    pub async fn _operator_of(
        contract: &Principal,
        token_id: &Nat,
    ) -> Result<Option<Principal>, String> {
        let call_res: Result<(NftPrincipalResult,), (RejectionCode, String)> =
            ic::call(*contract, "operatorOf", (token_id.clone(),)).await;

        call_res
            .map_err(|err| format!("{:?}", err))?
            .0
            .map_err(|err| format!("{:?}", err))
    }
}

// END DIP721v2 //
