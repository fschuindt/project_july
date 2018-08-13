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
      index = 1

      loop do
        v = video(index)

        p v.inspect
        yield v

        index += 1
      end
    end

    private

    def video(index)
      Streamer::Video.new(
        index: index.to_s,
        chunk: SecureRandom.uuid
      )
    end
  end
end
