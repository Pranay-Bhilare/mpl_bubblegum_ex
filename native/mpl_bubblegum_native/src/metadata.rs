use mpl_bubblegum::types::{
    Creator, MetadataArgs, TokenProgramVersion, TokenStandard, Collection, Uses, UseMethod
};
use rustler::{NifStruct, NifUnitEnum};
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

// NIF struct to receive creator data from Elixir
#[derive(NifStruct)]
#[module = "MplBubblegumEx.Creator"]
pub struct CreatorNif {
    pub address: String,
    pub verified: bool,
    pub share: u8,
}

// NIF struct to receive collection data from Elixir
#[derive(NifStruct)]
#[module = "MplBubblegumEx.Collection"]
pub struct CollectionNif {
    pub verified: bool,
    pub key: String,
}

// NIF enum for token standard
#[derive(NifUnitEnum)]
pub enum TokenStandardNif {
    NonFungible,
    FungibleAsset,
    Fungible,
    NonFungibleEdition,
}

// NIF struct to receive use data from Elixir
#[derive(NifStruct)]
#[module = "MplBubblegumEx.Uses"]
pub struct UsesNif {
    pub use_method: UseMethodNif,
    pub remaining: u64,
    pub total: u64,
}

// NIF enum for use method
#[derive(NifUnitEnum)]
pub enum UseMethodNif {
    Burn,
    Multiple,
    Single,
}

// NIF struct for metadata args from Elixir
#[derive(NifStruct)]
#[module = "MplBubblegumEx.MetadataArgs"]
pub struct MetadataArgsNif {
    pub name: String,   
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub edition_nonce: Option<u8>,
    pub token_standard: Option<TokenStandardNif>,
    pub collection: Option<CollectionNif>,
    pub uses: Option<UsesNif>,
    pub creators: Vec<CreatorNif>,
}

// Convert Elixir metadata to mpl-bubblegum MetadataArgs
pub fn convert_metadata_args(args: &MetadataArgsNif) -> Result<MetadataArgs, String> {
    // Convert creators
    let creators: Vec<Creator> = args.creators
        .iter()
        .map(|c| {
            let address = match Pubkey::from_str(&c.address) {
                Ok(pubkey) => pubkey,
                Err(_) => return Err(format!("Invalid creator address: {}", c.address)),
            };
            
            Ok(Creator {
                address,
                verified: c.verified,
                share: c.share,
            })
        })
        .collect::<Result<Vec<Creator>, String>>()?;
    
    // Validate creator shares
    let total_shares: u16 = creators.iter().map(|c| c.share as u16).sum();
    if !creators.is_empty() && total_shares != 100 {
        return Err(format!("Creator shares must sum to 100, got {}", total_shares));
    }
    
    // Convert collection if present
    let collection = args.collection.as_ref().map(|c| {
        let key = match Pubkey::from_str(&c.key) {
            Ok(pubkey) => pubkey,
            Err(_) => return Err(format!("Invalid collection key: {}", c.key)),
        };
        
        Ok(Collection {
            verified: c.verified,
            key,
        })
    }).transpose()?;
    
    // Convert uses if present
    let uses = args.uses.as_ref().map(|u| {
        let use_method = match u.use_method {
            UseMethodNif::Burn => UseMethod::Burn,
            UseMethodNif::Multiple => UseMethod::Multiple,
            UseMethodNif::Single => UseMethod::Single,
        };
        
        Uses {
            use_method,
            remaining: u64::try_from(u.remaining).unwrap(),
            total: u64::try_from(u.total).unwrap(),
        }
    });
    
    // Convert token standard if present
    let token_standard = args.token_standard.as_ref().map(|ts| {
        match ts {
            TokenStandardNif::NonFungible => TokenStandard::NonFungible,
            TokenStandardNif::FungibleAsset => TokenStandard::FungibleAsset,
            TokenStandardNif::Fungible => TokenStandard::Fungible,
            TokenStandardNif::NonFungibleEdition => TokenStandard::NonFungibleEdition,
        }
    });
    
    Ok(MetadataArgs {
        name: args.name.clone(),
        symbol: args.symbol.clone(),
        uri: args.uri.clone(),
        seller_fee_basis_points: args.seller_fee_basis_points,
        primary_sale_happened: args.primary_sale_happened,
        is_mutable: args.is_mutable,
        edition_nonce: args.edition_nonce,
        token_standard,
        collection,
        uses,
        token_program_version:TokenProgramVersion::Original,
        creators,
    })
}
