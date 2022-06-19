#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

mod systems;

static DEFAULT_TARGET_CRS: &str = "EPSG:3857";

#[derive(PartialEq, Eq)]
pub enum Tool {
    Pan,
    Query,
}

pub struct RgisSettings {
    pub target_crs: String,
    pub current_tool: Tool,
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(RgisSettings {
            target_crs: DEFAULT_TARGET_CRS.into(),
            current_tool: Tool::Pan,
        })
        .add_system(systems::handle_crs_changed_events);
    }
}
