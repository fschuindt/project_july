defmodule Streamer.Servers.Broadcast do
  require Logger

  use GRPC.Server, service: Streamer.Broadcast.Service

  alias Streamer, as: S

  def broadcast(request, _stream) do
    Logger.info "Started broadcast | Video Index: #{request.index} | Video Chunk: #{request.chunk}"

    S.BroadcastResponse.new(success: true, band: null_band())
  end

  defp null_band() do
    S.Band.new(uid: "000", key: "111")
  end
end
