# MPL Bubblegum NIF for Elixir

A Native Implemented Function (NIF) for interacting with the Metaplex Bubblegum protocol for compressed NFTs on Solana.

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

## Usage

[Add usage examples here]

## Development

This project uses a Rust NIF to interact with Solana. The Cargo.lock file is committed to the repository to ensure reproducible builds.

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/mpl_bubblegum_ex>.

