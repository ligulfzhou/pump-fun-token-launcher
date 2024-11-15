use pump_fun_token_launcher::instruction::get_create_instruction;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let payer = Keypair::from_base58_string("your keypair for deploying pump-fun token");
    let mint = Keypair::new();

    let name = "PumpFun";
    let symbol = "PF";
    let uri = "https://ipfs.io/ipfs/.....";

    let create_ix = get_create_instruction(&payer, &mint, name, symbol, uri)?;

    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let recent_hash = rpc_client.get_latest_blockhash().await?;
    let tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&payer.pubkey()),
        &[&mint, &payer],
        recent_hash,
    );
    let sig = rpc_client.send_and_confirm_transaction(&tx).await?;

    println!("Transaction hash: {:?}", sig.to_string());

    Ok(())
}
