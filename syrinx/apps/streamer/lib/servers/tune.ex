defmodule Streamer.Servers.Tune do
  require Logger

  use GRPC.Server, service: Streamer.Tune.Service

  alias Streamer, as: S

  def tune(request, stream) do
    Logger.info "Started tunning | Band UID: #{request.band.uid} | Band Key: #{request.band.key}"

    do_tune(stream, 1)
  end

  defp do_tune(stream, index) do
    video = random_video(index)
    GRPC.Server.send_reply(stream, video)
    Logger.info "Replied -> index: #{index}, chunk: #{video.chunk}"

    do_tune(stream, index + 1)
  end

  defp random_video(index) do
    S.Video.new(
      index: Integer.to_string(index),
      chunk: random_chunk())
  end

  defp random_chunk do
    SecureRandom.uuid
  end
end
