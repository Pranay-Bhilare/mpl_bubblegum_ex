defmodule MplBubblegumEx.NFT do
  @moduledoc """
  Functions for creating and managing compressed NFTs.
  """

  alias MplBubblegumEx.Native
  alias MplBubblegumEx.MetadataArgs

  @doc """
  Mints a new compressed NFT and confirms the transaction on Solana.

  ## Parameters
    - `tree_keypair`: Merkle tree keypair(Binary)
    - `leaf_owner_pubkey`: NFT owner pubkey (string)
    - `payer_keypair`: 64-byte keypair (binary)
    - `metadata`: a %MetadataArgs{} struct
    - `rpc_url`: Solana RPC URL

  ## Returns
    - `{:ok, signature}` on success
    - `{:error, reason}` on failure
  """
  def mint(
    tree_keypair,
    leaf_owner_pubkey,
    payer_keypair,
    %MetadataArgs{} = metadata,
    rpc_url
  ) do
    with {:ok, _} <- validate_keypair(tree_keypair),
         {:ok, _} <- validate_pubkey(leaf_owner_pubkey),
         {:ok, _} <- validate_keypair(payer_keypair),
         {:ok, _} <- MetadataArgs.validate(metadata) do
      # This already handles submission AND confirmation
      Native.mint_compressed_nft_tx(
        tree_keypair,
        leaf_owner_pubkey,
        payer_keypair,
        metadata,
        rpc_url
      )
    else
      error -> error
    end
  end

  @doc """
  Mints a new NFT to a collection and confirms the transaction on Solana.

  ## Parameters
    - `tree_keypair`: Merkle tree keypair(Binary)
    - `leaf_owner_pubkey`: NFT owner pubkey (string)
    - `payer_keypair`: 64-byte keypair (binary)
    - `metadata`: a %MetadataArgs{} struct
    - `collection_mint`: Collection mint pubkey (string)
    - `collection_authority`: 64-byte keypair for collection authority (binary)
    - `collection_authority_record_pda` (optional): PDA string (if required)
    - `rpc_url`: Solana RPC URL

  ## Returns
    - `{:ok, signature}` on success
    - `{:error, reason}` on failure
  """
  def mint_to_collection(
    tree_keypair,
    leaf_owner_pubkey,
    payer_keypair,
    %MetadataArgs{} = metadata,
    collection_mint,
    collection_authority,
    collection_authority_record_pda \\ nil,
    rpc_url
  ) do
    with {:ok, _} <- validate_pubkey(leaf_owner_pubkey),
         {:ok, _} <- validate_pubkey(collection_mint),
         {:ok, _} <- validate_keypair(payer_keypair),
         {:ok, _} <- validate_keypair(collection_authority),
         {:ok, _} <- validate_optional_pubkey(collection_authority_record_pda),
         {:ok, _} <- MetadataArgs.validate(metadata) do
      # This already handles submission AND confirmation
      Native.mint_to_collection_tx(
        tree_keypair,
        leaf_owner_pubkey,
        payer_keypair,
        metadata,
        collection_mint,
        collection_authority,
        collection_authority_record_pda,
        rpc_url
      )
    else
      error -> error
    end
  end

   @doc """
  Transfers a compressed NFT to a new owner.

  ## Parameters
    - `tree_keypair`: Keypair of the merkle tree (binary format)
    - `leaf_owner_keypair`: Keypair of the current owner (binary format)
    - `new_leaf_owner_pubkey`: Public key of the new owner (string)
    - `asset_id`: Asset ID of the NFT (string)
    - `options`: Optional parameters (root, data_hash, creator_hash, nonce, index)
    - `rpc_url`: Solana RPC URL

  ## Returns
    - `{:ok, signature}` on success
    - `{:error, reason}` on failure
  """
  def transfer(
    tree_keypair,
    leaf_owner_keypair,
    new_leaf_owner_pubkey,
    asset_id,
    options \\ %{},
    rpc_url
  ) do
    with {:ok, _} <- validate_keypair(tree_keypair),
         {:ok, _} <- validate_keypair(leaf_owner_keypair),
         {:ok, _} <- validate_pubkey(new_leaf_owner_pubkey),
         {:ok, _} <- validate_pubkey(asset_id) do

      root = Map.get(options, :root)
      data_hash = Map.get(options, :data_hash)
      creator_hash = Map.get(options, :creator_hash)
      nonce = Map.get(options, :nonce)
      index = Map.get(options, :index)

      Native.transfer_compressed_nft(
        tree_keypair,
        leaf_owner_keypair,
        new_leaf_owner_pubkey,
        asset_id,
        root,
        data_hash,
        creator_hash,
        nonce,
        index,
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

  defp validate_optional_pubkey(nil), do: {:ok, nil}
  defp validate_optional_pubkey(pubkey), do: validate_pubkey(pubkey)

  defp validate_keypair(keypair) do
    if byte_size(keypair) == 64,
      do: {:ok, keypair},
      else: {:error, "Keypair must be 64 bytes"}
  end
end
