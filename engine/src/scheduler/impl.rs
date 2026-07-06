use crate::*;

/// Implements default configuration and state initialization for scheduler types.
impl Default for SchedulerConfig {
    fn default() -> SchedulerConfig {
        SchedulerConfig::new(DEFAULT_FIXED_TIMESTEP, DEFAULT_MAX_FRAME_TIME)
    }
}

/// Implements `Default` for `SchedulerState` as a freshly created stopped state.
impl Default for SchedulerState {
    fn default() -> SchedulerState {
        SchedulerState::new(UNINITIALIZED_TIME)
    }
}

/// Implements time retrieval and tick execution for `SchedulerState`.
impl SchedulerState {
    /// Returns the current high-resolution timestamp in seconds from `performance.now()`.
    ///
    /// # Returns
    ///
    /// - `f64` - The current time in seconds.
    pub fn current_time() -> f64 {
        let window_value: Window = window().expect("no global window exists");
        let performance: JsValue = Reflect::get(
            window_value.as_ref(),
            &JsValue::from_str(PERFORMANCE_OBJECT),
        )
        .expect("performance object should exist");
        let now_value: JsValue =
            Reflect::get(&performance, &JsValue::from_str(PERFORMANCE_NOW_METHOD))
                .expect("performance.now should exist");
        let now_millis: f64 = now_value
            .as_f64()
            .expect("performance.now should return a number");
        now_millis / 1000.0
    }

    /// Performs one tick of the fixed-timestep scheduler.
    ///
    /// Calculates the elapsed frame time, clamps it to `max_frame_time`, accumulates it,
    /// then runs as many fixed updates as needed. Finally, computes the interpolation
    /// factor and calls the render callback.
    ///
    /// # Arguments
    ///
    /// - `&SchedulerConfig` - The scheduler configuration.
    /// - `&TickHandlerRc` - The handler receiving update and render callbacks.
    pub fn tick(&mut self, config: &SchedulerConfig, handler: &TickHandlerRc) {
        let current_time: f64 = Self::current_time();
        let frame_time: f64 = if self.get_last_time() == UNINITIALIZED_TIME {
            config.get_fixed_timestep()
        } else {
            current_time - self.get_last_time()
        };
        self.set_last_time(current_time);
        let clamped_frame_time: f64 = frame_time.min(config.get_max_frame_time());
        *self.get_mut_accumulator() += clamped_frame_time;
        while self.get_accumulator() >= config.get_fixed_timestep() {
            handler.borrow_mut().on_update(config.get_fixed_timestep());
            *self.get_mut_accumulator() -= config.get_fixed_timestep();
            *self.get_mut_update_count() += 1;
        }
        let interpolation: f64 = self.get_accumulator() / config.get_fixed_timestep();
        handler.borrow_mut().on_render(interpolation);
        *self.get_mut_frame_count() += 1;
    }
}

/// Implements lifecycle management for `SchedulerHandle`.
impl SchedulerHandle {
    /// Stops the scheduler and cancels any pending animation frame request.
    pub fn stop(&self) {
        let state_rc: Rc<RefCell<SchedulerState>> = self.get_state().clone();
        let mut state = state_rc.borrow_mut();
        state.set_running(false);
        if let Some(id) = state.get_mut_raf_id().take() {
            let window_value: Window = window().expect("no global window exists");
            let _ = window_value.cancel_animation_frame(id);
        }
        drop(state);
        self.get_closure_cell().borrow_mut().take();
    }

    /// Returns whether the scheduler is currently running.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the scheduler is running.
    pub fn is_running(&self) -> bool {
        self.get_state().borrow().get_running()
    }

    /// Returns the total number of fixed update steps executed.
    ///
    /// # Returns
    ///
    /// - `u64` - The update count.
    pub fn update_count(&self) -> u64 {
        self.get_state().borrow().get_update_count()
    }

    /// Returns the total number of render frames executed.
    ///
    /// # Returns
    ///
    /// - `u64` - The frame count.
    pub fn frame_count(&self) -> u64 {
        self.get_state().borrow().get_frame_count()
    }

    /// Starts the scheduler with the given configuration and handler.
    ///
    /// Creates a `requestAnimationFrame`-driven loop that calls `tick`
    /// on each animation frame. The returned `SchedulerHandle` can be used to stop the scheduler.
    ///
    /// # Arguments
    ///
    /// - `SchedulerConfig` - The scheduler configuration.
    /// - `TickHandlerRc` - The handler receiving update and render callbacks.
    ///
    /// # Returns
    ///
    /// - `SchedulerHandle` - A handle to control the running scheduler.
    pub fn start(config: SchedulerConfig, handler: TickHandlerRc) -> SchedulerHandle {
        let state: Rc<RefCell<SchedulerState>> =
            Rc::new(RefCell::new(SchedulerState::new(UNINITIALIZED_TIME)));
        let closure_cell: RafClosureCell = Rc::new(RefCell::new(None));
        state.borrow_mut().set_running(true);
        let state_clone: Rc<RefCell<SchedulerState>> = state.clone();
        let closure_cell_clone: RafClosureCell = closure_cell.clone();
        let handler_clone: TickHandlerRc = handler.clone();
        let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            {
                let mut state_ref = state_clone.borrow_mut();
                if !state_ref.get_running() {
                    return;
                }
                state_ref.tick(&config, &handler_clone);
            }
            if state_clone.borrow().get_running() {
                let window_value: Window = window().expect("no global window exists");
                let cell: RafClosureCell = closure_cell_clone.clone();
                let id: i32 = window_value
                    .request_animation_frame(
                        cell.borrow()
                            .as_ref()
                            .expect("raf closure should exist")
                            .as_ref()
                            .unchecked_ref(),
                    )
                    .unwrap_or(0);
                state_clone.borrow_mut().set_raf_id(Some(id));
            }
        }) as Box<dyn FnMut()>);
        let window_value: Window = window().expect("no global window exists");
        let id: i32 = window_value
            .request_animation_frame(raf_closure.as_ref().unchecked_ref())
            .unwrap_or(0);
        state.borrow_mut().set_raf_id(Some(id));
        *closure_cell.borrow_mut() = Some(raf_closure);
        SchedulerHandle::new(state, closure_cell)
    }
}
