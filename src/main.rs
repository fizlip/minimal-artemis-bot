use std::sync::Arc;
use anyhow::Result;

use artemis_core::{
    collectors::mevshare_collector::MevShareCollector,
    engine::Engine,
    types::CollectorMap,
};

use clap::Parser;

use ethers::{
    prelude::MiddlewareBuilder,
    providers::{Provider, Ws},
    signers::{LocalWallet, Signer},
    types::Address
};

mod strategy;
use strategy::ExampleStrat;

mod types;
use types::{Action, Event};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub wss: String,
    #[arg(long)]
    pub private_key: String,
}

#[tokio::main]
async fn main() -> Result<()> {

    let args = Args::parse();

    let ws = Ws::connect(args.wss).await?;
    let provider = Provider::new(ws);

    let wallet: LocalWallet = args.private_key.parse().unwrap();
    let address = wallet.address();

    let provider = Arc::new(provider.nonce_manager(address).with_signer(wallet.clone()));
    //let _fb_signer: LocalWallet = args.flashbots_signer.parse().unwrap();

    let mut engine: Engine<Event, Action> = Engine::default();

    let mevshare_collector = Box::new(MevShareCollector::new(String::from(
        "https://mev-share.flashbots.net",
    )));
    let mevshare_collector = CollectorMap::new(mevshare_collector, Event::MEVShareEvent);
    engine.add_collector(Box::new(mevshare_collector));
    
    let strategy = ExampleStrat::new(Arc::new(provider.clone())); 
    engine.add_strategy(Box::new(strategy));

    //let mev_share_executor = Box::new(MevshareExecutor::new(fb_signer));
    //let mev_share_executor = ExecutorMap::new(mev_share_executor, |action| match action {
    //    Action::SubmitBundle(bundle) => Some(bundle),
    //});

    //engine.add_executor(Box::new(mev_share_executor));

    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            println!("res: {:?}", res);
        }
    }

    Ok(())
}
