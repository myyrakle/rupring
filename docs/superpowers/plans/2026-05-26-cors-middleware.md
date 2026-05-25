# CORS Middleware Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add built-in CORS middleware with automatic preflight handling and no new dependencies.

**Architecture:** Add `rupring::middleware::cors` with a cloneable `Cors` builder and a runtime registry keyed by generated middleware id. The route pipeline uses the registry to answer valid preflight requests before reading the body or invoking route handlers.

**Tech Stack:** Rust 2021, hyper `HeaderName` and `Method`, existing Rupring middleware pipeline, standard library synchronization primitives.

---

## File Structure

- Create `rupring/src/middleware/mod.rs`: public middleware namespace.
- Create `rupring/src/middleware/cors.rs`: `Cors` builder, middleware factory, header application, preflight registry, and unit tests.
- Modify `rupring/src/http/header.rs`: add CORS request/response header constants.
- Modify `rupring/src/lib.rs`: export `middleware` module.
- Modify `rupring/src/core/route.rs`: add path+method preflight lookup that returns route middlewares for requested method.
- Modify `rupring/src/core/mod.rs`: parse request headers before route lookup, handle valid preflight via CORS registry, and reuse parsed headers for normal requests.

### Task 1: CORS Middleware Headers

**Files:**
- Create: `rupring/src/middleware/mod.rs`
- Create: `rupring/src/middleware/cors.rs`
- Modify: `rupring/src/http/header.rs`
- Modify: `rupring/src/lib.rs`

- [ ] **Step 1: Write failing middleware tests**

Add tests in `rupring/src/middleware/cors.rs` for default headers, origin filtering, and reflected preflight headers.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p rupring middleware::cors`
Expected: FAIL because `rupring/src/middleware/cors.rs` does not exist yet.

- [ ] **Step 3: Implement minimal CORS builder and middleware**

Implement `Cors`, `CorsConfig`, `apply`, `middleware`, and registry helpers using only std and existing Rupring types.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p rupring middleware::cors`
Expected: PASS.

### Task 2: Preflight Route Lookup

**Files:**
- Modify: `rupring/src/core/route.rs`

- [ ] **Step 1: Write failing route test**

Add a unit test showing `OPTIONS /path` can find middlewares for the route named by `Access-Control-Request-Method`.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p rupring core::route::tests::test_find_preflight_route`
Expected: FAIL because the helper does not exist.

- [ ] **Step 3: Implement preflight lookup helper**

Add `find_preflight_route(root_module, request_path, requested_method)` that delegates through module/controller hierarchy and returns matching middlewares.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p rupring core::route::tests::test_find_preflight_route`
Expected: PASS.

### Task 3: Pipeline Preflight Handling

**Files:**
- Modify: `rupring/src/core/mod.rs`

- [ ] **Step 1: Write failing pipeline test**

Add an async unit test for `execute_request_pipeline` using `AWSLambdaRequest`: an `OPTIONS` request with `Origin` and `Access-Control-Request-Method` receives status `204` and CORS headers.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p rupring core::tests::test_cors_preflight_returns_204`
Expected: FAIL because preflight currently returns `404`.

- [ ] **Step 3: Implement preflight branch**

Move header parsing before route lookup, detect preflight, call `find_preflight_route`, use registered CORS config, and return a `204` hyper response.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p rupring core::tests::test_cors_preflight_returns_204`
Expected: PASS.

### Task 4: Final Verification

**Files:**
- All changed files.

- [ ] **Step 1: Format**

Run: `cargo fmt`
Expected: no formatting errors.

- [ ] **Step 2: Test rupring crate**

Run: `cargo test -p rupring`
Expected: PASS.

- [ ] **Step 3: Workspace compile test**

Run: `cargo test --workspace`
Expected: PASS or identify unrelated existing failures.
