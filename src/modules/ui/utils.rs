//! Utils module.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
// use bevy::render::camera::RenderTarget;

use super::components::MainCamera;

/// Get the screen coordinates of the mouse cursor.
pub fn get_mouse_coords(window: &Window) -> Vec2 {
    window.cursor_position().map_or(Vec2::ZERO, |cursor| cursor)
}

/// Get the world coordinates of the mouse cursor.
pub fn get_world_coords(
    query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) -> Vec2 {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = query.single();


    // // get the window that the camera is displaying to (or the primary window)
    // let window = if let RenderTarget::Window(id) = camera.target {
    //     match windows.get(id) {
    //         Some(win) => win,
    //         None => return Vec2::ZERO,
    //     }
    // } else {
    //     windows.get_primary().unwrap()
    // };

    let window = windows.single();

    // check if the cursor is inside the window and get its position
    // if let Some(screen_pos) = window.cursor_position() {
    //     // get the size of the window
    //     let window_size = Vec2::new(window.width() as f32, window.height() as f32);

    //     // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    //     let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

    //     // matrix for undoing the projection and camera transform
    //     let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

    //     // use it to convert ndc to world-space coordinates
    //     let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

    //     // reduce it to a 2D value
    //     let world_pos: Vec2 = world_pos.truncate();

    //     world_pos
    // } else {
    //     Vec2::ZERO
    // }

    window.cursor_position().map_or(Vec2::ZERO, |screen_pos| {
        // get the size of the window
        let window_size = Vec2::new(window.width(), window.height());

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        world_pos
    })
}
