class CrudeLogger
  def self.info(msg)
    puts "INFO [#{timestamp}]: #{msg}"
  end

  def self.timestamp
    Time.now.utc.strftime("%Y-%m-%d %H:%M:%S")
  end
end
