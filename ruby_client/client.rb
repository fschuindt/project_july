require 'securerandom'

require_relative 'syrinx_services_pb'
require_relative 'broadcast_client'
require_relative 'tune_client'

SyrinxClient::Broadcast.new.perform if ARGV.first == '--broadcast'
SyrinxClient::Tune.new.perform if ARGV.first == '--tune'

