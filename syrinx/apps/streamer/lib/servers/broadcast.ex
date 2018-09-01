defmodule Streamer.Servers.Broadcast do
  use GRPC.Server, service: Streamer.Broadcast.Service

  alias Streamer, as: S
  alias Streamer.CrudeLogger, as: Logger

  def broadcast(request, _stream) do
    do_broadcast(request, null_band())
  end

  defp do_broadcast(request, band) do
    Logger.info "Started broadcast | Band UID: #{band.uid} | Band Key: #{band.key}"

    Enum.map(request, &(handle/1))

    S.BroadcastResponse.new(success: true, band: band)
  end

  defp handle(video) do
    Logger.info "index: #{video.index}, chunk: #{video.chunk}"
  end

  defp null_band do
    S.Band.new(uid: "000", key: "111")
  end
end
