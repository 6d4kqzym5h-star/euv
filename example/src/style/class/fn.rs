use super::*;

class! {
    pub c_game_stats_bar {
        display: "flex";
        gap: "16px";
        margin-bottom: "12px";
        font-size: "14px";
        font-weight: "600";
    }

    pub c_game_stats_label {
        color: "inherit";
    }

    pub c_game_stats_fps_value {
        color: "inherit";
    }

    pub c_game_stats_count_value {
        color: "inherit";
    }

    pub c_game_stats_total_value {
        color: "inherit";
    }

    pub c_game_description {
        line-height: "1.5";
        color: "inherit";
        margin-bottom: var!(gap-component);
    }

    pub c_game_canvas_wrapper(aspect_ratio: &str) {
        position: "relative";
        width: "100%";
        // Defensive fallback for browsers that reject the `aspect-ratio`
        // property (older WebKit, certain embedded engines). Without it
        // the wrapper collapses to height: 0 and the canvas is not
        // visible. The CSS values here are the largest demo canvas
        // dimensions used on this page (800x600 logical px) so the
        // canvas always has a non-zero layout box to render into.
        min-width: "800px";
        min-height: "600px";
        aspect-ratio: aspect_ratio;
    }

    pub c_game_3d_canvas {
        width: "100%";
        height: "100%";
        cursor: "grab";
        display: "block";
        background: var!(accent);
        touch-action: "none";
    }

    pub c_game_2d_canvas {
        width: "100%";
        height: "100%";
        cursor: "pointer";
        display: "block";
        background: var!(accent);
        touch-action: "none";
    }

    pub c_canvas_pixelated {
        image-rendering: "pixelated";
        image-rendering: "crisp-edges";
    }

    pub c_keep_alive_tab_visible {
        display: "block";
    }

    pub c_keep_alive_tab_hidden {
        display: "none";
    }

    pub c_binding_slider_label_accent {
        color: var!(accent);
    }

    pub c_binding_color_preview_bg(background: &str) {
        background: background;
    }

    pub c_slider_value(value_percent: &str) {
        {
            "--value"
        }
        : value_percent;
    }

    pub c_anim_scale_shrink {
        transform: "scale(0.85)";
    }

    pub c_anim_scale_normal {
        transform: "scale(1)";
    }
}
