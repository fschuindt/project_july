defmodule Streamer.Application do
  @moduledoc false

  use Application

  def start(_type, _args) do
    import Supervisor.Spec, warn: false

    children = [
      supervisor(GRPC.Server.Supervisor, [{
        [Streamer.Servers.Broadcast, Streamer.Servers.Tune],
        7171,
        idle_timeout: 10000
      }])
    ]

    opts = [strategy: :one_for_one, name: Streamer.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
