defmodule ElixirEpics.Application do
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    :observer.start()

    children = [
      {ElixirEpics.Monitor, []}
    ]

    opts = [strategy: :one_for_one, name: ElixirEpics.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
