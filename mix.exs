defmodule ElixirEpics.MixProject do
  use Mix.Project

  def project do
    [
      app: :elixir_epics,
      version: "0.1.0",
      elixir: "~> 1.15",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger, :observer, :wx, :runtime_tools],
      mod: {ElixirEpics.Application, []}
    ]
  end

  defp deps do
    [
      {:jason, "~> 1.4"},
    ]
  end
end
