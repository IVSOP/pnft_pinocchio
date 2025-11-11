use bytemuck::{Pod, Zeroable};
use pinocchio::pubkey::Pubkey;

use crate::{
    data::{nft::Creator, Serialize},
    Instructions,
};

#[repr(u8)]
pub enum DataState {
    AccountState,
    LedgerState,
}

impl Serialize for DataState {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[0] = match self {
            Self::AccountState => 0,
            Self::LedgerState => 1,
        };
        return 1;
    }
}

pub enum PrintSupply {
    /// The asset does not have any prints.
    Zero,
    /// The asset has a limited amount of prints.
    Limited(u64),
    /// The asset has an unlimited amount of prints.
    Unlimited,
}

impl Serialize for PrintSupply {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        match self {
            Self::Zero => {
                buffer[0] = 0;
                1
            }
            Self::Limited(num) => {
                buffer[0] = 1;
                num.serialize_to(&mut buffer[1..1 + size_of::<u64>()]);
                1 + size_of::<u64>()
            }
            Self::Unlimited => {
                buffer[0] = 2;
                1
            }
        }
    }
}

pub enum TokenStandard {
    NonFungible,                    // This is a master edition
    FungibleAsset,                  // A token with metadata that can also have attributes
    Fungible,                       // A token with simple metadata
    NonFungibleEdition,             // This is a limited edition
    ProgrammableNonFungible,        // NonFungible with programmable configuration
    ProgrammableNonFungibleEdition, // NonFungible with programmable configuration
}

impl Serialize for TokenStandard {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        match self {
            Self::NonFungible => {
                buffer[0] = 0;
                1
            }
            Self::FungibleAsset => {
                buffer[0] = 1;
                1
            }
            Self::Fungible => {
                buffer[0] = 2;
                1
            }
            Self::NonFungibleEdition => {
                buffer[0] = 3;
                1
            }
            Self::ProgrammableNonFungible => {
                buffer[0] = 4;
                1
            }
            Self::ProgrammableNonFungibleEdition => {
                buffer[0] = 5;
                1
            }
        }
    }
}

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct Collection {
    pub verified: u8, // this is a bool
    pub key: Pubkey,
}

impl Serialize for Collection {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = 0;
        offset += self.verified.serialize_to(buffer);
        offset += self.key.serialize_to(&mut buffer[offset..]);
        offset
    }
}

pub struct Uses {
    // 17 bytes + Option byte
    pub use_method: UseMethod, //1
    pub remaining: u64,        //8
    pub total: u64,            //8
}

impl Serialize for Uses {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = 0;
        offset += self.use_method.serialize_to(buffer);
        offset += self.remaining.serialize_to(&mut buffer[offset..]);
        offset += self.total.serialize_to(&mut buffer[offset..]);
        offset
    }
}

pub enum UseMethod {
    Burn,
    Multiple,
    Single,
}

impl Serialize for UseMethod {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        match self {
            Self::Burn => {
                buffer[0] = 0;
                1
            }
            Self::Multiple => {
                buffer[0] = 1;
                1
            }
            Self::Single => {
                buffer[0] = 2;
                1
            }
        }
    }
}

pub struct AssetData<'a> {
    /// The name of the asset.
    pub name: &'a [u8],
    /// The symbol for the asset.
    pub symbol: &'a [u8],
    /// URI pointing to JSON representing the asset.
    pub uri: &'a [u8],
    /// Royalty basis points that goes to creators in secondary sales (0-10000).
    pub seller_fee_basis_points: u16,
    /// Array of creators.
    pub creators: Option<&'a [Creator]>,
    // Immutable, once flipped, all sales of this metadata are considered secondary.
    pub primary_sale_happened: bool,
    // Whether or not the data struct is mutable (default is not).
    pub is_mutable: bool,
    /// Type of the token.
    pub token_standard: TokenStandard,
    /// Collection information.
    pub collection: Option<Collection>,
    /// Uses information.
    pub uses: Option<Uses>,
    /// Collection item details.
    pub collection_details: Option<CollectionDetails>,
    /// Programmable rule set for the asset.
    pub rule_set: Option<Pubkey>,
}

impl<'a> Serialize for AssetData<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = 0;

        offset += self.name.serialize_to(&mut buffer[offset..]);
        offset += self.symbol.serialize_to(&mut buffer[offset..]);
        offset += self.uri.serialize_to(&mut buffer[offset..]);
        offset += self
            .seller_fee_basis_points
            .serialize_to(&mut buffer[offset..]);
        offset += self.creators.serialize_to(&mut buffer[offset..]);
        offset += self
            .primary_sale_happened
            .serialize_to(&mut buffer[offset..]);
        offset += self.is_mutable.serialize_to(&mut buffer[offset..]);
        offset += self.token_standard.serialize_to(&mut buffer[offset..]);
        offset += self.collection.serialize_to(&mut buffer[offset..]);
        offset += self.uses.serialize_to(&mut buffer[offset..]);
        offset += self.collection_details.serialize_to(&mut buffer[offset..]);
        offset += self.rule_set.serialize_to(&mut buffer[offset..]);

        return offset;
    }
}

pub enum CollectionDetails {
    V1 { size: u64 },
    V2 { padding: [u8; 8] },
}

impl Serialize for CollectionDetails {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        match self {
            Self::V1 { size } => {
                buffer[0] = 0;
                size.serialize_to(&mut buffer[1..1 + size_of::<u64>()]);
                1 + size_of::<u64>()
            }
            // I assume padding is useless
            Self::V2 { padding: _ } => {
                buffer[1] = 0;
                1 + 8
            }
        }
    }
}

pub struct CreateAssetInstructionData<'a> {
    asset_data: AssetData<'a>,
    decimals: Option<u8>,
    print_supply: Option<PrintSupply>,
}

impl<'a> Serialize for CreateAssetInstructionData<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[0] = Instructions::Create.to_u8();
        let mut offset = 1;

        // instruction data is actually and enum so write the first byte
        buffer[offset] = 0;
        offset += 1;

        offset += self.asset_data.serialize_to(&mut buffer[offset..]);
        offset += self.decimals.serialize_to(&mut buffer[offset..]);
        offset += self.print_supply.serialize_to(&mut buffer[offset..]);

        return offset;
    }
}
