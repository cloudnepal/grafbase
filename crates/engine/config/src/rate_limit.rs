use std::{path::Path, time::Duration};

use super::{PathId, StringId};

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct GraphRateLimit {
    pub limit: usize,
    pub duration: Duration,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct RateLimitConfig {
    pub global: Option<GraphRateLimit>,
    pub storage: RateLimitStorage,
    pub redis: RateLimitRedisConfig,
}

#[derive(Debug, Clone, Copy)]
pub struct RateLimitConfigRef<'a> {
    pub storage: RateLimitStorage,
    pub redis: RateLimitRedisConfigRef<'a>,
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub enum RateLimitStorage {
    #[default]
    Memory,
    Redis,
}

impl RateLimitStorage {
    pub fn is_redis(&self) -> bool {
        matches!(self, Self::Redis)
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct RateLimitRedisConfig {
    pub url: StringId,
    pub key_prefix: StringId,
    pub tls: Option<RateLimitRedisTlsConfig>,
}

#[derive(Debug, Clone, Copy)]
pub struct RateLimitRedisConfigRef<'a> {
    pub url: &'a str,
    pub key_prefix: &'a str,
    pub tls: Option<RateLimitRedisTlsConfigRef<'a>>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct RateLimitRedisTlsConfig {
    pub cert: Option<PathId>,
    pub key: Option<PathId>,
    pub ca: Option<PathId>,
}

#[derive(Debug, Clone, Copy)]
pub struct RateLimitRedisTlsConfigRef<'a> {
    pub cert: Option<&'a Path>,
    pub key: Option<&'a Path>,
    pub ca: Option<&'a Path>,
}

#[derive(Clone, Copy, Debug)]
pub enum RateLimitKey<'a> {
    Global,
    Subgraph(&'a str),
}
