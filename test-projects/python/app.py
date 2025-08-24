"""
Sample Python Flask application with PostgreSQL and Redis dependencies
"""
import os
import json
from datetime import datetime
from dotenv import load_dotenv
from flask import Flask, jsonify, request
from sqlalchemy import create_engine, Column, Integer, String, DateTime
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker
import redis

# Load environment variables
load_dotenv()

app = Flask(__name__)
app.config['SECRET_KEY'] = os.getenv('SECRET_KEY', 'dev-secret-key')

# PostgreSQL setup
DATABASE_URL = os.getenv('DATABASE_URL')
engine = create_engine(DATABASE_URL)
Base = declarative_base()
Session = sessionmaker(bind=engine)

# Redis setup
redis_client = redis.Redis.from_url(os.getenv('REDIS_URL'))

# Define SQLAlchemy model
class User(Base):
    __tablename__ = 'users'
    
    id = Column(Integer, primary_key=True)
    username = Column(String(50), unique=True, nullable=False)
    email = Column(String(120), unique=True, nullable=False)
    created_at = Column(DateTime, default=datetime.utcnow)
    
    def to_dict(self):
        return {
            'id': self.id,
            'username': self.username,
            'email': self.email,
            'created_at': self.created_at.isoformat()
        }

# Create tables if they don't exist
def init_db():
    Base.metadata.create_all(engine)

# Routes
@app.route('/')
def home():
    return jsonify({'message': 'Python Sample App with PostgreSQL and Redis'})

@app.route('/users', methods=['GET'])
def get_users():
    # Try to get from cache first
    cached_users = redis_client.get('users:all')
    if cached_users:
        app.logger.info("Serving users from Redis cache")
        return jsonify(json.loads(cached_users))
    
    # If not in cache, get from database
    session = Session()
    try:
        users = session.query(User).all()
        result = [user.to_dict() for user in users]
        
        # Cache the result
        redis_client.setex('users:all', 60, json.dumps(result))  # Cache for 60 seconds
        
        return jsonify(result)
    finally:
        session.close()

@app.route('/users', methods=['POST'])
def create_user():
    data = request.get_json()
    if not data or not data.get('username') or not data.get('email'):
        return jsonify({'error': 'Username and email are required'}), 400
    
    session = Session()
    try:
        user = User(
            username=data['username'],
            email=data['email']
        )
        session.add(user)
        session.commit()
        
        # Invalidate cache
        redis_client.delete('users:all')
        
        return jsonify(user.to_dict()), 201
    except Exception as e:
        session.rollback()
        return jsonify({'error': str(e)}), 500
    finally:
        session.close()

@app.route('/redis-test')
def redis_test():
    test_key = 'test:key'
    test_value = f'Hello at {datetime.utcnow().isoformat()}'
    
    # Set a value in Redis
    redis_client.set(test_key, test_value)
    
    # Get it back
    retrieved_value = redis_client.get(test_key)
    
    return jsonify({
        'key': test_key,
        'set_value': test_value,
        'retrieved_value': retrieved_value.decode('utf-8') if retrieved_value else None
    })

if __name__ == '__main__':
    init_db()
    port = int(os.getenv('PORT', 5000))
    app.run(host='0.0.0.0', port=port)