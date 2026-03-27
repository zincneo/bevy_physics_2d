use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct PrismaticPlayer;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugins(RapierPickingPlugin)
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(
            Startup,
            (
                set_camera,
                create_fixed_joint,
                create_revolute_joint,
                create_prismatic_joint,
            ),
        )
        .run()
}

fn set_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/**
# 关节

1. 关节，也称为接头，允许限制一个物体相对于另一个物体的运动
2. ImpulseJoint、MultibodyJoint 两种类型，均实现了Component特征，用来包裹具体的关节实例
2. 关节类型，作为关节组件包裹内容用于指定关节类型和锚点位置
    1. FixedJointBuilder 创建固定关节，两个被固定的刚体无法产生相对运动
    2. RevoluteJointBuilder 旋转关节，刚体可以绕着锚点旋转，通常用于模拟车轮、风扇
    3. PrismaticJointBuilder 菱柱关节，控制只能在指定的轴线上运动

*/
fn create_fixed_joint(mut commands: Commands) {
    // 创建地面
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(300., 10.),
        Transform::from_xyz(0., -200., 0.),
    ));

    let parent_entity = commands
        .spawn((
            RigidBody::Dynamic,
            Collider::cuboid(5., 20.),
            Sprite {
                color: Color::Srgba(Srgba::WHITE),
                custom_size: Some(Vec2::new(10., 40.)),
                ..Default::default()
            },
            Transform::from_xyz(50., -100., 0.),
        ))
        .id();

    // 创建一个固定关节
    let mut joint = FixedJointBuilder::new()
        .local_anchor1(Vec2::new(0., 20.))
        .local_anchor2(Vec2::new(-20., 0.))
        .build();
    joint.set_contacts_enabled(false);
    // 通过固定关节绑定可以不用给定位置组件
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(20., 5.),
        Sprite {
            color: Color::Srgba(Srgba::BLUE),
            custom_size: Some(Vec2::new(40., 10.)),
            ..Default::default()
        },
        ImpulseJoint::new(parent_entity, joint),
    ));
}

fn create_revolute_joint(mut commands: Commands) {
    let parent_entity = commands
        .spawn((
            RigidBody::Fixed,
            Collider::ball(6.0),
            Transform::from_xyz(0.0, 160.0, 0.0),
        ))
        .id();

    // 创建一个旋转关节
    let mut joint = RevoluteJointBuilder::new()
        // 第一个锚点(相对于父实体的中心点座标)
        .local_anchor1(Vec2::ZERO)
        // 第二个锚点(相对于使用添加了关节组件的实体中心点的座标)
        .local_anchor2(Vec2::new(0.0, 60.0))
        .build();
    // 关节连接的两个刚体如果继续发生碰撞，锚点附近会出现明显抖动。
    // 该方法关闭两个刚体之间的碰撞
    joint.set_contacts_enabled(false);

    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(10.0, 60.0),
        Transform::from_xyz(0.0, 100.0, 0.0),
        // 给一点初始角速度，方便看到它开始摆动。
        Velocity::angular(3.0),
        ImpulseJoint::new(parent_entity, joint),
    ));
}

fn create_prismatic_joint(mut commands: Commands) {
    let parent_entity = commands
        .spawn((
            RigidBody::Fixed,
            Collider::cuboid(5., 20.),
            Sprite {
                color: Color::Srgba(Srgba::RED),
                custom_size: Some(Vec2::new(10., 40.)),
                ..Default::default()
            },
            Transform::from_xyz(100., -170., 0.),
        ))
        .id();

    // 创建一个菱柱关节
    let mut joint = PrismaticJointBuilder::new(Vec2::Y)
        // 第一个锚点放在红色柱体的右侧中心。
        .local_anchor1(Vec2::new(5., 0.))
        // 第二个锚点放在绿色方块的左侧中心。
        .local_anchor2(Vec2::new(-20., 0.))
        .limits([-20., 20.])
        .build();
    joint.set_contacts_enabled(false);

    commands
        .spawn((
            PrismaticPlayer,
            RigidBody::Dynamic,
            Collider::cuboid(20., 5.),
            // 默认密度会按像素面积算出较大的质量，600 的冲量不明显。
            ColliderMassProperties::Mass(5.),
            Sprite {
                color: Color::Srgba(Srgba::GREEN),
                custom_size: Some(Vec2::new(40., 10.)),
                ..Default::default()
            },
            Transform::from_xyz(125., -170., 0.),
            ExternalImpulse::default(),
            ImpulseJoint::new(parent_entity, joint),
        ))
        .observe(on_prismatic_player_clicked);
}

fn on_prismatic_player_clicked(
    click: On<Pointer<Click>>,
    mut player_impulses: Query<&mut ExternalImpulse, With<PrismaticPlayer>>,
) {
    if let Ok(mut player_impulse) = player_impulses.get_mut(click.entity) {
        player_impulse.impulse += Vec2::Y * 1200.;
    }
}
