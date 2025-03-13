# Just for environment testing purpose
defmodule MplBubblegumEx.Native do
  use Rustler,
    otp_app: :mpl_bubblegum_ex,
    crate: "mpl_bubblegum_native",
    mode: if(Mix.env() == :prod, do: :release, else: :debug)

  def validate_pubkey_nif(_pubkey), do: :erlang.nif_error(:nif_not_loaded)
  def validate_keypair_nif(_keypair), do: :erlang.nif_error(:nif_not_loaded)
  def create_tree_config_tx(_max_depth,_max_buffer_size,_payer_keypair,_merkle_tree_pubkey,_rpc_url),
    do: :erlang.nif_error(:nif_not_loaded)
  def mint_compressed_nft_tx(_tree_pubkey, _leaf_owner_pubkey, _payer_keypair, _metadata, _rpc_url),
    do: :erlang.nif_error(:nif_not_loaded)
  def mint_to_collection_tx(_tree_pubkey, _leaf_owner_pubkey, _payer_keypair, _metadata,_collection_mint, _collection_authority, _collection_authority_record_pda, _rpc_url),
    do: :erlang.nif_error(:nif_not_loaded)
end
