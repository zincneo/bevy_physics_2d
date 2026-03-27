use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run()
}

/**
# Sprite
1. Sprite是bevy提供的自定义结构体，实现Component特征，实体持有该组件会默认显示在当前默认的2d摄像头中
2. 可以通过bevy默认提供的资源asset_server来加载文件，加载文件的启始路径是project/assets
3. 2d对象会在默认的2d摄像头中显示，不指定位置默认将中心点放置在摄像头原点位置
4. color字段是会将指定的颜色乘到每一个像素上
5. custom_size字段指定Sprite大小，不指定则大小为图片默认大小
6. rect则用于裁剪图片的部分区域，这时候大小变为裁剪区域的大小
*/
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands
        .spawn(Sprite {
            image: asset_server.load("bevy_bird_dark.png"),
            color: Color::Srgba(Srgba::RED),
            ..Default::default()
        })
        .insert(Transform::from_xyz(0., 50., 0.));
}
