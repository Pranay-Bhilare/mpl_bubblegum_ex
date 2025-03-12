defmodule MplBubblegumEx.Solana do
  @moduledoc """
  Helper functions for Solana RPC interactions
  """

  @doc """
  Creates a new keypair
  """
  def create_keypair do
    :crypto.strong_rand_bytes(64)
  end

  @doc """
  Extracts the public key from a keypair and returns it as a base58 string
  """
  # def pubkey_from_keypair(keypair) do
  #   # Extract the public key (last 32 bytes of the keypair)
  #   <<_secret::binary-size(32), pubkey::binary-size(32)>> = keypair
  #   Base58.encode(pubkey)
  # end
  def pubkey_from_keypair(keypair) do
    <<_secret::binary-size(32), pubkey::binary-size(32)>> = keypair
    Base58.encode(:binary.decode_unsigned(pubkey))  # Convert binary to integer first
  end

  @doc """
  Submits a base58-encoded transaction to Solana
  """
  def submit_transaction(tx_base58, rpc_url) do
    params = [tx_base58, %{"encoding" => "base58"}]

    response = HTTPoison.post!(
      rpc_url,
      Jason.encode!(%{
        jsonrpc: "2.0",
        id: 1,
        method: "sendTransaction",
        params: params
      }),
      [{"Content-Type", "application/json"}]
    )

    case Jason.decode!(response.body) do
      %{"result" => signature} -> {:ok, signature}
      %{"error" => error} -> {:error, error}
    end
  end

  @doc """
  Gets the status of a transaction
  """
  def get_transaction_status(signature, rpc_url) do
    response = HTTPoison.post!(
      rpc_url,
      Jason.encode!(%{
        jsonrpc: "2.0",
        id: 1,
        method: "getTransaction",
        params: [signature, %{"encoding" => "json"}]
      }),
      [{"Content-Type", "application/json"}]
    )

    case Jason.decode!(response.body) do
      %{"result" => nil} -> {:error, :not_found}
      %{"result" => %{"meta" => %{"err" => nil}}} -> {:ok, :confirmed}
      %{"result" => %{"meta" => %{"err" => err}}} -> {:error, err}
      %{"error" => error} -> {:error, error}
    end
  end

  @doc """
  Requests an airdrop of SOL to a public key
  """
  def request_airdrop(pubkey, amount_sol, rpc_url) do
    # Convert SOL to lamports (1 SOL = 1 billion lamports)
    lamports = trunc(amount_sol * 1_000_000_000)

    response = HTTPoison.post!(
      rpc_url,
      Jason.encode!(%{
        jsonrpc: "2.0",
        id: 1,
        method: "requestAirdrop",
        params: [pubkey, lamports]
      }),
      [{"Content-Type", "application/json"}]
    )

    case Jason.decode!(response.body) do
      %{"result" => signature} -> {:ok, signature}
      %{"error" => error} -> {:error, error}
    end
  end

  @doc """
  Gets the balance of a public key in SOL
  """
  def get_balance(pubkey, rpc_url) do
    response = HTTPoison.post!(
      rpc_url,
      Jason.encode!(%{
        jsonrpc: "2.0",
        id: 1,
        method: "getBalance",
        params: [pubkey]
      }),
      [{"Content-Type", "application/json"}]
    )

    case Jason.decode!(response.body) do
      %{"result" => %{"value" => lamports}} ->
        sol = lamports / 1_000_000_000
        {:ok, sol}
      %{"error" => error} -> {:error, error}
    end
  end

  @doc """
  Waits for a transaction to be confirmed
  """
  def get_transaction_status(signature, rpc_url) do
    case confirm_transaction(signature, rpc_url) do
      {:ok, %{status: "confirmed"}} -> {:ok, :confirmed}
      {:ok, %{status: "failed"}} -> {:error, :transaction_failed}
      error -> error
    end
  end

  def wait_for_confirmation(signature, rpc_url, timeout_ms \\ 30_000) do
    start_time = System.monotonic_time(:millisecond)

    check_confirmation = fn ->
      case get_transaction_status(signature, rpc_url) do
        {:ok, :confirmed} -> {:ok, signature}
        {:error, :not_found} ->
          if System.monotonic_time(:millisecond) - start_time > timeout_ms do
            {:error, :timeout}
          else
            :timer.sleep(1000)
            :continue
          end
        error -> error
      end
    end

    # Keep checking until we get a final result or timeout
    do_wait_for_confirmation(check_confirmation)
  end

  defp do_wait_for_confirmation(check_fn) do
    case check_fn.() do
      :continue -> do_wait_for_confirmation(check_fn)
      result -> result
    end
  end

  def confirm_transaction(signature, url) do
      response = HTTPoison.post!(
        url,
        Jason.encode!(%{
          jsonrpc: "2.0",
          id: 1,
          method: "getTransaction",
          params: [
            signature,
            %{"encoding" => "json", "commitment" => "confirmed"}
          ]
        }),
        [{"Content-Type", "application/json"}]
      )

      case Jason.decode!(response.body) do
        %{"result" => response} when not is_nil(response) ->
          {:ok, process_transaction_response(response)}
        %{"result" => nil} ->
          {:error, :not_found}
        error ->
          error
      end
    end

    def get_account(pubkey, url) do
        response = HTTPoison.post!(
          url,
          Jason.encode!(%{
            jsonrpc: "2.0",
            id: 1,
            method: "getAccountInfo",
            params: [
              pubkey,
              %{"encoding" => "jsonParsed"}
            ]
          }),
          [{"Content-Type", "application/json"}]
        )

        case Jason.decode!(response.body) do
          %{"result" => %{"value" => account_info}} when not is_nil(account_info) ->
            {:ok, process_account_response(account_info)}
          %{"result" => %{"value" => nil}} ->
            {:error, :account_not_found}
          %{"error" => error} ->
            {:error, error}
        end
      end
    defp process_transaction_response(response) do
      %{
        status: if(response["meta"]["err"] == nil, do: "confirmed", else: "failed"),
        instructions: response["transaction"]["message"]["instructions"],
        error: response["meta"]["err"]
      }
    end

    # Helper to process account response
    defp process_account_response(response) do
      %{
        owner: response["owner"],
        lamports: response["lamports"],
        data: response["data"]
      }
    end
end
