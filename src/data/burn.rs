use crate::{data::Serialize, Instructions};

pub struct BurnInstructionData {
    /// The amount of the token to burn
    pub amount: u64,
}

impl Serialize for BurnInstructionData {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[0] = Instructions::Burn as u8;
        let mut offset = 1;

        offset += self.amount.serialize_to(&mut buffer[offset..]);

        return offset;
    }
}
