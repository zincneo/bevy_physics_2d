use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Player;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugins(RapierPickingPlugin)
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (set_camera, create_physics_entities).chain())
        .run()
}

fn set_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/**
# 速度和重力

## 速度

1. Velocity组件作用于具有RigidBody组件的实体
2. 2d中只关心线速度，包含一个Vec2，指定x,y轴方向的速度

## 重力

1. GravityScale组件用于修改默认的重力系数，默认所有具有RigidBody组件的实体会受到标准设置的重力影响

*/
fn create_physics_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(300., 20.),
        Sprite {
            color: Color::Srgba(Srgba::WHITE),
            custom_size: Some(Vec2::new(600., 40.)),
            ..Default::default()
        },
        Transform::from_xyz(0., 0., 0.),
    ));
    commands
        .spawn((
            RigidBody::Dynamic,
            Collider::ball(20.),
            Mesh2d(meshes.add(Circle::new(20.0))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
            // 速度单位和 Transform 一致；这里使用的是 px/s。
            Velocity::linear(vec2(0., 600.)),
            // 重力系数配置为默认的0.5倍
            GravityScale(0.5),
            Transform::from_xyz(0., 40., 0.),
            Player,
        ))
        // 监听点击事件
        .observe(on_player_clicked);
}

fn on_player_clicked(
    _click: On<Pointer<Click>>,
    mut player_velocity: Single<&mut Velocity, With<Player>>,
) {
    player_velocity.linvel.y = 300.0;
}
