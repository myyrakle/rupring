use std::sync::Arc;

use super::{
    dto::{DeleteUserRequest, GetUserRequest, ListUsersRequest, UpdateUserRequest},
    interface::IUserService,
};

#[derive(Debug, Clone)]
#[rupring::Controller(
    prefix=/, 
    routes=[get_user, create_user, update_user, delete_user, list_users], 
    middlewares=[],
)]
pub struct UserController {}

#[rupring::Get(path = /users/:id)]
#[tags = [user]]
#[summary = "user 조회"]
pub fn get_user(
    request: rupring::Request,
    #[path = "id"]
    #[desc = "user 고유 id"]
    #[required]
    id: i32,
) -> rupring::Response {
    let user_service = request.get_provider::<Arc<dyn IUserService>>().unwrap();

    let request = GetUserRequest { id };

    let response = user_service.get_user(request);

    if let Ok(response) = response {
        rupring::Response::new().json(response)
    } else {
        rupring::Response::new().status(500).text("error")
    }
}

#[rupring::Post(path = /users)]
#[tags = [user]]
#[summary = "user 생성"]
#[RequestBody = crate::domains::users::dto::CreateUserRequest]
pub fn create_user(request: rupring::Request, _: rupring::Response) -> rupring::Response {
    let user_service = request.get_provider::<Arc<dyn IUserService>>().unwrap();

    let request = serde_json::from_str(&request.body);

    let request = match request {
        Ok(request) => request,
        Err(_) => return rupring::Response::new().status(400).text("bad request"),
    };

    let response = user_service.create_user(request);

    if let Ok(response) = response {
        rupring::Response::new().json(response)
    } else {
        rupring::Response::new().status(500).text("error")
    }
}

#[rupring::Put(path = /users/:id)]
#[tags = [user]]
#[summary = "user 정보 수정"]
pub fn update_user(
    request: rupring::Request,
    #[path = "id"]
    #[desc = "user 고유 id"]
    #[required]
    id: i32,
) -> rupring::Response {
    let user_service = request.get_provider::<Arc<dyn IUserService>>().unwrap();

    let request = serde_json::from_str(&request.body);

    let mut request: UpdateUserRequest = match request {
        Ok(request) => request,
        Err(_) => return rupring::Response::new().status(400).text("bad request"),
    };

    request.id = id;

    let response = user_service.update_user(request);

    if let Ok(response) = response {
        rupring::Response::new().json(response)
    } else {
        rupring::Response::new().status(500).text("error")
    }
}

#[rupring::Delete(path = /users/:id)]
#[tags = [user]]
#[summary = "user 삭제"]
pub fn delete_user(
    request: rupring::Request,
    #[path = "id"]
    #[desc = "user 고유 id"]
    #[required]
    id: i32,
) -> rupring::Response {
    let user_service = request.get_provider::<Arc<dyn IUserService>>().unwrap();

    let request = DeleteUserRequest { id };

    let response = user_service.delete_user(request);

    if let Ok(response) = response {
        rupring::Response::new().json(response)
    } else {
        rupring::Response::new().status(500).text("error")
    }
}

#[rupring::Get(path = /users)]
#[tags = [user]]
#[summary = "user 리스트 조회"]
pub fn list_users(request: rupring::Request) -> rupring::Response {
    let user_service = request.get_provider::<Arc<dyn IUserService>>().unwrap();

    let limit = request.query_parameters.get("limit").map(|e|e.to_owned()).unwrap_or(vec!["10".to_owned()]);
    let offset = request.query_parameters.get("offset").map(|e|e.to_owned()).unwrap_or(vec!["1".to_owned()]);

    let limit = match limit.get(0).map(|x| x.parse::<i32>()) {
        Some(Ok(limit)) => limit,
        _ => return rupring::Response::new().status(400).text("bad request"),
    };

    let offset = match offset.get(0).map(|x| x.parse::<i32>()) {
        Some(Ok(offset)) => offset,
        _ => return rupring::Response::new().status(400).text("bad request"),
    };

    let request = ListUsersRequest {  offset, limit };

    let response = user_service.list_users(request);

    if let Ok(response) = response {
        rupring::Response::new().json(response)
    } else {
        rupring::Response::new().status(500).text("error")
    }
}