//! euv-engine
//!
//! A high-performance 2D and 3D game engine built on the euv framework for WebAssembly,
//! featuring an ECS-style entity system, fixed-timestep game loop, canvas rendering,
//! WebGPU rendering, physics simulation, collision detection, sprite animation,
//! scene management, asset loading, and Web Audio integration.

mod asset;
mod audio;
mod cell;
mod collider;
mod config;
mod easing;
mod engine;
mod entity;
mod input;
mod math;
mod particle;
mod physics;
mod renderer;
mod scene;
mod scheduler;
mod spatial;
mod sprite;
mod timer;
mod tween;

pub use {
    asset::*, audio::*, cell::*, collider::*, config::*, easing::*, engine::*, entity::*, input::*,
    math::*, particle::*, physics::*, renderer::*, scene::*, scheduler::*, spatial::*, sprite::*,
    timer::*, tween::*,
};

use euv::*;

use std::{
    cell::UnsafeCell,
    collections::{HashMap, HashSet},
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
};

use {
    js_sys::*, lombok_macros::*, wasm_bindgen::prelude::*, wasm_bindgen_futures::JsFuture,
    web_sys::*,
};

#[cfg(test)]
mod tests;
