use crate::helpers::spawn_app;
use aloha_backend::dto::query::{DtoQuery, TweetFilterQuery};
use aloha_backend::mappers::tweet::{
    delete_tweet_by_id, get_all_tweets, get_tweet_by_id, insert_tweet, update_tweet,
};
use aloha_backend::mappers::user::insert_user;
use aloha_backend::models::tweet::Tweet;
use aloha_backend::models::user::User;
use uuid::Uuid;

#[tokio::test]
async fn insert_tweet_success() {
    let app = spawn_app().await;
    let transaction = app.db_pool.begin().await.unwrap();

    // Create a user first
    let user = User::default_test();
    let user_result = insert_user(transaction, &user).await.unwrap();

    // Now create a tweet with the user ID
    let transaction = app.db_pool.begin().await.unwrap();
    let tweet = Tweet::default_test(user_result.id);

    let result = insert_tweet(transaction, &tweet).await.unwrap();

    assert_ne!(result.id, Uuid::nil()); // ID should be a valid UUID
    assert_eq!(result.content, tweet.content);
    assert_eq!(result.user_id, user_result.id);
    assert!(result.created_at.is_some());
    assert!(result.updated_at.is_some());
}

#[tokio::test]
async fn get_tweet_by_id_success() {
    let app = spawn_app().await;
    let transaction = app.db_pool.begin().await.unwrap();

    // Create a user first
    let user = User::default_test();
    let user_result = insert_user(transaction, &user).await.unwrap();

    // Now create a tweet with the user ID
    let transaction = app.db_pool.begin().await.unwrap();
    let tweet = Tweet::default_test(user_result.id);

    // Insert the tweet
    let insert_result = insert_tweet(transaction, &tweet).await.unwrap();

    // Get the tweet by ID
    let transaction = app.db_pool.begin().await.unwrap();
    let get_result = get_tweet_by_id(transaction, insert_result.id)
        .await
        .unwrap();

    assert!(get_result.is_some());
    let get_result = get_result.unwrap();
    assert_eq!(get_result.id, insert_result.id);
    assert_eq!(get_result.content, insert_result.content);
    assert_eq!(get_result.user_id, insert_result.user_id);
}

#[tokio::test]
async fn get_tweet_by_id_not_found() {
    let app = spawn_app().await;
    let transaction = app.db_pool.begin().await.unwrap();

    // Try to get a non-existent tweet
    let non_existent_id = Uuid::new_v4();
    let result = get_tweet_by_id(transaction, non_existent_id).await.unwrap();

    assert!(result.is_none());
}

#[tokio::test]
async fn update_tweet_success() {
    let app = spawn_app().await;
    let transaction = app.db_pool.begin().await.unwrap();

    // Create a user first
    let user = User::default_test();
    let user_result = insert_user(transaction, &user).await.unwrap();

    // Now create a tweet with the user ID
    let transaction = app.db_pool.begin().await.unwrap();
    let tweet = Tweet::default_test(user_result.id);

    // Insert the tweet
    let insert_result = insert_tweet(transaction, &tweet).await.unwrap();

    // Update the tweet
    let transaction = app.db_pool.begin().await.unwrap();
    let mut update_tweet_obj = insert_result.clone();
    update_tweet_obj.content = "Updated content".to_string();

    let update_result = update_tweet(transaction, &update_tweet_obj).await.unwrap();

    assert_eq!(update_result.id, insert_result.id);
    assert_eq!(update_result.content, "Updated content");
    assert_eq!(update_result.user_id, user_result.id);

    // Verify the tweet was updated
    let transaction = app.db_pool.begin().await.unwrap();
    let get_result = get_tweet_by_id(transaction, insert_result.id)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(get_result.id, insert_result.id, "Tweet ID should match");
    assert_eq!(
        get_result.content, "Updated content",
        "Content should be updated"
    );
    assert_eq!(
        get_result.user_id, user_result.id,
        "User ID should remain unchanged"
    );
}

