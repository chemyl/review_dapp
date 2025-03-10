use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ReviewPayload {
    pub tittle: String,
    pub rating: u8,
    pub decription: String,
}
