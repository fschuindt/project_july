module SyrinxClient
  class Tune
    def initialize
      @stub = Streamer::Tune::Stub
        .new('localhost:7172', :this_channel_is_insecure)
    end

    def perform
      msg = @stub.tune(tune_request)

      p "#{msg.inspect}"
    end

    private

    def tune_request
      Streamer::TuneRequest.new(
        band: null_band
      )
    end

    def null_band
      Streamer::Band.new(
        uid: "000",
        key: "syrinx"
      )
    end
  end
end
