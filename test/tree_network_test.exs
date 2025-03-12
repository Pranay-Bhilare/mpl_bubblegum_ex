defmodule MplBubblegumEx.TreeNetworkTest do
  use ExUnit.Case
  alias MplBubblegumEx.Tree
  alias MplBubblegumEx.Solana
  import TestHelpers

  @moduletag :network
  @devnet_url "https://api.devnet.solana.com"

  setup do
    # Ensure we have a funded account for testing
    {:ok, balance} = ensure_funded_account(@devnet_url)
    IO.puts("Using test account with #{balance} SOL")

    # Generate a random pubkey for the merkle tree
    tree_keypair = Solana.create_keypair()
    tree_pubkey = Solana.pubkey_from_keypair(tree_keypair)

    %{
      keypair: load_test_keypair(),
      tree_pubkey: tree_pubkey
    }
  end

  test "can create and submit a small tree transaction", %{keypair: keypair, tree_pubkey: tree_pubkey} do
    # Create the transaction
    {:ok, tx} = Tree.create_tree_config(3, 8, keypair, tree_pubkey, @devnet_url)

    # Submit the transaction
    {:ok, signature} = Solana.submit_transaction(tx, @devnet_url)
    assert is_binary(signature)

    # Wait for confirmation
    {:ok, confirmed_signature} = Solana.wait_for_confirmation(signature, @devnet_url)
    assert confirmed_signature == signature
  end

  test "can create and submit a medium tree transaction", %{keypair: keypair, tree_pubkey: tree_pubkey} do
    # Create the transaction
    {:ok, tx} = Tree.create_tree_config(14, 64, keypair, tree_pubkey, @devnet_url)

    # Submit the transaction
    {:ok, signature} = Solana.submit_transaction(tx, @devnet_url)
    assert is_binary(signature)

    # Wait for confirmation
    {:ok, confirmed_signature} = Solana.wait_for_confirmation(signature, @devnet_url)
    assert confirmed_signature == signature
  end

  # This test is more expensive in terms of SOL, so we might want to skip it in regular testing
  @tag :expensive
  test "can create and submit a large tree transaction", %{keypair: keypair, tree_pubkey: tree_pubkey} do
    # Create the transaction
    {:ok, tx} = Tree.create_tree_config(24, 256, keypair, tree_pubkey, @devnet_url)

    # Submit the transaction
    {:ok, signature} = Solana.submit_transaction(tx, @devnet_url)
    assert is_binary(signature)

    # Wait for confirmation
    {:ok, confirmed_signature} = Solana.wait_for_confirmation(signature, @devnet_url)
    assert confirmed_signature == signature
  end
end
