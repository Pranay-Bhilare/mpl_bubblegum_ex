// File: native/mpl_bubblegum_native/src/mint.rs

// use solana_program::system_program;
use solana_sdk::{
    pubkey::Pubkey,
    signature::Keypair,
    transaction::Transaction,
    signer::Signer
};
use mpl_bubblegum::{
    instructions::{MintV1Builder, MintToCollectionV1Builder},
    types::MetadataArgs,
    ID as BUBBLEGUM_PROGRAM_ID,
    programs::{SPL_ACCOUNT_COMPRESSION_ID, SPL_NOOP_ID},
    accounts::TreeConfig,
};
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;
use rustler::{NifStruct, atoms, Binary};
use crate::metadata::{MetadataArgsNif, convert_metadata_args};
use bincode::serialize;
use bs58;

// Define atoms for error handling
rustler::atoms! {
    ok,
    error,
    invalid_keypair,
    invalid_pubkey,
    invalid_metadata,
    rpc_error,
    serialization_error,
    instruction_error
}

#[rustler::nif]
pub fn mint_compressed_nft_tx(
    tree_keypair: Binary,  // Changed from tree_pubkey_str
    leaf_owner_pubkey_str: &str, 
    payer_keypair: Binary,
    metadata_args: MetadataArgsNif,
    rpc_url: &str
) -> Result<(rustler::Atom, String), rustler::Error> {
    // Parse tree keypair
    let tree_keypair_vec = tree_keypair.as_slice().to_vec();
    let merkle_tree = match Keypair::from_bytes(&tree_keypair_vec) {
        Ok(keypair) => keypair,
        Err(_) => return Err(rustler::Error::Term(Box::new(invalid_keypair())))
    };
    let merkle_tree_pubkey = merkle_tree.pubkey();

    let payer_vec = payer_keypair.as_slice().to_vec();
    let payer = match Keypair::from_bytes(&payer_vec) {
        Ok(keypair) => keypair,
        Err(_) => return Err(rustler::Error::Term(Box::new(invalid_keypair())))
    };
    
    let leaf_owner = match Pubkey::from_str(leaf_owner_pubkey_str) {
        Ok(pubkey) => pubkey,
        Err(_) => return Err(rustler::Error::Term(Box::new(invalid_pubkey())))
    };
    
    // Convert metadata
    let metadata = match convert_metadata_args(&metadata_args) {
        Ok(meta) => meta,
        Err(e) => return Err(rustler::Error::Term(Box::new(format!("Invalid metadata: {}", e))))
    };
    
    // Get PDA for tree_authority
    let (tree_authority, _) = TreeConfig::find_pda(&merkle_tree_pubkey);
    
    // Setup RPC client
    let rpc_client = RpcClient::new(rpc_url);
    let recent_blockhash = match rpc_client.get_latest_blockhash() {
        Ok(hash) => hash,
        Err(_) => return Err(rustler::Error::Term(Box::new(rpc_error())))
    };
    
    // Create regular mint instruction
    let mint_ix = MintV1Builder::new()
        .tree_config(tree_authority)
        .leaf_owner(leaf_owner)
        .leaf_delegate(leaf_owner) // Default to owner as delegate
        .merkle_tree(merkle_tree_pubkey)
        .payer(payer.pubkey())
        .tree_creator_or_delegate(payer.pubkey())
        .metadata(metadata)
        .instruction();
        
    // Build and sign transaction
    let transaction = Transaction::new_signed_with_payer(
        &[mint_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    
    let signature = rpc_client.send_and_confirm_transaction(&transaction)
    .map_err(|_| rustler::Error::Term(Box::new(rpc_error())))?;

    Ok((ok(), signature.to_string()))
}

#[rustler::nif]
pub fn mint_to_collection_tx(
    tree_keypair: Binary,
    leaf_owner_pubkey_str: &str,
    payer_keypair: Binary,
    metadata_args: MetadataArgsNif,
    collection_mint_str: &str,
    collection_authority_keypair: Binary,
    collection_authority_record_pda_str: Option<String>,
    rpc_url: &str
) -> Result<(rustler::Atom, String), rustler::Error> {
    // Parse tree keypair
    let tree_keypair_vec = tree_keypair.as_slice().to_vec();
    let merkle_tree = match Keypair::from_bytes(&tree_keypair_vec) {
        Ok(keypair) => keypair,
        Err(_) => return Err(rustler::Error::Term(Box::new(invalid_keypair())))
    };
    let merkle_tree_pubkey = merkle_tree.pubkey();
    // Parse inputs
    let payer_vec = payer_keypair.as_slice().to_vec();
    let payer = match Keypair::from_bytes(&payer_vec) {
        Ok(keypair) => keypair,
        Err(_) => return Err(rustler::Error::Term(Box::new(invalid_keypair())))
    };

    let collection_vec = collection_authority_keypair.as_slice().to_vec();
    let collection_authority = match Keypair::from_bytes(&collection_vec) {
        Ok(keypair) => keypair,
        Err(_) => return Err(rustler::Error::Term(Box::new(invalid_keypair())))
    };
    
    let leaf_owner = match Pubkey::from_str(leaf_owner_pubkey_str) {
        Ok(pubkey) => pubkey,
        Err(_) => return Err(rustler::Error::Term(Box::new(invalid_pubkey())))
    };
    
    let collection_mint = match Pubkey::from_str(collection_mint_str) {
        Ok(pubkey) => pubkey,
        Err(_) => return Err(rustler::Error::Term(Box::new(invalid_pubkey())))
    };
    
    // Parse optional collection authority record PDA
    let collection_authority_record = if let Some(pda_str) = collection_authority_record_pda_str {
        match Pubkey::from_str(&pda_str) {
            Ok(pubkey) => Some(pubkey),
            Err(_) => return Err(rustler::Error::Term(Box::new(invalid_pubkey())))
        }
    } else {
        None
    };
    
    // Convert metadata
    let metadata = match convert_metadata_args(&metadata_args) {
        Ok(meta) => meta,
        Err(e) => return Err(rustler::Error::Term(Box::new(format!("Invalid metadata: {}", e))))
    };
    
    // Get PDA for tree_authority
    let (tree_authority, _) = TreeConfig::find_pda(&merkle_tree_pubkey);
    
    // Setup RPC client
    let rpc_client = RpcClient::new(rpc_url);
    let recent_blockhash = match rpc_client.get_latest_blockhash() {
        Ok(hash) => hash,
        Err(_) => return Err(rustler::Error::Term(Box::new(rpc_error())))
    };
    
    // Create mint to collection instruction
    let mint_ix = MintToCollectionV1Builder::new()
        .tree_config(tree_authority)
        .leaf_owner(leaf_owner)
        .leaf_delegate(leaf_owner) // Default to owner as delegate
        .merkle_tree(merkle_tree_pubkey)
        .payer(payer.pubkey())
        .tree_creator_or_delegate(payer.pubkey())
        .collection_authority(collection_authority.pubkey())
        .collection_authority_record_pda(collection_authority_record)
        .collection_mint(collection_mint)
        .metadata(metadata)
        .instruction();
    
    // Get signers - we need both payer and collection authority
    let mut signers = vec![&payer];
    
    // Only add collection authority if it's different from payer
    if collection_authority.pubkey() != payer.pubkey() {
        signers.push(&collection_authority);
    }
    
    // Build and sign transaction
    let transaction = Transaction::new_signed_with_payer(
        &[mint_ix],
        Some(&payer.pubkey()),
        &signers,
        recent_blockhash,
    );
    
    let signature = rpc_client.send_and_confirm_transaction(&transaction)
    .map_err(|_| rustler::Error::Term(Box::new(rpc_error())))?;

    Ok((ok(), signature.to_string()))
}