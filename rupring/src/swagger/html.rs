pub const DOCS_INDEX_HTML: &str = r###"
    <!DOCTYPE html>
    <html>
    <head>
    <link type="text/css" rel="stylesheet" href="/docs/swagger-ui.css">
    <!--<link rel="shortcut icon" href="https://fastapi.tiangolo.com/img/favicon.png">-->
    <title>Rupring - Swagger UI</title>
    </head>
    <body>
    <div id="swagger-ui">
    </div>
    <script src="/docs/swagger-ui-bundle.js"></script>
    <!-- `SwaggerUIBundle` is now available on the page -->
    <script>
    const ui = SwaggerUIBundle({
        url: '/docs/swagger.json',
    "dom_id": "#swagger-ui",
"layout": "BaseLayout",
"deepLinking": true,
"showExtensions": true,
"showCommonExtensions": true,
oauth2RedirectUrl: window.location.origin + '/docs/oauth2-redirect',
    presets: [
        SwaggerUIBundle.presets.apis,
        SwaggerUIBundle.SwaggerUIStandalonePreset
        ],
    })
    </script>
    </body>
    </html>
"###;
