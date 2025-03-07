use solana_program::system_instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::{signature::Keypair ,transaction::Transaction, system_instruction::create_account as CreateAccount,}; 
// use solana_instruction::Instruction as SolanaInstruction; 
use mpl_bubblegum::instructions::{CreateTreeConfig, CreateTreeConfigBuilder, CreateTreeConfigInstructionArgs};  
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;  
use mpl_bubblegum::programs::{MPL_BUBBLEGUM_ID,SPL_ACCOUNT_COMPRESSION_ID,SPL_NOOP_ID}; 
use mpl_bubblegum::accounts::TreeConfig;
use spl_account_compression::{state::CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1, ConcurrentMerkleTree};
use rustler::{NifStruct,atoms,nif};
use bs58;
use bincode::serialize;

rustler::atoms!{
    ok,
    error,
    invalid_keypair,
    invalid_pubkey,
    rpc_error,
    serialization_error,
    instruction_error
}

#[derive(NifStruct)]
#[module = "MplBubblegumEx.TreeOptions"]

struct TreeOptions{
    public: Option<bool>,
    log_wrapper:Option<String>,
    compression_program:Option<String>,
    system_program:Option<String>
}

#[rustler::nif]
fn create_tree_config_tx(
    max_depth: u32,
    max_buffer_size: u32,
    payer_keypair: Vec<u8>,
    merkle_tree_pubkey: &str,
    rpc_url: &str,
    options : Option<TreeOptions>
    ) -> Result<(rustler::Atom,String), rustler::Error>{

        const MAX_DEPTH : u32 = 14;
        const MAX_BUFFER_SIZE : u32 = 64;

        let payer: Keypair = match Keypair::from_bytes(&payer_keypair) {
            Ok(keypair) => keypair,
            Err(_)=>return Err(rustler::Error::Term(Box::new(invalid_keypair())))  
        };

        let merkle_tree_pubkey_sdk  = match Pubkey::from_str(merkle_tree_pubkey){
            Ok(pubkey )=>pubkey,
            Err(_)=>return Err(rustler::Error::Term(Box::new(invalid_pubkey())))  
        };  
        // let merkle_tree_pubkey_prog :solana_program::pubkey::Pubkey = solana_program::pubkey::Pubkey::new_from_array(merkle_tree_pubkey_sdk.to_bytes());

        // account size calculation
        let account_size = CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1 + 
            std::mem::size_of::<ConcurrentMerkleTree<{ MAX_DEPTH as usize }, { MAX_BUFFER_SIZE as usize }>>();

        // Generate PDA for tree config
        let (tree_config_pda, _) = TreeConfig::find_pda(&merkle_tree_pubkey_sdk);

        // Get required program IDs - use defaults or override from options
        // let log_wrapper = match &options {
        //     Some(opts) if opts.log_wrapper.is_some() => {
        //         match Pubkey::from_str(opts.log_wrapper.as_ref().unwrap()) {
        //             Ok(pubkey) => pubkey,
        //             Err(_) => return Err(rustler::Error::Term(Box::new(invalid_pubkey())))
        //         }
        //     },
        //     _ => Pubkey::from_str(SPL_NOOP_ID.to_string().as_str()).unwrap()
        // };
        let compression_program = match &options {
            Some(opts) if opts.compression_program.is_some() => {
                match Pubkey::from_str(opts.compression_program.as_ref().unwrap()) {
                    Ok(pubkey) => pubkey,
                    Err(_) => return Err(rustler::Error::Term(Box::new(invalid_pubkey())))
                }
            },
            _ => Pubkey::from_str(&SPL_ACCOUNT_COMPRESSION_ID.to_string()).unwrap()
        };

        // let system_program_id = match &options {
        //     Some(opts) if opts.system_program.is_some() => {
        //         match Pubkey::from_str(opts.system_program.as_ref().unwrap()) {
        //             Ok(pubkey) => pubkey,
        //             Err(_) => return Err(rustler::Error::Term(Box::new(invalid_pubkey())))
        //         }
        //     },
        //     _ => system_program::ID
        // };

        // Get public flag from options or default to true
        let is_public = match &options {
            Some(opts) => opts.public.unwrap_or(true),
            None => true
        };
        let rpc_client = RpcClient::new(rpc_url);
        let rent = rpc_client.get_minimum_balance_for_rent_exemption(account_size)
           .map_err(|_| rustler::Error::Term(Box::new(rpc_error())))?;
        // Build accounts
        let create_account_ix = CreateAccount(
            &payer.pubkey(),
            &merkle_tree_pubkey_sdk,
            rent,
            account_size as u64, 
            &compression_program
        );

        let create_tree_ix = CreateTreeConfigBuilder::new()
            .tree_config(tree_config_pda)
            .merkle_tree(merkle_tree_pubkey_sdk)
            .payer(payer.pubkey())
            .tree_creator(payer.pubkey())
            .max_depth(MAX_DEPTH as u32)
            .max_buffer_size(MAX_BUFFER_SIZE as u32)
            .instruction();

        // let instruction_program_id= solana_program::pubkey::Pubkey::new_from_array(create_tree_ix.program_id.to_bytes());

        // Convert AccountMeta vec between crate versions
        // let converted_accounts = create_tree_ix.accounts.into_iter().map(|meta| {
        //     solana_sdk::instruction::AccountMeta {
        //         pubkey: solana_sdk::pubkey::Pubkey::new_from_array(meta.pubkey.to_bytes()),
        //         is_signer: meta.is_signer,
        //         is_writable: meta.is_writable,
        //     }
        // }).collect();
        // Convert `create_tree_ix`(solana_program::instruction::Instruction) to `solana_instruction::Instruction`
        // let create_tree_ix_2 = SolanaInstruction{
        //     program_id: create_tree_ix.program_id,
        //     accounts: create_tree_ix.accounts,
        //     data: create_tree_ix.data,
        // };
           

        // Build transaction
        let recent_blockhash = rpc_client.get_latest_blockhash()
            .map_err(|_| rustler::Error::Term(Box::new(rpc_error())))?;

        let transaction = Transaction::new_signed_with_payer(
            &[create_account_ix, create_tree_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        // Serialize
        let serialized_tx = serialize(&transaction)
            .map_err(|_| rustler::Error::Term(Box::new(serialization_error())))?;

        Ok((ok(), bs58::encode(serialized_tx).into_string()))
}
