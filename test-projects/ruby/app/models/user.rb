# == Schema Information
#
# Table name: users
#
#  id                  :bigint           not null, primary key
#  email               :string           not null
#  encrypted_password  :string           not null
#  reset_password_token:string
#  reset_password_at   :datetime
#  username            :string           not null
#  created_at          :datetime         not null
#  updated_at          :datetime         not null
#
# Indexes:
#  index_users_on_email                 (email) UNIQUE
#  index_users_on_reset_password_token  (reset_password_token) UNIQUE
#  index_users_on_username              (username) UNIQUE
#

class User < ApplicationRecord
  # Include devise modules (authentication)
  devise :database_authenticatable, :registerable,
         :recoverable, :rememberable, :validatable
         
  # Validations
  validates :username, presence: true, uniqueness: { case_sensitive: false },
                      format: { with: /\A[a-zA-Z0-9_]+\z/ },
                      length: { minimum: 3, maximum: 25 }
  validates :email, presence: true, uniqueness: true,
                   format: { with: URI::MailTo::EMAIL_REGEXP }
                   
  # Callbacks
  after_create :cache_user_info
  after_update :update_cached_info
  after_destroy :remove_from_cache
  
  # Redis cache methods
  def cache_user_info
    Rails.cache.write("user:#{id}", cache_attributes)
    $redis.sadd("users", id)
  end
  
  def update_cached_info
    Rails.cache.write("user:#{id}", cache_attributes)
  end
  
  def remove_from_cache
    Rails.cache.delete("user:#{id}")
    $redis.srem("users", id)
  end
  
  private
  
  def cache_attributes
    {
      username: username,
      email: email,
      created_at: created_at.to_i,
      updated_at: updated_at.to_i
    }
  end
end