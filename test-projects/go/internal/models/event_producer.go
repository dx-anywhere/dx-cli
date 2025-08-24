package models

import (
	"context"
	"encoding/json"
	"log"
	"time"

	"github.com/segmentio/kafka-go"
)

// EventProducer handles publishing events to Kafka
type EventProducer struct {
	writer *kafka.Writer
}

// NewEventProducer creates a new event producer with the given Kafka writer
func NewEventProducer(writer *kafka.Writer) *EventProducer {
	return &EventProducer{
		writer: writer,
	}
}

// PublishUserEvent publishes a user event to Kafka
func (p *EventProducer) PublishUserEvent(ctx context.Context, event UserEvent) error {
	// Convert event to JSON bytes
	value, err := json.Marshal(event)
	if err != nil {
		log.Printf("Error marshaling user event: %v", err)
		return err
	}

	// Create Kafka message
	msg := kafka.Message{
		Key:   []byte(event.UserID), // Use UserID as the key for partitioning
		Value: value,
		Time:  time.Now(),
		Headers: []kafka.Header{
			{
				Key:   "event_type",
				Value: []byte(event.EventType),
			},
		},
	}

	// Write message to Kafka
	if err := p.writer.WriteMessages(ctx, msg); err != nil {
		log.Printf("Error writing message to Kafka: %v", err)
		return err
	}

	log.Printf("Published %s event for user %s to Kafka", event.EventType, event.UserID)
	return nil
}

// Close closes the Kafka writer
func (p *EventProducer) Close() error {
	return p.writer.Close()
}