#[tokio::test]
async fn delete_tweet_by_id_success() {
    let app = spawn_app().await;
    let transaction = app.db_pool.begin().await.unwrap();

    // Create a user first
    let user = User::default_test();
    let user_result = insert_user(transaction, &user).await.unwrap();

    // Now create a tweet with the user ID
    let transaction = app.db_pool.begin().await.unwrap();
    let tweet = Tweet::default_test(user_result.id);

    // Insert the tweet
    let insert_result = insert_tweet(transaction, &tweet).await.unwrap();

    // Verify tweet exists before deletion
    let transaction = app.db_pool.begin().await.unwrap();
    let get_before_delete = get_tweet_by_id(transaction, insert_result.id)
        .await
        .unwrap();
    assert!(
        get_before_delete.is_some(),
        "Tweet should exist before deletion"
    );

    // Now delete the tweet
    let transaction = app.db_pool.begin().await.unwrap();
    let delete_result = delete_tweet_by_id(transaction, insert_result.id)
        .await
        .unwrap();

    assert_eq!(delete_result.id, insert_result.id);
    assert_eq!(delete_result.content, insert_result.content);
    assert_eq!(delete_result.user_id, user_result.id);

    // Verify it's deleted
    let transaction = app.db_pool.begin().await.unwrap();
    let get_result = get_tweet_by_id(transaction, insert_result.id)
        .await
        .unwrap();
    assert!(
        get_result.is_none(),
        "Tweet should not exist after deletion"
    );
}

