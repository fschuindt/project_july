defmodule Streamer.Servers.Broadcast do
  require Logger

  use GRPC.Server, service: Streamer.Broadcast.Service

  alias Streamer, as: S

  def broadcast(request, _stream) do
    do_broadcast(request, null_band())
  end

  defp do_broadcast(request, band) do
    Logger.info "Started broadcast | Band UID: #{band.uid} | Band Key: #{band.key}"

    Enum.map(request, &(handle(&1)))
  end

  defp handle(video) do
    Logger.info "Gathered -> index: #{video.index}, chunk: #{video.chunk}"
  end

  defp null_band do
    S.Band.new(uid: "000", key: "111")
  end
end
