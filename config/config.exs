# config/dev.exs
import Config

config :mpl_bubblegum_ex,
  rpc_url: "https://api.devnet.solana.com",
  payer_key: System.get_env("DEV_PAYER_KEY")
