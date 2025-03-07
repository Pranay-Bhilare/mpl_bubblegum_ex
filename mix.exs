defmodule MplBubblegumEx.MixProject do
  use Mix.Project

  def project do
    [
      app: :mpl_bubblegum_ex,
      version: "0.1.0",
      elixir: "~> 1.18",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger, :crypto]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.36.1"},

      {:jason, "~> 1.4"},
      {:ex_doc, "~> 0.27", only: :dev, runtime: false}
    ]
  end

end
