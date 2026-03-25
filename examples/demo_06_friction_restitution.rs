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
# 摩擦力和弹性

## 摩擦力

1. Friction组件用来定义实体的摩擦力系数
2. 摩擦力是一种与两个刚体之间相对切向运动发生相反的力，该力的方向与接触法线垂直，且与接触点处的相对刚体运动方向相反
3. 摩擦系数为0意味着完全没有摩擦（完全滑动接触），而系数大于或等于1则意味着摩擦力非常强
4. 默认值是0

## 弹性

1. Restitution组件用于控制实体碰撞之后恢复这个力值的系数
2. 默认系数为0
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
        // 给定摩擦系数
        Friction {
            coefficient: 0.2,
            combine_rule: CoefficientCombineRule::Average,
        },
        Transform::from_xyz(0., 0., 0.),
    ));
    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(20.),
        Mesh2d(meshes.add(Circle::new(20.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        // 禁用滚动，否则球体将滑动转换为滚动摩擦对滚动无效
        LockedAxes::ROTATION_LOCKED,
        Velocity::linear(vec2(-200., 0.)),
        Friction {
            coefficient: 0.1,
            combine_rule: CoefficientCombineRule::Average,
        },
        Transform::from_xyz(0., 40., 0.),
        Player,
    ));
    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(20.),
        Mesh2d(meshes.add(Circle::new(20.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.8, 0.0))),
        Velocity::linear(vec2(0., 200.)),
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        },
        Transform::from_xyz(120., 40., 0.),
        Player,
    ));
}
