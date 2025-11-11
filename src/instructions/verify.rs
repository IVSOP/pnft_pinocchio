use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke_signed,
    instruction::{AccountMeta, Instruction, Signer},
    ProgramResult,
};

use crate::data::{verify::VerifyInstructionData, Serialize};

/// Transfer an asset
///
/// ### Accounts:
///   0. `[SIGNER]` Authority (account to be verified)
///   1. `[OPTIONAL]` Delegate Record
///   2. `[WRITE]` Metadata
///   3. `[OPTIONAL]` Collection Mint
///   4. `[OPTIONAL, WRITE]` Collection Metadata
///   5. `[OPTIONAL]` Collection Master Edition
///   6. `[]` System Program
///   7. `[]` Sysvar Instructions
///   8. `[]` MPL Token Metadata
///
/// Accounts being optional is very cursed but mimics the behaviour of the official lib.
/// Accounts set to None get replaced by mpl program's account.
pub struct Verify<'a> {
    pub authority: &'a AccountInfo,
    pub delegate_record: Option<&'a AccountInfo>,
    pub metadata: &'a AccountInfo,
    pub collection_mint: Option<&'a AccountInfo>,
    pub collection_metadata: Option<&'a AccountInfo>,
    pub collection_master_edition: Option<&'a AccountInfo>,
    pub system_program: &'a AccountInfo,
    pub sysvar_instructions: &'a AccountInfo,
    pub mpl_token_metadata: &'a AccountInfo,
}

impl Verify<'_> {
    #[inline(always)]
    pub fn invoke(
        &self,
        data: &VerifyInstructionData,
        serialization_buffer: &mut [u8],
    ) -> ProgramResult {
        self.invoke_signed(data, &[], serialization_buffer)
    }

    pub fn invoke_signed(
        &self,
        data: &VerifyInstructionData,
        signers: &[Signer],
        serialization_buffer: &mut [u8],
    ) -> ProgramResult {
        // account metadata
        let account_metas: &[AccountMeta] = &[
            AccountMeta::readonly_signer(self.authority.key()),
            match self.delegate_record {
                Some(delegate) => AccountMeta::readonly(delegate.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            AccountMeta::writable(self.metadata.key()),
            match self.collection_mint {
                Some(mint) => AccountMeta::readonly(mint.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            match self.collection_metadata {
                Some(meta) => AccountMeta::writable(meta.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            match self.collection_master_edition {
                Some(edition) => AccountMeta::readonly(edition.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            AccountMeta::readonly(self.system_program.key()),
            AccountMeta::readonly(self.sysvar_instructions.key()),
            AccountMeta::readonly(self.mpl_token_metadata.key()),
        ];

        let len = data.serialize_to(serialization_buffer);
        let data = &serialization_buffer[..len];

        let instruction = Instruction {
            program_id: &crate::MPL_TOKEN_METADATA_ID,
            accounts: &account_metas,
            data,
        };

        invoke_signed(
            &instruction,
            &[
                self.authority,
                self.delegate_record.unwrap_or(self.mpl_token_metadata),
                self.metadata,
                self.collection_mint.unwrap_or(self.mpl_token_metadata),
                self.collection_metadata.unwrap_or(self.mpl_token_metadata),
                self.collection_master_edition
                    .unwrap_or(self.mpl_token_metadata),
                self.system_program,
                self.sysvar_instructions,
                self.mpl_token_metadata,
            ],
            signers,
        )
    }
}
