use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke_signed,
    instruction::{AccountMeta, Instruction, Signer},
    ProgramResult,
};

use crate::data::{transfer::TransferInstructionData, Serialize};

/// Transfer an asset
///
/// ### Accounts:
///   0. `[WRITE]` Token Account
///   1. `[]` Owner
///   2. `[WRITE]` Destination Token Account
///   3. `[]` New Owner
///   4. `[]` mint
///   5. `[WRITE]` Metadata
///   6. `[OPTIONAL]` Edition
///   7. `[OPTIONAL, WRITE]` Owner Token Record
///   8. `[OPTIONAL, WRITE]` Destination Token Record
///   9. `[SIGNER]` Authority
///   10. `[SIGNER, WRITE]` Payer
///   11. `[]` System Program
///   12. `[]` Sysvar Instructions
///   13. `[]` SPL Token Program
///   14. `[]` SPL Associated Token Program
///   15. `[OPTIONAL]` Auth Rules Program
///   16. `[OPTIONAL]` Auth Rules Account
///   17. `[]` MPL Token Metadata
///
/// Accounts being optional is very cursed but mimics the behaviour of the official lib.
/// Accounts set to None get replaced by mpl program's account.
pub struct Transfer<'a> {
    pub src_token_account: &'a AccountInfo,
    pub owner: &'a AccountInfo,
    pub dest_token_account: &'a AccountInfo,
    pub new_owner: &'a AccountInfo,
    pub mint: &'a AccountInfo,
    pub metadata: &'a AccountInfo,
    pub edition: Option<&'a AccountInfo>,
    pub owner_token_record: Option<&'a AccountInfo>,
    pub dest_token_record: Option<&'a AccountInfo>,
    pub authority: &'a AccountInfo,
    pub payer: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub sysvar_instructions: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub associated_token_program: &'a AccountInfo,
    pub auth_rules_program: Option<&'a AccountInfo>,
    pub auth_rules: Option<&'a AccountInfo>,
    pub mpl_token_metadata: &'a AccountInfo,
}

impl Transfer<'_> {
    #[inline(always)]
    pub fn invoke(
        &self,
        data: &TransferInstructionData,
        serialization_buffer: &mut [u8],
    ) -> ProgramResult {
        self.invoke_signed(data, &[], serialization_buffer)
    }

    pub fn invoke_signed(
        &self,
        data: &TransferInstructionData,
        signers: &[Signer],
        serialization_buffer: &mut [u8],
    ) -> ProgramResult {
        // account metadata
        let account_metas: &[AccountMeta] = &[
            AccountMeta::writable(self.src_token_account.key()),
            AccountMeta::readonly(self.owner.key()),
            AccountMeta::writable(self.dest_token_account.key()),
            AccountMeta::readonly(self.new_owner.key()),
            AccountMeta::readonly(self.mint.key()),
            AccountMeta::writable(self.metadata.key()),
            match self.edition {
                Some(edition) => AccountMeta::readonly(edition.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            match self.owner_token_record {
                Some(owner_token_record) => AccountMeta::writable(owner_token_record.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            match self.dest_token_record {
                Some(dest_token_record) => AccountMeta::writable(dest_token_record.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            AccountMeta::readonly_signer(self.authority.key()),
            AccountMeta::writable_signer(self.payer.key()),
            AccountMeta::readonly(self.system_program.key()),
            AccountMeta::readonly(self.sysvar_instructions.key()),
            AccountMeta::readonly(self.token_program.key()),
            AccountMeta::readonly(self.associated_token_program.key()),
            match self.auth_rules_program {
                Some(auth_rules_program) => AccountMeta::readonly(auth_rules_program.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            match self.auth_rules {
                Some(auth_rules) => AccountMeta::readonly(auth_rules.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
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
                self.src_token_account,
                self.owner,
                self.dest_token_account,
                self.new_owner,
                self.mint,
                self.metadata,
                self.edition.unwrap_or(self.mpl_token_metadata),
                self.owner_token_record.unwrap_or(self.mpl_token_metadata),
                self.dest_token_record.unwrap_or(self.mpl_token_metadata),
                self.authority,
                self.payer,
                self.system_program,
                self.sysvar_instructions,
                self.token_program,
                self.associated_token_program,
                self.auth_rules_program.unwrap_or(self.mpl_token_metadata),
                self.auth_rules.unwrap_or(self.mpl_token_metadata),
            ],
            signers,
        )
    }
}
