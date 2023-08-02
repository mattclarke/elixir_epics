defmodule ElixirEpics.MonitorSupervisor do
  use DynamicSupervisor

  def start_link(init_arg) do
    DynamicSupervisor.start_link(__MODULE__, init_arg, name: __MODULE__)
  end

  @impl true
  def init(_init_arg) do
    DynamicSupervisor.init(strategy: :one_for_one)
  end

  def start_child(init_args) do
    # spec = %{id: ElixirEpics.Monitor, start: {ElixirEpics.Monitor, :start_link, init_args}}
    DynamicSupervisor.start_child(__MODULE__, {ElixirEpics.Monitor, init_args})
  end
end
