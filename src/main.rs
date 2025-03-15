use bevy::{asset::saver::*, color::palettes::css::*, prelude::*, state::commands};
const PALETTE: [Srgba; 10] = [
    RED, YELLOW, WHITE, BEIGE, AQUA, CRIMSON, NAVY, AZURE, LIME, BLACK,
];
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
#[derive(Component)]
struct Cursor;
#[derive(Component, Default, PartialEq)]
enum Coords {
    #[default]
    Viewport,
    Pixel,
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let background_image: Handle<Image> = asset_server.load("branding/background.png");
    commands.spawn((
        ImageNode::new(background_image.clone()),
        Node {
            width: Val::Px(1280.0),
            height: Val::Px(720.0),
            ..default()
        },
        ZIndex(-1),
    ));
    commands.spawn((
        Node {
            width: Val::Px(1280.),
            height: Val::Px(720.),
            ..default()
        },
        BackgroundColor(Color::srgba(0., 0., 0., 0.75)),
        ZIndex(0),
    ));
    commands.spawn((
        ImageNode::new(asset_server.load("branding/wolf1.png")),
        Node {
            left: Val::Px(100.),
            top: Val::Px(50.),
            width: Val::Px(300.),
            height: Val::Px(300.),
            ..default()
        },
        ZIndex(2),
    ));
    commands
        .spawn((
            Node {
                left: Val::Px(100.),
                top: Val::Px(400.),
                width: Val::Px(300.),
                height: Val::Px(300.),
                border: UiRect::axes(Val::Vw(1.), Val::Vh(1.)),
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
            BorderColor(PALETTE[2].into()),
            BackgroundColor(Color::srgba(0., 0., 0., 1.)),
            ZIndex(1),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("たたかう"),
                TextFont {
                    font: asset_server.load("fonts/japan.ttf"),
                    font_size: 50.,
                    ..default()
                },
                Node {
                    left: Val::Px(65.),
                    ..default()
                },
            ));
        });

    commands.spawn((
        Node {
            left: Val::Px(420.),
            top: Val::Px(400.),
            width: Val::Px(700.),
            height: Val::Px(300.),
            border: UiRect::axes(Val::Vw(1.), Val::Vh(1.)),
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        BorderColor(PALETTE[2].into()),
        BackgroundColor(Color::srgba(0., 0., 0., 1.)),
        ZIndex(1),
    ));
}
