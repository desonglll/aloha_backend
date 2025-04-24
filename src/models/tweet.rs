use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::dto::response::get_time_formatter;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tweet {
    pub id: Uuid,
    pub content: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub user_id: Uuid,
}

impl Tweet {
    pub fn new(content: String, user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(), // Generate a new UUID
            content,
            created_at: Some(OffsetDateTime::now_utc()),
            updated_at: Some(OffsetDateTime::now_utc()),
            user_id,
        }
    }

    pub fn default_test(user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            content: "Test tweet content".to_string(),
            created_at: Some(OffsetDateTime::now_utc()),
            updated_at: Some(OffsetDateTime::now_utc()),
            user_id,
        }
    }

    pub fn default_vec_test(count: Option<usize>, user_id: Uuid) -> Vec<Self> {
        let count = count.unwrap_or(5);
        let mut tweets = Vec::with_capacity(count);

        for i in 1..=count {
            tweets.push(Self {
                id: Uuid::new_v4(),
                content: format!("Test tweet content {}", i),
                created_at: Some(OffsetDateTime::now_utc()),
                updated_at: Some(OffsetDateTime::now_utc()),
                user_id,
            });
        }

        tweets
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct TweetResponse {
    pub id: Uuid,
    pub content: String,
    #[schema(value_type = String)]
    pub created_at: Option<String>,
    #[schema(value_type = String)]
    pub updated_at: Option<String>,
    pub user_id: Uuid,
}

impl From<Tweet> for TweetResponse {
    fn from(tweet: Tweet) -> Self {
        Self {
            id: tweet.id,
            content: tweet.content,
            created_at: Some(
                tweet
                    .created_at
                    .unwrap()
                    .format(&get_time_formatter())
                    .unwrap(),
            ),
            updated_at: Some(
                tweet
                    .updated_at
                    .unwrap()
                    .format(&get_time_formatter())
                    .unwrap(),
            ),
            user_id: tweet.user_id,
        }
    }
}
