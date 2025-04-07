use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Tweet {
    pub id: i32,
    pub content: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub created_at: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub updated_at: Option<String>,
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TweetResponse {
    pub id: i32,
    pub content: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub user_id: Uuid,
}

impl From<Tweet> for TweetResponse {
    fn from(tweet: Tweet) -> Self {
        Self {
            id: tweet.id,
            content: tweet.content,
            created_at: tweet.created_at,
            updated_at: tweet.updated_at,
            user_id: tweet.user_id,
        }
    }
}

impl Tweet {
    pub fn default_test(user_id: Uuid) -> Self {
        Self {
            id: 0, // The database will assign the actual ID
            content: String::from("This is a test tweet"),
            created_at: Some(chrono::Utc::now().to_rfc3339()),
            updated_at: Some(chrono::Utc::now().to_rfc3339()),
            user_id,
        }
    }

    pub fn default_vec_test(user_id: Uuid, number: Option<i32>) -> Vec<Self> {
        let number = number.unwrap_or(2);
        let mut result = Vec::<Self>::new();
        (0..number).for_each(|i| {
            let new = Self {
                id: 0, // The database will assign the actual ID
                content: format!("This is test tweet #{}", i),
                created_at: Some(chrono::Utc::now().to_rfc3339()),
                updated_at: Some(chrono::Utc::now().to_rfc3339()),
                user_id,
            };
            result.push(new);
        });
        result
    }

    pub fn new(content: String, user_id: Uuid) -> Self {
        Self {
            id: 0, // The database will assign the actual ID
            content,
            created_at: Some(chrono::Utc::now().to_rfc3339()),
            updated_at: Some(chrono::Utc::now().to_rfc3339()),
            user_id,
        }
    }
}
