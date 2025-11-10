// I have absolutely no idea why, but the strings have a fixed size
// nothing in the code allows me to conclude this
// but their pnft diagram thing does show it
// these are all the constants I could find:

use bytemuck::{Pod, Zeroable, try_cast_slice};
use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

use crate::data::{DeserializeSized, Serialize, create::Collection};

/// Maximum number of characters in a metadata name.
pub const MAX_NAME_LENGTH: usize = 32;

/// Maximum number of characters in a metadata symbol.
pub const MAX_SYMBOL_LENGTH: usize = 10;

/// Maximum number of characters in a metadata uri.
pub const MAX_URI_LENGTH: usize = 200;

/// Maximum number of creators in a metadata.
pub const MAX_CREATOR_LIMIT: usize = 5;

/// Maximum number of bytes used by a creator data.
pub const MAX_CREATOR_LEN: usize = 32 + 1 + 1;

/// Maximum number of bytes used by a edition marker.
pub const MAX_EDITION_MARKER_SIZE: usize = 32;

/// Number of bits used by a edition marker.
pub const EDITION_MARKER_BIT_SIZE: u64 = 248;

#[derive(Pod, Zeroable, Copy, Clone)]
#[repr(C)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: u8, // this is a bool,
    pub share: u8,
}

impl Serialize for Creator {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = 0;
        offset += self.address.serialize_to(&mut buffer[offset..]);
        offset += self.verified.serialize_to(&mut buffer[offset..]);
        offset += self.share.serialize_to(&mut buffer[offset..]);
        offset
    }
}

pub struct Info<'a> {
    pub basis_points: u16,
    pub creators: &'a [Creator],
    pub collection: Option<&'a Collection>,
}

// For now, all I need is to be able to deserialize royalties and nothing else
// Since I know the exact length of all fields before the royalties, I can very conveniently just skip all of the bytes
// Just like in the mpl core lib, the metaplex gods have bestowed upon me a struct with no alignment needs, so I can just
// zero copy the whole thing
pub fn read_royalties_and_collection<'a>(
    bytes: &'a [u8],
) -> Result<Info<'a>, ProgramError> {
    // I can skip everything but I'm just going to check that the Key is correct
    if bytes[0] != 4 {
        return Err(ProgramError::InvalidAccountData);
    }

    // see the diagram https://github.com/metaplex-foundation/mpl-token-metadata/blob/main/programs/token-metadata/program/ProgrammableNFTGuide.md
    // it already has sizes. note that name has 4 bytes for the length + 200 for the actual string. this is absolutely completely retarded, btw, they are wasting space just because. who the fuck designed this?
    const BYTES_TO_SKIP: usize = 319;
    let mut offset = BYTES_TO_SKIP;

    // next two bytes are the basis points
    let basis_points = u16::deserialize(&bytes[offset..])?;
    offset += size_of::<u16>();

    // then an Option<Vec<Creator>>
    // read the option discriminator
    let option_disc = bytes[offset];
    offset += 1;

    let creators = match option_disc {
        0 => {
            &[]
        },
        _ => {
            // read the len
            let num_creators = usize::try_from(u32::deserialize(&bytes[offset..])?)
                .map_err(|_| ProgramError::ArithmeticOverflow)?;
            offset += size_of::<u32>();
            let creators_start = offset;
            let creators_end = creators_start + (num_creators * size_of::<Creator>());
        
            // read the creators
            let creators: &[Creator] = try_cast_slice(&bytes[creators_start..creators_end])
                .map_err(|_| ProgramError::InvalidAccountData)?;
            offset += creators_end;

            creators
        }
    };

    // skip over some more stuff
    let _primary_sale_happened = bytes[offset];
    offset += 1;
    let _is_mutable = bytes[offset];
    offset += 1;

    // Option<TokenStandard>
    match bytes[offset] {
        0 => {
            offset += 1;
        },
        _ => {
            offset += 2;
        }
    }

    // collection is an Option<Collection>
    // the collection also has no alignment needs, so just zero copy the entire thing
    let collection = match bytes[offset] {
        0 => {
            None
        },
        _ => {
            offset += 1;
            Some(bytemuck::from_bytes(&bytes[offset..offset + size_of::<Collection>()]))
        }
    };

    Ok(Info {
        basis_points,
        creators,
        collection
    })
}
