//! Assets plugin

// use bevy::input::mouse::MouseButtonInput;
// use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::render::view::Visibility;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
//use bevy::render::camera::RenderTarget;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use super::super::components::{MainCamera, MouseCursor};
use super::super::constants::SPRITE_WIDTH;
use super::super::resources::UiResource;
use super::super::states::GameState;
use super::super::utils::{get_mouse_coords, get_world_coords};
// use crate::Engine;

/// ECS System. Run once. Initialize the on-board mouse cursor.
fn init_mouse_cursor(mut commands: Commands) {
    let mut rng = SmallRng::seed_from_u64(1_u64);
    let mut color = Color::from(rng.gen::<[f32; 3]>());
    color.set_a(0.65);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SPRITE_WIDTH, SPRITE_WIDTH)),
                color,
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 1.0),
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(MouseCursor);
}

/// ECS System. Run once. Initialize the From Square on-board cursor.
fn _init_from_square_cursor(mut commands: Commands) {
    let mut rng = SmallRng::seed_from_u64(2_u64);
    let mut color = Color::from(rng.gen::<[f32; 3]>());
    color.set_a(0.65);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SPRITE_WIDTH, SPRITE_WIDTH)),
                color,
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 1.0),
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            ..Default::default()
        });
}

/// ECS System. Run once. Initialize the To Square on-board cursor.
fn _init_to_square_cursor(mut commands: Commands) {
    let mut rng = SmallRng::seed_from_u64(3_u64);
    let mut color = Color::from(rng.gen::<[f32; 3]>());
    color.set_a(0.65);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SPRITE_WIDTH, SPRITE_WIDTH)),
                color,
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 1.0),
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            ..Default::default()
        });
}

/// ECS System. Run on each frame. Update the on-board mouse cursor.
fn update_mouse_cursor(
    // mut mouse_query: Query<(&mut Visibility, &mut Transform), With<MouseCursor>>,
    mut mouse_query: Query<(&mut Visibility, &mut Transform), With<MouseCursor>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    //windows: Res<Windows>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut ui_state: ResMut<UiResource>,
) {
    let window = windows.single();
    let mouse_coords = get_mouse_coords(window);
    let world_coords = get_world_coords(camera_query, windows);
    let (mut visibility, mut transform) = mouse_query.get_single_mut().unwrap();
    // let (scale, _, _) = compute_coords(ui_state.square_pixels);
    let x = (world_coords[0] / ui_state.square_pixels).floor() * ui_state.square_pixels;
    let y = (world_coords[1] / ui_state.square_pixels).floor() * ui_state.square_pixels;
    // let min = START_X_COORD * ui_state.square_pixels;
    // let max = START_Y_COORD * ui_state.square_pixels;

    ui_state.mouse_cursor_screen_coords = mouse_coords;
    ui_state.mouse_cursor_world_coords = world_coords;

    // if x < min
    //     || x >= max
    //     || y < min
    //     || y >= max
    //     || (world_coords[0] == 0. && world_coords[1] == 0.)
    // {
    //     visibility.is_visible = false;
    //     return;
    // }

    transform.translation = Vec3::new(x, y, 0.2);
    // transform.scale = Vec3::new(scale, scale, 0.);
    // visibility.is_visible = ui_state.show_mouse_cursor;
    if ui_state.show_mouse_cursor {
        *visibility = Visibility::Inherited;
    }
}

/// Mouse Bevy plugin.
pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        // app.add_system_set(
        //     SystemSet::on_enter(GameState::Next)
        //         .with_system(init_mouse_cursor)
        //         .with_system(init_from_square_cursor)
        //         .with_system(init_to_square_cursor),
        // )
        // .add_system_set(
        //     SystemSet::on_update(GameState::Next)
        //         .with_system(update_mouse_cursor)
        //         // .with_system(update_mouse_click),
        // );
        // app.add_systems((init_mouse_cursor, init_from_square_cursor, init_to_square_cursor))
        // app.add_systems((init_mouse_cursor, update_mouse_cursor));
        app.add_system(init_mouse_cursor.in_schedule(OnEnter(GameState::AssetLoading)))
            .add_system(update_mouse_cursor.in_schedule(OnEnter(GameState::Next)));
    }
}
