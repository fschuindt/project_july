module SyrinxClient
  class Tune
    def initialize
      @stub = Streamer::Tune::Stub
        .new('localhost:7171', :this_channel_is_insecure)
    end

    def perform
      @stub.tune(tune_request) { |e| puts e.inspect }
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
