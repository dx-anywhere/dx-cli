require 'redis'

# Main Redis connection (for general purpose use)
$redis = Redis.new(url: ENV.fetch("REDIS_URL") { "redis://localhost:6379/0" })

# Set up Redis for Rails cache store
Rails.application.config.cache_store = :redis_cache_store, {
  url: ENV.fetch("REDIS_CACHE_URL") { "redis://localhost:6379/1" },
  expires_in: 1.day,
  namespace: "cache"
}

# Set up Redis for Sidekiq if Sidekiq is present
if defined?(Sidekiq)
  Sidekiq.configure_server do |config|
    config.redis = { url: ENV.fetch("REDIS_SIDEKIQ_URL") { "redis://localhost:6379/2" } }
  end

  Sidekiq.configure_client do |config|
    config.redis = { url: ENV.fetch("REDIS_SIDEKIQ_URL") { "redis://localhost:6379/2" } }
  end
end

# Log a message to indicate Redis initialization
Rails.logger.info "Redis initialized with URL: #{ENV.fetch("REDIS_URL") { "redis://localhost:6379/0" }}"

# Perform a health check
begin
  $redis.ping
  Rails.logger.info "Redis connection successful!"
rescue Redis::CannotConnectError => e
  Rails.logger.error "Could not connect to Redis: #{e.message}"
end