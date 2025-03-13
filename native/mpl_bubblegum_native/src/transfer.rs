use solana_program::system_program;
use solana_sdk::{
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};
use mpl_bubblegum::{
    instructions::TransferBuilder,
    ID as BUBBLEGUM_PROGRAM_ID,
    accounts::TreeConfig,
};
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;
use rustler::{atoms, Binary};
use bincode::serialize;
use bs58;

// Define atoms for error handling
rustler::atoms! {
    ok,
    error,
    invalid_keypair,
    invalid_pubkey,
    rpc_error,
    serialization_error,
    instruction_error
}

#[rustler::nif]
pub fn transfer_compressed_nft(
    tree_keypair: Binary,
    leaf_owner_keypair: Binary,
    new_leaf_owner_pubkey_str: &str,
    asset_id: &str,
    root_str: Option<String>,
    data_hash_str: Option<String>,
    creator_hash_str: Option<String>,
    nonce: Option<u64>,
    index: Option<u32>,
    rpc_url: &str
) -> Result<(rustler::Atom, String), rustler::Error> {
    // Parse tree keypair
    let tree_keypair_vec = tree_keypair.as_slice().to_vec();
    let merkle_tree = match Keypair::from_bytes(&tree_keypair_vec) {
        Ok(keypair) => keypair,
        Err(_) => return Err(rustler::Error::Term(Box::new(invalid_keypair())))
    };
    let merkle_tree_pubkey = merkle_tree.pubkey();

    // Parse leaf owner keypair (current owner)
    let leaf_owner_vec = leaf_owner_keypair.as_slice().to_vec();
    let leaf_owner = match Keypair::from_bytes(&leaf_owner_vec) {
        Ok(keypair) => keypair,
        Err(_) => return Err(rustler::Error::Term(Box::new(invalid_keypair())))
    };

    // Parse new owner pubkey
    let new_leaf_owner = match Pubkey::from_str(new_leaf_owner_pubkey_str) {
        Ok(pubkey) => pubkey,
        Err(_) => return Err(rustler::Error::Term(Box::new(invalid_pubkey())))
    };

    // Parse optional parameters
    let root = if let Some(root_str) = root_str {
        match <[u8; 32]>::try_from(bs58::decode(root_str).into_vec().map_err(|_| 
            rustler::Error::Term(Box::new(invalid_pubkey())))?.as_slice()) {
            Ok(bytes) => Some(bytes),
            Err(_) => return Err(rustler::Error::Term(Box::new(invalid_pubkey())))
        }
    } else {
        None
    };

    let data_hash = if let Some(data_hash_str) = data_hash_str {
        match <[u8; 32]>::try_from(bs58::decode(data_hash_str).into_vec().map_err(|_| 
            rustler::Error::Term(Box::new(invalid_pubkey())))?.as_slice()) {
            Ok(bytes) => Some(bytes),
            Err(_) => return Err(rustler::Error::Term(Box::new(invalid_pubkey())))
        }
    } else {
        None
    };

    let creator_hash = if let Some(creator_hash_str) = creator_hash_str {
        match <[u8; 32]>::try_from(bs58::decode(creator_hash_str).into_vec().map_err(|_| 
            rustler::Error::Term(Box::new(invalid_pubkey())))?.as_slice()) {
            Ok(bytes) => Some(bytes),
            Err(_) => return Err(rustler::Error::Term(Box::new(invalid_pubkey())))
        }
    } else {
        None
    };

    // Parse asset ID
    let asset_id = match Pubkey::from_str(asset_id) {
        Ok(pubkey) => pubkey,
        Err(_) => return Err(rustler::Error::Term(Box::new(invalid_pubkey())))
    };

    // Get PDA for tree_authority
    let (tree_authority, _) = TreeConfig::find_pda(&merkle_tree_pubkey);

    // Setup RPC client
    let rpc_client = RpcClient::new(rpc_url);
    let recent_blockhash = match rpc_client.get_latest_blockhash() {
        Ok(hash) => hash,
        Err(_) => return Err(rustler::Error::Term(Box::new(rpc_error())))
    };

    // Build transfer instruction
    let mut transfer_builder = TransferBuilder::new();
    transfer_builder = transfer_builder
        .tree_config(tree_authority)
        .leaf_owner(leaf_owner.pubkey())
        .leaf_delegate(leaf_owner.pubkey()) // Usually delegate is same as owner
        .new_leaf_owner(new_leaf_owner)
        .merkle_tree(merkle_tree_pubkey);

    // Add optional parameters if provided
    if let Some(nonce_val) = nonce {
        transfer_builder = transfer_builder.nonce(nonce_val);
    }

    if let Some(index_val) = index {
        transfer_builder = transfer_builder.index(index_val);
    }

    if let Some(root_val) = root {
        transfer_builder = transfer_builder.root(root_val);
    }

    if let Some(data_hash_val) = data_hash {
        transfer_builder = transfer_builder.data_hash(data_hash_val);
    }

    if let Some(creator_hash_val) = creator_hash {
        transfer_builder = transfer_builder.creator_hash(creator_hash_val);
    }

    // Build the instruction
    let transfer_ix = match transfer_builder.instruction() {
        Ok(ix) => ix,
        Err(_) => return Err(rustler::Error::Term(Box::new(instruction_error())))
    };

    // Create and sign transaction
    let transaction = Transaction::new_signed_with_payer(
        &[transfer_ix],
        Some(&leaf_owner.pubkey()),
        &[&leaf_owner],
        recent_blockhash,
    );

    // Submit and confirm transaction
    let signature = match rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(sig) => sig,
        Err(_) => return Err(rustler::Error::Term(Box::new(rpc_error())))
    };

    // Return signature
    Ok((ok(), signature.to_string()))
}
