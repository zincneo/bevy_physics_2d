use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // 1. 加入物理系统插件
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        // 2. 加入物理模拟系统测试渲染插件
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (set_camera, create_physics_entities).chain())
        .run()
}

fn set_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/**
# 刚体

1. RigidBody是物理插件系统提供的枚举类型，实现了Component特征
2. 具有该组件的实体可以参与物理模拟运算，不同的枚举值决定参与哪些物理运算
3. 注意添加该类型的组件只是指定参与哪些物理运算，但并不会指定参与物理运算的碰撞体，因此还必须添加碰撞体才会实际上参与物理运算
4. 枚举值 - 适合的用途
    - RigidBodyType::Dynamic - 受到外部作用力和接触的影响
    - RigidBodyType::Fixed - 表示本身无法移动。并且表现为具有无限大质量，不会受到任何力的作用可以与其他碰撞体持续产生碰撞，用于制作固定的物体如:墙体、敌人的尸体等
    - RigidBodyType::KinematicPositionBased - 表示物理引擎不得改变物体位置。用户可以自由设置下一个位置，每次更新时都会相应推断物体速度，以确保与其接触的动态物体的真实行为。这通常用于移动平台、电梯等
    - RigidBodyType::KinematicVelocityBased - 表示物理引擎不得改变物体速度。用户可以自由设置其速度，每次更新时都会相应推断下一个物体位置，以确保动态物体与其接触时的真实行为。这通常用于移动平台、电梯等

## 碰撞体

下一节具体介绍，这里只需要知道该组件决定与其他具有刚体以及碰撞体组件的实体产生碰撞的物理体积

*/
fn create_physics_entities(mut commands: Commands) {
    commands.spawn((
        // 指定物理模拟类型
        RigidBody::Fixed,
        // 指定物理碰撞体
        Collider::cuboid(300., 20.),
        // 实际会渲染的精灵图
        Sprite {
            color: Color::Srgba(Srgba::WHITE),
            custom_size: Some(Vec2::new(600., 40.)),
            ..Default::default()
        },
        // 指定初始位置
        Transform::from_xyz(0., 0., 0.),
    ));
}
