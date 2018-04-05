defmodule ReedSolomonErasure.Native do
  use Rustler, otp_app: :reed_solomon_erasure, crate: :reed_solomon_erasure

  def encode(_k, _msg), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
