use std::path::PathBuf;

use rust_embed::RustEmbed;

use common::types::ResolverMessageLevel;

pub use crate::file_watcher::FileEventType;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Assets;

#[derive(Clone, Debug)]
pub enum ServerMessage {
    Ready(u16),
    Reload(PathBuf, FileEventType),
    StartResolverBuild(String),
    CompleteResolverBuild {
        name: String,
        duration: std::time::Duration,
    },
    ResolverMessage {
        resolver_name: String,
        level: ResolverMessageLevel,
        message: String,
    },
}
