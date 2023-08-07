defmodule FlatBuffers do
  use Rustler, otp_app: :elixir_epics, crate: "flatbuffers"

  # When the NIF is loaded, it will override these functions.
  def convert_to_f144_double(_source, _timestamp, _value),
    do: :erlang.nif_error(:nif_not_loaded)

  def convert_to_al00(_source, _timestamp, _severity, _message),
    do: :erlang.nif_error(:nif_not_loaded)
end
