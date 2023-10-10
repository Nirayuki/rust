use bevy::{prelude::*, math::*, sprite::collide_aabb::*, render::camera::*};
use pig::PigPlugin;
use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::InspectorOptions;

mod pig;

// Player variaveis
const PLAYER_INITIAL_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const PLAYER_SIZE: Vec2 = Vec2::new(16.0, 16.0);
const PLAYER_SPEED: f32 = 200.0;
const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Money(pub f32);



fn main(){
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Logic Farming Rougelike".into(),
                        resolution: (853.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(Money(100.0))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .register_type::<Money>()
        .add_plugins(PigPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run()
}

fn setup(mut commands: Commands, assets_server: Res<AssetServer>){
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);

    let player_texture = assets_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: PLAYER_INITIAL_POSITION,
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR,
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            texture: player_texture,
            ..default()
        },
        Player,
    ));
}

fn move_player(
    input: Res<Input<KeyCode>>,
    time_step: Res<FixedTime>,
    mut query: Query<&mut Transform, With<Player>>,
){
    let mut player_query = query.single_mut();
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    if input.pressed(KeyCode::A){
        direction_x -= 1.0;
    }

    if input.pressed(KeyCode::D){
        direction_x += 1.0;
    }

    if input.pressed(KeyCode::W){
        direction_y += 1.0;
    }

    if input.pressed(KeyCode::S){
        direction_y -= 1.0;
    } 
    
    let mut new_x =
        player_query.translation.x + direction_x * PLAYER_SPEED * time_step.period.as_secs_f32();

    let mut new_y =
        player_query.translation.y + direction_y * PLAYER_SPEED * time_step.period.as_secs_f32();

    player_query.translation.x = new_x;
    player_query.translation.y = new_y;
}


