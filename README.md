# rupring

![](https://img.shields.io/badge/language-Rust-red) ![](https://img.shields.io/badge/version-0.14.0-brightgreen) [![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/myyrakle/rupring/blob/master/LICENSE)

Spring Comes to Rust

## Get Started

required dependency list

```toml
rupring = "0.14.0"
serde = { version="1.0.193", features=["derive"] }
```

And you can write your server like this:

```rust
#[derive(Debug, Clone, Copy)]
#[rupring::Module(controllers=[HomeController{}], modules=[])]
pub struct RootModule {}

#[derive(Debug, Clone)]
#[rupring::Controller(prefix=/, routes=[hello, echo])]
pub struct HomeController {}

#[rupring::Get(path = /)]
pub fn hello(_request: rupring::Request) -> rupring::Response {
    rupring::Response::new().text("Hello, World!".to_string())
}

#[rupring::Get(path = /echo)]
pub fn echo(request: rupring::Request) -> rupring::Response {
    rupring::Response::new().text(request.body)
}

fn main() {
    rupring::run(RootModule {})
}
```

And if you run the program, it will work fine.  
![hello_world](https://github.com/user-attachments/assets/76d30d84-c7ed-4723-83fc-9394874c9779)

## More Details

Please refer to [the documentation](https://docs.rs/rupring/latest/rupring) for more details.
