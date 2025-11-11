use crate::{data::Serialize, Instructions};

pub enum VerifyInstructionData {
    CreatorV1,
    CollectionV1,
}

impl Serialize for VerifyInstructionData {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[0] = Instructions::Verify.to_u8();
        let offset = 1;

        // instruction data is actually and enum so write the first byte
        buffer[offset] = match self {
            Self::CreatorV1 => 0,
            Self::CollectionV1 => 1,
        };

        return offset;
    }
}
