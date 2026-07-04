use crate::*;

/// Implementation of touch point extraction from DOM touch events.
impl NativeTouchPoint {
    /// Extracts all active touch points from a `TouchEvent`.
    ///
    /// Iterates over the `touches` list of the given `TouchEvent` and
    /// builds a `Vec<NativeTouchPoint>` with each touch point's
    /// identifier, viewport coordinates, screen coordinates, page
    /// coordinates, and offset coordinates relative to the target element.
    ///
    /// The offset coordinates (`offset_x`, `offset_y`) are computed by
    /// subtracting the target element's bounding rect from the touch's
    /// client coordinates, since the browser `Touch` object does not
    /// provide `offsetX`/`offsetY` directly.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The native DOM touch event.
    ///
    /// # Returns
    ///
    /// - `Vec<NativeTouchPoint>` - All currently active touch points.
    pub fn extract_all(event: &Event) -> Vec<NativeTouchPoint> {
        let touches_value: JsValue = Reflect::get(event.as_ref(), &JsValue::from_str("touches"))
            .ok()
            .unwrap_or(JsValue::NULL);
        let touches: Array = touches_value.unchecked_into();
        let target: JsValue = event
            .target()
            .map_or(JsValue::NULL, |event_target: EventTarget| {
                event_target.into()
            });
        let element: Element = target.unchecked_into();
        let rect: DomRect = element.get_bounding_client_rect();
        let rect_left: f64 = rect.left();
        let rect_top: f64 = rect.top();
        (0..touches.length())
            .map(|index: u32| {
                let touch: JsValue = touches.get(index);
                let identifier: i32 = Reflect::get(&touch, &JsValue::from_str("identifier"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .map(|value: f64| value as i32)
                    .unwrap_or(0);
                let client_x: i32 = Reflect::get(&touch, &JsValue::from_str("clientX"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .map(|value: f64| value as i32)
                    .unwrap_or(0);
                let client_y: i32 = Reflect::get(&touch, &JsValue::from_str("clientY"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .map(|value: f64| value as i32)
                    .unwrap_or(0);
                let screen_x: i32 = Reflect::get(&touch, &JsValue::from_str("screenX"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .map(|value: f64| value as i32)
                    .unwrap_or(0);
                let screen_y: i32 = Reflect::get(&touch, &JsValue::from_str("screenY"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .map(|value: f64| value as i32)
                    .unwrap_or(0);
                let page_x: i32 = Reflect::get(&touch, &JsValue::from_str("pageX"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .map(|value: f64| value as i32)
                    .unwrap_or(0);
                let page_y: i32 = Reflect::get(&touch, &JsValue::from_str("pageY"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .map(|value: f64| value as i32)
                    .unwrap_or(0);
                let offset_x: i32 = (client_x as f64 - rect_left).round() as i32;
                let offset_y: i32 = (client_y as f64 - rect_top).round() as i32;
                NativeTouchPoint {
                    identifier,
                    client_x,
                    client_y,
                    screen_x,
                    screen_y,
                    offset_x,
                    offset_y,
                    page_x,
                    page_y,
                }
            })
            .collect()
    }

    /// Extracts the changed touch points from a `TouchEvent`.
    ///
    /// The `changedTouches` list contains touch points that have changed
    /// since the last touch event:
    /// - For `touchstart` - newly added touch points.
    /// - For `touchmove` - touch points that have moved.
    /// - For `touchend` / `touchcancel` - removed touch points.
    ///
    /// This is useful for determining which specific fingers were lifted
    /// in a `touchend` event, since the `touches` list no longer contains
    /// them.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The native DOM touch event.
    ///
    /// # Returns
    ///
    /// - `Vec<NativeTouchPoint>` - The touch points that changed in this event.
    pub fn extract_changed(event: &Event) -> Vec<NativeTouchPoint> {
        let touches_value: JsValue =
            Reflect::get(event.as_ref(), &JsValue::from_str("changedTouches"))
                .ok()
                .unwrap_or(JsValue::NULL);
        let touches: Array = touches_value.unchecked_into();
        let target: JsValue = event
            .target()
            .map_or(JsValue::NULL, |event_target: EventTarget| {
                event_target.into()
            });
        let element: Element = target.unchecked_into();
        let rect: DomRect = element.get_bounding_client_rect();
        let rect_left: f64 = rect.left();
        let rect_top: f64 = rect.top();
        (0..touches.length())
            .map(|index: u32| {
                let touch: JsValue = touches.get(index);
                let identifier: i32 = Reflect::get(&touch, &JsValue::from_str("identifier"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .map(|value: f64| value as i32)
                    .unwrap_or(0);
                let client_x: i32 = Reflect::get(&touch, &JsValue::from_str("clientX"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .map(|value: f64| value as i32)
                    .unwrap_or(0);
                let client_y: i32 = Reflect::get(&touch, &JsValue::from_str("clientY"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .map(|value: f64| value as i32)
                    .unwrap_or(0);
                let screen_x: i32 = Reflect::get(&touch, &JsValue::from_str("screenX"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .map(|value: f64| value as i32)
                    .unwrap_or(0);
                let screen_y: i32 = Reflect::get(&touch, &JsValue::from_str("screenY"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .map(|value: f64| value as i32)
                    .unwrap_or(0);
                let page_x: i32 = Reflect::get(&touch, &JsValue::from_str("pageX"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .map(|value: f64| value as i32)
                    .unwrap_or(0);
                let page_y: i32 = Reflect::get(&touch, &JsValue::from_str("pageY"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .map(|value: f64| value as i32)
                    .unwrap_or(0);
                let offset_x: i32 = (client_x as f64 - rect_left).round() as i32;
                let offset_y: i32 = (client_y as f64 - rect_top).round() as i32;
                NativeTouchPoint {
                    identifier,
                    client_x,
                    client_y,
                    screen_x,
                    screen_y,
                    offset_x,
                    offset_y,
                    page_x,
                    page_y,
                }
            })
            .collect()
    }
}

/// Implementation of high-precision touch point extraction from DOM touch events.
impl NativeTouchPointF64 {
    /// Extracts all active touch points with high-precision `f64` offset coordinates
    /// from a `TouchEvent`.
    ///
    /// Similar to `NativeTouchPoint::extract_all`, but returns `f64` precision for
    /// offset/client coordinates, which is essential for canvas drawing
    /// and other pixel-precise interactions.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The native DOM touch event.
    ///
    /// # Returns
    ///
    /// - `Vec<NativeTouchPointF64>` - All currently active touch points with `f64` coordinates.
    pub fn extract_all(event: &Event) -> Vec<NativeTouchPointF64> {
        let touches_value: JsValue = Reflect::get(event.as_ref(), &JsValue::from_str("touches"))
            .ok()
            .unwrap_or(JsValue::NULL);
        let touches: Array = touches_value.unchecked_into();
        let target: JsValue = event
            .target()
            .map_or(JsValue::NULL, |event_target: EventTarget| {
                event_target.into()
            });
        let element: Element = target.unchecked_into();
        let rect: DomRect = element.get_bounding_client_rect();
        let rect_left: f64 = rect.left();
        let rect_top: f64 = rect.top();
        (0..touches.length())
            .map(|index: u32| {
                let touch: JsValue = touches.get(index);
                let identifier: i32 = Reflect::get(&touch, &JsValue::from_str("identifier"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .map(|value: f64| value as i32)
                    .unwrap_or(0);
                let client_x: f64 = Reflect::get(&touch, &JsValue::from_str("clientX"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .unwrap_or(0.0);
                let client_y: f64 = Reflect::get(&touch, &JsValue::from_str("clientY"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .unwrap_or(0.0);
                let screen_x: f64 = Reflect::get(&touch, &JsValue::from_str("screenX"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .unwrap_or(0.0);
                let screen_y: f64 = Reflect::get(&touch, &JsValue::from_str("screenY"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .unwrap_or(0.0);
                let page_x: f64 = Reflect::get(&touch, &JsValue::from_str("pageX"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .unwrap_or(0.0);
                let page_y: f64 = Reflect::get(&touch, &JsValue::from_str("pageY"))
                    .ok()
                    .and_then(|value: JsValue| value.as_f64())
                    .unwrap_or(0.0);
                let offset_x: f64 = client_x - rect_left;
                let offset_y: f64 = client_y - rect_top;
                NativeTouchPointF64 {
                    identifier,
                    client_x,
                    client_y,
                    screen_x,
                    screen_y,
                    offset_x,
                    offset_y,
                    page_x,
                    page_y,
                }
            })
            .collect()
    }
}
