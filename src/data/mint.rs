use crate::{data::Serialize, Instructions};

pub struct MintInstructionData {
    pub amount: u64,
    /// Required authorization data to validate the request.
    pub authorization_data: Option<AuthorizationData>,
}

pub struct AuthorizationData {}
impl Serialize for AuthorizationData {
    fn serialize_to(&self, _buffer: &mut [u8]) -> usize {
        panic!("Not implemented, did not feel like serializing a hashmap by hand");
    }
}

impl Serialize for MintInstructionData {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[0] = Instructions::Mint.to_u8();
        let mut offset = 1;

        // instruction data is actually and enum so write the first byte
        buffer[offset] = 0;
        offset += 1;

        offset += self.amount.serialize_to(&mut buffer[offset..]);
        offset += self.authorization_data.serialize_to(&mut buffer[offset..]);

        return offset;
    }
}
