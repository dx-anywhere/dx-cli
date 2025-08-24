<?php

namespace App\Models;

use App\Services\RedisService;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Collection;

/**
 * User model
 * 
 * @property int $id
 * @property string $username
 * @property string $email
 * @property string $password
 * @property \DateTime $created_at
 * @property \DateTime $updated_at
 */
class User extends Model
{
    /**
     * The table associated with the model.
     *
     * @var string
     */
    protected $table = 'users';

    /**
     * The attributes that are mass assignable.
     *
     * @var array<string>
     */
    protected $fillable = [
        'username',
        'email',
        'password',
    ];

    /**
     * The attributes that should be hidden for serialization.
     *
     * @var array<string>
     */
    protected $hidden = [
        'password',
    ];

    /**
     * Redis service instance
     *
     * @var RedisService|null
     */
    private static ?RedisService $redisService = null;

    /**
     * Get Redis service instance
     *
     * @return RedisService
     */
    private static function getRedisService(): RedisService
    {
        if (self::$redisService === null) {
            self::$redisService = new RedisService();
        }

        return self::$redisService;
    }

    /**
     * Find a user by ID with Redis caching
     *
     * @param int $id
     * @return User|null
     */
    public static function findCached(int $id): ?User
    {
        $redis = self::getRedisService();
        $cacheKey = "user:{$id}";

        // Try to get from cache first
        return $redis->remember($cacheKey, function () use ($id) {
            return self::find($id);
        }, 3600); // Cache for 1 hour
    }

    /**
     * Find a user by email with Redis caching
     *
     * @param string $email
     * @return User|null
     */
    public static function findByEmail(string $email): ?User
    {
        $redis = self::getRedisService();
        $cacheKey = "user:email:{$email}";

        return $redis->remember($cacheKey, function () use ($email) {
            return self::where('email', $email)->first();
        }, 3600); // Cache for 1 hour
    }

    /**
     * Get all users with Redis caching
     *
     * @return Collection
     */
    public static function getAllCached(): Collection
    {
        $redis = self::getRedisService();
        $cacheKey = "users:all";

        return $redis->remember($cacheKey, function () {
            return self::all();
        }, 600); // Cache for 10 minutes
    }

    /**
     * Create a new user and invalidate caches
     *
     * @param array $data
     * @return User
     */
    public static function createUser(array $data): User
    {
        // Hash password if present
        if (isset($data['password'])) {
            $data['password'] = password_hash($data['password'], PASSWORD_DEFAULT);
        }

        $user = self::create($data);
        
        // Invalidate caches
        self::invalidateUserCaches();
        
        return $user;
    }

    /**
     * Update a user and invalidate caches
     *
     * @param int $id
     * @param array $data
     * @return User|null
     */
    public static function updateUser(int $id, array $data): ?User
    {
        $user = self::find($id);
        
        if (!$user) {
            return null;
        }
        
        // Hash password if present and changed
        if (isset($data['password'])) {
            $data['password'] = password_hash($data['password'], PASSWORD_DEFAULT);
        }
        
        $user->update($data);
        
        // Invalidate caches
        self::invalidateUserCaches($user);
        
        return $user;
    }

    /**
     * Delete a user and invalidate caches
     *
     * @param int $id
     * @return bool
     */
    public static function deleteUser(int $id): bool
    {
        $user = self::find($id);
        
        if (!$user) {
            return false;
        }
        
        $deleted = $user->delete();
        
        if ($deleted) {
            // Invalidate caches
            self::invalidateUserCaches($user);
        }
        
        return $deleted;
    }

    /**
     * Invalidate user-related caches
     *
     * @param User|null $user
     * @return void
     */
    private static function invalidateUserCaches(?User $user = null): void
    {
        $redis = self::getRedisService();
        
        // Always invalidate the all users cache
        $redis->delete("users:all");
        
        if ($user) {
            // Invalidate specific user caches
            $redis->delete("user:{$user->id}");
            $redis->delete("user:email:{$user->email}");
        }
    }
}