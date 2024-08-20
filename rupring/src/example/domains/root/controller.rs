#[derive(Debug, Clone)]
#[rupring::Controller(prefix=/, routes=[index], tags=["root"])]
pub struct RootController {}

#[rupring::Get(path = /)]
#[summary = "기본 root API입니다."]
#[description = "별다른 기능은 없습니다."]
#[tags = [root]]
pub fn index(request: rupring::Request) -> rupring::Response {
    let body = request.body;
    println!("body: {}", body);

    rupring::Response::new().text("123214")
}
