//! euv CLI
//!
//! The official CLI tool for the euv UI framework,
//! providing run/build/fmt modes with hot reload and
//! wasm-pack integration.

mod build;
mod error;
mod fmt;
mod logger;
mod mode;
mod server;

use {build::*, error::*, fmt::*, logger::*, mode::*, server::*};

use std::{
    ffi,
    fmt::Arguments,
    io,
    path::{Component, Path, PathBuf},
    process::{Output, Stdio},
    sync::{Arc, OnceLock},
    time::Duration,
};

use {
    clap::Parser,
    hyperlane::*,
    ignore::gitignore::{Gitignore, GitignoreBuilder},
    lombok_macros::*,
    notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher},
    qrcode::{QrCode, render::unicode::Dense1x2},
    serde::Serialize,
    tokio::{
        fs::{
            ReadDir, canonicalize, create_dir_all, metadata, read, read_dir, read_to_string,
            remove_dir_all, remove_file, write,
        },
        process::Command,
        spawn,
        sync::{
            RwLock, RwLockWriteGuard, broadcast,
            mpsc::{Receiver, Sender, channel},
        },
        time::{Interval, interval, sleep},
    },
};

type Result<T = ()> = std::result::Result<T, EuvError>;

/// Entry point for the euv CLI.
///
/// Parses command-line arguments and dispatches to the appropriate
/// mode (run/build/fmt).
///
/// - `run` — build + file watcher + dev server
/// - `build` — build only
/// - `fmt` — format euv macro invocations
#[tokio::main]
pub async fn main() -> std::result::Result<(), EuvError> {
    Logger::init(log::LevelFilter::Info);
    let cli: Cli = Cli::parse();
    match cli.get_command() {
        Mode::Run(mode_args) => {
            run_mode(mode_args.clone()).await?;
        }
        Mode::Build(mode_args) => {
            build_mode(mode_args.clone()).await?;
        }
        Mode::Fmt(fmt_args) => {
            fmt_mode(fmt_args.clone()).await?;
        }
    }
    Ok(())
}
