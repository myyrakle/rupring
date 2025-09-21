use std::sync::Arc;

use rupring::request;

use super::{
    dto::{DeleteUserRequest, GetUserRequest, ListUsersRequest, UpdateUserRequest},
    interface::IUserService,
};

#[derive(Debug, Clone)]
#[rupring::Controller(
    prefix=/, 
    routes=[get_user, create_user, update_user, delete_user, list_users, serve_sse_page, serve_sse], 
    middlewares=[],
)]
pub struct UserController {}

#[rupring::Get(path = /users/:id)]
#[tags = [user]]
#[summary = "user 조회"]
#[response = crate::domains::users::dto::GetUserResponse]
#[auth = BearerAuth]
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

    match  response {
        Ok(response) =>
        rupring::Response::new().json(response), 
        Err(error) => rupring::Response::new().status(500).text(error.to_string())
    } 
}

#[rupring::Post(path = /users)]
#[tags = [user]]
#[summary = "user 생성"]
#[params = crate::domains::users::dto::CreateUserRequest]
#[auth]
pub fn create_user(request: rupring::Request, _: rupring::Response) -> rupring::Response {
    let user_service = request.get_provider::<Arc<dyn IUserService>>().cloned().unwrap();

    let request = match request::BindFromRequest::bind(request) {
        Ok(request) => request,
        Err(err) => {
            println!("error: {:?}", err); 
            return rupring::Response::new().status(400).text("bad request");
        },
    };
    
    println!("{:?}", request);

    let response = user_service.create_user(request);

    match response {
        Ok(response) => rupring::Response::new().json(response),
        Err(error) => rupring::Response::new().status(500).text(error.to_string())
    }
}

#[rupring::Put(path = /users/:id)]
#[tags = [user]]
#[summary = "user 정보 수정"]
#[params = crate::domains::users::dto::UpdateUserRequest]
pub fn update_user(
    request: rupring::Request,
) -> rupring::Response {
    let user_service = request.get_provider::<Arc<dyn IUserService>>().unwrap();

    let request = rupring::serde_json::from_str(&request.body);

    let request: UpdateUserRequest = match request {
        Ok(request) => request,
        Err(_) => return rupring::Response::new().status(400).text("bad request"),
    };


    let response = user_service.update_user(request);

    match response {
        Ok(response) => rupring::Response::new().json(response),
        Err(error) => rupring::Response::new().status(500).text(error.to_string())
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

    match response {
        Ok(response) => rupring::Response::new().json(response),
        Err(error) => rupring::Response::new().status(500).text(error.to_string())
    }
}

#[rupring::Get(path = /users)]
#[tags = [user]]
#[summary = "user 리스트 조회"]
#[params = crate::domains::users::dto::ListUsersRequest]
#[response = crate::domains::users::dto::ListUsersResponse]
pub fn list_users(request: rupring::Request) -> rupring::Response {
    let user_service = request.get_provider::<Arc<dyn IUserService>>().unwrap();

    let limit = request.query_parameters.get("limit").map(|e|e.to_owned()).unwrap_or(vec!["10".to_owned()]);
    let offset = request.query_parameters.get("offset").map(|e|e.to_owned()).unwrap_or(vec!["1".to_owned()]);

    let limit = match limit.first().map(|x| x.parse::<i32>()) {
        Some(Ok(limit)) => limit,
        _ => return rupring::Response::new().status(400).text("bad request"),
    };

    let offset = match offset.first().map(|x| x.parse::<i32>()) {
        Some(Ok(offset)) => offset,
        _ => return rupring::Response::new().status(400).text("bad request"),
    };

    let request = ListUsersRequest {  offset, limit };

    let response = user_service.list_users(request);

    match response {
        Ok(response) => rupring::Response::new().json(response),
        Err(error) => rupring::Response::new().status(500).text(error.to_string())
    }
}

const SERVE_SSE_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>SSE Demo</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        #messages { 
            border: 1px solid #ccc; 
            padding: 20px; 
            height: 400px; 
            overflow-y: auto; 
            background: #f9f9f9;
        }
        .message { 
            margin: 5px 0; 
            padding: 5px; 
            background: white; 
            border-radius: 3px;
        }
        .timestamp { color: #666; font-size: 0.8em; }
    </style>
</head>
<body>
    <h1>Server-Sent Events Demo</h1>
    <div id="messages"></div>
    
    <script>
        const eventSource = new EventSource('/sse');
        const messages = document.getElementById('messages');
        
        eventSource.onmessage = function(event) {
            const messageDiv = document.createElement('div');
            messageDiv.className = 'message';
            messageDiv.innerHTML = `
                <div class="timestamp">${new Date().toLocaleTimeString()}</div>
                <div>${event.data}</div>
            `;
            messages.appendChild(messageDiv);
            messages.scrollTop = messages.scrollHeight;
        };
        
        eventSource.addEventListener('custom-event', function(event) {
            const messageDiv = document.createElement('div');
            messageDiv.className = 'message';
            messageDiv.style.backgroundColor = '#e3f2fd';
            messageDiv.innerHTML = `
                <div class="timestamp">${new Date().toLocaleTimeString()}</div>
                <div><strong>Custom Event:</strong> ${event.data}</div>
            `;
            messages.appendChild(messageDiv);
            messages.scrollTop = messages.scrollHeight;
        });
        
        eventSource.onerror = function(event) {
            console.error('SSE error:', event);
        };
    </script>
</body>
</html>
    "#;


#[rupring::Get(path = /sse-page)]
#[tags = [user]]
#[summary = "SSE 페이지"]
pub fn serve_sse_page(request: rupring::Request) -> rupring::Response {
    rupring::Response::new()
        .html(SERVE_SSE_HTML)
}



#[rupring::Get(path = /sse)]
#[tags = [user]]
#[summary = "SSE 페이지"]
pub fn serve_sse(request: rupring::Request) -> rupring::Response {
    rupring::Response::new()
        .sse(async move |stream_handler|  {
            let mut count = 0;
            loop {
                if stream_handler.is_closed() {
                    println!("Client disconnected, stopping SSE");
                    break;
                }

                let message = format!("data: Message number {}\n\n", count);
                println!("Sending: {}", message.trim());
                if let Err(e) = stream_handler.send(message.as_bytes()).await {
                    eprintln!("Error sending message: {}", e);
                }
                count += 1;
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        })
}


