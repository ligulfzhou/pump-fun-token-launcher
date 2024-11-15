use crate::constants::*;
use anchor_lang::prelude::*;
use solana_sdk::{
    instruction::Instruction,
    signature::{Keypair, Signer},
    system_program::ID as SYSTEM_PROGRAM_ID,
    sysvar::rent::ID as RENT_ID,
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CreateLayout {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub fn get_create_instruction(
    payer: &Keypair,
    mint: &Keypair,
    name: &str,
    symbol: &str,
    uri: &str,
) -> anyhow::Result<Instruction> {
    let create_layout = CreateLayout {
        name: name.to_string(),
        symbol: symbol.to_string(),
        uri: uri.to_string(),
    };
    let mut concatenated_data = Vec::from(CREATE_DISCRIMINATOR);
    let serialized_data = create_layout.try_to_vec()?;
    concatenated_data.extend_from_slice(&serialized_data);

    let bonding_curve = Pubkey::find_program_address(
        &["bonding-curve".as_bytes(), &mint.pubkey().to_bytes()],
        &PUMPFUN_PROGRAM,
    )
    .0;

    let bonding_curve_ata = Pubkey::find_program_address(
        &[
            &bonding_curve.to_bytes(),
            &TOKEN_PROGRAM_ID.to_bytes(),
            &mint.to_bytes(),
        ],
        &ASSOCIATED_TOKEN_PROGRAM_ID,
    )
    .0;
    let metadata = Pubkey::find_program_address(
        &[
            "metadata".as_bytes(),
            &MPL_TOKEN_METADATA.to_bytes(),
            &mint.pubkey().to_bytes(),
        ],
        &MPL_TOKEN_METADATA,
    )
    .0;

    let instruction_accounts = vec![
        AccountMeta::new(mint.pubkey(), true),
        AccountMeta::new_readonly(PUMPFUN_TOKEN_MINT_AUTHORITY, false),
        AccountMeta::new(bonding_curve, false),
        AccountMeta::new(bonding_curve_ata, false),
        AccountMeta::new_readonly(PUMPFUN_GLOBAL, false),
        AccountMeta::new_readonly(MPL_TOKEN_METADATA, false),
        AccountMeta::new(metadata, false),
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
        AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
        AccountMeta::new_readonly(ASSOCIATED_TOKEN_PROGRAM_ID, false),
        AccountMeta::new_readonly(RENT_ID, false),
        AccountMeta::new_readonly(PUMPFUN_EVENT_AUTHORITY, false),
        AccountMeta::new_readonly(PUMPFUN_PROGRAM, false),
    ];

    let ix_final =
        Instruction::new_with_bytes(PUMPFUN_PROGRAM, &concatenated_data, instruction_accounts);

    Ok(ix_final)
}
