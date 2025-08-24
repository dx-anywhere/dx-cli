package com.example.javasampleapp;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.kafka.annotation.EnableKafka;

@SpringBootApplication
@EnableKafka
public class JavaSampleAppApplication {

    public static void main(String[] args) {
        SpringApplication.run(JavaSampleAppApplication.class, args);
    }
}