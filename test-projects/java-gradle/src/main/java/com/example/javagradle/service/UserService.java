package com.example.javagradle.service;

import com.example.javagradle.kafka.UserEvent;
import com.example.javagradle.model.User;
import com.example.javagradle.repository.UserRepository;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.kafka.core.KafkaTemplate;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.time.LocalDateTime;
import java.util.List;
import java.util.Optional;

@Service
@RequiredArgsConstructor
@Slf4j
public class UserService {

    private final UserRepository userRepository;
    private final KafkaTemplate<String, UserEvent> kafkaTemplate;
    
    @Value("${app.kafka.topic.user-events}")
    private String userEventsTopic;
    
    public List<User> getAllUsers() {
        return userRepository.findAll();
    }
    
    public Optional<User> getUserById(Long id) {
        return userRepository.findById(id);
    }
    
    @Transactional
    public User createUser(String username, String email) {
        // Check if username or email already exists
        if (userRepository.existsByUsername(username)) {
            throw new IllegalArgumentException("Username already exists: " + username);
        }
        
        if (userRepository.existsByEmail(email)) {
            throw new IllegalArgumentException("Email already exists: " + email);
        }
        
        // Create new user
        User user = User.builder()
                .username(username)
                .email(email)
                .createdAt(LocalDateTime.now())
                .build();
        
        // Save to database
        User savedUser = userRepository.save(user);
        log.info("Created new user: {}", savedUser);
        
        // Publish event to Kafka
        UserEvent event = UserEvent.forUserCreated(
                savedUser.getId(), 
                savedUser.getUsername(), 
                savedUser.getEmail());
        
        kafkaTemplate.send(userEventsTopic, event);
        log.info("Published user created event: {}", event);
        
        return savedUser;
    }
    
    @Transactional
    public Optional<User> updateUser(Long id, String username, String email) {
        return userRepository.findById(id)
                .map(user -> {
                    // Update user properties
                    user.setUsername(username);
                    user.setEmail(email);
                    
                    // Save to database
                    User updatedUser = userRepository.save(user);
                    log.info("Updated user: {}", updatedUser);
                    
                    // Publish event to Kafka
                    UserEvent event = UserEvent.forUserUpdated(
                            updatedUser.getId(),
                            updatedUser.getUsername(),
                            updatedUser.getEmail());
                    
                    kafkaTemplate.send(userEventsTopic, event);
                    log.info("Published user updated event: {}", event);
                    
                    return updatedUser;
                });
    }
    
    @Transactional
    public boolean deleteUser(Long id) {
        return userRepository.findById(id)
                .map(user -> {
                    // Delete from database
                    userRepository.delete(user);
                    log.info("Deleted user: {}", user);
                    
                    // Publish event to Kafka
                    UserEvent event = UserEvent.forUserDeleted(
                            user.getId(),
                            user.getUsername(),
                            user.getEmail());
                    
                    kafkaTemplate.send(userEventsTopic, event);
                    log.info("Published user deleted event: {}", event);
                    
                    return true;
                })
                .orElse(false);
    }
}