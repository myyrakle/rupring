use rupring::response::Cookie;

#[derive(Debug, Clone)]
#[rupring::Controller(prefix=/, routes=[index, slow], tags=["root"])]
pub struct RootController {}

#[rupring::Get(path = /)]
#[summary = "기본 root API입니다."]
#[description = "별다른 기능은 없습니다."]
#[tags = [root]]
pub fn index(request: rupring::Request) -> rupring::Response {
    let body = request.body;
    println!("body: {}", body);

    rupring::Response::new()
        .text("123214")
        .add_cookie(Cookie::new("name", "value").http_only(true).secure(true))
        .add_cookie(Cookie::new("name2", "value2").http_only(true).secure(true))
}

#[rupring::Get(path = /slow)]
#[summary = "그냥 느린 API입니다."]
#[description = "별다른 기능은 없습니다."]
#[tags = [root]]
pub fn slow(request: rupring::Request) -> rupring::Response {
    std::thread::sleep(std::time::Duration::from_secs(30));

    rupring::Response::new().text("Slow")
}
