defmodule MplBubblegumEx.Creator do
  @moduledoc """
  Represents a creator of an NFT.
  """
  defstruct [
    :address,    # Base58 encoded Solana public key
    :verified,   # Boolean indicating if the creator is verified
    :share       # Percentage share (0-100) of royalties
  ]

  @type t :: %__MODULE__{
    address: String.t(),
    verified: boolean(),
    share: integer()
  }
end

defmodule MplBubblegumEx.Collection do
  @moduledoc """
  Represents a collection that an NFT belongs to.
  """
  defstruct [
    :verified,   # Boolean indicating if the collection is verified
    :key         # Base58 encoded Solana public key of the collection
  ]

  @type t :: %__MODULE__{
    verified: boolean(),
    key: String.t()
  }
end

defmodule MplBubblegumEx.Uses do
  @moduledoc """
  Represents the usage configuration for an NFT.
  """
  defstruct [
    :use_method,   # Atom: :burn, :multiple, or :single
    :remaining,    # Integer representing remaining uses
    :total         # Integer representing total uses
  ]

  @type use_method :: :burn | :multiple | :single
  @type t :: %__MODULE__{
    use_method: use_method(),
    remaining: non_neg_integer(),
    total: non_neg_integer()
  }
end

defmodule MplBubblegumEx.TokenStandard do
  @type t :: :non_fungible | :fungible_asset | :fungible | :non_fungible_edition
end

defmodule MplBubblegumEx.MetadataArgs do
  @moduledoc """
  Represents the metadata for a compressed NFT.
  """
  defstruct [
    :name,                    # Name of the NFT
    :symbol,                  # Symbol/ticker for the NFT
    :uri,                     # URI to the JSON metadata
    :seller_fee_basis_points, # Royalty fee in basis points (100 = 1%)
    :primary_sale_happened,   # Whether the primary sale has happened
    :is_mutable,              # Whether the NFT can be updated after minting
    :edition_nonce,           # Edition nonce (optional)
    :token_standard,          # Token standard (optional)
    :collection,              # Collection info (optional)
    :uses,                    # Uses info (optional)
    :creators                 # List of creators
  ]

  @type t :: %__MODULE__{
    name: String.t(),
    symbol: String.t(),
    uri: String.t(),
    seller_fee_basis_points: integer(),
    primary_sale_happened: boolean(),
    is_mutable: boolean(),
    edition_nonce: integer() | nil,
    token_standard: MplBubblegumEx.TokenStandard.t() | nil,
    collection: MplBubblegumEx.Collection.t() | nil,
    uses: MplBubblegumEx.Uses.t() | nil,
    creators: [MplBubblegumEx.Creator.t()]
  }

  @doc """
  Creates a new MetadataArgs struct with sane defaults.
  """
  def new(name, uri, creators \\ []) do
    %__MODULE__{
      name: name,
      symbol: "",
      uri: uri,
      seller_fee_basis_points: 0,
      primary_sale_happened: false,
      is_mutable: true,
      edition_nonce: nil,
      token_standard: :non_fungible,
      collection: nil,
      uses: nil,
      creators: creators
    }
  end

  @doc """
  Validates a metadata struct.
  """
  def validate(%__MODULE__{} = metadata) do
    with :ok <- validate_name(metadata.name),
         :ok <- validate_uri(metadata.uri),
         :ok <- validate_creators(metadata.creators) do
      {:ok, metadata}
    end
  end

  defp validate_name(name) when byte_size(name) > 0 and byte_size(name) <= 32, do: :ok
  defp validate_name(_), do: {:error, "Name must be between 1 and 32 bytes"}

  defp validate_uri(uri) when byte_size(uri) > 0 and byte_size(uri) <= 200, do: :ok
  defp validate_uri(_), do: {:error, "URI must be between 1 and 200 bytes"}

  defp validate_creators([]), do: :ok
  defp validate_creators(creators) do
    # Check if all creators have valid addresses
    invalid_addresses = Enum.filter(creators, fn c ->
      !MplBubblegumEx.Native.validate_pubkey_nif(c.address)
    end)

    if length(invalid_addresses) > 0 do
      {:error, "Invalid creator addresses: #{inspect(Enum.map(invalid_addresses, & &1.address))}"}
    else
      # Check if shares add up to 100
      total_shares = Enum.reduce(creators, 0, fn c, acc -> acc + c.share end)
      if total_shares != 100, do: {:error, "Creator shares must sum to 100, got #{total_shares}"}, else: :ok
    end
  end
end

defmodule MplBubblegumEx.MintOptions do
  @moduledoc """
  Options for minting NFTs.
  """
  defstruct [
    :mint_to_collection,              # Boolean - whether to mint to a collection
    :collection_key,                  # Base58 pubkey of the collection
    :collection_authority,            # Binary - keypair of collection authority
    :collection_authority_record_pda  # Base58 pubkey of authority record (if delegated)
  ]

  @type t :: %__MODULE__{
    mint_to_collection: boolean() | nil,
    collection_key: String.t() | nil,
    collection_authority: binary() | nil,
    collection_authority_record_pda: String.t() | nil
  }
end
