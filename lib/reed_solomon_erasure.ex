defmodule ReedSolomonErasure do
  def encode(k, msg) do
    ReedSolomonErasure.Native.encode(k, msg)
  end
end
