mod animation;
mod r#async;
mod attrs;
mod badge;
mod binding;
mod browser;
mod camera;
mod canvas;
mod conditional;
mod counter;
mod dynamic;
mod event;
mod file;
mod form;
mod home;
mod keep_alive;
mod lifecycle;
mod list;
mod modal;
mod not_found;
mod observer;
mod select;
mod sse;
mod tags;
mod timer;
mod virtual_list;
mod websocket;

pub(crate) use {
    animation::*, r#async::*, attrs::*, badge::*, binding::*, browser::*, camera::*, canvas::*,
    conditional::*, counter::*, dynamic::*, event::*, file::*, form::*, home::*, keep_alive::*,
    lifecycle::*, list::*, modal::*, not_found::*, observer::*, select::*, sse::*, tags::*,
    timer::*, virtual_list::*, websocket::*,
};
