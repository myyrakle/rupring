/// Cache Control Option
#[derive(Debug, Clone, Default)]
pub struct CacheControl {
    pub max_age: Option<u64>,
    pub s_max_age: Option<u64>,
    pub private: bool,
    pub no_cache: bool,
    pub no_store: bool,
    pub no_transform: bool,
    pub must_revalidate: bool,
    pub proxy_revalidate: bool,
    /// non-standard feature
    pub immutable: bool,
    /// non-standard, experimental feature
    pub stale_while_revalidate: Option<u64>,
    /// non-standard, experimental feature
    pub stale_if_error: Option<u64>,
}

impl CacheControl {
    /// Create a new cache control option.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the maximum age of the cache. (in seconds)
    pub fn max_age(mut self, max_age: impl Into<u64>) -> Self {
        self.max_age = Some(max_age.into());
        self
    }

    /// Set the shared maximum age of the cache. (in seconds)
    pub fn s_max_age(mut self, s_max_age: impl Into<u64>) -> Self {
        self.s_max_age = Some(s_max_age.into());
        self
    }

    /// Set the private flag.
    pub fn private(mut self, private: bool) -> Self {
        self.private = private;
        self
    }

    /// Set the no cache flag.
    pub fn no_cache(mut self, no_cache: bool) -> Self {
        self.no_cache = no_cache;
        self
    }

    /// Set the no store flag.
    pub fn no_store(mut self, no_store: bool) -> Self {
        self.no_store = no_store;
        self
    }

    /// Set the no transform flag.
    pub fn no_transform(mut self, no_transform: bool) -> Self {
        self.no_transform = no_transform;
        self
    }

    /// Set the must revalidate flag.
    pub fn must_revalidate(mut self, must_revalidate: bool) -> Self {
        self.must_revalidate = must_revalidate;
        self
    }

    /// Set the proxy revalidate flag.
    pub fn proxy_revalidate(mut self, proxy_revalidate: bool) -> Self {
        self.proxy_revalidate = proxy_revalidate;
        self
    }

    /// Set the immutable flag.
    pub fn immutable(mut self, immutable: bool) -> Self {
        self.immutable = immutable;
        self
    }

    /// Set the stale while revalidate value. (in seconds)
    pub fn stale_while_revalidate(mut self, stale_while_revalidate: impl Into<u64>) -> Self {
        self.stale_while_revalidate = Some(stale_while_revalidate.into());
        self
    }

    /// Set the stale if error value. (in seconds)
    pub fn stale_if_error(mut self, stale_if_error: impl Into<u64>) -> Self {
        self.stale_if_error = Some(stale_if_error.into());
        self
    }
}
