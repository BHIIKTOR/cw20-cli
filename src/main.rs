mod contract;
use clap::builder::TypedValueParser as _;
use clap::{Args, Parser, Subcommand};

use contract::MyContractoor;
use dotenvy::dotenv;

// use cw_orc::networks::ChainInfo;
use std::sync::Arc;
use tokio::runtime::Runtime;

use cw_orc::{networks::*, *};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(bin_name = env!("CARGO_PKG_NAME"))]
struct Opts {
    /// Name of the target chain. NOTE: testing is local juno.
    #[arg(
        short,
        long,
        required=true,
        value_parser = clap::builder::PossibleValuesParser::new(
           [
            "UNI_6",
            "JUNO_1",
            "TESTING",
            "PISCO_1",
            "PHOENIX_1",
            "LOCAL_TERRA",
            "INJECTIVE_888",
            "CONSTANTINE_1",
            "BARYON_1",
            "INJECTIVE_1",
            "HARPOON_4",
            "OSMO_4",
            "LOCAL_OSMO",
           ]
        )
            .map(|s| s.parse::<String>().unwrap())
    )]
    chain: String,

    #[command(subcommand)]
    commands: Commands,
}

#[derive(Args, Debug)]
struct Init {
    /// Name of the token
    #[arg(short, long, required = true)]
    name: String,
    /// Symbol of the token
    #[arg(short, long, required = true)]
    symbol: String,
    /// Decimals to be used
    #[arg(short, long, default_value_t = 6u8)]
    decimals: u8,
}

#[derive(Args, Debug)]
struct Deploy {}

#[derive(Args, Debug)]
struct Transfer {
    /// Recipient wallet
    #[arg(short, long, required = true)]
    recipient: String,

    /// Amount of tokens to be transfered
    #[arg(short, long)]
    amount: u128,
}

#[derive(Args, Debug)]
struct FindTx {
    #[arg(long, required = true)]
    hash: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Deploy contract to configurable chain
    Deploy(Deploy),
    /// Initialize our contract
    Initialize(Init),
    /// Transfer tokens
    Transfer(Transfer),
    /// Search TX by hash
    FindTx(FindTx)
}

fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    let args = Opts::parse();

    let runtime = Arc::new(Runtime::new().unwrap());

    let chain = args.chain.to_lowercase();
    let net = parse_network(&chain);

    let contract = MyContractoor::new(&runtime, env!("CARGO_PKG_NAME"), net);

    match args.commands {
        Commands::Deploy(_) => match contract.inner.upload_if_needed() {
            Ok(msg) => match msg {
                Some(res) => {
                    println!("Contract deployed: {:#?}", res.txhash);
                }
                None => {
                    println!("Contract is already deployed");
                }
            },
            Err(err) => panic!("Error: {}", err.to_string()),
        },
        Commands::Initialize(opts) => match contract.init(opts.name, opts.symbol, opts.decimals) {
            Ok(msg) => {
                println!("Contract Initialized: {:#?}", msg.txhash);
            }
            Err(err) => panic!("Error: {}", err.to_string()),
        },
        Commands::Transfer(opts) => match contract.transfer(opts.recipient, opts.amount) {
            Ok(msg) => {
                println!("Token transfer successful: {:#?}", msg);
            }
            Err(err) => panic!("Error: {}", err.to_string()),
        },
        Commands::FindTx(opts) => {
            match contract.find_tx(opts.hash) {
                Ok(res) => println!("{:#?}", res),
                Err(err) => panic!("Error: {}", err.to_string()),
            }
        },

    }
}
