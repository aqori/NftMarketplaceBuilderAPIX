// src/main.rs
/*
 * Main executable for NftMarketplaceBuilderAPIX
 */

use clap::Parser;
use nftmarketplacebuilderapix::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftMarketplaceBuilderAPIX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
