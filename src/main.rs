use clap::Parser;
use event_listener::look_for_event;
use starknet::core::utils::starknet_keccak;
use starknet_crypto::Felt;
pub mod event_listener;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    contract_address: Felt,
    #[arg(short, long)]
    start_block: u64,
    #[arg(short, long)]
    rpc_url: String,
}

// Watek glowny odpala watek ktory bedzie skanowal siec w poszukiwaniu eventu shard dla danego kontraktu 
// Gdy znajdziemy taki event to odpalamy katane na ktorej bedziemy przyjmowac transakcje i szukac eventu 
// ktory odpowiada za zakonczenie katany.

// Gdy katana zostanie uruchomiona to odpalamy watek ktory zaciagac bedzie bloki i robic z nich dowody. 
// Po zrobieniu owych dowodow, przesylamy je na proxy kontrakt ktory zupdejtuje nam pierwotny kontrakt.
#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let rpc = args.rpc_url.parse().unwrap();
    let event_hash = starknet_keccak("Transfer".as_bytes());
    look_for_event(args.contract_address, args.start_block, rpc, event_hash).await; //looking for starting event 
    fork_katana().await; //starting katana

    prepare_proofs().await; //preparing proof

    send_proof().await; //sending proof
}



async fn fork_katana(){
    println!("Starting katana");
}
async fn prepare_proofs(){
    println!("Preparing proof");
}
async fn send_proof(){
    println!("Sending proof");
}

