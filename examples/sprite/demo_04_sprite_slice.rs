use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run()
}

/**
# Sprite

1. 2d加载图片资源拉伸方式
*/
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let font = asset_server.load("FiraSans-Bold.ttf");
    let style = TextFont {
        font: font.into(),
        ..default()
    };

    let handle_1 = asset_server.load("slice_square.png");
    let handle_2 = asset_server.load("slice_square_2.png");
    spawn_sprites(
        &mut commands,
        handle_1,
        Vec3::new(-600.0, 150.0, 0.0),
        200.0,
        style.clone(),
        40.,
    );

    spawn_sprites(
        &mut commands,
        handle_2,
        Vec3::new(-600.0, -150.0, 0.0),
        80.0,
        style,
        40.,
    );
}

fn spawn_sprites(
    commands: &mut Commands,
    texture_handle: Handle<Image>,
    mut position: Vec3,
    slice_border: f32,
    style: TextFont,
    gap: f32,
) {
    // 输入图片规格为512x512
    let cases = [
        // 1. 大小符合原始大小比例Auto自动拉伸
        (
            "Original",
            style.clone(),
            Vec2::splat(100.0),
            SpriteImageMode::Auto,
        ),
        // 2. 大小不符合原始比例Auto自动拉伸填满大小
        (
            "Stretched",
            style.clone(),
            Vec2::new(100.0, 200.0),
            SpriteImageMode::Auto,
        ),
        // 3. Sliced模式指定拉伸比例
        (
            "With Slicing",
            style.clone(),
            Vec2::new(100.0, 200.0),
            SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(slice_border),
                center_scale_mode: SliceScaleMode::Stretch,
                ..default()
            }),
        ),
        // 4. Sliced模式指定拉伸比例
        (
            "With Tiling",
            style.clone(),
            Vec2::new(100.0, 200.0),
            SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(slice_border),
                center_scale_mode: SliceScaleMode::Tile { stretch_value: 0.5 },
                sides_scale_mode: SliceScaleMode::Tile { stretch_value: 0.2 },
                ..default()
            }),
        ),
        // 5. Sliced模式指定拉伸比例
        (
            "With Tiling",
            style.clone(),
            Vec2::new(300.0, 200.0),
            SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(slice_border),
                center_scale_mode: SliceScaleMode::Tile { stretch_value: 0.2 },
                sides_scale_mode: SliceScaleMode::Tile { stretch_value: 0.3 },
                ..default()
            }),
        ),
        // 6. Sliced模式指定拉伸比例
        (
            "With Corners Constrained",
            style,
            Vec2::new(300.0, 200.0),
            SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(slice_border),
                center_scale_mode: SliceScaleMode::Tile { stretch_value: 0.1 },
                sides_scale_mode: SliceScaleMode::Tile { stretch_value: 0.2 },
                max_corner_scale: 0.2,
            }),
        ),
    ];

    for (label, text_style, size, scale_mode) in cases {
        position.x += 0.5 * size.x;
        commands.spawn((
            Sprite {
                image: texture_handle.clone(),
                custom_size: Some(size),
                image_mode: scale_mode,
                ..default()
            },
            Transform::from_translation(position),
            // 创建子实体
            children![(
                Text2d::new(label),
                text_style,
                TextLayout::new_with_justify(Justify::Center),
                Transform::from_xyz(0., -0.5 * size.y - 10., 0.0),
                bevy::sprite::Anchor::TOP_CENTER,
            )],
        ));
        position.x += 0.5 * size.x + gap;
    }
}
