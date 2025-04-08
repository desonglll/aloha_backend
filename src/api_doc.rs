use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::group_permission::insert_group_permission_route,
        crate::routes::group_permission::get_all_group_permissions_route,
        crate::routes::group_permission::get_group_permissions_by_group_id_route,
        crate::routes::group_permission::get_group_permissions_by_permission_id_route,
        crate::routes::group_permission::delete_group_permission_route,
        crate::routes::group_permission::delete_group_permissions_by_group_id_route,
        crate::routes::group_permission::delete_group_permissions_by_permission_id_route,
    ),
    components(
        schemas(
            crate::models::group_permission::GroupPermission,
            crate::routes::group_permission::FormData,
            crate::dto::response::DtoResponse<crate::models::group_permission::GroupPermission>,
            crate::dto::pagination::Pagination,
            crate::error::AlohaError,
        )
    ),
    tags(
        (name = "group-permissions", description = "Group Permission Management API")
    )
)]
pub struct ApiDoc;
