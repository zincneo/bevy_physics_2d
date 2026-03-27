use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate)
        .run()
}

#[derive(Resource)]
struct AnimationState {
    min: f32,
    max: f32,
    current: f32,
    speed: f32,
}

/**
# Sprite
1. image_mode用来指定图片显示方式，Tiled模式允许图片重复显示
2. tile_x,tile_y指定在哪些轴线上复制图片
3. 该模式下图片会在允许复制的轴线上自动复制以填满custom_size
*/
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.insert_resource(AnimationState {
        min: 128.0,
        max: 512.0,
        current: 128.0,
        speed: 50.0,
    });
    commands.spawn(Sprite {
        image: asset_server.load("icon.png"),
        image_mode: SpriteImageMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 0.5, // The image will tile every 128px
        },
        ..default()
    });
}

fn animate(mut sprites: Query<&mut Sprite>, mut state: ResMut<AnimationState>, time: Res<Time>) {
    if state.current >= state.max || state.current <= state.min {
        state.speed = -state.speed;
    };
    state.current += state.speed * time.delta_secs();
    for mut sprite in &mut sprites {
        sprite.custom_size = Some(Vec2::splat(state.current));
    }
}
