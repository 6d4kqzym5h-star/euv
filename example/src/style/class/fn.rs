use crate::*;

class! {
    pub c_game_stats_bar {
        display: "flex";
        gap: "16px";
        margin-bottom: "12px";
        font-size: "14px";
        font-weight: "600";
    }

    pub c_game_stats_label {
        color: var!(game-stats-label);
    }

    pub c_game_stats_fps_value {
        color: var!(game-stats-fps);
    }

    pub c_game_stats_count_value {
        color: var!(game-stats-count);
    }

    pub c_game_stats_total_value {
        color: var!(game-stats-total);
    }

    pub c_game_description {
        line-height: "1.6";
        color: var!(game-description);
    }

    pub c_game_3d_canvas(aspect_ratio: &str, background: &str) {
        width: "100%";
        aspect-ratio: aspect_ratio;
        height: "auto";
        border-radius: "8px";
        cursor: "grab";
        display: "block";
        background: background;
        touch-action: "none";
    }

    pub c_game_2d_canvas(aspect_ratio: &str, background: &str) {
        width: "100%";
        aspect-ratio: aspect_ratio;
        height: "auto";
        border-radius: "8px";
        cursor: "pointer";
        display: "block";
        background: background;
        touch-action: "none";
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
