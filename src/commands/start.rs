//! `start` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;
use std::{path::PathBuf, process, str::FromStr};

use crate::config::AreciboConfig;
use abscissa_core::{config, Command, FrameworkError, Options, Runnable};
use tendermint_rpc::Client;
use crate::{application::APP};
use tendermint::{net::Address, genesis::Genesis, block::Height};
use tokio::{fs::File,io::AsyncReadExt};
/// `start` subcommand
///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>
#[derive(Command, Debug, Options)]
pub struct StartCmd {
    /// To whom are we saying hello?
    #[options(free)]
    rpc:Option<Address>,
}

impl Default for StartCmd {
    fn default() -> Self {
        Self {
            rpc: Some(Address::from_str("http://127.0.1:26657").unwrap()),
        }
    }
}

const URL: &'static str ="test";

impl Runnable for StartCmd {
    /// Start the application.
    fn run(&self) {
        let config = APP.config();

        abscissa_tokio::run(&APP, async {

            let rpc =match self.rpc.clone() {
                Some(rpc) => {rpc },
                None => {Address::from_str("http://127.0.1:26657").unwrap()}
            };





            let rpc = tendermint_rpc::HttpClient::new(rpc.clone()).unwrap();

            let status = rpc.status().await.unwrap();

            if status.sync_info.catching_up{
                error!("Node is still synching.");
                process::exit(1);
            }

            let validators =if status.sync_info.latest_block_height.value() ==0 {

                let mut genesis_file = File::open("genesis.json").await.unwrap();
                let mut buffer = String::new();

                genesis_file.read_to_string(&mut buffer).await.unwrap();

                let genesis:Genesis = serde_json::from_str(&buffer).unwrap();

                genesis.validators.clone()
    
    
    

            } else{

                let vals =rpc.validators(status.sync_info.latest_block_height).await.unwrap();

                vals.validators.clone()

            };




            let consensus_state = rpc.consensus_state().await.unwrap();


            let last_round_votes =consensus_state.round_state.height_vote_set.last().unwrap();

            for (iter, ref vote) in last_round_votes.prevotes.iter().enumerate(){
                 match vote{
                     tendermint_rpc::endpoint::consensus_state::RoundVote::Nil => {
                        let non_voting_validator=validators.get(iter).unwrap();

                        non_voting_validator.pub_key.to_bech32("cosmosconspub");

                         info!("Not yet voting pubkey: {} power: {}",non_voting_validator.pub_key.to_bech32("cosmosconspub"), non_voting_validator.power());
                        
                     }
                     tendermint_rpc::endpoint::consensus_state::RoundVote::Vote(_) => {}
                 }
            }
            




        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            process::exit(1);
        });
    }
}

// impl config::Override<AreciboConfig> for StartCmd {
//     // Process the given command line options, overriding settings from
//     // a configuration file using explicit flags taken from command-line
//     // arguments.
//     fn override_config(
//         &self,
//         mut config: AreciboConfig,
//     ) -> Result<AreciboConfig, FrameworkError> {
//         if !self.recipient.is_empty() {
//             config.hello.recipient = self.recipient.join(" ");
//         }

//         Ok(config)
//     }
// }
