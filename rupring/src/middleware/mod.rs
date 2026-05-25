//! Built-in middleware provided by Rupring.
//!
//! Middleware in this module follows the same function shape as user-defined
//! middleware: it receives a [`crate::Request`], a [`crate::Response`], and a
//! [`crate::NextFunction`], then returns the final [`crate::Response`].

/// Cross-Origin Resource Sharing middleware.
pub mod cors;
