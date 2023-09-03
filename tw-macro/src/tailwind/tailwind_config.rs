use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TailwindConfig {
    pub theme: Theme,
    pub variants: Variants,
    pub plugins: Plugins,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    #[serde(flatten)]
    pub overrides: CustomisableClasses,
    pub extend: CustomisableClasses,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomisableClasses {
    pub screens: HashMap<String, String>,
    pub colors: HashMap<String, String>,
    pub spacing: HashMap<String, String>,

    pub border_colors: HashMap<String, HashMap<String, String>>,

    pub border_widths: HashMap<String, String>,

    pub width: HashMap<String, String>,
    pub height: HashMap<String, String>,

    pub min_width: HashMap<String, String>,

    pub min_height: HashMap<String, String>,

    pub max_width: HashMap<String, String>,

    pub max_height: HashMap<String, String>,

    pub padding: HashMap<String, String>,
    pub margin: HashMap<String, String>,
    pub negative_margin: HashMap<String, String>,

    pub shadows: HashMap<String, String>,

    pub z_index: HashMap<String, String>,
    pub opacity: HashMap<String, String>,
    pub stroke: HashMap<String, String>,

    pub accent_color: HashMap<String, String>,
    pub accessibility: HashMap<String, String>,

    pub align_content: HashMap<String, String>,
    pub align_items: HashMap<String, String>,
    pub align_self: HashMap<String, String>,
    pub animation: HashMap<String, String>,
    pub appearance: HashMap<String, String>,
    pub aspect_ratio: HashMap<String, String>,
    pub backdrop_blur: HashMap<String, String>,
    pub backdrop_brightness: HashMap<String, String>,
    pub backdrop_contrast: HashMap<String, String>,
    pub backdrop_filter: HashMap<String, String>,
    pub backdrop_grayscale: HashMap<String, String>,
    pub backdrop_hue_rotate: HashMap<String, String>,
    pub backdrop_invert: HashMap<String, String>,
    pub backdrop_opacity: HashMap<String, String>,
    pub backdrop_saturate: HashMap<String, String>,
    pub backdrop_sepia: HashMap<String, String>,
    pub background_attachment: HashMap<String, String>,
    pub background_blend_mode: HashMap<String, String>,
    pub background_clip: HashMap<String, String>,
    pub background_color: HashMap<String, String>,
    pub background_image: HashMap<String, String>,
    pub background_opacity: HashMap<String, String>,
    pub background_origin: HashMap<String, String>,
    pub background_position: HashMap<String, String>,
    pub background_repeat: HashMap<String, String>,
    pub background_size: HashMap<String, String>,
    pub blur: HashMap<String, String>,
    pub border_collapse: HashMap<String, String>,
    pub border_color: HashMap<String, String>,
    pub border_opacity: HashMap<String, String>,
    pub border_radius: HashMap<String, String>,
    pub border_spacing: HashMap<String, String>,
    pub border_style: HashMap<String, String>,
    pub border_width: HashMap<String, String>,
    pub box_decoration_break: HashMap<String, String>,
    pub box_shadow: HashMap<String, String>,
    pub box_shadow_color: HashMap<String, String>,
    pub box_sizing: HashMap<String, String>,
    pub break_after: HashMap<String, String>,
    pub break_before: HashMap<String, String>,
    pub break_inside: HashMap<String, String>,
    pub brightness: HashMap<String, String>,
    pub caption_side: HashMap<String, String>,
    pub caret_color: HashMap<String, String>,
    pub clear: HashMap<String, String>,
    pub columns: HashMap<String, String>,
    pub container: HashMap<String, String>,
    pub content: HashMap<String, String>,
    pub contrast: HashMap<String, String>,
    pub cursor: HashMap<String, String>,
    pub display: HashMap<String, String>,
    pub divide_color: HashMap<String, String>,
    pub divide_opacity: HashMap<String, String>,
    pub divide_style: HashMap<String, String>,
    pub divide_width: HashMap<String, String>,
    pub drop_shadow: HashMap<String, String>,
    pub fill: HashMap<String, String>,
    pub filter: HashMap<String, String>,
    pub flex: HashMap<String, String>,
    pub flex_basis: HashMap<String, String>,
    pub flex_direction: HashMap<String, String>,
    pub flex_grow: HashMap<String, String>,
    pub flex_shrink: HashMap<String, String>,
    pub flex_wrap: HashMap<String, String>,
    pub float: HashMap<String, String>,
    pub font_family: HashMap<String, String>,
    pub font_size: HashMap<String, String>,
    pub font_smoothing: HashMap<String, String>,
    pub font_style: HashMap<String, String>,
    pub font_variant_numeric: HashMap<String, String>,
    pub font_weight: HashMap<String, String>,
    pub gap: HashMap<String, String>,
    pub gradient_color_stops: HashMap<String, String>,
    pub grayscale: HashMap<String, String>,
    pub grid_auto_columns: HashMap<String, String>,
    pub grid_auto_flow: HashMap<String, String>,
    pub grid_auto_rows: HashMap<String, String>,
    pub grid_column: HashMap<String, String>,
    pub grid_column_end: HashMap<String, String>,
    pub grid_column_start: HashMap<String, String>,
    pub grid_row: HashMap<String, String>,
    pub grid_row_end: HashMap<String, String>,
    pub grid_row_start: HashMap<String, String>,
    pub grid_template_columns: HashMap<String, String>,
    pub grid_template_rows: HashMap<String, String>,
    pub hue_rotate: HashMap<String, String>,
    pub hyphens: HashMap<String, String>,
    pub inset: HashMap<String, String>,
    pub invert: HashMap<String, String>,
    pub isolation: HashMap<String, String>,
    pub justify_content: HashMap<String, String>,
    pub justify_items: HashMap<String, String>,
    pub justify_self: HashMap<String, String>,
    pub letter_spacing: HashMap<String, String>,
    pub line_clamp: HashMap<String, String>,
    pub line_height: HashMap<String, String>,
    pub list_style_image: HashMap<String, String>,
    pub list_style_position: HashMap<String, String>,
    pub list_style_type: HashMap<String, String>,
    pub mix_blend_mode: HashMap<String, String>,
    pub object_fit: HashMap<String, String>,
    pub object_position: HashMap<String, String>,
    pub order: HashMap<String, String>,
    pub outline_color: HashMap<String, String>,
    pub outline_offset: HashMap<String, String>,
    pub outline_style: HashMap<String, String>,
    pub outline_width: HashMap<String, String>,
    pub overflow: HashMap<String, String>,
    pub overscroll_behavior: HashMap<String, String>,
    pub place_content: HashMap<String, String>,
    pub place_items: HashMap<String, String>,
    pub place_self: HashMap<String, String>,
    pub placeholder_color: HashMap<String, String>,
    pub placeholder_opacity: HashMap<String, String>,
    pub pointer_events: HashMap<String, String>,
    pub position: HashMap<String, String>,
    pub preflight: HashMap<String, String>,
    pub resize: HashMap<String, String>,
    pub ring_color: HashMap<String, String>,
    pub ring_offset_color: HashMap<String, String>,
    pub ring_offset_width: HashMap<String, String>,
    pub ring_opacity: HashMap<String, String>,
    pub ring_width: HashMap<String, String>,
    pub rotate: HashMap<String, String>,
    pub saturate: HashMap<String, String>,
    pub scale: HashMap<String, String>,
    pub scroll_behavior: HashMap<String, String>,
    pub scroll_margin: HashMap<String, String>,
    pub scroll_padding: HashMap<String, String>,
    pub scroll_snap_align: HashMap<String, String>,
    pub scroll_snap_stop: HashMap<String, String>,
    pub scroll_snap_type: HashMap<String, String>,
    pub sepia: HashMap<String, String>,
    pub skew: HashMap<String, String>,
    pub space: HashMap<String, String>,
    pub stroke_width: HashMap<String, String>,
    pub table_layout: HashMap<String, String>,
    pub text_align: HashMap<String, String>,
    pub text_color: HashMap<String, String>,
    pub text_decoration: HashMap<String, String>,
    pub text_decoration_color: HashMap<String, String>,
    pub text_decoration_style: HashMap<String, String>,
    pub text_decoration_thickness: HashMap<String, String>,
    pub text_indent: HashMap<String, String>,
    pub text_opacity: HashMap<String, String>,
    pub text_overflow: HashMap<String, String>,
    pub text_transform: HashMap<String, String>,
    pub text_underline_offset: HashMap<String, String>,
    pub touch_action: HashMap<String, String>,
    pub transform: HashMap<String, String>,
    pub transform_origin: HashMap<String, String>,

    pub transition_delay: HashMap<String, String>,
    pub transition_duration: HashMap<String, String>,
    pub transition_property: HashMap<String, String>,
    pub transition_timing_function: HashMap<String, String>,
    pub translate: HashMap<String, String>,
    pub user_select: HashMap<String, String>,
    pub vertical_align: HashMap<String, String>,
    pub visibility: HashMap<String, String>,
    pub whitespace: HashMap<String, String>,
    pub will_change: HashMap<String, String>,
    pub word_break: HashMap<String, String>,
    // pub extend: HashMap<String, HashMap<String, String>>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Extend {
//     pub screens: HashMap<String, String>,
//     // add other fields as needed
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Variants {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Plugins {}
