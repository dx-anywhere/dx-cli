// Sample Node.js application with MongoDB and Redis dependencies
require('dotenv').config();
const express = require('express');
const mongoose = require('mongoose');
const redis = require('redis');

const app = express();
const port = process.env.PORT || 3000;

// Connect to MongoDB
async function connectToMongoDB() {
  try {
    await mongoose.connect(process.env.MONGODB_URI, {
      useNewUrlParser: true,
      useUnifiedTopology: true
    });
    console.log('Connected to MongoDB');
  } catch (error) {
    console.error('MongoDB connection error:', error);
  }
}

// Connect to Redis
async function connectToRedis() {
  try {
    const redisClient = redis.createClient({
      url: process.env.REDIS_URL,
      password: process.env.REDIS_PASSWORD || undefined
    });
    
    redisClient.on('error', (err) => console.error('Redis Client Error', err));
    await redisClient.connect();
    console.log('Connected to Redis');
    
    return redisClient;
  } catch (error) {
    console.error('Redis connection error:', error);
    return null;
  }
}

// Define a simple Mongoose schema
const UserSchema = new mongoose.Schema({
  username: String,
  email: String,
  createdAt: { type: Date, default: Date.now }
});

const User = mongoose.model('User', UserSchema);

// Initialize connections
let redisClient;
(async () => {
  await connectToMongoDB();
  redisClient = await connectToRedis();
  
  // Start the server only after attempting to connect to databases
  app.listen(port, () => {
    console.log(`Server running on port ${port}`);
  });
})();

// Basic routes
app.get('/', (req, res) => {
  res.send('Node.js Sample App with MongoDB and Redis');
});

// API routes
app.get('/api/users', async (req, res) => {
  try {
    const users = await User.find().limit(10);
    res.json(users);
  } catch (error) {
    res.status(500).json({ error: 'Failed to fetch users' });
  }
});

app.get('/api/redis-test', async (req, res) => {
  if (!redisClient) {
    return res.status(500).json({ error: 'Redis not connected' });
  }
  
  try {
    const cacheKey = 'test-key';
    await redisClient.set(cacheKey, 'test-value');
    const value = await redisClient.get(cacheKey);
    res.json({ key: cacheKey, value });
  } catch (error) {
    res.status(500).json({ error: 'Redis operation failed' });
  }
});

// Graceful shutdown
process.on('SIGINT', async () => {
  console.log('Shutting down gracefully');
  if (redisClient) {
    await redisClient.quit();
  }
  await mongoose.connection.close();
  process.exit(0);
});