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
    _Update = 50,
    Verify = 52,
}

pub const MPL_TOKEN_METADATA_ID: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
