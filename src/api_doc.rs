use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Group Permission routes
        crate::routes::group_permission::insert_group_permission_route,
        crate::routes::group_permission::get_all_group_permissions_route,
        crate::routes::group_permission::get_group_permissions_by_group_id_route,
        crate::routes::group_permission::get_group_permissions_by_permission_id_route,
        crate::routes::group_permission::delete_group_permission_route,
        crate::routes::group_permission::delete_group_permissions_by_group_id_route,
        crate::routes::group_permission::delete_group_permissions_by_permission_id_route,

        // Permission routes
        crate::routes::permission::insert_permission_route,
        crate::routes::permission::get_all_permissions_route,
        crate::routes::permission::get_permission_by_id_route,
        crate::routes::permission::update_permission_by_id_route,
        crate::routes::permission::delete_permission_by_id_route,

        // User routes
        crate::routes::user::insert_user_route,
        crate::routes::user::get_all_users_route,
        crate::routes::user::get_user_route,
        crate::routes::user::update_user_route,
        crate::routes::user::delete_user_route,
        crate::routes::user::delete_users_route,

        // User Group routes
        crate::routes::user_group::insert_user_group_route,
        crate::routes::user_group::get_all_user_groups_route,
        crate::routes::user_group::get_user_group_route,
        crate::routes::user_group::update_user_group_route,
        crate::routes::user_group::delete_user_group_route,
        
        // Tweet routes
        crate::routes::tweet::insert_tweet_route,
        crate::routes::tweet::get_all_tweets_route,
        crate::routes::tweet::get_tweet_route,
        crate::routes::tweet::update_tweet_route,
        crate::routes::tweet::delete_tweet_route,
        crate::routes::tweet::delete_tweets_route,

        // Health Check route
        crate::routes::health_check::health_check,
    ),
    components(
        schemas(
            // Group Permission schemas
            crate::models::group_permission::GroupPermission,
            crate::routes::group_permission::CreateGroupPermissionFormData,
            crate::routes::group_permission::DeleteGroupPermissionFormData,
            crate::dto::response::DtoResponse<crate::models::group_permission::GroupPermission>,
            // Permission schemas
            crate::models::permission::Permission,
            crate::routes::permission::CreatePermissionFormData,
            crate::routes::permission::PutPermissionFormData,
            crate::dto::response::DtoResponse<crate::models::permission::Permission>,
            // User schemas
            crate::models::user::User,
            crate::models::user::UserResponse,
            crate::routes::user::CreateUserFormData,
            crate::routes::user::PutUserFormData,
            crate::dto::response::DtoResponse<crate::models::user::UserResponse>,
            // User Group schemas
            crate::models::user_group::UserGroup,
            crate::routes::user_group::CreateUserGroupFormData,
            crate::routes::user_group::PutUserGroupFormData,
            crate::dto::response::DtoResponse<crate::models::user_group::UserGroup>,
            // Tweet schemas
            crate::models::tweet::TweetResponse,
            crate::routes::tweet::CreateTweetFormData,
            crate::routes::tweet::PutTweetFormData,
            crate::dto::response::DtoResponse<crate::models::tweet::TweetResponse>,
            // Common schemas
            crate::dto::pagination::Pagination,
            crate::error::AlohaError,
        )
    ),
    tags(
        (name = "group-permissions", description = "Group Permission Management API"),
        (name = "permissions", description = "Permission Management API"),
        (name = "users", description = "User Management API"),
        (name = "user-groups", description = "User Group Management API"),
        (name = "tweets", description = "Tweet Management API"),
        (name = "health", description = "Health Check API")
    )
)]
pub struct ApiDoc;
