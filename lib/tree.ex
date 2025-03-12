# defmodule MplBubblegumEx.TreeOptions do
#   @moduledoc """
#   Options for tree configuration.
#   """
# #   defstruct [
# #     public: nil,
# #     log_wrapper: nil,
# #     compression_program: nil,
# #     system_program: nil
# #   ]
# end

defmodule MplBubblegumEx.Tree do
  @moduledoc """
  Functions for creating and managing compressed NFT Merkle Trees.
  """
  alias MplBubblegumEx.Native

  @doc """
  Creates a new Merkle Tree configuration on Solana.

  ## Parameters
  - `max_depth`: Maximum depth of the Merkle tree (e.g., 14).
  - `max_buffer_size`: Maximum buffer size (e.g., 64).
  - `payer_keypair`: Keypair of the payer (binary format).
  - `merkle_tree_pubkey`: Public key of the Merkle tree (string).
  - `rpc_url`: Solana RPC URL (e.g., "https://api.devnet.solana.com").
  - `options`: Optional parameters for tree configuration:
    - `:public` - Boolean indicating if the tree is public (default: true)
    - `:log_wrapper` - Custom log wrapper program ID
    - `:compression_program` - Custom compression program ID
    - `:system_program` - Custom system program ID

  ## Returns
  `{:ok, tx_signature}` or `{:error, reason}`
  """
  def create_tree_config(
    max_depth,
    max_buffer_size,
    payer_keypair,
    merkle_tree_keypair,
    rpc_url
  ) do

    # Validate inputs before calling Rust function
    with {:ok, _} <- validate_keypair(merkle_tree_keypair),
         {:ok, _} <- validate_keypair(payer_keypair)
    do
      Native.create_tree_config_tx(
        max_depth,
        max_buffer_size,
        payer_keypair,
        merkle_tree_keypair,
        rpc_url
      )
    else
      error -> error
    end
  end
  defp validate_pubkey(pubkey) do
    if Native.validate_pubkey_nif(pubkey),
      do: {:ok, pubkey},
      else: {:error, "Invalid pubkey: #{pubkey}"}
  end
  defp validate_keypair(keypair) do
    if byte_size(keypair) == 64,
      do: {:ok, keypair},
      else: {:error, "Keypair must be 64 bytes"}
  end
end
