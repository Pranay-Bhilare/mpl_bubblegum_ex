// // use rustler::{Env, Term, NifResult, nif};  
// use solana_sdk::pubkey::Pubkey;  
// use solana_sdk::signer::keypair::Keypair;  
// use std::str::FromStr;  
// mod create_tree;
// mod valid_depth_size_pairs;
// use create_tree::*;

// #[rustler::nif]  
// fn validate_pubkey(pubkey_str: &str) -> bool {  
//     Pubkey::from_str(pubkey_str).is_ok()  
// }  

// // Keypair validation  
// #[rustler::nif]  
// fn validate_keypair(keypair_bytes: Vec<u8>) -> bool {  
//     Keypair::from_bytes(&keypair_bytes).is_ok()  
// }  

// rustler::init!("Elixir.MplBubblegumEx.Native", [  
//     create_tree_config_tx,  
//     validate_pubkey,  
//     validate_keypair  
// ]);

use rustler::{Encoder, Env, Error, Term};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use std::str::FromStr;

mod create_tree;
mod valid_depth_size_pairs;
mod metadata;

#[rustler::nif]
pub fn validate_pubkey_nif(pubkey_str: &str) -> bool {
    Pubkey::from_str(pubkey_str).is_ok()
}

#[rustler::nif]
pub fn validate_keypair_nif(keypair_bytes: Vec<u8>) -> bool {
    keypair_bytes.len() == 64 && Keypair::from_bytes(&keypair_bytes).is_ok()
}

rustler::init!("Elixir.MplBubblegumEx.Native", [
    validate_pubkey_nif,
    validate_keypair_nif,
    create_tree::create_tree_config_tx
]);
