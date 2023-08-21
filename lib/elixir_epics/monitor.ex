defmodule ElixirEpics.Monitor do
  use GenServer, restart: :transient
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
        args: [Application.fetch_env!(:elixir_epics, :pvmonitor), "-M", "raw", pvname]
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
       cached_value: nil,
       cached_alarm: nil,
       cached_connection: nil,
       partial_message: ""
     }}
  end

  def terminate(reason, %{port: _port} = _state) do
    Logger.warning("Terminated: #{reason}")
    :normal
  end

  # Triggered by a timer
  def handle_info(:update, state) do
    if state.connected do
      send_to_kafka(state.cached_value, state.topic)
      send_to_kafka(state.cached_alarm, state.topic)
    end

    send_to_kafka(state.cached_connection, state.topic)

    schedule_update()
    {:noreply, state}
  end

  # Triggered when the port uses STDOUT
  def handle_info({port, {:data, text_line}}, %{port: port} = state) do
    cond do
      String.contains?(text_line, "<Disconnect>") ->
        Logger.info("#{state.pvname} is disconnected!")
        {:noreply, on_disconnect(state)}

      String.ends_with?(text_line, "=====\n") ->
        Logger.info("Update for #{state.pvname}")
        message = state.partial_message <> text_line
        data = extract_epics_data(message)
        data = Map.merge(state.latest_data, data)

        new_state =
          %{state | partial_message: ""}
          |> handle_value_update(data)
          |> on_connect(data)
          |> handle_alarm_update(data)

        {:noreply, new_state}

      true ->
        # incomplete message
        Logger.info("start of update for #{state.pvname}")
        message = state.partial_message <> text_line
        new_state = %{state | partial_message: message}
        {:noreply, new_state}
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

  defp extract_epics_data(message) do
    String.split(message, "\n")
    |> Enum.reduce(%{}, fn x, acc ->
      case String.trim(x) do
        "double value " <> value ->
          # Put the type in too?
          {result, _} = Float.parse(value)
          Map.put(acc, "value", result)

        "long[] value " <> value ->
          # Store the raw value as we will do the conversion in Rust
          Map.put(acc, "value", value)

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
  end

  defp generate_f144_for_double(pvname, data) do
    %{"secondsPastEpoch" => seconds, "nanoseconds" => nanoseconds, "value" => value} = data
    timestamp_ns = seconds * 1_000_000_000 + nanoseconds
    buffer = FlatBuffers.convert_to_f144_double(pvname, timestamp_ns, value)
    timestamp_ms = trunc(timestamp_ns / 1_000_000)
    {timestamp_ms, buffer}
  end

  defp generate_f144_for_long_array(pvname, data) do
    %{"secondsPastEpoch" => seconds, "nanoseconds" => nanoseconds, "value" => value} = data
    timestamp_ns = seconds * 1_000_000_000 + nanoseconds
    buffer = FlatBuffers.convert_to_f144_long_array(pvname, timestamp_ns, value)
    timestamp_ms = trunc(timestamp_ns / 1_000_000)
    {timestamp_ms, buffer}
  end

  defp send_to_kafka({timestamp_ms, buffer}, topic) do
    :brod.produce_sync(:kafka_client, topic, :hash, <<>>, {timestamp_ms, buffer})
  end

  defp on_disconnect(state) do
    # TODO: some duplication!
    # On disconnect we don't get a timestamp from EPICS so we need to generate it ourselves
    timestamp_ns = System.os_time()
    buffer = FlatBuffers.convert_to_ep01(state.pvname, timestamp_ns, 3)
    timestamp_ms = trunc(timestamp_ns / 1_000_000)
    send_to_kafka({timestamp_ms, buffer}, state.topic)

    %{
      state
      | connected: false,
        cached_value: nil,
        cached_alarm: nil,
        cached_connection: {timestamp_ms, buffer},
        latest_data: %{}
    }
  end

  defp on_connect(state, data) do
    # TODO: Don't update/send if there is no change

    # TODO: some duplication!
    %{
      "secondsPastEpoch" => seconds,
      "nanoseconds" => nanoseconds
    } = data

    timestamp_ns = seconds * 1_000_000_000 + nanoseconds
    buffer = FlatBuffers.convert_to_ep01(state.pvname, timestamp_ns, 2)
    timestamp_ms = trunc(timestamp_ns / 1_000_000)
    send_to_kafka({timestamp_ms, buffer}, state.topic)
    %{state | connected: true, cached_connection: {timestamp_ms, buffer}}
  end

  defp handle_alarm_update(state, data) do
    # TODO: Don't update/send if there is no change

    %{
      "secondsPastEpoch" => seconds,
      "nanoseconds" => nanoseconds,
      "severity" => severity,
      "message" => message
    } = data

    timestamp_ns = seconds * 1_000_000_000 + nanoseconds
    buffer = FlatBuffers.convert_to_al00(state.pvname, timestamp_ns, severity, message)
    timestamp_ms = trunc(timestamp_ns / 1_000_000)
    send_to_kafka({timestamp_ms, buffer}, state.topic)
    %{state | cached_alarm: {timestamp_ms, buffer}}
  end

  defp handle_value_update(state, data) do
    # TODO: Don't update/send if there is no change

    result = generate_f144_for_double(state.pvname, data)
    send_to_kafka(result, state.topic)
    state = on_first_connection(state)
    %{state | latest_data: data, cached_value: result}
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
