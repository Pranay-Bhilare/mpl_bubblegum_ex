defmodule MplBubblegumEx.TreeIntegrationTest do
  use ExUnit.Case
  alias MplBubblegumEx.Tree
  alias MplBubblegumEx.Solana
  import TestHelpers

  @moduletag :integration
  @devnet_url "https://api.devnet.solana.com"

  setup do
    # Ensure we have a funded account for testing
    {:ok, balance} = ensure_funded_account(@devnet_url)
    IO.puts("Using test account with #{balance} SOL")

    # Use predefined keypair instead of generating new one
    # tree_keypair = Solana.create_keypair()
    # tree_pubkey = Solana.pubkey_from_keypair(tree_keypair)
    payer_keypair = load_test_keypair()
    tree_keypair = load_test_merkle_keypair()
    tree_pubkey = Solana.pubkey_from_keypair(tree_keypair)

    {:ok, %{
      tree_keypair: tree_keypair,
      tree_pubkey: tree_pubkey,
      payer_keypair: payer_keypair
    }}
  end

  test "successful tree creation", %{payer_keypair: payer, tree_keypair: tree_keypair} do
    {:ok, tx} = Tree.create_tree_config(
      3,
      8,
      payer,
      tree_keypair,
      @devnet_url
    )

    tree_pubkey = Solana.pubkey_from_keypair(tree_keypair)

    # Directly check transaction confirmation
    assert {:ok, %{status: "confirmed"}} = Solana.confirm_transaction(tx, @devnet_url)

    # Verify tree account exists with correct program owner
    assert {:ok, tree_account} = Solana.get_account(tree_pubkey, @devnet_url)
    assert tree_account.owner == "DtjqUcS2m7TQNR1J8rdBMv7UfUsXBedw9191rHjuEJba"
  end
end
