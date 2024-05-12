// crates
use bevy::prelude::*;
use rand::Rng;



// Ball Component marker
#[derive(Component)]
pub struct Ball;



// Ball Bundle
#[derive(Component)]
pub struct BallBundle {
    speed: f32,
}


// Paddle1 Component Marker
#[derive(Component)]
pub struct Paddle1;


// Paddle2 Component Marker
#[derive(Component)]
pub struct Paddle2;



// Paddle Bundle
#[derive(Component)]
pub struct PaddleBundle {
    speed: f32,
}


// main 
fn main() {
    App::new()
    // Default plugins
    .add_plugins(DefaultPlugins
    .set(ImagePlugin::default_nearest())
    .set(WindowPlugin {
        primary_window: Some(Window {
            title: "Pong game".into(),
            resolution: (960.0, 540.0).into(),
            resizable: false,
            ..default()
        }),
        ..default()
    })
    .build(),
)
    // Systems
    .add_systems(Startup, setup)
    .add_systems(Startup, spawn_ball)
    .add_systems(Startup, spawn_paddle1)
    .add_systems(Startup, spawn_paddle2)
    .add_systems(Update, ball_movement)
    .add_systems(Update, paddle1_movement)
    .add_systems(Update, paddle2_movement)
    .run();
}


// camera
pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(1.0, 1.0, 0.),
            rotation: Quat::from_rotation_x(0.),
            scale: Vec3::new(0.5, 0.5, 1.0),
            ..default() 
        },
        ..default() 
    }); 
}


// Ball
pub fn spawn_ball(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Ball, BallBundle {
        speed: 100.0,
    },
    SpriteBundle {
        texture: asset_server.load("ball.png"),
        transform: Transform {
            ..default()
        },
        ..default()
    },
));
}


// ball movement
pub fn ball_movement(
    mut query: Query<(&mut Transform, &BallBundle), With<Ball>>,
    time: Res<Time>,
) {
    for (mut transform, ballbundle) in &mut query {
        let movement_amount = ballbundle.speed * time.delta_seconds();     // speed is the speed parameter from for loop!
        let direction = rand::thread_rng().gen_range(0..=360);
        let dir_rad = f32::to_radians(direction as f32);
        let x = dir_rad.cos();
        let y = dir_rad.sin();
        let movement = Vec3::new(x, y, 0.0) * movement_amount;
        transform.translation += movement;
    }
}


// Paddle1
pub fn spawn_paddle1(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Paddle1, PaddleBundle {
        speed: 100.0,
    },
    SpriteBundle {
        texture: asset_server.load("paddle.png"),
        transform: Transform {
            translation: Vec3::new(200., 0., 0.),
            scale: Vec3::new(2.5, 3.0, 0.),
            ..default()
        },
        ..default()
    }
));
}

pub fn paddle1_movement(
    mut query: Query<(&mut Transform, &PaddleBundle), With<Paddle1>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, paddlebundle) in &mut query {
        let movement_amount = paddlebundle.speed * time.delta_seconds();

        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= movement_amount;
        }
    }
}


// Paddle2
pub fn spawn_paddle2(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Paddle2, PaddleBundle {
        speed: 100.0,
    },
    SpriteBundle {
        texture: asset_server.load("paddle.png"),
        transform: Transform {
            translation: Vec3::new(-200., 0., 0.),
            scale: Vec3::new(2.5, 3.0, 0.),
            ..default()
        },
        ..default()
    }));
}

// Paddle2 Movement
pub fn paddle2_movement(
    mut query: Query<(&mut Transform, &PaddleBundle), With<Paddle2>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, paddlebundle) in &mut query {
        let movement_amount = paddlebundle.speed * time.delta_seconds();

        if input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= movement_amount;
        }
    }
}


// checking or collisions
//pub fn ch_co(mut commands: Commands,
//    p1q: Query<(&mut Transform, &PaddleBundle), With<Paddle1>>,
//    p2q: Query<(&mut Transform, &PaddleBundle), With<Paddle2>>,
//    bq: Query<(&mut Transform, &BallBundle), With<Ball>>) {

//    }