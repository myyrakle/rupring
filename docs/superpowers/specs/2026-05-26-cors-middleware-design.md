# CORS Middleware Design

## Goal

Add built-in CORS support to Rupring without new dependencies.

## Architecture

Expose a new `rupring::middleware::cors` module. Users can create a middleware with `Cors::default().middleware()` and register it in existing module or controller middleware arrays.

The middleware adds CORS response headers to normal requests after calling `next`. The core request pipeline also detects valid CORS preflight requests before route lookup. If a configured CORS middleware exists in the matching module/controller chain for that path, the pipeline returns a `204` response with the configured CORS headers instead of requiring users to define an explicit `OPTIONS` route.

## API

`Cors` is a cloneable builder with these options:

- `allow_any_origin()`
- `allow_origin(origin)`
- `allow_origins(origins)`
- `allow_methods(methods)`
- `allow_headers(headers)`
- `expose_headers(headers)`
- `allow_credentials(bool)`
- `max_age(seconds)`
- `middleware()`

`Cors::default()` allows any origin and common methods. It reflects requested headers during preflight when explicit allowed headers are not configured.

## Data Flow

Normal request:

1. Route lookup succeeds.
2. Middleware chain runs.
3. CORS middleware calls `next`.
4. CORS middleware appends headers to the returned response.

Preflight request:

1. Request method is `OPTIONS`.
2. Request has `Origin` and `Access-Control-Request-Method` headers.
3. The pipeline finds a route by path and requested method.
4. The pipeline applies the CORS middleware config and returns `204`.

## Error Handling

Invalid header names are ignored when adding response headers, matching `Response::header`.

If a preflight path or requested method does not match any real route, the request falls through to the existing `404` behavior.

## Testing

Add unit tests for:

- CORS headers are added after `next`.
- Preflight responses return `204` without executing the route handler.
- Explicit origin lists allow matching origins and omit CORS headers for non-matching origins.
- Requested preflight headers are reflected when no explicit allowed headers are configured.
