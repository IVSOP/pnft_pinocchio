use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke_signed,
    instruction::{AccountMeta, Instruction, Signer},
    ProgramResult,
};

use crate::data::{mint::MintInstructionData, Serialize};

/// Mint an asset (use after Create)
///
/// ### Accounts:
///   0. `[WRITE]` Token Account
///   1. `[OPTIONAL]` Owner
///   2. `[]` Metadata
///   3. `[OPTIONAL, WRITE]` Master Edition
///   4. `[OPTIONAL, WRITE]` Token Record
///   5. `[WRITE]` Mint
///   6. `[SIGNER]` (Mint or Update) Authority
///   7. `[OPTIONAL]` Metadata Delegate Record
///   8. `[SIGNER, WRITE]` Payer
///   9. `[]` System Program
///   10. `[]` Sysvar Instructions
///   11. `[]` SPL Token Program
///   12. `[]` SPL Associated Token Program
///   13. `[OPTIONAL]` Auth Rules Program
///   14. `[OPTIONAL]` Auth Rules Account
///   15. `[]` MPL Metadata Program
///
/// Accounts being optional is very cursed but mimics the behaviour of the official lib.
/// Accounts set to None get replaced by mpl program's account.
pub struct MintAsset<'a> {
    pub token_account: &'a AccountInfo,
    pub owner: Option<&'a AccountInfo>,
    pub metadata: &'a AccountInfo,
    pub master_edition: Option<&'a AccountInfo>,
    pub token_record: Option<&'a AccountInfo>,
    pub mint: &'a AccountInfo,
    pub authority: &'a AccountInfo,
    pub metadata_delegate_record: Option<&'a AccountInfo>,
    pub payer: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub sysvar_instructions: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub associated_token_program: &'a AccountInfo,
    pub auth_rules_program: Option<&'a AccountInfo>,
    pub auth_rules: Option<&'a AccountInfo>,
    pub mpl_token_metadata: &'a AccountInfo,
}

impl MintAsset<'_> {
    #[inline(always)]
    pub fn invoke(
        &self,
        data: &MintInstructionData,
        serialization_buffer: &mut [u8],
    ) -> ProgramResult {
        self.invoke_signed(data, &[], serialization_buffer)
    }

    pub fn invoke_signed(
        &self,
        data: &MintInstructionData,
        signers: &[Signer],
        serialization_buffer: &mut [u8],
    ) -> ProgramResult {
        // account metadata
        let account_metas: &[AccountMeta] = &[
            AccountMeta::writable(self.token_account.key()),
            match self.owner {
                Some(owner) => AccountMeta::readonly(owner.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            AccountMeta::readonly(self.metadata.key()),
            match self.master_edition {
                Some(edition) => AccountMeta::writable(edition.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            match self.token_record {
                Some(record) => AccountMeta::writable(record.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            AccountMeta::writable(self.mint.key()),
            AccountMeta::readonly_signer(self.authority.key()),
            match self.metadata_delegate_record {
                Some(delegate) => AccountMeta::readonly(delegate.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            AccountMeta::writable_signer(self.payer.key()),
            AccountMeta::readonly(self.system_program.key()),
            AccountMeta::readonly(self.sysvar_instructions.key()),
            AccountMeta::readonly(self.token_program.key()),
            AccountMeta::readonly(self.associated_token_program.key()),
            match self.auth_rules_program {
                Some(program) => AccountMeta::readonly(program.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            match self.auth_rules {
                Some(rules) => AccountMeta::readonly(rules.key()),
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
                self.token_account,
                self.owner.unwrap_or(self.mpl_token_metadata),
                self.metadata,
                self.master_edition.unwrap_or(self.mpl_token_metadata),
                self.token_record.unwrap_or(self.mpl_token_metadata),
                self.mint,
                self.authority,
                self.metadata_delegate_record
                    .unwrap_or(self.mpl_token_metadata),
                self.payer,
                self.system_program,
                self.sysvar_instructions,
                self.token_program,
                self.associated_token_program,
                self.auth_rules_program.unwrap_or(self.mpl_token_metadata),
                self.auth_rules.unwrap_or(self.mpl_token_metadata),
                self.mpl_token_metadata,
            ],
            signers,
        )
    }
}
