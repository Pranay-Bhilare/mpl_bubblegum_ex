// use rustler::{Env, Term, NifResult, nif};  
use solana_sdk::pubkey::Pubkey;  
use solana_sdk::signer::keypair::Keypair;  
use std::str::FromStr;  
mod create_tree;
use create_tree::*;

#[rustler::nif]  
fn validate_pubkey(pubkey_str: &str) -> bool {  
    Pubkey::from_str(pubkey_str).is_ok()  
}  

// Keypair validation  
#[rustler::nif]  
fn validate_keypair(keypair_bytes: Vec<u8>) -> bool {  
    Keypair::from_bytes(&keypair_bytes).is_ok()  
}  

rustler::init!("Elixir.MplBubblegumEx.Native", [  
    create_tree_config_tx,  
    validate_pubkey,  
    validate_keypair  
]);  
