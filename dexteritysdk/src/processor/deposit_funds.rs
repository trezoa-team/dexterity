use crate::common::utils::*;

use crate::{sdk_client::SDKClient, KeypairD};
use trezoaanchor_lang::{InstructionData, ToAccountMetas};
use dex::{accounts, instruction, utils::numeric::Fractional};
use trezoa_program::{instruction::Instruction, pubkey::Pubkey};
use trezoa_program_test::ProgramTestContext;
use trezoa_sdk::signature::{Keypair, Signer};

pub fn deposit_funds_ixs(
    user: Pubkey,
    user_token_account: Pubkey,
    trader_risk_group: Pubkey,
    market_product_group: Pubkey,
    market_product_group_vault: Pubkey,
    quantity: Fractional,
) -> Vec<Instruction> {
    let params = dex::DepositFundsParams { quantity };
    let account_metas = accounts::DepositFunds {
        token_program: tpl_token::ID,
        user,
        user_token_account,
        trader_risk_group,
        market_product_group,
        market_product_group_vault,
    }
    .to_account_metas(None);
    vec![Instruction {
        program_id: dex::ID,
        data: instruction::DepositFunds { params }.data(),
        accounts: account_metas,
    }]
}

pub async fn deposit_funds(
    client: &SDKClient,
    dex_program_id: Pubkey,
    tpl_token_program_id: Pubkey,
    user: &KeypairD,
    user_wallet: Pubkey,
    user_trader_risk_group: Pubkey,
    market_product_group: Pubkey,
    market_product_group_vault: Pubkey,
    quantity: Fractional,
) -> SDKResult {
    let ixs = deposit_funds_ixs(
        user.pubkey(),
        user_wallet,
        user_trader_risk_group,
        market_product_group,
        market_product_group_vault,
        quantity,
    );
    client.sign_send_instructions(ixs, vec![user]).await
}
