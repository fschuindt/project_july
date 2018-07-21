module SyrinxClient
  class Broadcast
    def initialize
      @stub = Streamer::Broadcast::Stub
                .new('localhost:7171', :this_channel_is_insecure)
    end

    def perform
      reqs = RandomVideo.new
      resp = @stub.broadcast(reqs.each)
      p "response: #{resp.inspect}"
    end
  end

  class RandomVideo
    def initialize; end

    def each
      return enum_for(:each) unless block_given?
      99999.times do
      # loop do
        v = video

        p v.inspect
        yield v
      end
    end

    private

    def video
      Streamer::Video.new(
        index: rand(0..255).to_s,
        chunk: SecureRandom.uuid
      )
    end
  end
end
