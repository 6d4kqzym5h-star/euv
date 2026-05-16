mod r#fn;
mod formatter;
mod r#struct;

pub use r#struct::*;

pub(crate) use {r#fn::*, formatter::*};

use std::{
    path::PathBuf,
    process::{Output, Stdio},
    sync::Arc,
};

use {
    anyhow::{Context as AnyhowContext, Result},
    clap::Parser,
    ignore::gitignore::{Gitignore, GitignoreBuilder},
    notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher},
    tokio::{
        process::Command,
        sync::{
            MutexGuard,
            mpsc::{Receiver, Sender, channel},
        },
        time::{Duration, sleep},
    },
};

use crate::server::{AppState, ReloadEvent};
