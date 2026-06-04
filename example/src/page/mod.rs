mod animation;
mod r#async;
mod attrs;
mod binding;
mod browser;
mod conditional;
mod dynamic;
mod event;
mod file;
mod form;
mod home;
mod keep_alive;
mod lifecycle;
mod list;
mod modal;
mod multimedia;
mod not_found;
mod observer;
mod select;
mod signals;
mod timer;
mod virtual_list;

pub(crate) use {
    animation::*, r#async::*, attrs::*, binding::*, browser::*, conditional::*, dynamic::*,
    event::*, file::*, form::*, home::*, keep_alive::*, lifecycle::*, list::*, modal::*,
    multimedia::*, not_found::*, observer::*, select::*, signals::*, timer::*, virtual_list::*,
};
