defmodule Streamer.Band do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          uid: String.t(),
          key: String.t()
        }
  defstruct [:uid, :key]

  field :uid, 1, type: :string
  field :key, 2, type: :string
end

defmodule Streamer.TuneRequest do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          band: Streamer.Band.t()
        }
  defstruct [:band]

  field :band, 1, type: Streamer.Band
end

defmodule Streamer.Video do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          index: String.t(),
          chunk: String.t()
        }
  defstruct [:index, :chunk]

  field :index, 1, type: :string
  field :chunk, 2, type: :bytes
end

defmodule Streamer.BroadcastResponse do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          success: boolean,
          band: Streamer.Band.t()
        }
  defstruct [:success, :band]

  field :success, 1, type: :bool
  field :band, 2, type: Streamer.Band
end

defmodule Streamer.Broadcast.Service do
  @moduledoc false
  use GRPC.Service, name: "Streamer.Broadcast"

  rpc :Broadcast, stream(Streamer.Video), Streamer.BroadcastResponse
end

defmodule Streamer.Broadcast.Stub do
  @moduledoc false
  use GRPC.Stub, service: Streamer.Broadcast.Service
end

defmodule Streamer.Tune.Service do
  @moduledoc false
  use GRPC.Service, name: "Streamer.Tune"

  rpc :Tune, Streamer.TuneRequest, stream(Streamer.Video)
end

defmodule Streamer.Tune.Stub do
  @moduledoc false
  use GRPC.Stub, service: Streamer.Tune.Service
end
