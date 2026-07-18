//! euv-engine
//!
//! A high-performance 2D and 3D game engine built on the euv framework for WebAssembly,
//! featuring an ECS-style entity system, fixed-timestep game loop, canvas rendering,
//! WebGPU rendering, physics simulation, collision detection, sprite animation,
//! scene management, asset loading, and Web Audio integration.

mod asset;
mod audio;
mod collider;
mod config;
mod engine;
mod entity;
mod input;
mod math;
mod physics;
mod renderer;
mod scene;
mod scheduler;
mod spatial;
mod sprite;

pub use {
    asset::*, audio::*, collider::*, config::*, engine::*, entity::*, input::*, math::*,
    physics::*, renderer::*, scene::*, scheduler::*, spatial::*, sprite::*,
};

use euv::*;

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
};

use {
    js_sys::*, lombok_macros::*, wasm_bindgen::prelude::*, wasm_bindgen_futures::JsFuture,
    web_sys::*,
};
