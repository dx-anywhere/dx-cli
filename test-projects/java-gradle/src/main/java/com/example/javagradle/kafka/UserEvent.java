package com.example.javagradle.kafka;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.time.LocalDateTime;

/**
 * Event object that will be published to Kafka when user actions occur
 */
@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
public class UserEvent {
    
    private String eventId;
    private String eventType;
    private Long userId;
    private String username;
    private String email;
    private LocalDateTime timestamp;
    
    public static UserEvent forUserCreated(Long userId, String username, String email) {
        return UserEvent.builder()
                .eventId(java.util.UUID.randomUUID().toString())
                .eventType("USER_CREATED")
                .userId(userId)
                .username(username)
                .email(email)
                .timestamp(LocalDateTime.now())
                .build();
    }
    
    public static UserEvent forUserUpdated(Long userId, String username, String email) {
        return UserEvent.builder()
                .eventId(java.util.UUID.randomUUID().toString())
                .eventType("USER_UPDATED")
                .userId(userId)
                .username(username)
                .email(email)
                .timestamp(LocalDateTime.now())
                .build();
    }
    
    public static UserEvent forUserDeleted(Long userId, String username, String email) {
        return UserEvent.builder()
                .eventId(java.util.UUID.randomUUID().toString())
                .eventType("USER_DELETED")
                .userId(userId)
                .username(username)
                .email(email)
                .timestamp(LocalDateTime.now())
                .build();
    }
}