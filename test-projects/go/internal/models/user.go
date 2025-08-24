package models

import (
	"time"

	"go.mongodb.org/mongo-driver/bson/primitive"
)

// User represents a user in the system
type User struct {
	ID        primitive.ObjectID `json:"id" bson:"_id,omitempty"`
	Username  string             `json:"username" bson:"username"`
	Email     string             `json:"email" bson:"email"`
	Password  string             `json:"-" bson:"password"` // Password not included in JSON responses
	CreatedAt time.Time          `json:"created_at" bson:"created_at"`
	UpdatedAt time.Time          `json:"updated_at" bson:"updated_at"`
}

// UserInput is the data structure for creating or updating users
type UserInput struct {
	Username string `json:"username" binding:"required,min=3,max=30"`
	Email    string `json:"email" binding:"required,email"`
	Password string `json:"password" binding:"required,min=6"`
}

// UserEvent represents an event related to a user that will be sent to Kafka
type UserEvent struct {
	EventID    string    `json:"event_id"`
	EventType  string    `json:"event_type"`
	UserID     string    `json:"user_id"`
	Username   string    `json:"username"`
	Email      string    `json:"email"`
	OccurredAt time.Time `json:"occurred_at"`
}

// EventType constants
const (
	EventTypeUserCreated = "USER_CREATED"
	EventTypeUserUpdated = "USER_UPDATED"
	EventTypeUserDeleted = "USER_DELETED"
)

// NewUserCreatedEvent creates a new UserEvent for user creation
func NewUserCreatedEvent(user User) UserEvent {
	return UserEvent{
		EventID:    primitive.NewObjectID().Hex(),
		EventType:  EventTypeUserCreated,
		UserID:     user.ID.Hex(),
		Username:   user.Username,
		Email:      user.Email,
		OccurredAt: time.Now(),
	}
}

// NewUserUpdatedEvent creates a new UserEvent for user updates
func NewUserUpdatedEvent(user User) UserEvent {
	return UserEvent{
		EventID:    primitive.NewObjectID().Hex(),
		EventType:  EventTypeUserUpdated,
		UserID:     user.ID.Hex(),
		Username:   user.Username,
		Email:      user.Email,
		OccurredAt: time.Now(),
	}
}

// NewUserDeletedEvent creates a new UserEvent for user deletion
func NewUserDeletedEvent(user User) UserEvent {
	return UserEvent{
		EventID:    primitive.NewObjectID().Hex(),
		EventType:  EventTypeUserDeleted,
		UserID:     user.ID.Hex(),
		Username:   user.Username,
		Email:      user.Email,
		OccurredAt: time.Now(),
	}
}