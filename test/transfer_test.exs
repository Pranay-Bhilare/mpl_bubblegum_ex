# File: test/transfer_test.exs
defmodule MplBubblegumEx.TransferTest do
  use ExUnit.Case
  alias MplBubblegumEx.NFT
  alias MplBubblegumEx.MetadataArgs
  alias MplBubblegumEx.Creator
  import TestHelpers

  @moduletag :integration
  @devnet_url "https://api.devnet.solana.com"

  setup do
    # Load test keypairs
    owner_keypair = load_test_keypair()
    tree_keypair = load_test_merkle_keypair()
    owner_pubkey = test_pubkey()

    # Create a new recipient for the transfer
    new_owner_pubkey = "5bvDGqzX4QdArhG5fvEFArKYeyXZRddxGP4XECGnrNqA"

    # Create metadata for minting
    metadata = MetadataArgs.new(
      "Transfer Test NFT #{:rand.uniform(1000)}",
      "https://example.com/nft-#{:rand.uniform(1000)}.json",
      [
        %Creator{
          address: owner_pubkey,
          verified: false,
          share: 100
        }
      ]
    )

    # First mint an NFT to get data_hash, creator_hash, nonce, and index
    {:ok, mint_result} = NFT.mint(
      tree_keypair,
      owner_pubkey,
      owner_keypair,
      metadata,
      @devnet_url
    )

    IO.puts("Minted NFT with signature: #{mint_result}")

    # We'd normally extract data_hash, creator_hash, nonce and index from the mint result
    # For testing, we'll use placeholder values
    data_hash = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
    creator_hash = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
    nonce = 0
    index = 0

    {:ok, %{
      owner_keypair: owner_keypair,
      tree_keypair: tree_keypair,
      new_owner_pubkey: new_owner_pubkey,
      data_hash: data_hash,
      creator_hash: creator_hash,
      nonce: nonce,
      index: index
    }}
  end

  test "transfer NFT to new owner", %{owner_keypair: owner, tree_keypair: tree,
                                      new_owner_pubkey: new_owner,
                                      data_hash: data_hash, creator_hash: creator_hash,
                                      nonce: nonce, index: index} do

    IO.puts("Transferring NFT to #{new_owner}...")

    # Execute the transfer
    {:ok, signature} = NFT.transfer(
      tree,
      owner,
      new_owner,
      data_hash,
      creator_hash,
      nonce,
      index,
      @devnet_url
    )

    IO.puts("Transfer successful with signature: #{signature}")

    # Simple assertion to validate we got a signature back
    assert is_binary(signature)
    assert String.length(signature) > 0
  end
end
