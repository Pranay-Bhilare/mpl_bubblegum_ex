ExUnit.start()
ExUnit.configure(exclude: [])

defmodule TestHelpers do
  @moduledoc """
  Helper functions for testing
  """

  alias MplBubblegumEx.Solana

  # Fixed test public key for all tests (valid Solana address)
  @test_pubkey "DtjqUcS2m7TQNR1J8rdBMv7UfUsXBedw9191rHjuEJba"

  # Fixed test keypair for when we need to sign transactions
  # This is a dummy keypair that corresponds to the public key above
  @test_keypair <<52,80,184,118,95,99,182,55,126,126,157,204,175,130,236,84,202,189,215,58,78,71,69,155,191,100,39,1,37,215,236,63,191,141,76,137,4,35,10,73,108,27,246,207,221,54,121,47,92,184,197,4,114,101,106,110,90,171,203,174,71,131,197,65>>
  @test_merkle_tree_keypair <<71,104,75,149,222,180,43,60,128,144,58,81,45,223,121,155,109,123,90,68,143,51,85,120,245,191,129,123,193,127,254,82,211,41,220,213,77,0,30,206,163,39,124,40,239,23,250,45,214,187,107,68,241,148,86,203,180,73,208,6,54,229,130,54>>
  @doc """
  Returns the fixed test public key
  """
  def test_pubkey do
    @test_pubkey
  end

  @doc """
  Returns the fixed test keypair
  """
  def load_test_keypair do
    @test_keypair
  end

  @doc """
  Returns the predefined merkle tree keypair
  """
  def load_test_merkle_keypair do
    @test_merkle_tree_keypair
  end

  @doc """
  Ensures the test account has enough SOL for testing
  """
  def ensure_funded_account(rpc_url, min_sol \\ 0.5) do
    pubkey = test_pubkey()

    case Solana.get_balance(pubkey, rpc_url) do
      {:ok, balance} when balance < min_sol ->
        # Request airdrop to get enough SOL
        {:ok, signature} = Solana.request_airdrop(pubkey, min_sol, rpc_url)
        {:ok, _} = Solana.wait_for_confirmation(signature, rpc_url)
        {:ok, balance + min_sol}

      {:ok, balance} ->
        {:ok, balance}

      error ->
        error
    end
  end
end
