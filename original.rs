use bevy::prelude::*;

#[derive(Component)]
struct Player {
    direction: Vec2,
    velocity: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d::default(), Transform::default()));
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::new(100.0, 100.0)),
        Transform::default(),
        Player {
            direction: Vec2::ZERO,
            velocity: Vec2::ZERO,
        },
    ));
}

fn update(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut transform, mut player)) = player_query.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }

    player.velocity = player.direction.normalize_or_zero();
    player.velocity *= 0.9;

    transform.translation += player.velocity.extend(0.0) * 10.0;
}
