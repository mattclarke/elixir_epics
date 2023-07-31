defmodule ElixirEpics.Monitor do
  use GenServer
  require Logger

  @wrapper "run_wrapper"
  @command "/opt/epics/base/bin/darwin-aarch64/pvmonitor"

  def start_link(args \\ [], opts \\ []) do
    GenServer.start_link(__MODULE__, args, opts)
  end

  def init(_args) do
    Process.flag(:trap_exit, true)

    port =
      Port.open({:spawn_executable, @wrapper}, [
        :binary,
        args: [@command, "-M", "json", "SIMPLE:VALUE2"]
      ])

    Port.monitor(port)

    Logger.info("pid = #{Port.info(port)[:os_pid]}")

    {:ok, %{port: port, connected: False, latest_data: %{}, exit_status: nil}}
  end

  def terminate(reason, %{port: _port} = _state) do
    Logger.warning("Terminated: #{reason}")
    :normal
  end

  # Triggered when the port uses STDOUT
  def handle_info({port, {:data, text_line}}, %{port: port} = state) do
    [pv, payload] = String.split(text_line, " ", parts: 2)
    {status, data} = Jason.decode(payload)

    Logger.info("#{pv}")

    case status do
      :ok ->
        updated = Map.merge(state.latest_data, data)
        Logger.info("Data: #{inspect(updated)}")
        generate_flatbuffer_for_double(pv, updated)
        {:noreply, %{state | latest_data: updated, connected: true}}

      :error ->
        Logger.info("Payload: #{inspect(payload)}")
        {:noreply, %{state | latest_data: %{}, connected: false}}
    end
  end

  # Triggered when the port exits normally
  def handle_info({port, {:exit_status, status}}, %{port: port} = state) do
    Logger.info("Port exit: :exit_status: #{status}")

    new_state = %{state | exit_status: status}

    {:noreply, new_state}
  end

  # Triggered when the port crashes
  def handle_info({:DOWN, _ref, :port, port, :normal}, state) do
    Logger.info("Handled :DOWN message from port: #{inspect(port)}")
    {:stop, "port disappeared", state}
  end

  # Triggered when the process is asked to exit
  def handle_info({:EXIT, _port, :normal}, state) do
    Logger.warning("handle_info: EXIT")
    {:noreply, state}
  end

  def handle_info(msg, state) do
    Logger.info("Unhandled message: #{inspect(msg)}")
    {:noreply, state}
  end

  defp generate_flatbuffer_for_double(pv, data) do
    %{"secondsPastEpoch" => seconds, "nanoseconds" => nanoseconds} = data["timeStamp"]
    timestamp = seconds * 1_000_000_000 + nanoseconds
    buffer = FlatBuffers.convert_flatbuffer_double(pv, timestamp, data["value"])
    Logger.info("FlatBuffer: #{inspect(buffer)}")
  end
end
