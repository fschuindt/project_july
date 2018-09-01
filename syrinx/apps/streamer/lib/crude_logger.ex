defmodule Streamer.CrudeLogger do
  def info(msg) when is_binary(msg) do
    IO.puts("INFO " <> timestamp() <> msg)
  end

  defp timestamp do
    Timex.now
    |> Timex.format("%Y-%m-%d %H:%M:%S", :strftime)
    |> stamp()
  end

  defp stamp({:ok, time}), do: "[#{time}]: "
  defp stamp(_), do: "[NOTIMEDATA]: "
end

