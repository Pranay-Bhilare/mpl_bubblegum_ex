# File: test/nft_test.exs
defmodule MplBubblegumEx.NFTTest do
  use ExUnit.Case
  alias MplBubblegumEx.NFT
  alias MplBubblegumEx.MetadataArgs
  alias MplBubblegumEx.Creator
  alias MplBubblegumEx.Solana
  import TestHelpers

  @moduletag :integration
  @devnet_url "https://api.devnet.solana.com"

  setup do
    # Ensure we have a funded account for testing
    {:ok, balance} = ensure_funded_account(@devnet_url)
    IO.puts("Using test account with #{balance} SOL")

    # Use predefined keypairs instead of generating new ones
    payer_keypair = load_test_keypair()
    tree_keypair = load_test_merkle_keypair()
    owner_pubkey = test_pubkey()

    # Create test metadata
    metadata = MetadataArgs.new(
      "Test NFT #{:rand.uniform(1000)}",
      "https://example.com/nft-metadata-#{:rand.uniform(1000)}.json",
      [
        %Creator{
          address: owner_pubkey,
          verified: false,
          share: 100
        }
      ]
    )

    # First we need to make sure the tree exists - create it if not already created
    # This will be safe to retry even if the tree already exists
    # {:ok, _} = MplBubblegumEx.Tree.create_tree_config(3, 8, payer_keypair, tree_keypair, @devnet_url)
    # Wait a moment for tree to propagate
    :timer.sleep(2000)

    {:ok, %{
      payer_keypair: payer_keypair,
      tree_keypair: tree_keypair,
      owner_pubkey: owner_pubkey,
      metadata: metadata
    }}
  end

  test "successful NFT minting", %{payer_keypair: payer, tree_keypair: tree_keypair, owner_pubkey: owner, metadata: metadata} do
    # Mint NFT
    {:ok, tx} = NFT.mint(
      tree_keypair,
      owner,
      payer,
      metadata,
      @devnet_url
    )

    # Check transaction confirms successfully
    assert {:ok, %{status: "confirmed"}} = Solana.confirm_transaction(tx, @devnet_url)

    # Note: Unlike regular NFTs, compressed NFTs don't have their own accounts
    # that we can directly query. The state is stored in the merkle tree.
    # So this is sufficient to verify the transaction succeeded.
  end
end
