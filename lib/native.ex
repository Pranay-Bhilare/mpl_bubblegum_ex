# Just for environment testing purpose
defmodule MplBubblegumEx.Native do
  use Rustler,
    otp_app: :mpl_bubblegum_ex,
    crate: "mpl_bubblegum_native",
    mode: if(Mix.env() == :prod, do: :release, else: :debug)

  def validate_pubkey_nif(_pubkey), do: :erlang.nif_error(:nif_not_loaded)
  def validate_keypair_nif(_keypair), do: :erlang.nif_error(:nif_not_loaded)

  def create_tree_config_tx(_max_depth,_max_buffer_size,_payer_keypair,_merkle_tree_pubkey,_rpc_url), do: :erlang.nif_error(:nif_not_loaded)
end
