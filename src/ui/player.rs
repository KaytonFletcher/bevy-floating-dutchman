use bevy::prelude::*;

pub fn spawn_player_ui(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let full_heart_handle = asset_server.load("sprites/full_heart.png");
    let heart_outline_handle = asset_server.load("sprites/heart_outline.png");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceEvenly,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            let pos = Rect {
                left: Val::Px(50.0),
                bottom: Val::Px(10.0),
                ..Default::default()
            };

            let node = NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                    position_type: PositionType::Absolute,
                    position: pos,
                    ..Default::default()
                },
                material: materials.add(full_heart_handle.into()),
                ..Default::default()
            };

            // println!("Heart position: {:?}", pos);

            parent.spawn_bundle(node);

            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                    position_type: PositionType::Absolute,
                    position: pos,
                    ..Default::default()
                },
                material: materials.add(heart_outline_handle.into()),
                ..Default::default()
            });

            // parent.spawn_bundle(NodeBundle {
            //     style: Style {
            //         size: Size::new(Val::Percent(80.0), Val::Px(100.0)),
            //         ..Default::default()
            //     },
            //     material: materials.add(Color::NONE.into()),
            //     ..Default::default()
            // });
        });
}
