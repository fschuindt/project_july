module SyrinxClient
  class Broadcast
    def initialize
      @stub = Streamer::Broadcast::Stub
                .new('localhost:7171', :this_channel_is_insecure)
    end

    def perform
      reqs = RandomVideo.new(1024)
      resp = @stub.broadcast(reqs.each)
      p "response: #{resp.inspect}"
    end
  end

  class RandomVideo
    def initialize(size)
      @size = size
    end

    def each
      return enum_for(:each) unless block_given?
      @size.times do
        p video.inspect
        yield video
        sleep(rand(1..2))
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
