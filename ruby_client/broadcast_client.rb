module SyrinxClient
  class Broadcast
    def initialize
      @stub = Streamer::Broadcast::Stub
        .new('localhost:7172', :this_channel_is_insecure)
    end

    def perform
      msg = @stub.broadcast([video, video])

      p "#{msg.inspect}"
    end

    private

    def video
      Streamer::Video.new(
        index: "000",
        chunk: "0b1011010010"
      )
    end
  end
end
