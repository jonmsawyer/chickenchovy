//! The User Interface

use bevy::prelude::*;
use bevy::render::{settings::{Backends, WgpuSettings}, RenderPlugin};
use bevy_egui::EguiPlugin;
use bevy::window::{MonitorSelection, PresentMode};
//use bevy_editor_pls::prelude::*; // Wait til this is in crates.io
//use bevy_inspector_egui::WorldInspectorPlugin;

pub mod components;
pub mod constants;
pub mod events;
pub mod plugins;
pub mod resources;
pub mod states;
pub mod utils;

/// Chickenchovy's version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The main struct for the User Interface. Defines one function called `run()`,
/// which runs the Bevy engine and User Interface.
#[derive(Debug, Copy, Clone)]
pub struct Ui;

impl Ui {
    /// The main function to run the User Interface.
    pub fn run() {
        App::new()
            // Default Bevy plugins
            .add_plugins(DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: format!(r#"Chickenchovy v{}"#, VERSION),
                        position: WindowPosition::Centered(MonitorSelection::Current),
                        present_mode: PresentMode::AutoVsync,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(RenderPlugin {
                    wgpu_settings: WgpuSettings {
                        // NOTE: This allows GL support in wgpu, which only has "best-effort" support
                        // NOTE: Additionally, wgpu only supports GL on Windows via ANGLE, which may not be available
                        backends: Some(Backends::PRIMARY | Backends::GL),
                        ..default()
                    },
                })
            )
            .add_plugin(EguiPlugin) // Default Egui plugins
            //.add_plugin(WorldInspectorPlugin::new()) // bevy_inspector_egui plugin
            // .insert_resource(WgpuSettings {
            //     // NOTE: This allows GL support in wgpu, which only has "best-effort" support
            //     // NOTE: Additionally, wgpu only supports GL on Windows via ANGLE, which may not be available
            //     backends: Some(Backends::PRIMARY | Backends::GL),
            //     ..default()
            // })
            //.add_plugin(EditorPlugin) // Wait til this is in crates.io
            // Chickenchovy's resources
            //.init_resource::<resources::Engine>()
            // Chickenchovy's plugins
            .add_plugin(plugins::WindowDescriptorPlugin)
            .add_plugin(plugins::CameraControllerPlugin)
            .add_plugin(plugins::GameStatePlugin)
            .add_plugin(plugins::UiStatePlugin)
            .add_plugin(plugins::AssetsPlugin)
            .add_plugin(plugins::MousePlugin)
            //.add_plugin(plugins::EguiPanelsPlugin)
            // Chickenchovy's custom events
            .add_event::<events::ResizeBoardEvent>()
            // Run it
            .run();
    }
}
