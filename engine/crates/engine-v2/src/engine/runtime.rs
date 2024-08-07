use futures::future::BoxFuture;
use grafbase_telemetry::otel::opentelemetry::metrics::Meter;
use runtime::{entity_cache::EntityCache, fetch::Fetcher, kv::KvStore, rate_limiting::RateLimiter};

pub trait Runtime: Send + Sync + 'static {
    type Hooks: runtime::hooks::Hooks;
    type CacheFactory: runtime::hot_cache::HotCacheFactory;

    fn fetcher(&self) -> &Fetcher;
    fn kv(&self) -> &KvStore;
    fn trusted_documents(&self) -> &runtime::trusted_documents_client::Client;
    fn meter(&self) -> &Meter;
    fn hooks(&self) -> &Self::Hooks;
    fn cache_factory(&self) -> &Self::CacheFactory;
    fn rate_limiter(&self) -> &RateLimiter;
    fn sleep(&self, duration: std::time::Duration) -> BoxFuture<'static, ()>;
    fn entity_cache(&self) -> &dyn EntityCache;
}
