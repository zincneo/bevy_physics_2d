use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct InitialForceTimer(Timer);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // pixels_per_meter表示多少像素相当于1m
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugins(RapierPickingPlugin)
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(InitialForceTimer(Timer::from_seconds(3., TimerMode::Once)))
        .add_systems(Startup, (set_camera, create_physics_entities).chain())
        .add_systems(Update, stop_initial_force_after_3s)
        .run()
}

fn set_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/**
# 力和冲量

## 力

1. ExternalForce组件作用于具有RigidBody组件的实体，会在每一个物理时间步持续施加指定方向和大小的力
2. 力的效果不是瞬间把速度改成某个值，而是持续改变物体的加速度，所以施加时间越长，速度变化越明显
3. ExternalForce包裹的Vec2的单位是mass_unit * px / (s * s)

## 冲量

1. ExternalImpulse组件也作用于具有RigidBody组件的实体，但它只会在组件发生变化时瞬时生效一次
2. 冲量可以理解为“在极短时间内施加的力”，它会立即改变物体当前的动量和速度，适合表现点击、击打、跳跃这类瞬时动作
3. 本示例中点击小球时会给它一个向上的冲量，因此每次点击都会立刻看到小球被弹起

## 质量

1. ColliderMassProperties组件有多种方法设置物体的质量，通常通过指定密度系数或者直接指定重力
2. 密度方法Density指定密度系数，不设置该组件的时候具有刚体和碰撞体的实体会有默认密度系数1.0计算出来质量，2d中系数就是1.0 mass_unit / 面积，算表面积的时候单位是用px计算而不是通过pixels_per_meter换算成m
3. 质量方法Mass直接指定质量，单位是mass_unit
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
            // 施加一个力，在3s之后取消
            ExternalForce {
                force: Vec2::Y * 1200.,
                ..Default::default()
            },
            ExternalImpulse::default(),
            // 这里手动指定质量，因为如果按照默认密度，则得到的加速度为 a = f / m(面积*密度) 得到的值只有0.955px/(s*s) 太小了
            // 按照标准密度的话1.0就是1.0kg/(m*m)
            ColliderMassProperties::Mass(1.),
            Transform::from_xyz(0., 40., 0.),
            Player,
        ))
        .observe(on_player_clicked);
}

fn stop_initial_force_after_3s(
    time: Res<Time>,
    mut timer: ResMut<InitialForceTimer>,
    mut player_force: Single<&mut ExternalForce, With<Player>>,
) {
    if timer.0.is_finished() {
        return;
    }

    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        // 消除这个力
        player_force.force = Vec2::ZERO;
    }
}

fn on_player_clicked(
    _click: On<Pointer<Click>>,
    mut player_impulse: Single<&mut ExternalImpulse, With<Player>>,
) {
    player_impulse.impulse += Vec2::Y * 600.;
}
