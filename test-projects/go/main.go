package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"
	"github.com/segmentio/kafka-go"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.mongodb.org/mongo-driver/mongo/readpref"

	"github.com/example/go-sample-app/internal/handlers"
	"github.com/example/go-sample-app/internal/models"
	"github.com/example/go-sample-app/internal/repository"
)

func main() {
	// Load environment variables
	if err := godotenv.Load(); err != nil {
		log.Printf("Warning: Error loading .env file: %v", err)
	}

	// Setup context with cancellation for graceful shutdown
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Connect to MongoDB
	mongoClient, err := connectToMongoDB(ctx)
	if err != nil {
		log.Fatalf("Failed to connect to MongoDB: %v", err)
	}
	defer mongoClient.Disconnect(ctx)
	log.Println("Connected to MongoDB")

	// Get MongoDB database
	dbName := os.Getenv("MONGODB_DATABASE")
	if dbName == "" {
		dbName = "go_sample_app"
	}
	db := mongoClient.Database(dbName)

	// Initialize repositories
	userRepo := repository.NewUserRepository(db)

	// Connect to Kafka
	kafkaWriter := connectToKafka()
	defer kafkaWriter.Close()
	log.Println("Connected to Kafka")

	// Initialize Kafka event producer
	eventProducer := models.NewEventProducer(kafkaWriter)

	// Setup Gin router
	router := gin.Default()
	router.Use(gin.Recovery())

	// Initialize handlers
	userHandler := handlers.NewUserHandler(userRepo, eventProducer)

	// Define routes
	router.GET("/", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "Go Sample App with MongoDB and Kafka",
		})
	})

	// User routes
	api := router.Group("/api")
	{
		users := api.Group("/users")
		{
			users.GET("", userHandler.GetAllUsers)
			users.GET("/:id", userHandler.GetUserByID)
			users.POST("", userHandler.CreateUser)
			users.PUT("/:id", userHandler.UpdateUser)
			users.DELETE("/:id", userHandler.DeleteUser)
		}
	}

	// Start the server
	port := os.Getenv("APP_PORT")
	if port == "" {
		port = "8080"
	}
	srv := &http.Server{
		Addr:    ":" + port,
		Handler: router,
	}

	// Start server in a goroutine
	go func() {
		log.Printf("Server starting on port %s...", port)
		if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("Failed to start server: %v", err)
		}
	}()

	// Setup graceful shutdown
	quit := make(chan os.Signal, 1)
	signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
	<-quit
	log.Println("Shutting down server...")

	// Give outstanding requests 5 seconds to complete
	shutdownCtx, shutdownCancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer shutdownCancel()

	if err := srv.Shutdown(shutdownCtx); err != nil {
		log.Fatalf("Server forced to shutdown: %v", err)
	}

	log.Println("Server exited properly")
}

// connectToMongoDB establishes a connection to MongoDB
func connectToMongoDB(ctx context.Context) (*mongo.Client, error) {
	uri := os.Getenv("MONGODB_URI")
	if uri == "" {
		uri = "mongodb://localhost:27017"
	}

	clientOptions := options.Client().ApplyURI(uri)

	// Set timeout
	timeoutStr := os.Getenv("MONGODB_TIMEOUT_MS")
	var timeout int64 = 5000 // default 5 seconds
	if timeoutStr != "" {
		fmt.Sscanf(timeoutStr, "%d", &timeout)
	}
	connectCtx, cancel := context.WithTimeout(ctx, time.Duration(timeout)*time.Millisecond)
	defer cancel()

	// Connect to MongoDB
	client, err := mongo.Connect(connectCtx, clientOptions)
	if err != nil {
		return nil, err
	}

	// Ping to verify connection
	pingCtx, cancel := context.WithTimeout(ctx, 2*time.Second)
	defer cancel()
	if err := client.Ping(pingCtx, readpref.Primary()); err != nil {
		return nil, err
	}

	return client, nil
}

// connectToKafka establishes a connection to Kafka
func connectToKafka() *kafka.Writer {
	brokers := os.Getenv("KAFKA_BROKERS")
	if brokers == "" {
		brokers = "localhost:9092"
	}

	topic := os.Getenv("KAFKA_TOPIC_USERS")
	if topic == "" {
		topic = "users"
	}

	clientID := os.Getenv("KAFKA_CLIENT_ID")
	if clientID == "" {
		clientID = "go-sample-app-client"
	}

	return kafka.NewWriter(kafka.WriterConfig{
		Brokers:  []string{brokers},
		Topic:    topic,
		Balancer: &kafka.LeastBytes{},
		ClientID: clientID,
	})
}