//! Assets plugin

use bevy::{asset::LoadState, prelude::*};

use super::super::states::GameState;

asset_collection!(
    SpriteCollection,
    Atlas(tiles, "default_board.png", 256., 256., 14, 1, None, None),
    Image(board_background, "board_background.png")
);

/// System to initialize our asset collection
fn init_collection(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    server: Res<AssetServer>,
) {
    let collection = SpriteCollection::init(&server, &mut texture_atlases);
    commands.insert_resource(collection);
}

/// System to check that our asset collection is ready
fn check_assets_ready(
    server: Res<AssetServer>,
    collection: Res<SpriteCollection>,
    atlases: Res<Assets<TextureAtlas>>,
    // mut app_state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>) {
    if let LoadState::Loaded = collection.get_collection_load_state(&server, &atlases) {
        next_state.set(GameState::Next);
    }
}

/// Assets Bevy plugin.
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system_set(
        //     // Load our assets during the AssetLoading state
        //     SystemSet::on_enter(GameState::AssetLoading).with_system(init_collection),
        // )
        // .add_system_set(
        //     // Load our assets during the AssetLoading state
        //     SystemSet::on_update(GameState::AssetLoading).with_system(check_assets_ready),
        // );
        // app.add_systems((init_collection, check_assets_ready));
        // app.add_system(init_collection);
        app.add_startup_system(init_collection)
            .add_system(check_assets_ready.in_set(OnUpdate(GameState::AssetLoading)));
    }
}
