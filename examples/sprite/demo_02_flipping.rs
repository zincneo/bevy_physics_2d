use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run()
}

/**
# Sprite
1. flip_x, flip_y字段用于指定图片按照轴线翻转
*/
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands
        .spawn(Sprite {
            image: asset_server.load("bevy_bird_dark.png"),
            flip_x: true,
            flip_y: true,
            ..Default::default()
        })
        .insert(Transform::from_xyz(0., 50., 0.));
}
