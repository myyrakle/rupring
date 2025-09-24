use crate::{
    self as rupring, header,
    http::meme,
    swagger::{swagger_ui_bundle, swagger_ui_css},
};

#[rupring_macro::GetMapping(path = /)]
pub fn get_docs(_: rupring::Request) -> rupring::Response {
    rupring::Response::new()
        .text(super::html::DOCS_INDEX_HTML)
        .header(header::CONTENT_TYPE, meme::HTML)
}

#[rupring_macro::GetMapping(path = /favicon-32x32.png)]
pub fn get_favicon32(_: rupring::Request) -> rupring::Response {
    let base64 = r#"iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAMAAABEpIrGAAAAkFBMVEUAAAAQM0QWNUYWNkYXNkYALjoWNUYYOEUXN0YaPEUPMUAUM0QVNUYWNkYWNUYWNUUWNUYVNEYWNkYWNUYWM0eF6i0XNkchR0OB5SwzZj9wyTEvXkA3az5apTZ+4C5DgDt31C9frjU5bz5uxTI/eDxzzjAmT0IsWUEeQkVltzR62S6D6CxIhzpKijpJiDpOkDl4b43lAAAAFXRSTlMAFc304QeZ/vj+ECB3xKlGilPXvS2Ka/h0AAABfklEQVR42oVT2XaCMBAdJRAi7pYJa2QHxbb//3ctSSAUPfa+THLmzj4DBvZpvyauS9b7kw3PWDkWsrD6fFQhQ9dZLfVbC5M88CWCPERr+8fLZodJ5M8QJbjbGL1H2M1fIGfEm+wJN+bGCSc6EXtNS/8FSrq2VX6YDv++XLpJ8SgDWMnwqznGo6alcTbIxB2CHKn8VFikk2mMV2lEnV+CJd9+jJlxXmMr5dW14YCqwgbFpO8FNvJxwwM4TPWPo5QalEsRMAcusXpi58/QUEWPL0AK1ThM5oQCUyXPoPINkdd922VBw4XgTV9zDGWWFrgjIQs4vwvOg6xr+6gbCTqE+DYhlMGX0CF2OknK5gQ2JrkDh/W6TOEbYDeVecKbJtyNXiCfGmW7V93J2hDus1bDfhxWbIZVYDXITA7Lo6E0Ktgg9eB4KWuR44aj7ppBVPazhQH7/M/KgWe9X1qAg8XypT6nxIMJH+T94QCsLvj29IYwZxyO9/F8vCbO9tX5/wDGjEZ7vrgFZwAAAABJRU5ErkJggg=="#;

    rupring::Response::new()
        .text(base64)
        .header(header::CONTENT_TYPE, meme::PNG)
}

#[rupring_macro::GetMapping(path = /favicon-16x16.png)]
pub fn get_favicon16(_: rupring::Request) -> rupring::Response {
    let base64 = r#"iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAMAAAAoLQ9TAAABNVBMVEVisTRhsTReqzVbpTVXoDdVnTdSlzhRljgvXkAuXUAtWkErV0EzZj40Zj85bz0lTkMkTUMkT0MmTUIkS0IjTEIhSUMkS0IkTEIkTUIlTUIkTkMlTkMcQUQcP0UfQ0QdQ0QfREQgRUMiSUMiSUMjSkInU0EkTEMmUEEiR0IiSEMpVkErWT8kTUElTUIUNkYVNEQVMkcRM0QSNUYQMUIMMUkVK0AAJEkAM00AMzMAAAAAAACF6i2E6SyD6CyC5i2B5Sx/4i6A4S593S583S520jB00DByyjFxyTFwyDFvxjJtxTFtxDFswzJrwDJqvzJpvjNouzNoujNnuDNLjTlKijpKiTpEfztDfzxAeT0+dz05bj44bT44bj82aj81aD8zZT8bPUUbPkUcP0UcPUUeQ0UfREQgRkRgJREvAAAAO3RSTlP09PX19vX39u7u7/Dq6ufh4eDg4+Pf3Nvb2tnY2NvPv7y6rKupqaGZlpSOiYWETDEkHh0fFQwHCgUBAAcHrskAAADYSURBVHjaPc/ZLkNRGIbhz26KjVJpqSKGtjHPc9a7W7OEEhtBjDWUO3XghqQSwVrNTp+j///OXhlrLpdJdg9MLblbxqwPd5RLUDpOjK66YWMwTqRpaM0OhZbo3dskljea9+HyAevxHtoWVAjhfQtr5w3CSfUE8BrgvEDQpxRc3eyfH5wenlQuIO39Sb9x/8uv+bXvmPSjbABPRZznIkGvxkOo7mJtV+FsQsutcFvBuruG9kWZMY+G5pzxlMp/KPKZSUs2cLrzyMWVEyP1OGtlNpvs6p+p5/8DzUo5hMDku9EAAAAASUVORK5CYII="#;

    rupring::Response::new()
        .text(base64)
        .header(header::CONTENT_TYPE, meme::PNG)
}

#[rupring_macro::GetMapping(path = /swagger.json)]
pub fn get_json(request: rupring::Request) -> rupring::Response {
    let swagger_context = request
        .di_context
        .get::<super::context::SwaggerContext>()
        .unwrap();

    let json = swagger_context.openapi_json.read().unwrap().to_owned();

    rupring::Response::new()
        .text(json)
        .header(header::CONTENT_TYPE, meme::JAVASCRIPT)
}

#[rupring_macro::GetMapping(path = /swagger-ui-bundle.js)]
pub fn get_swagger_ui_bundle(request: rupring::Request) -> rupring::Response {
    let json = swagger_ui_bundle::SWAGGER_UI_BUNDLE_JS;

    rupring::Response::new()
        .text(json)
        .header(
            header::CONTENT_TYPE,
            format!("{}; charset=utf-8", meme::JAVASCRIPT),
        )
        .header("access-control-allow-origin", "*")
}

#[rupring_macro::GetMapping(path = /swagger-ui.css)]
pub fn get_swagger_ui_css(request: rupring::Request) -> rupring::Response {
    let css = swagger_ui_css::SWAGGER_UI_CSS;

    rupring::Response::new()
        .text(css)
        .header(
            header::CONTENT_TYPE,
            format!("{}; charset=utf-8", meme::CSS),
        )
        .header("access-control-allow-origin", "*")
}
