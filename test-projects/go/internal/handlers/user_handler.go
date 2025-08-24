package handlers

import (
	"context"
	"log"
	"net/http"

	"github.com/gin-gonic/gin"

	"github.com/example/go-sample-app/internal/models"
	"github.com/example/go-sample-app/internal/repository"
)

// UserHandler handles HTTP requests for user operations
type UserHandler struct {
	userRepo      *repository.UserRepository
	eventProducer *models.EventProducer
}

// NewUserHandler creates a new UserHandler
func NewUserHandler(userRepo *repository.UserRepository, eventProducer *models.EventProducer) *UserHandler {
	return &UserHandler{
		userRepo:      userRepo,
		eventProducer: eventProducer,
	}
}

// GetAllUsers returns all users
func (h *UserHandler) GetAllUsers(c *gin.Context) {
	users, err := h.userRepo.FindAll(c.Request.Context())
	if err != nil {
		log.Printf("Error fetching users: %v", err)
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to fetch users"})
		return
	}

	c.JSON(http.StatusOK, users)
}

// GetUserByID returns a user by ID
func (h *UserHandler) GetUserByID(c *gin.Context) {
	id := c.Param("id")
	if id == "" {
		c.JSON(http.StatusBadRequest, gin.H{"error": "User ID is required"})
		return
	}

	user, err := h.userRepo.FindByID(c.Request.Context(), id)
	if err != nil {
		log.Printf("Error fetching user: %v", err)
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to fetch user"})
		return
	}

	if user == nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "User not found"})
		return
	}

	c.JSON(http.StatusOK, user)
}

// CreateUser creates a new user
func (h *UserHandler) CreateUser(c *gin.Context) {
	var input models.UserInput
	if err := c.ShouldBindJSON(&input); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	user, err := h.userRepo.Create(c.Request.Context(), &input)
	if err != nil {
		log.Printf("Error creating user: %v", err)
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	// Publish event to Kafka
	event := models.NewUserCreatedEvent(*user)
	go func() {
		// Use background context for async operations
		ctx := context.Background()
		if err := h.eventProducer.PublishUserEvent(ctx, event); err != nil {
			log.Printf("Error publishing user created event: %v", err)
		}
	}()

	c.JSON(http.StatusCreated, user)
}

// UpdateUser updates an existing user
func (h *UserHandler) UpdateUser(c *gin.Context) {
	id := c.Param("id")
	if id == "" {
		c.JSON(http.StatusBadRequest, gin.H{"error": "User ID is required"})
		return
	}

	var input models.UserInput
	if err := c.ShouldBindJSON(&input); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	user, err := h.userRepo.Update(c.Request.Context(), id, &input)
	if err != nil {
		log.Printf("Error updating user: %v", err)
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	// Publish event to Kafka
	event := models.NewUserUpdatedEvent(*user)
	go func() {
		// Use background context for async operations
		ctx := context.Background()
		if err := h.eventProducer.PublishUserEvent(ctx, event); err != nil {
			log.Printf("Error publishing user updated event: %v", err)
		}
	}()

	c.JSON(http.StatusOK, user)
}

// DeleteUser deletes a user
func (h *UserHandler) DeleteUser(c *gin.Context) {
	id := c.Param("id")
	if id == "" {
		c.JSON(http.StatusBadRequest, gin.H{"error": "User ID is required"})
		return
	}

	// Get user before deletion for event publishing
	user, err := h.userRepo.FindByID(c.Request.Context(), id)
	if err != nil {
		log.Printf("Error fetching user for deletion: %v", err)
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to fetch user"})
		return
	}

	if user == nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "User not found"})
		return
	}

	// Delete the user
	if err := h.userRepo.Delete(c.Request.Context(), id); err != nil {
		log.Printf("Error deleting user: %v", err)
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to delete user"})
		return
	}

	// Publish event to Kafka
	event := models.NewUserDeletedEvent(*user)
	go func() {
		// Use background context for async operations
		ctx := context.Background()
		if err := h.eventProducer.PublishUserEvent(ctx, event); err != nil {
			log.Printf("Error publishing user deleted event: %v", err)
		}
	}()

	c.JSON(http.StatusNoContent, nil)
}