#[tokio::test]
async fn get_all_tweets_no_filter() {
    let app = spawn_app().await;

    // Create user and tweets in a single transaction
    let test_user = User::default_test();
    let test_tweets = Vec::from([
        Tweet {
            id: Uuid::new_v4(),
            content: "Test tweet 1".to_string(),
            created_at: Some(time::OffsetDateTime::now_utc()),
            updated_at: Some(time::OffsetDateTime::now_utc()),
            user_id: test_user.id,
        },
        Tweet {
            id: Uuid::new_v4(),
            content: "Test tweet 2".to_string(),
            created_at: Some(time::OffsetDateTime::now_utc()),
            updated_at: Some(time::OffsetDateTime::now_utc()),
            user_id: test_user.id,
        },
        Tweet {
            id: Uuid::new_v4(),
            content: "Test tweet 3".to_string(),
            created_at: Some(time::OffsetDateTime::now_utc()),
            updated_at: Some(time::OffsetDateTime::now_utc()),
            user_id: test_user.id,
        },
    ]);

    // Set up test data directly in the database
    let pool = app.db_pool.clone();
    sqlx::query!(
        r#"INSERT INTO users (id, username, password_hash) VALUES ($1, $2, $3)"#,
        test_user.id,
        test_user.username,
        "test_hash",
    )
    .execute(&pool)
    .await
    .expect("Failed to insert test user");

    // Insert tweets directly
    for tweet in &test_tweets {
        sqlx::query!(
            r#"INSERT INTO tweet (id, content, user_id) VALUES ($1, $2, $3)"#,
            tweet.id,
            tweet.content,
            tweet.user_id,
        )
        .execute(&pool)
        .await
        .expect("Failed to insert test tweet");
    }

    // Now verify the tweets are in the database by direct query
    let tweets_in_db = sqlx::query!(
        r#"SELECT id, content, user_id FROM tweet WHERE user_id = $1"#,
        test_user.id
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch tweets from database");

    println!("Tweets directly from DB: {}", tweets_in_db.len());
    for tweet in &tweets_in_db {
        println!("DB tweet: {}, {}", tweet.id, tweet.content);
    }

    assert_eq!(
        tweets_in_db.len(),
        test_tweets.len(),
        "Should have found all inserted tweets in direct DB query"
    );

    // Now test the get_all_tweets function
    let transaction = pool.begin().await.expect("Failed to begin transaction");
    let query = DtoQuery::<TweetFilterQuery>::default_query();
    println!("query: {:?}", &query);
    let result = get_all_tweets(transaction, query)
        .await
        .expect("Failed to get tweets");

    println!("Tweets from get_all_tweets: {}", result.data.len());
    for tweet in &result.data {
        println!("Result tweet: {}, {}", tweet.id, tweet.content);
    }

    // Check that at least one of our test tweets is found
    let found_test_tweet = test_tweets
        .iter()
        .any(|t| result.data.iter().any(|rt| rt.id == t.id));

    assert!(
        found_test_tweet,
        "get_all_tweets should return at least one of our test tweets"
    );
}

#[tokio::test]
async fn get_all_tweets_with_user_filter() {
    let app = spawn_app().await;
    let transaction = app.db_pool.begin().await.unwrap();

    // Create two users
    let user1 = User::default_test();
    let user1_result = insert_user(transaction, &user1).await.unwrap();

    let transaction = app.db_pool.begin().await.unwrap();
    let mut user2 = User::default_test();
    user2.username = "second_user".to_string();
    let user2_result = insert_user(transaction, &user2).await.unwrap();

    // Create tweets for user1
    for i in 1..=2 {
        let transaction = app.db_pool.begin().await.unwrap();
        let mut tweet = Tweet::default_test(user1_result.id);
        tweet.content = format!("User1 tweet {}", i);
        let _ = insert_tweet(transaction, &tweet).await.unwrap();
    }

    // Create tweets for user2
    for i in 1..=3 {
        let transaction = app.db_pool.begin().await.unwrap();
        let mut tweet = Tweet::default_test(user2_result.id);
        tweet.content = format!("User2 tweet {}", i);
        let _ = insert_tweet(transaction, &tweet).await.unwrap();
    }

    // Get tweets filtered by user1
    let transaction = app.db_pool.begin().await.unwrap();
    let mut query = DtoQuery::<TweetFilterQuery>::default_query();
    query.filter = Some(TweetFilterQuery {
        user_id: Some(user1_result.id),
    });

    let result = get_all_tweets(transaction, query).await.unwrap();

    // Should only have tweets from user1
    assert_eq!(result.data.len(), 2);
    for tweet in &result.data {
        assert_eq!(tweet.user_id, user1_result.id);
    }
}

#[tokio::test]
async fn delete_tweets_by_ids_success() {
    let app = spawn_app().await;

    // Start a single transaction for all operations
    let mut tx = app
        .db_pool
        .begin()
        .await
        .expect("Failed to begin transaction");

    // Create a user first
    let user = User::default_test();
    let user_result = sqlx::query!(
        r#"
        INSERT INTO users (id, username, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        user.id,
        user.username,
        "test_hash"
    )
    .fetch_one(&mut *tx)
    .await
    .expect("Failed to create test user");

    // Create multiple tweets
    let mut tweet_ids = Vec::new();
    for i in 1..=3 {
        let tweet = Tweet::default_test(user_result.id);
        let result = sqlx::query!(
            r#"
            INSERT INTO tweet (id, content, user_id)
            VALUES ($1, $2, $3)
            RETURNING id, content, created_at, updated_at, user_id
            "#,
            tweet.id,
            format!("Test tweet {}", i),
            tweet.user_id
        )
        .fetch_one(&mut *tx)
        .await
        .expect("Failed to create test tweet");

        tweet_ids.push(result.id);
    }

    // Check for each tweet individually
    for (i, tweet_id) in tweet_ids.iter().enumerate() {
        let tweet = sqlx::query!(
            r#"
            SELECT id, content, created_at, updated_at, user_id 
            FROM tweet 
            WHERE id = $1
            "#,
            tweet_id
        )
        .fetch_optional(&mut *tx)
        .await
        .expect("Failed to check if tweet exists");

        if tweet.is_none() {
            panic!(
                "Tweet {} (ID: {}) not found before deletion check",
                i + 1,
                tweet_id
            );
        }
    }

    // Get all tweets to verify they exist
    let all_tweets = sqlx::query!(
        r#"
        SELECT id, content, created_at, updated_at, user_id 
        FROM tweet 
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&mut *tx)
    .await
    .expect("Failed to fetch all tweets");

    let all_tweet_data = all_tweets
        .into_iter()
        .map(|row| Tweet {
            id: row.id,
            content: row.content,
            created_at: row.created_at,
            updated_at: row.updated_at,
            user_id: row.user_id,
        })
        .collect::<Vec<_>>();

    // Debug info
    println!("Expected tweet IDs: {:?}", tweet_ids);
    println!("Found tweets count: {}", all_tweet_data.len());
    if !all_tweet_data.is_empty() {
        println!(
            "Found tweet IDs: {:?}",
            all_tweet_data.iter().map(|t| t.id).collect::<Vec<_>>()
        );
    }

    let has_tweet1 = all_tweet_data.iter().any(|t| t.id == tweet_ids[0]);
    let has_tweet2 = all_tweet_data.iter().any(|t| t.id == tweet_ids[1]);
    let has_tweet3 = all_tweet_data.iter().any(|t| t.id == tweet_ids[2]);

    assert!(has_tweet1, "Tweet 1 should exist before deletion");
    assert!(has_tweet2, "Tweet 2 should exist before deletion");
    assert!(has_tweet3, "Tweet 3 should exist before deletion");

    // Commit the transaction to ensure tweets are persisted
    tx.commit().await.expect("Failed to commit transaction");

    // Start a new transaction for deletion
    let mut tx = app
        .db_pool
        .begin()
        .await
        .expect("Failed to begin transaction");

    // Now delete only the first two tweets
    let ids_to_delete = vec![tweet_ids[0], tweet_ids[1]];
    let deleted_tweets = sqlx::query!(
        r#"
        DELETE FROM tweet
        WHERE id = ANY($1)
        RETURNING id, content, created_at, updated_at, user_id
        "#,
        &ids_to_delete as &[Uuid]
    )
    .fetch_all(&mut *tx)
    .await
    .expect("Failed to delete tweets");

    let delete_result = deleted_tweets
        .into_iter()
        .map(|row| Tweet {
            id: row.id,
            content: row.content,
            created_at: row.created_at,
            updated_at: row.updated_at,
            user_id: row.user_id,
        })
        .collect::<Vec<_>>();

    // Should have deleted 2 tweets
    assert_eq!(delete_result.len(), 2);
    assert!(delete_result.iter().any(|t| t.id == ids_to_delete[0]));
    assert!(delete_result.iter().any(|t| t.id == ids_to_delete[1]));

    // Commit the deletion
    tx.commit().await.expect("Failed to commit deletion");

    // Start a new transaction for verification
    let mut tx = app
        .db_pool
        .begin()
        .await
        .expect("Failed to begin transaction");

    // Verify the third tweet still exists but the first two are gone
    let remaining_tweets = sqlx::query!(
        r#"
        SELECT id, content, created_at, updated_at, user_id 
        FROM tweet 
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&mut *tx)
    .await
    .expect("Failed to fetch remaining tweets");

    let remaining_tweet_data = remaining_tweets
        .into_iter()
        .map(|row| Tweet {
            id: row.id,
            content: row.content,
            created_at: row.created_at,
            updated_at: row.updated_at,
            user_id: row.user_id,
        })
        .collect::<Vec<_>>();

    // Find tweets by their IDs
    let has_tweet1_now = remaining_tweet_data.iter().any(|t| t.id == tweet_ids[0]);
    let has_tweet2_now = remaining_tweet_data.iter().any(|t| t.id == tweet_ids[1]);
    let has_tweet3_now = remaining_tweet_data.iter().any(|t| t.id == tweet_ids[2]);

    assert!(!has_tweet1_now, "Tweet 1 should be deleted");
    assert!(!has_tweet2_now, "Tweet 2 should be deleted");
    assert!(has_tweet3_now, "Tweet 3 should still exist");

    // Commit the final transaction
    tx.commit()
        .await
        .expect("Failed to commit final transaction");
}
