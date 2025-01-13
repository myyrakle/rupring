use rupring::response::Cookie;

#[derive(Debug, Clone)]
#[rupring::Controller(prefix=/, routes=[index, slow, download, multipart, multipart_page], tags=["root"])]
pub struct RootController {}

#[rupring::Get(path = /)]
#[summary = "기본 root API입니다."]
#[description = "별다른 기능은 없습니다."]
#[tags = [root]]
pub fn index(request: rupring::Request) -> rupring::Response {
    let body = request.body;
    println!("body: {}", body);

    println!("cookies: {:?}", request.cookies);

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

#[rupring::Get(path = /download)]
#[summary = "단순 다운로드 API입니다."]
#[description = "별다른 기능은 없습니다."]
#[tags = [root]]
pub fn download(request: rupring::Request) -> rupring::Response {
    rupring::Response::new().download("foo.txt", "Hello, World!")
}

#[rupring::Post(path = /multipart-upload)]
#[summary = "단순 파일 업로드 API입니다."]
#[description = "별다른 기능은 없습니다."]
#[tags = [root]]
pub fn multipart(mut request: rupring::Request) -> rupring::Response {
    for (i, file) in request.files.iter().enumerate() {
        std::fs::write(format!("{i}.foo"), &file.data).unwrap();
    }

    rupring::Response::new().text("Hello, World!")
}

#[rupring::Get(path = /multipart-upload-page)]
#[summary = "단순 파일 업로드 API입니다."]
#[description = "별다른 기능은 없습니다."]
#[tags = [root]]
pub fn multipart_page(request: rupring::Request) -> rupring::Response {
    rupring::Response::new().html(
        r#"
        <html>
            <body>
                <form action="/multipart-upload" method="post" enctype="multipart/form-data">
                    <input type="file" name="file1" />
                    <input type="file" name="file2" />
                    <input type="submit" value="Submit" />
                </form>
            </body>
        </html>
        "#,
    )
}
