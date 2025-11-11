#![no_std]

use pinocchio::pubkey::Pubkey;
use pinocchio_pubkey::pubkey;

pub mod data;
pub mod instructions;

/// For internal use, to get the discriminant of the instruction
#[repr(u8)]
pub(crate) enum Instructions {
    Burn = 41,
    Create = 42,
    Mint = 43,
    Transfer = 49,
    // Update = 50,
    Verify = 52,
}

impl Instructions {
    pub fn to_u8(self) -> u8 {
        match self {
            Self::Burn => 41,
            Self::Create => 42,
            Self::Mint => 43,
            Self::Transfer => 49,
            Self::Verify => 52,
        }
    }
}

pub const MPL_TOKEN_METADATA_ID: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
