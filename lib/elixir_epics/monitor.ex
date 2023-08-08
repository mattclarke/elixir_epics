defmodule ElixirEpics.Monitor do
  use GenServer
  require Logger

  @wrapper "run_wrapper"

  def start_link(args \\ [], opts \\ []) do
    GenServer.start_link(__MODULE__, args, opts)
  end

  def init({pvname, topic, _schema}) do
    Process.flag(:trap_exit, true)

    port =
      Port.open({:spawn_executable, @wrapper}, [
        :binary,
        args: [Application.fetch_env!(:pv_monitor, :path), "-M", "raw", pvname]
      ])

    Port.monitor(port)

    Logger.info("pid = #{Port.info(port)[:os_pid]}")

    {:ok,
     %{
       port: port,
       connected: False,
       latest_data: %{},
       exit_status: nil,
       has_connected: false,
       pvname: pvname,
       topic: topic,
       cached_value: nil
     }}
  end

  def terminate(reason, %{port: _port} = _state) do
    Logger.warning("Terminated: #{reason}")
    :normal
  end

  # Triggered by a timer
  def handle_info(:update, state) do
    if state.connected do
      send_to_kafka(state.cached_value, "test_topic")
    end

    schedule_update()
    {:noreply, state}
  end

  # Triggered when the port uses STDOUT
  def handle_info({port, {:data, text_line}}, %{port: port} = state) do
    # TODO: handle disconnection
    data =
      String.split(text_line, "\n")
      |> Enum.reduce(%{}, fn x, acc ->
        case String.trim(x) do
          "double value " <> value ->
            # Put the type in too?
            {result, _} = Float.parse(value)
            Map.put(acc, "value", result)

          "int severity " <> value ->
            Map.put(acc, "severity", String.to_integer(value))

          "int status " <> value ->
            Map.put(acc, "status", String.to_integer(value))

          "string message " <> value ->
            Map.put(acc, "message", value)

          "long secondsPastEpoch " <> value ->
            Map.put(acc, "secondsPastEpoch", String.to_integer(value))

          "int nanoseconds " <> value ->
            Map.put(acc, "nanoseconds", String.to_integer(value))

          _ ->
            acc
        end
      end)

    Logger.info("Updated for #{state.pvname}")

    {:noreply, handle_value_update(state, data)}
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

  defp generate_f144_for_double(pvname, data) do
    %{"secondsPastEpoch" => seconds, "nanoseconds" => nanoseconds, "value" => value} = data
    timestamp_ns = seconds * 1_000_000_000 + nanoseconds
    buffer = FlatBuffers.convert_to_f144_double(pvname, timestamp_ns, value)
    timestamp_ms = trunc(timestamp_ns / 1_000_000)
    {timestamp_ms, buffer}
  end

  defp generate_alOO(pvname, data) do
    %{
      "secondsPastEpoch" => seconds,
      "nanoseconds" => nanoseconds,
      "severity" => severity,
      "message" => message
    } = data

    timestamp_ns = seconds * 1_000_000_000 + nanoseconds
    buffer = FlatBuffers.convert_to_al00(pvname, timestamp_ns, severity, message)
    timestamp_ms = trunc(timestamp_ns / 1_000_000)
    {timestamp_ms, buffer}
  end

  defp send_to_kafka({timestamp_ms, buffer}, topic) do
    :brod.produce_sync(:kafka_client, topic, :hash, <<>>, {timestamp_ms, buffer})
  end

  defp handle_value_update(state, data) do
    updated = Map.merge(state.latest_data, data)
    Logger.info("Data: #{inspect(updated)}")
    result = generate_f144_for_double(state.pvname, updated)
    send_to_kafka(result, state.topic)
    # TODO: handle alarms properly
    foo = generate_alOO(state.pvname, updated)
    send_to_kafka(foo, state.topic)
    state = on_first_connection(state)
    %{state | latest_data: updated, connected: true, cached_value: result}
  end

  defp on_first_connection(state) do
    %{has_connected: is_first_time} = state

    case is_first_time do
      false ->
        state
        schedule_update()
        %{state | has_connected: true}

      true ->
        state
    end
  end

  defp schedule_update() do
    Process.send_after(self(), :update, 5000)
  end
end
