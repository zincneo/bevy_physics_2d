use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct SleepForTimer(Timer);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugins(RapierPickingPlugin)
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (set_camera, create_physics_entities).chain())
        .add_systems(Update, wake_sleeping_balls)
        .run()
}

fn set_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/**
# 锁定、阻尼和睡眠

## 锁定

1. LockedAxes是一个枚举类型组件，持有该组件的实体可以被设置为某方向禁止旋转

## 阻尼

1. Damping是一个结构体类型组件，该组件可以为实体定义参与物理模拟时候的阻力系数，不主动持有的时候默认系数为0

## 睡眠

1. Sleep是一个结构体类型组件，sleeping字段决定当前实体是否暂停参与物理模拟
2. 注意这里的睡眠是指不主动参与到碰撞运算，当前有很小的速度还是会保持，如果撞到其他物体可能会提前结束睡眠状态

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
    [
        (
            Transform::from_xyz(-80., 320., 0.),
            LockedAxes::ROTATION_LOCKED,
            Damping {
                linear_damping: 1.0,
                angular_damping: 0.,
            },
        ),
        (
            Transform::from_xyz(80., 320., 0.),
            LockedAxes::ROTATION_LOCKED,
            Damping {
                linear_damping: 10.0,
                angular_damping: 0.,
            },
        ),
    ]
    .iter()
    .enumerate()
    .for_each(|(index, ele)| {
        commands
            .spawn((
                RigidBody::Dynamic,
                Collider::ball(20.),
                Mesh2d(meshes.add(Circle::new(20.0))),
                MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0 + index as f32, 0.0))),
                Velocity::linear(vec2(0., 600.)),
                ele.0,
                ele.1,
                ele.2,
                Player,
            ))
            .observe(on_ball_clicked);
    });
}

fn on_ball_clicked(click: On<Pointer<Click>>, mut commands: Commands) {
    println!("{:?}", click.entity);

    commands.entity(click.entity).insert((
        Sleeping {
            sleeping: true,
            ..Default::default()
        },
        // 自定义的睡眠计时器组件
        SleepForTimer(Timer::from_seconds(3.0, TimerMode::Once)),
    ));
}

fn wake_sleeping_balls(
    time: Res<Time>,
    mut commands: Commands,
    mut sleeping_balls: Query<(Entity, &mut Sleeping, &mut SleepForTimer), With<Player>>,
) {
    for (entity, mut sleeping, mut sleep_timer) in &mut sleeping_balls {
        sleep_timer.tick(time.delta());

        if sleep_timer.just_finished() {
            sleeping.sleeping = false;
            commands.entity(entity).remove::<SleepForTimer>();
        }
    }
}
