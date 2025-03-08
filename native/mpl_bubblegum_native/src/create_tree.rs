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
use crate::valid_depth_size_pairs::{is_valid_pair, get_valid_pairs_string};

rustler::atoms!{
    ok,
    error,
    invalid_keypair,
    invalid_pubkey,
    rpc_error,
    serialization_error,
    instruction_error,
    invalid_tree_parameters
}

#[derive(NifStruct)]
#[module = "MplBubblegumEx.TreeOptions"]

struct TreeOptions{
    public: Option<bool>,
    log_wrapper:Option<String>,
    compression_program:Option<String>,
    system_program:Option<String>
}

// Helper function with const generics for depth and buffer size
fn create_tree_with_const<const MAX_DEPTH: usize, const MAX_BUFFER_SIZE: usize>(
    payer: Keypair, 
    merkle_tree_pubkey : Pubkey, 
    rpc_url : &str, 
    options : Option<TreeOptions>
) -> Result<(rustler::Atom,String), rustler::Error> {

        let account_size = CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1 + 
            std::mem::size_of::<ConcurrentMerkleTree<{ MAX_DEPTH }, { MAX_BUFFER_SIZE }>>();

        // Generate PDA for tree config
        let (tree_config_pda, _) = TreeConfig::find_pda(&merkle_tree_pubkey);

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
            &merkle_tree_pubkey,
            rent,
            account_size as u64, 
            &compression_program
        );

        let create_tree_ix = CreateTreeConfigBuilder::new()
            .tree_config(tree_config_pda)
            .merkle_tree(merkle_tree_pubkey)
            .payer(payer.pubkey())
            .tree_creator(payer.pubkey())
            .max_depth(MAX_DEPTH as u32)
            .max_buffer_size(MAX_BUFFER_SIZE as u32)
            .instruction();

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

#[rustler::nif]
pub fn create_tree_config_tx(
    max_depth: u32,
    max_buffer_size: u32,
    payer_keypair: Vec<u8>,
    merkle_tree_pubkey_str: &str,
    rpc_url: &str,
    options : Option<TreeOptions>
    ) -> Result<(rustler::Atom,String), rustler::Error>{
        
        if !is_valid_pair(max_depth, max_buffer_size) {
            let valid_pairs = get_valid_pairs_string();
            return Err(rustler::Error::Term(Box::new(format!(
                "Invalid depth/buffer combination: ({}, {}). Valid combinations are: {}",
                max_depth, max_buffer_size, valid_pairs
            ))));
        }
        let payer: Keypair = match Keypair::from_bytes(&payer_keypair) {
            Ok(keypair) => keypair,
            Err(_)=>return Err(rustler::Error::Term(Box::new(invalid_keypair())))  
        };

        let merkle_tree_pubkey  = match Pubkey::from_str(merkle_tree_pubkey_str){
            Ok(pubkey )=>pubkey,
            Err(_)=>return Err(rustler::Error::Term(Box::new(invalid_pubkey())))  
        };

        macro_rules! generate_tree_config_match {
            ($(($depth:expr, $buffer:expr)),* $(,)?) => {
                match (max_depth, max_buffer_size) {
                    $(
                        ($depth, $buffer) => create_tree_with_const::<{$depth as usize}, {$buffer as usize}>(
                            payer, merkle_tree_pubkey, rpc_url, options
                        ),
                    )*
                    _ => Err(rustler::Error::Term(Box::new(invalid_tree_parameters())))
                }
            };
        }

        generate_tree_config_match!(
            // Small buffers
            (3, 8), (5, 8),
            // 16-byte buffers
            (6, 16), (7, 16), (8, 16), (9, 16),
            // 32-byte buffers
            (10, 32), (11, 32), (12, 32), (13, 32),
            // 64-byte buffers
            (14, 64), (15, 64), (16, 64), (17, 64), (18, 64), (19, 64), (20, 64), (24, 64),
            // 256-byte buffers
            (14, 256), (20, 256), (24, 256),
            // 512-byte buffers
            (24, 512), (26, 512), (30, 512),
            // 1024-byte buffers
            (14, 1024), (20, 1024), (24, 1024), (26, 1024), (30, 1024),
            // 2048-byte buffers
            (14, 2048), (20, 2048), (24, 2048), (26, 2048), (30, 2048),
        )
}
