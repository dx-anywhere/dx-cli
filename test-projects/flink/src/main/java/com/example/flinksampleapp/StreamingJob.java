package com.example.flinksampleapp;

import org.apache.flink.api.common.eventtime.WatermarkStrategy;
import org.apache.flink.api.common.functions.FilterFunction;
import org.apache.flink.api.common.functions.MapFunction;
import org.apache.flink.api.common.serialization.SimpleStringSchema;
import org.apache.flink.api.java.tuple.Tuple2;
import org.apache.flink.connector.jdbc.JdbcConnectionOptions;
import org.apache.flink.connector.jdbc.JdbcExecutionOptions;
import org.apache.flink.connector.jdbc.JdbcSink;
import org.apache.flink.connector.kafka.source.KafkaSource;
import org.apache.flink.connector.kafka.source.enumerator.initializer.OffsetsInitializer;
import org.apache.flink.streaming.api.datastream.DataStream;
import org.apache.flink.streaming.api.environment.StreamExecutionEnvironment;
import org.apache.flink.streaming.api.windowing.assigners.TumblingProcessingTimeWindows;
import org.apache.flink.streaming.api.windowing.time.Time;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.time.Duration;
import java.util.Properties;

/**
 * Sample Streaming Job that reads from Kafka, processes the data, and writes to PostgreSQL.
 */
public class StreamingJob {
    private static final Logger LOG = LoggerFactory.getLogger(StreamingJob.class);

    public static void main(String[] args) throws Exception {
        // Set up the streaming execution environment
        final StreamExecutionEnvironment env = StreamExecutionEnvironment.getExecutionEnvironment();
        
        // Configure Kafka source
        KafkaSource<String> source = KafkaSource.<String>builder()
                .setBootstrapServers("localhost:9092")
                .setTopics("input-topic")
                .setGroupId("flink-consumer-group")
                .setStartingOffsets(OffsetsInitializer.earliest())
                .setValueOnlyDeserializer(new SimpleStringSchema())
                .build();
        
        // Create a data stream from Kafka
        DataStream<String> kafkaStream = env.fromSource(
                source,
                WatermarkStrategy.<String>forBoundedOutOfOrderness(Duration.ofSeconds(5))
                        .withIdleness(Duration.ofMinutes(1)),
                "Kafka Source"
        );
        
        // Process the stream
        DataStream<Tuple2<String, Integer>> processedStream = kafkaStream
                .filter((FilterFunction<String>) value -> value != null && !value.isEmpty())
                .map((MapFunction<String, Tuple2<String, Integer>>) value -> {
                    String[] parts = value.split(",");
                    if (parts.length >= 2) {
                        try {
                            String key = parts[0].trim();
                            int count = Integer.parseInt(parts[1].trim());
                            return new Tuple2<>(key, count);
                        } catch (NumberFormatException e) {
                            LOG.warn("Invalid input format: {}", value, e);
                            return new Tuple2<>("invalid", 0);
                        }
                    } else {
                        LOG.warn("Invalid input format: {}", value);
                        return new Tuple2<>("invalid", 0);
                    }
                })
                .keyBy(value -> value.f0)
                .window(TumblingProcessingTimeWindows.of(Time.seconds(10)))
                .sum(1);
        
        // Output the results
        processedStream.print();
        
        // Write results to PostgreSQL
        processedStream.addSink(
                JdbcSink.sink(
                        "INSERT INTO results (key, count) VALUES (?, ?) ON CONFLICT (key) DO UPDATE SET count = results.count + EXCLUDED.count",
                        (statement, tuple) -> {
                            statement.setString(1, tuple.f0);
                            statement.setInt(2, tuple.f1);
                        },
                        JdbcExecutionOptions.builder()
                                .withBatchSize(1000)
                                .withBatchIntervalMs(200)
                                .withMaxRetries(5)
                                .build(),
                        new JdbcConnectionOptions.JdbcConnectionOptionsBuilder()
                                .withUrl("jdbc:postgresql://localhost:5432/app")
                                .withDriverName("org.postgresql.Driver")
                                .withUsername("postgres")
                                .withPassword("example")
                                .build()
                )
        );
        
        // Execute the streaming pipeline
        env.execute("Kafka to PostgreSQL Streaming Job");
    }
}