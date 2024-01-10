use crate::{self as rupring, header, meme};

#[rupring_macro::GetMapping(path = /)]
pub fn get_docs(_: rupring::Request) -> rupring::Response {
    let index_html = r#"<!-- HTML for static distribution bundle build -->
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="UTF-8">
        <title>Swagger UI</title>
        <link rel="stylesheet" type="text/css" href="/docs/swagger-ui.css" />
        <link rel="stylesheet" type="text/css" href="/docs/index.css" />
        <link rel="icon" type="image/png" href="/docs/favicon-32x32.png" sizes="32x32" />
        <link rel="icon" type="image/png" href="/docs/favicon-16x16.png" sizes="16x16" />
      </head>
    
      <body>
        <div id="swagger-ui"></div>
        <script src="/docs/swagger-ui-bundle.js" charset="UTF-8"> </script>
        <script src="/docs/swagger-ui-standalone-preset.js" charset="UTF-8"> </script>
        <script src="/docs/swagger-initializer.js" charset="UTF-8"> </script>
      </body>
    </html>
    "#;

    rupring::Response::new()
        .text(index_html)
        .header(header::CONTENT_TYPE, meme::HTML)
}

#[rupring_macro::GetMapping(path = /index.css)]
pub fn get_index_css(_: rupring::Request) -> rupring::Response {
    let index_html = r#"html {
        box-sizing: border-box;
        overflow: -moz-scrollbars-vertical;
        overflow-y: scroll;
    }
    
    *,
    *:before,
    *:after {
        box-sizing: inherit;
    }
    
    body {
        margin: 0;
        background: #fafafa;
    }
    "#;

    rupring::Response::new()
        .text(index_html)
        .header(header::CONTENT_TYPE, meme::CSS)
}

#[rupring_macro::GetMapping(path = /swagger-ui.css)]
pub fn get_swagger_css(_: rupring::Request) -> rupring::Response {
    rupring::Response::new()
        .text(super::css::CSS)
        .header(header::CONTENT_TYPE, meme::CSS)
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

#[rupring_macro::GetMapping(path = /swagger-ui-bundle.js)]
pub fn get_jsbundle(_: rupring::Request) -> rupring::Response {
    rupring::Response::new()
        .text(super::js_bundle::JS_BUNDLE)
        .header(header::CONTENT_TYPE, meme::JAVASCRIPT)
}

#[rupring_macro::GetMapping(path = /swagger-ui-standalone-preset.js)]
pub fn get_jspreset(_: rupring::Request) -> rupring::Response {
    rupring::Response::new()
        .text(super::js_preset::JS_PRESET)
        .header(header::CONTENT_TYPE, meme::JAVASCRIPT)
}

#[rupring_macro::GetMapping(path = /swagger-initializer.js)]
pub fn get_jsinitializer(_: rupring::Request) -> rupring::Response {
    let js = r###"window.onload = function() {
        window.ui = SwaggerUIBundle({
          url: "/docs/swagger.json",
          dom_id: '#swagger-ui',
          deepLinking: true,
          presets: [
            SwaggerUIBundle.presets.apis,
            SwaggerUIStandalonePreset
          ],
          plugins: [
            SwaggerUIBundle.plugins.DownloadUrl
          ],
          layout: "StandaloneLayout"
        });
      };
      "###;

    rupring::Response::new()
        .text(js)
        .header(header::CONTENT_TYPE, meme::JAVASCRIPT)
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
