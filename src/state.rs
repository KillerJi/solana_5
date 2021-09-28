use borsh::{BorshDeserialize, BorshSerialize};
// use serde::{Deserialize, Serialize};

#[derive( BorshSerialize, BorshDeserialize, Debug )]
pub struct Greeting {
    pub counter: u32,
}
