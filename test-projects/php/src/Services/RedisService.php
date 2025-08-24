<?php

namespace App\Services;

use Predis\Client;

/**
 * Redis service for handling caching and other Redis operations
 */
class RedisService
{
    /**
     * @var Client
     */
    private Client $redis;
    
    /**
     * @var string
     */
    private string $prefix;
    
    /**
     * Constructor
     */
    public function __construct()
    {
        $this->prefix = $_ENV['REDIS_PREFIX'] ?? 'php_sample_app:';
        
        // Initialize Redis client with configuration from environment variables
        $this->redis = new Client([
            'scheme'   => 'tcp',
            'host'     => $_ENV['REDIS_HOST'] ?? 'localhost',
            'port'     => $_ENV['REDIS_PORT'] ?? 6379,
            'password' => $_ENV['REDIS_PASSWORD'] !== 'null' ? $_ENV['REDIS_PASSWORD'] : null,
        ]);
    }
    
    /**
     * Get Redis client instance
     *
     * @return Client
     */
    public function getClient(): Client
    {
        return $this->redis;
    }
    
    /**
     * Set a key-value pair in Redis with optional expiration
     *
     * @param string $key
     * @param mixed $value
     * @param int|null $ttl TTL in seconds, null for no expiration
     * @return bool
     */
    public function set(string $key, mixed $value, ?int $ttl = null): bool
    {
        $key = $this->prefixKey($key);
        
        if (is_array($value) || is_object($value)) {
            $value = json_encode($value);
        }
        
        if ($ttl !== null) {
            return $this->redis->setex($key, $ttl, $value) === 'OK';
        }
        
        return $this->redis->set($key, $value) === 'OK';
    }
    
    /**
     * Get a value by key from Redis
     *
     * @param string $key
     * @param bool $decode Whether to json_decode the value if it's a valid JSON string
     * @return mixed
     */
    public function get(string $key, bool $decode = true): mixed
    {
        $key = $this->prefixKey($key);
        $value = $this->redis->get($key);
        
        if ($value !== null && $decode) {
            $decoded = json_decode($value, true);
            if (json_last_error() === JSON_ERROR_NONE) {
                return $decoded;
            }
        }
        
        return $value;
    }
    
    /**
     * Delete a key from Redis
     *
     * @param string $key
     * @return bool
     */
    public function delete(string $key): bool
    {
        $key = $this->prefixKey($key);
        return (bool) $this->redis->del([$key]);
    }
    
    /**
     * Check if a key exists in Redis
     *
     * @param string $key
     * @return bool
     */
    public function exists(string $key): bool
    {
        $key = $this->prefixKey($key);
        return (bool) $this->redis->exists($key);
    }
    
    /**
     * Set expiration time for a key
     *
     * @param string $key
     * @param int $ttl TTL in seconds
     * @return bool
     */
    public function expire(string $key, int $ttl): bool
    {
        $key = $this->prefixKey($key);
        return (bool) $this->redis->expire($key, $ttl);
    }
    
    /**
     * Cache a value with auto-expiry - useful for frequently accessed data
     *
     * @param string $key
     * @param mixed $value
     * @param int $ttl TTL in seconds, default 1 hour
     * @return bool
     */
    public function cache(string $key, mixed $value, int $ttl = 3600): bool
    {
        return $this->set('cache:' . $key, $value, $ttl);
    }
    
    /**
     * Get a cached value, if it doesn't exist call the callback to generate it
     *
     * @param string $key
     * @param callable $callback Function to generate the value if not in cache
     * @param int $ttl TTL in seconds, default 1 hour
     * @return mixed
     */
    public function remember(string $key, callable $callback, int $ttl = 3600): mixed
    {
        $key = 'cache:' . $key;
        $value = $this->get($key);
        
        if ($value === null) {
            $value = $callback();
            $this->set($key, $value, $ttl);
        }
        
        return $value;
    }
    
    /**
     * Add prefix to a key
     *
     * @param string $key
     * @return string
     */
    private function prefixKey(string $key): string
    {
        return $this->prefix . $key;
    }
}