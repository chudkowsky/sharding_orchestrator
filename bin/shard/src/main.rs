use clap::Parser;
use shard_core::event_listener::look_for_event;
use starknet::core::utils::starknet_keccak;
use starknet_crypto::Felt;

#[derive(Parser)]
struct Cli {
    #[arg(env, short, long)]
    contract_address: Felt,
    #[arg(env, short, long)]
    start_block: u64,
    #[arg(env, short, long)]
    rpc_url: String,
    #[arg(env, short, long)]
    event_name: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let rpc = args.rpc_url.parse().unwrap();
    let event_hash = starknet_keccak(args.event_name.as_bytes());
    look_for_event(args.contract_address, args.start_block, rpc, event_hash).await;
}
