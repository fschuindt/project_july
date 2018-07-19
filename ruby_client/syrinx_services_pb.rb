# Generated by the protocol buffer compiler.  DO NOT EDIT!
# Source: syrinx.proto for package 'Streamer'

require 'grpc'
require_relative 'syrinx_pb'

module Streamer
  module Broadcast
    class Service

      include GRPC::GenericService

      self.marshal_class_method = :encode
      self.unmarshal_class_method = :decode
      self.service_name = 'Streamer.Broadcast'

      rpc :Broadcast, stream(Video), BroadcastResponse
    end

    Stub = Service.rpc_stub_class
  end
  module Tune
    class Service

      include GRPC::GenericService

      self.marshal_class_method = :encode
      self.unmarshal_class_method = :decode
      self.service_name = 'Streamer.Tune'

      rpc :Tune, TuneRequest, stream(Video)
    end

    Stub = Service.rpc_stub_class
  end
end
