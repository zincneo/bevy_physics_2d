use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (set_camera, create_physics_entities).chain())
        .run()
}

fn set_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/**
# 碰撞体

1. Collider是物理插件系统提供的结构体类型，实现了Component特征
2. 该组件用于在物理模拟计算的时候作为碰撞的接触形状
3. 有一些附属组件可以定制碰撞体的一些参数，后续详细介绍
4. 常用的碰撞体
    - cuboid 矩形
    - ball 圆
    - compound 复合形状
    - convex meshes 顶点网格
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
    commands.spawn((
        RigidBody::Fixed,
        Collider::ball(20.),
        // 几何形状为半径20的圆形
        Mesh2d(meshes.add(Circle::new(20.0))),
        // 用红色填充形状表面
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(0., 40., 0.),
    ));
}
