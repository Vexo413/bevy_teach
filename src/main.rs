use bevy::prelude::*;

#[derive(Component)]
struct Player {
    velocity: Vec2,
    speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Transform::from_xyz(0.0, 100.0, 0.0),
        Sprite::from_color(Color::WHITE, Vec2::new(50.0, 50.0)),
        Player {
            velocity: Vec2::ZERO,
            speed: 10.0,
        },
    ));
}

fn update(mut query: Query<(&mut Transform, &mut Player)>, keyboard: Res<ButtonInput<KeyCode>>) {
    let Ok((mut transform, mut player)) = query.get_single_mut() else {
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
    player.velocity = direction.normalize_or_zero();
    player.velocity *= 0.9;
    transform.translation += player.velocity.extend(0.0) * player.speed;
}
