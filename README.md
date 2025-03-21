# Event Fetcher

This project provides a tool to fetch and listen for specific events from the Starknet blockchain. It uses the `starknet` and `starknet-crypto` libraries to interact with the blockchain and fetch events.

## Prerequisites

- Rust and Cargo installed on your system.
- Access to a Starknet RPC endpoint.

## Setup

1. **Clone the Repository:**

   ```bash
   git clone <repository-url>
   cd sharding_orchestrator
   ```

2. **Build the Project:**

   Ensure you are in the root directory of the project and run:

   ```bash
   cargo build --release
   ```

## Usage

The event fetcher can be run using the command line interface. It requires several parameters to be specified, either through command line arguments or environment variables.

### Command Line Arguments

- `--contract-address` or `-c`: The address of the contract to listen for events.
- `--start-block` or `-s`: The block number to start listening from.
- `--rpc-url` or `-r`: The URL of the Starknet RPC endpoint.
- `--event-name` or `-e`: The name of the event to listen for.

### Example Command

```bash
cargo run --release --bin shard -- --contract-address 0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7 --start-block 612520 --rpc-url https://starknet-sepolia.public.blastapi.io --event-name Transfer
```

### Environment Variables

You can also set the environment variables for the arguments:

- `CONTRACT_ADDRESS`
- `START_BLOCK`
- `RPC_URL`
- `EVENT_NAME`

Then run the command without arguments:

```bash
cargo run --release --bin shard
```

## Testing

The project includes a test for the `look_for_event` function. You can run the tests using:

```bash
cargo test -- --nocapture
```

## Notes

- Ensure the RPC URL is correct and accessible.
- The event fetcher will continuously poll for events starting from the specified block.
