use bevy::{prelude::{ResMut, Commands, UiCameraBundle, NodeBundle, Color, BuildChildren}, window::Windows, ui::{AlignItems, JustifyContent, Style, Val, PositionType}, math::Size};

pub fn setup_cursor(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);
}

pub fn setup_crosshair(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(2.0), Val::Px(2.0)),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    ..Default::default()
                },
                color: Color::rgba(0.98, 0.92, 0.84, 0.5).into(),
                ..Default::default()
            });
        });
}
