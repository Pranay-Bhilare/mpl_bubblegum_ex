# MPL Bubblegum NIF for Elixir

A Native Implemented Function (NIF) for interacting with the Metaplex Bubblegum protocol for compressed NFTs on Solana.

## Overview

MPL Bubblegum Ex provides Elixir developers with a way to interact with Solana's compressed NFTs using the Metaplex Bubblegum protocol. This library uses Rustler to create NIFs (Native Implemented Functions) that bridge Elixir and the Solana blockchain.

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `mpl_bubblegum_ex` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:mpl_bubblegum_ex, "~> 0.1.0"}
  ]
end
```

2. Ensure Rust is installed on your system (version 1.65.0 or later recommended)
3. Run `mix deps.get` followed by `mix compile`

The Rust NIF will be automatically compiled during the Elixir compilation process.

## Features

- Create compressed NFT trees
- Mint compressed NFTs
- Transfer compressed NFTs
- And more...

## Testing

Before running tests, ensure that you have added the necessary keypairs in the test/test_helper.exs file. These keypairs are essential for interacting with the Solana blockchain during testing. You can generate keypairs using the Solana CLI or any other tool that supports Solana keypair generation.

Example setup in test/test_helper.exs :

```elixir
ExUnit.start()
ExUnit.configure(exclude: [])

defmodule TestHelpers do
  @moduledoc """
  Helper functions for testing
  """

  alias MplBubblegumEx.Solana

  # Fixed test public key for all tests (valid Solana address)
  @test_pubkey "YOUR VALID SOLANA ADDRESS IN STRING FORMAT"

  # Fixed test keypair for when we need to sign transactions
  # This is a dummy keypair that corresponds to the public key above
  @test_keypair <<52,80,184,118,95,99,182,55,126,126,157,204,175,130,236,84,202,189,215,58,78,71,69,155,191,100,39,1,37,215,236,63,191,141,76,137,4,35,10,73,108,27,246,207,221,54,121,47,92,184,197,4,114,101,106,110,90,171,203,174,71,131,197,65>>

  # Fixed test keypair for the merkle tree
  # This is a dummy keypair
  @test_merkle_tree_keypair <<71,104,75,149,222,180,43,60,128,144,58,81,45,223,121,155,109,123,90,68,143,51,85,120,245,191,129,123,193,127,254,82,211,41,220,213,77,0,30,206,163,39,124,40,239,23,250,45,214,187,107,68,241,148,86,203,180,73,208,6,54,229,130,54>>

# ... rest of code ...
```
### Running Tests
To run the tests, use the following commands in your terminal:

```bash

```bash
# Run tree creation test
mix test test/tree_creation_test.exs
 ```

```bash
# Run cNFT minting test
mix test test/nft_test.exs
```

```bash
# Run transfer cNFT test
mix test test/transfer_test.exs
 ```

```bash
# Run tests with detailed output
mix test --trace
 ```
Ensure that your test account is funded with enough SOL for the tests to execute successfully. The ensure_funded_account/2 function in TestHelpers can help with this by requesting an airdrop if necessary.

For confirming the tests the easiest way is to use Xray Xplorer : 
1. Go to Xray Explorer
2. Connect to Devnet using the network dropdown
3. Search for your tree address (the merkle_tree_pubkey associatwed with your merkle_tree keypair that you used)
4. You'll see all assets in that tree including your newly minted NFT
![Compressed NFT Verification](Screenshot_2025-03-13_150643.png)
5. Click on the NFT to see all its metadata details



## Development

This project uses a Rust NIF to interact with Solana. The Cargo.lock file is committed to the repository to ensure reproducible builds.

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/mpl_bubblegum_ex>.

