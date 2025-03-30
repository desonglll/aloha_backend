// use crate::models::user_group::UserGroup;
// use sqlx::PgPool;
//
// async fn get_group_by_id(pool: &PgPool, id: i32) -> Result<UserGroup, sqlx::Error> {
//     sqlx::query_as!(UserGroup, "select * from user_group where id=$1", id).fetch_one(pool).await?
// }
//
// #[cfg(tests)]
// mod tests {
//     #[tests]
//     fn test_get_group_by_id() {
//
//
//     }
// }
