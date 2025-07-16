/*!
# Swagger Support
- When rupring starts the server, it automatically serves swagger documents to the `/docs` path.

Additional annotations such as `summary`, `description`, and `tags` are provided for swagger documentation.
```rust
#[rupring::Get(path = /echo/:id)]
#[summary = "echo API"]
#[description = "It's echo API"]
#[tags = ["echo"]]
pub fn echo(
    #[path="id"] #[description="just integer id"] id: Option<i32>
) -> rupring::Response {
    //...

    rupring::Response::new().text("OK".to_string())
}
```

Using the RupringDto derive macro, you can perform document definition for Request Parameter.
```rust
use rupring::RupringDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, RupringDto)]
pub struct CreateUserRequest {
    #[desc = "user name"]
    #[example = "foobar"]
    pub username: String,

    pub email: String,

    #[desc = "user password"]
    #[example = "q1w2e3r4"]
    pub password: String,
}
```
### RupringDto attribute Details
1. `#[desc = ""]` or `#[description = ""]`: Description of the field.
2. `#[example = ""]`: Example value of the field.
3. `#[name = "id"]`: If the field name is different from the variable name, you can add this annotation.
4. `#[required]`: If the field is required, you can add this annotation.
5. `#[path_param = "id"]` or `#[param = "id"]`: If the field is a path parameter, you can add this annotation.
6. `#[query = "query"]`: If the field is a query parameter, you can add this annotation.
7. `#[ignore]`: If you want to ignore the field, you can add this annotation.

Then, you can specify request information in the API through the params attribute as follows.
```rust
use rupring::RupringDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, RupringDto)]
pub struct CreateUserRequest {
    #[desc = "user name"]
    #[example = "foobar"]
    pub username: String,

    pub email: String,

    #[desc = "user password"]
    #[example = "q1w2e3r4"]
    pub password: String,
}

#[rupring::Post(path = /users)]
#[tags = [user]]
#[summary = "user create"]
#[params = CreateUserRequest]
pub fn create_user(request: rupring::Request, _: rupring::Response) -> rupring::Response {
    // ...

    rupring::Response::new().text("OK".to_string())
}
```

Response documentation can also be defined through the RupringDto macro and response attribute.
```rust
use rupring::RupringDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, RupringDto)]
pub struct GetUserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[rupring::Get(path = /users/:id)]
#[tags = [user]]
#[summary = "find user"]
#[response = GetUserResponse]
pub fn get_user(request: rupring::Request, _: rupring::Response) -> rupring::Response {
    return rupring::Response::new().text("OK".to_string());
}
```

If you want to activate BearerAuth for the API, activate the auth attribute as follows. (The default is BearerAuth.
```rust
use rupring::RupringDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, RupringDto)]
pub struct GetUserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[rupring::Get(path = /users/:id)]
#[tags = [user]]
#[summary = "find user"]
#[response = GetUserResponse]
#[auth = BearerAuth]
pub fn get_user(request: rupring::Request, _: rupring::Response) -> rupring::Response {
    return rupring::Response::new().text("OK".to_string());
}
```
*/

pub mod context;
pub mod controller;
pub mod html;
pub mod json;
pub mod module;
pub mod routes;

pub mod macros;

pub use json::*;
