<?php

namespace App\Config;

use Illuminate\Database\Capsule\Manager as Capsule;
use Illuminate\Events\Dispatcher;
use Illuminate\Container\Container;

/**
 * Database configuration class
 * Sets up Eloquent ORM with MySQL connection from environment variables
 */
class Database
{
    /**
     * Initialize the database connection
     *
     * @return void
     */
    public static function init(): void
    {
        $capsule = new Capsule;

        $capsule->addConnection([
            'driver'    => $_ENV['DB_CONNECTION'] ?? 'mysql',
            'host'      => $_ENV['DB_HOST'] ?? 'localhost',
            'port'      => $_ENV['DB_PORT'] ?? 3306,
            'database'  => $_ENV['DB_DATABASE'] ?? 'php_sample_app',
            'username'  => $_ENV['DB_USERNAME'] ?? 'root',
            'password'  => $_ENV['DB_PASSWORD'] ?? '',
            'charset'   => 'utf8mb4',
            'collation' => 'utf8mb4_unicode_ci',
            'prefix'    => '',
            'strict'    => true,
            'engine'    => null,
        ]);

        // Set the event dispatcher
        $capsule->setEventDispatcher(new Dispatcher(new Container));

        // Make this Capsule instance available globally
        $capsule->setAsGlobal();

        // Setup the Eloquent ORM
        $capsule->bootEloquent();
        
        // If in development, enable query logging
        if ($_ENV['APP_DEBUG'] ?? false) {
            $capsule->getConnection()->enableQueryLog();
        }
    }
    
    /**
     * Get the query log if in debug mode
     *
     * @return array
     */
    public static function getQueryLog(): array
    {
        if (!($_ENV['APP_DEBUG'] ?? false)) {
            return [];
        }
        
        return Capsule::getConnection()->getQueryLog();
    }
}