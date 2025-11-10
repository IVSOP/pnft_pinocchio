use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke_signed,
    instruction::{AccountMeta, Instruction, Signer},
    ProgramResult,
};

use crate::data::{create::CreateAssetInstructionData, Serialize};

/// Create an asset
///
/// ### Accounts:
///   0. `[WRITE]` Metadata
///   1. `[WRITE, OPTIONAL]` Master Edition
///   2. `[WRITE]` Mint
///   3. `[SIGNER]` Authority
///   3. `[SIGNER, WRITE]` Payer
///   4. `[]` Update Authority
///   5. `[]` System Program
///   6. `[]` Sysvar Instructions
///   7. `[OPTIONAL]` SPL Token Program
///   8. `[]` MPL Metadata Program
///
/// Accounts being optional is very cursed but mimics the behaviour of the official lib.
/// Accounts set to None get replaced by mpl program's account.
pub struct CreateAsset<'a> {
    pub metadata: &'a AccountInfo,
    pub master_edition: Option<&'a AccountInfo>,
    pub mint: &'a AccountInfo,
    pub authority: &'a AccountInfo,
    pub payer: &'a AccountInfo,
    pub update_authority: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub sysvar_instructions: &'a AccountInfo,
    pub token_program: Option<&'a AccountInfo>,
    pub mpl_token_metadata: &'a AccountInfo,
}

impl CreateAsset<'_> {
    #[inline(always)]
    pub fn invoke(
        &self,
        data: &CreateAssetInstructionData,
        serialization_buffer: &mut [u8],
    ) -> ProgramResult {
        self.invoke_signed(data, &[], serialization_buffer)
    }

    pub fn invoke_signed(
        &self,
        data: &CreateAssetInstructionData,
        signers: &[Signer],
        serialization_buffer: &mut [u8],
    ) -> ProgramResult {
        // account metadata
        let account_metas: &[AccountMeta] = &[
            AccountMeta::writable(self.metadata.key()),
            match self.master_edition {
                Some(master_edition) => AccountMeta::writable(master_edition.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            AccountMeta::writable(self.mint.key()),
            AccountMeta::readonly_signer(self.authority.key()),
            AccountMeta::writable_signer(self.payer.key()),
            AccountMeta::readonly(self.update_authority.key()),
            AccountMeta::readonly(self.system_program.key()),
            AccountMeta::readonly(self.sysvar_instructions.key()),
            match self.token_program {
                Some(token_program) => AccountMeta::writable(token_program.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
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
                self.metadata,
                self.master_edition.unwrap_or(self.mpl_token_metadata),
                self.mint,
                self.authority,
                self.payer,
                self.update_authority,
                self.system_program,
                self.token_program.unwrap_or(self.mpl_token_metadata),
            ],
            signers,
        )
    }
}
