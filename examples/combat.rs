use std::cmp::max;

use bevy::{prelude::*, render::{options::WgpuOptions, render_resource::WgpuFeatures}, ecs::component::TableStorage};
use big_brain::prelude::*;
use space::{scale::M_TO_UNIT_SCALE, controller::{tag::ControllerPlayerTag, ControllerPlugin}, tag::{PlayerTag, PlayerModelTag, MyRaycastSet}, origin::SimulationBundle, camera::{LookEntity, LookDirection, tag::CameraTag, CameraPlugin}, raycast::{RayCastSource, RayCastMesh}, lock_on::LockOnPlugin, util::{setup_cursor, setup_crosshair}, projectile::Target, fps::FpsPlugin};
use big_brain::thinker::Actor;


fn main() {
    App::new()
    .insert_resource(WgpuOptions {
        features: WgpuFeatures::POLYGON_MODE_LINE,
        ..Default::default()
    })
    .insert_resource(WindowDescriptor {
        vsync: false,
        ..Default::default()
    })
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(CameraPlugin)
    .add_plugin(ControllerPlugin)
    .add_plugin(LockOnPlugin)
    .add_plugin(FpsPlugin)
    .add_plugin(BigBrainPlugin)
    .add_startup_system(setup)
    .add_startup_system(setup_cursor)
    .add_startup_system(spawn_light)
    .add_startup_system(setup_cursor)
    .add_startup_system(spawn_target)
    .add_startup_system(setup_crosshair)
    .add_system(approach_system)
    .add_system_to_stage(BigBrainStage::Actions, drink_action_system)
    .add_system_to_stage(BigBrainStage::Scorers, thirsty_scorer_system)
    .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let spaceship_handle = asset_server.load("models/spaceship.gltf#Mesh0/Primitive0");

    let dimensions = Vec3::new(0.5, 0.5, 1.0) * M_TO_UNIT_SCALE;

    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        reflectance: 1.0,
        ..Default::default()
    });

    let body = commands
        .spawn_bundle((GlobalTransform::identity(), Transform::identity()))
        .insert(ControllerPlayerTag)
        .insert(PlayerTag)
        .insert_bundle(SimulationBundle::default())
        .id();

    let player = commands
        .spawn_bundle(PbrBundle {
            mesh: spaceship_handle,
            material: cube_material_handle,
            transform: Transform::from_scale(dimensions),
            ..Default::default()
        })
        .insert(PlayerModelTag)
        // .insert(Wireframe)
        .id();

    let camera = commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 2.25, 15.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            perspective_projection: PerspectiveProjection {
                far: 1_000.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RayCastSource::<MyRaycastSet>::new_transform_empty())
        .insert_bundle((LookDirection::default(), CameraTag))
        .id();

    commands
        .entity(body)
        .insert(LookEntity(camera))
        .push_children(&[player, camera]);
}

fn spawn_light(mut commands: Commands) {
    let theta = std::f32::consts::FRAC_PI_4;
    let light_transform = Mat4::from_euler(EulerRot::ZYX, 0.0, std::f32::consts::FRAC_PI_2, -theta);
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 9_999.0,
            shadow_projection: OrthographicProjection {
                left: -0.35,
                right: 500.35,
                bottom: -0.1,
                top: 5.0,
                near: -5.0,
                far: 5.0,
                ..Default::default()
            },
            shadow_depth_bias: 0.0,
            shadow_normal_bias: 0.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_matrix(light_transform),
        ..Default::default()
    });
}

fn spawn_target(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let spaceship_handle = asset_server.load("models/spaceship.gltf#Mesh0/Primitive0");

    let mut spawn_cube = |position, color, name| {
        let scale = 10.0;
        let material_handle = materials.add(StandardMaterial {
            base_color: color,
            reflectance: 0.02,
            unlit: false,
            ..Default::default()
        });
        commands
            .spawn_bundle(PbrBundle {
                mesh: spaceship_handle,
                material: material_handle.clone(),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(scale)),
                ..Default::default()
            })
            .insert(Enemy::new(75.0, 1.0))
            .insert(Target::default())
            .insert(Name::new(name))
            .insert(
                Thinker::build()
                .picker(FirstToScore { threshold: 0.8 })
                .when(
                    Thirsty,
                Drink {
                        until: 50.0,
                        per_second: 5.0,
                    },
                )
            )
            .insert(RayCastMesh::<MyRaycastSet>::default());
    };

    spawn_cube(Vec3::new(-15.0, 0.0, -15.0), Color::NAVY, "Sara");
}

#[derive(Component, Debug)]
pub struct Enemy {
    pub per_second: f32,
    pub distance: f32,
}

impl Enemy {
    pub fn new(thirst: f32, per_second: f32) -> Self {
        Self { distance: thirst, per_second }
    }
}

pub fn approach_system(time: Res<Time>, 
    player_query: Query<&GlobalTransform, With<PlayerModelTag>>,
    mut enemies: Query<(&mut Enemy, &mut Transform)>,
) {
        let camera_global_transform = player_query.single();
    for (mut enemy, mut transform) in enemies.iter_mut() {
        let diff = camera_global_transform.translation - transform.translation;
        transform.translation += diff * 1.0 * time.delta_seconds();
        enemy.distance = diff.length();
    }
}

#[derive(Clone, Component, Debug)]
pub struct Drink {
    until: f32,
    per_second: f32,
}

fn drink_action_system(
    time: Res<Time>,
    player_query: Query<&GlobalTransform, With<PlayerModelTag>>,
    mut thirsts: Query<(&mut Enemy, &mut Transform)>,
    // We execute actions by querying for their associated Action Component
    // (Drink in this case). You'll always need both Actor and ActionState.
    mut query: Query<(&Actor, &mut ActionState, &Drink)>,
) {
    let camera_global_transform = player_query.single();
    for (Actor(actor), mut state, drink) in query.iter_mut() {
        // Use the drink_action's actor to look up the corresponding Thirst Component.
        if let Ok((mut enemy, mut transform)) = thirsts.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    let diff = camera_global_transform.translation - transform.translation;
                    transform.translation -= diff * 5.0 * time.delta_seconds();
                    enemy.distance = diff.length();
                    if enemy.distance >= 125.0 {
                        println!("Too far");
                        // To "finish" an action, we set its state to Success or
                        // Failure.
                        *state = ActionState::Success;
                    }
                }
                // All Actions should make sure to handle cancellations!
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

// Then, we have something called "Scorers". These are special components that
// run in the background, calculating a "Score" value, which is what Big Brain
// will use to pick which Actions to execute.
//
// Just like with Actions, there's two pieces to this: the Scorer and the
// ScorerBuilder. And just like with Actions, there's a blanket implementation
// for Clone, so we only need the Component here.
#[derive(Clone, Component, Debug)]
pub struct Thirsty;

// Looks familiar? It's a lot like Actions!
pub fn thirsty_scorer_system(
    thirsts: Query<&Enemy>,
    // Same dance with the Actor here, but now we use look up Score instead of ActionState.
    mut query: Query<(&Actor, &mut Score), With<Thirsty>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(thirst) = thirsts.get(*actor) {
            // This is really what the job of a Scorer is. To calculate a
            // generic "Utility" score that the Big Brain engine will compare
            // against others, over time, and use to make decisions. This is
            // generally "the higher the better", and "first across the finish
            // line", but that's all configurable using Pickers!
            //
            // The score here must be between 0.0 and 1.0.
            let a_score = f32::min(1.0, 15. / thirst.distance);
            score.set(a_score);
        }
    }
}
