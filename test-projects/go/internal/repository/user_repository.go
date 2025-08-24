package repository

import (
	"context"
	"errors"
	"time"

	"github.com/example/go-sample-app/internal/models"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

// UserRepository handles database operations for users
type UserRepository struct {
	collection *mongo.Collection
}

// NewUserRepository creates a new UserRepository
func NewUserRepository(db *mongo.Database) *UserRepository {
	return &UserRepository{
		collection: db.Collection("users"),
	}
}

// FindAll retrieves all users from the database
func (r *UserRepository) FindAll(ctx context.Context) ([]models.User, error) {
	var users []models.User

	// Define options to sort by creation date, newest first
	opts := options.Find().SetSort(bson.D{{Key: "created_at", Value: -1}})

	// Execute the query
	cursor, err := r.collection.Find(ctx, bson.M{}, opts)
	if err != nil {
		return nil, err
	}
	defer cursor.Close(ctx)

	// Decode results
	if err := cursor.All(ctx, &users); err != nil {
		return nil, err
	}

	return users, nil
}

// FindByID retrieves a user by ID
func (r *UserRepository) FindByID(ctx context.Context, id string) (*models.User, error) {
	var user models.User

	// Convert string ID to ObjectID
	objectID, err := primitive.ObjectIDFromHex(id)
	if err != nil {
		return nil, err
	}

	// Find user by ID
	err = r.collection.FindOne(ctx, bson.M{"_id": objectID}).Decode(&user)
	if err != nil {
		if errors.Is(err, mongo.ErrNoDocuments) {
			return nil, nil // User not found
		}
		return nil, err
	}

	return &user, nil
}

// Create inserts a new user into the database
func (r *UserRepository) Create(ctx context.Context, input *models.UserInput) (*models.User, error) {
	// Check if username or email already exists
	if exists, err := r.existsByField(ctx, "username", input.Username); err != nil {
		return nil, err
	} else if exists {
		return nil, errors.New("username already exists")
	}

	if exists, err := r.existsByField(ctx, "email", input.Email); err != nil {
		return nil, err
	} else if exists {
		return nil, errors.New("email already exists")
	}

	// Create new user
	now := time.Now()
	user := models.User{
		ID:        primitive.NewObjectID(),
		Username:  input.Username,
		Email:     input.Email,
		Password:  input.Password, // In a real app, would hash this password
		CreatedAt: now,
		UpdatedAt: now,
	}

	// Insert into database
	_, err := r.collection.InsertOne(ctx, user)
	if err != nil {
		return nil, err
	}

	return &user, nil
}

// Update updates an existing user
func (r *UserRepository) Update(ctx context.Context, id string, input *models.UserInput) (*models.User, error) {
	// Convert string ID to ObjectID
	objectID, err := primitive.ObjectIDFromHex(id)
	if err != nil {
		return nil, err
	}

	// Check if user exists
	existingUser, err := r.FindByID(ctx, id)
	if err != nil {
		return nil, err
	}
	if existingUser == nil {
		return nil, errors.New("user not found")
	}

	// Check username uniqueness if changed
	if input.Username != existingUser.Username {
		if exists, err := r.existsByField(ctx, "username", input.Username); err != nil {
			return nil, err
		} else if exists {
			return nil, errors.New("username already exists")
		}
	}

	// Check email uniqueness if changed
	if input.Email != existingUser.Email {
		if exists, err := r.existsByField(ctx, "email", input.Email); err != nil {
			return nil, err
		} else if exists {
			return nil, errors.New("email already exists")
		}
	}

	// Update the user
	update := bson.M{
		"$set": bson.M{
			"username":   input.Username,
			"email":      input.Email,
			"password":   input.Password, // In a real app, would hash this password
			"updated_at": time.Now(),
		},
	}

	// Execute update
	result := r.collection.FindOneAndUpdate(
		ctx,
		bson.M{"_id": objectID},
		update,
		options.FindOneAndUpdate().SetReturnDocument(options.After),
	)

	// Check for errors
	if result.Err() != nil {
		return nil, result.Err()
	}

	// Decode updated user
	var updatedUser models.User
	if err := result.Decode(&updatedUser); err != nil {
		return nil, err
	}

	return &updatedUser, nil
}

// Delete removes a user from the database
func (r *UserRepository) Delete(ctx context.Context, id string) error {
	// Convert string ID to ObjectID
	objectID, err := primitive.ObjectIDFromHex(id)
	if err != nil {
		return err
	}

	// Execute delete
	result, err := r.collection.DeleteOne(ctx, bson.M{"_id": objectID})
	if err != nil {
		return err
	}

	// Check if user was found and deleted
	if result.DeletedCount == 0 {
		return errors.New("user not found")
	}

	return nil
}

// existsByField checks if a user exists with the given field value
func (r *UserRepository) existsByField(ctx context.Context, field, value string) (bool, error) {
	count, err := r.collection.CountDocuments(ctx, bson.M{field: value})
	if err != nil {
		return false, err
	}
	return count > 0, nil
}