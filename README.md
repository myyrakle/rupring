# rupring

![](https://img.shields.io/badge/language-Rust-red) ![](https://img.shields.io/badge/version-0.3.0-brightgreen) [![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/myyrakle/rupring/blob/master/LICENSE)

spring on rust

## Get Started

There is only one dependency.
```
cargo add rupring
```

And you can write your server like this:
```
#[derive(Debug, Clone, Copy)]
#[rupring::Module(controllers=[HomeController{}], modules=[])]
pub struct RootModule {}

#[derive(Debug, Clone)]
#[rupring::Controller(prefix=/, routes=[hello, echo])]
pub struct HomeController {}

#[rupring::Get(path = /)]
pub fn hello(_request: rupring::Request) -> rupring::Response {
    rupring::Response {
        status: 200,
        body: "Hello, World!".to_string(),
        headers: Default::default(),
    }
}

#[rupring::Get(path = /echo)]
pub fn echo(request: rupring::Request) -> rupring::Response {
    rupring::Response {
        status: 200,
        body: request.body,
        headers: Default::default(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = rupring::RupringFactory::create(RootModule {});

    app.listen(3000).await?;

    Ok(())
}

```

And if you run the program, it will work fine.  
![Alt text](./example/hello_world.png)

## More Details

Please refer to [the documentation](https://docs.rs/rupring/latest/rupring) for more details.