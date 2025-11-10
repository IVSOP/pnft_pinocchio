use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke_signed,
    instruction::{AccountMeta, Instruction, Signer},
    ProgramResult,
};

use crate::data::{burn::BurnInstructionData, Serialize};

/// Burn an asset
///
/// ### Accounts:
///   0. `[WRITE, SIGNER]` Authority
///   1. `[WRITE, OPTIONAL]` Collection Metadata
///   2. `[WRITE]` Metadata
///   3. `[SIGNER, OPTIONAL]` Authority
///   4. `[WRITE, OPTIONAL]` Edition
///   5. `[WRITE]` Mint
///   6. `[WRITE]` Token Account
///   7. `[WRITE, OPTIONAL]` Master Edition
///   8. `[OPTIONAL]` Master Edition Mint
///   9. `[OPTIONAL]` Master Edition Token Account
///   10. `[WRITE, OPTIONAL]` Edition Marker
///   11. `[WRITE, OPTIONAL]` Token Record
///   12. `[]` System Program
///   13. `[]` Sysvar Instructions
///   14. `[]` SPL Token Program
///   15. `[]` MPL Token Metadata
///
/// Accounts being optional is very cursed but mimics the behaviour of the official lib.
/// Accounts set to None get replaced by mpl program's account.
pub struct Burn<'a> {
    pub authority: &'a AccountInfo,
    pub collection_metadata: Option<&'a AccountInfo>,
    pub metadata: &'a AccountInfo,
    pub delegate_authority: Option<&'a AccountInfo>,
    pub edition: Option<&'a AccountInfo>,
    pub mint: &'a AccountInfo,
    pub token_account: &'a AccountInfo,
    pub master_edition: Option<&'a AccountInfo>,
    pub master_edition_mint: Option<&'a AccountInfo>,
    pub master_edition_token_account: Option<&'a AccountInfo>,
    pub edition_marker: Option<&'a AccountInfo>,
    pub token_record: Option<&'a AccountInfo>,
    pub system_program: &'a AccountInfo,
    pub sysvar_instructions: &'a AccountInfo,
    pub spl_token_program: &'a AccountInfo,
    pub mpl_token_metadata: &'a AccountInfo,
}

impl Burn<'_> {
    #[inline(always)]
    pub fn invoke(
        &self,
        data: &BurnInstructionData,
        serialization_buffer: &mut [u8],
    ) -> ProgramResult {
        self.invoke_signed(data, &[], serialization_buffer)
    }

    pub fn invoke_signed(
        &self,
        data: &BurnInstructionData,
        signers: &[Signer],
        serialization_buffer: &mut [u8],
    ) -> ProgramResult {
        // account metadata
        let account_metas: &[AccountMeta] = &[
            AccountMeta::writable_signer(self.authority.key()),
            match self.collection_metadata {
                Some(cm) => AccountMeta::writable(cm.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            AccountMeta::writable(self.metadata.key()),
            match self.delegate_authority {
                Some(da) => AccountMeta::readonly_signer(da.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            match self.edition {
                Some(edition) => AccountMeta::writable(edition.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            AccountMeta::writable(self.mint.key()),
            AccountMeta::writable(self.token_account.key()),
            match self.master_edition {
                Some(me) => AccountMeta::writable(me.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            match self.master_edition_mint {
                Some(mem) => AccountMeta::readonly(mem.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            match self.master_edition_token_account {
                Some(meta) => AccountMeta::readonly(meta.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            match self.edition_marker {
                Some(marker) => AccountMeta::writable(marker.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            match self.token_record {
                Some(record) => AccountMeta::writable(record.key()),
                None => AccountMeta::readonly(self.mpl_token_metadata.key()),
            },
            AccountMeta::readonly(self.system_program.key()),
            AccountMeta::readonly(self.sysvar_instructions.key()),
            AccountMeta::readonly(self.spl_token_program.key()),
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
                self.collection_metadata.unwrap_or(self.mpl_token_metadata),
                self.metadata,
                self.delegate_authority.unwrap_or(self.mpl_token_metadata),
                self.edition.unwrap_or(self.mpl_token_metadata),
                self.mint,
                self.token_account,
                self.master_edition.unwrap_or(self.mpl_token_metadata),
                self.master_edition_mint.unwrap_or(self.mpl_token_metadata),
                self.master_edition_token_account
                    .unwrap_or(self.mpl_token_metadata),
                self.edition_marker.unwrap_or(self.mpl_token_metadata),
                self.token_record.unwrap_or(self.mpl_token_metadata),
                self.system_program,
                self.sysvar_instructions,
                self.spl_token_program,
                self.mpl_token_metadata,
            ],
            signers,
        )
    }
}
