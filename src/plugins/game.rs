use std::time::Duration;
use bevy::prelude::*;
use rand::RngExt;

use crate::{
    assets::{LevelList, ShipAssets, SoundAssets}, components::{Asteroid, Bullet, Ship, ShipBody, ShipPart, Velocity, WrapAround}, events::{BreakShip, NextLevel, SpawnAsteroid}, plugins::{in_game_ui, menu::exit_button_visibility}, resources::{LevelNumber, Lives, StartingLives}, states::{AppState, InGameState}, systems::{
        asteroid::on_asteroid_break,
        bullet::{bullet_update_system, on_fire_projectile},
        collision::collision_update_system,
        movement::movement_system,
        ship::{on_ship_hit, ship_setup_system, ship_update_system},
        wrap_objects::wrap_around_system,
    }, utils::button,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(in_game_ui::InGameUiPlugin);

        app.add_systems(OnEnter(AppState::InGame), 
            |mut next_game_state: ResMut<NextState<InGameState>>| {
                next_game_state.set(InGameState::Start);
            }
        );

        app.add_systems(OnExit(AppState::InGame), cleanup);

        app.add_systems(OnEnter(InGameState::Start), (
            cleanup,
            ship_setup_system,
            start_game,
        ).chain().run_if(in_state(AppState::InGame)));
        
        app.add_systems(
            Update,
            (
                movement_system,
                wrap_around_system,
                bullet_update_system,
            ).run_if(
                in_state(AppState::InGame).and_then(not(in_state(InGameState::Paused)))
            ),
        );

        app.add_systems(
            Update,
            (
                ship_update_system,
                collision_update_system,
            ).run_if(
                in_state(AppState::InGame)
                .and_then(in_state(InGameState::Running))
            ),
        );

        app.add_systems(Update, check_pause_system.run_if(
            in_state(AppState::InGame)
            .and_then(in_state(InGameState::Running))
            .or_else(in_state(InGameState::Paused))
        ));
        
        app.add_systems(OnEnter(InGameState::Paused), pause_setup.spawn());
        app.add_systems(OnExit(InGameState::Paused), pause_cleanup);
        
        app.add_systems(OnEnter(InGameState::Respawning), 
            respawn_setup_system.run_if(in_state(AppState::InGame))
        );
        app.add_systems(Update, 
            respawn_update_system.run_if(in_state(AppState::InGame))
        );
        
        app.add_systems(OnEnter(InGameState::Gameover), (
            gameover_setup,
            gameover_menu_setup.spawn(),
        ));
        app.add_systems(OnExit(InGameState::Gameover), gameover_cleanup);

        app.add_observer(on_fire_projectile.run_if(in_state(AppState::InGame)));
        app.add_observer(on_asteroid_break.run_if(in_state(AppState::InGame)));
        app.add_observer(on_ship_hit.run_if(in_state(AppState::InGame)));
        app.add_observer(on_next_level.run_if(in_state(AppState::InGame)));
        app.add_observer(on_break_ship.run_if(in_state(AppState::InGame)));
    }
}

fn start_game(
    mut commands: Commands,
    mut lives: ResMut<Lives>,
    starting_lives: Res<StartingLives>,
    mut next_game_state: ResMut<NextState<InGameState>>,
) {
    lives.0 = starting_lives.0;
    commands.trigger(NextLevel);
    next_game_state.set(InGameState::Running);
}

fn cleanup(
    mut commands: Commands,
    ship_quey: Query<Entity, With<Ship>>,
    ship_parts: Query<Entity, With<ShipPart>>,
    bullets: Query<Entity, With<Bullet>>,
    asteroids: Query<Entity, With<Asteroid>>,
    mut level_number: ResMut<LevelNumber>,
) {
    level_number.0 = 0;
    
    for ship in &ship_quey {
        commands.entity(ship).despawn();
    }

    for part in &ship_parts {
        commands.entity(part).despawn();
    }

    for entity in &bullets {
        commands.entity(entity).despawn();
    }

    for entity in &asteroids {
        commands.entity(entity).despawn();
    }
}

#[derive(Component, Clone, Default)]
pub struct PauseMenuUi;

fn pause_setup() -> impl Scene {
    bsn! {
        PauseMenuUi
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(15),
            flex_direction: FlexDirection::Column,
        }
        Children [
            @button("Resume")
            on(|_event: On<PointerPress>, mut next_game_state: ResMut<NextState<InGameState>>| {
                next_game_state.set(InGameState::Running);
            })
            --
            @button("Menu")
            on(|
                _event: On<PointerPress>,
                mut next_app_state: ResMut<NextState<AppState>>,
                mut next_game_state: ResMut<NextState<InGameState>>,
            | {
                next_app_state.set(AppState::Menu);
                next_game_state.set(InGameState::Running);
            })
            --
            @button("Quit")
            on(|_event: On<PointerPress>, mut app_exit_writer: MessageWriter<AppExit>| {
                    app_exit_writer.write(AppExit::Success);
            })
            BackgroundColor(Color::srgb(0.3, 0.1, 0.1))
            exit_button_visibility()
        ]
    }
}

fn pause_cleanup(
    mut commands: Commands,
    query: Query<Entity, With<PauseMenuUi>>
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn on_next_level(
    _trigger: On<NextLevel>,
    mut commands: Commands,
    mut level_number: ResMut<LevelNumber>,
    level_list: Res<LevelList>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    level_number.0 += 1;

    let level = level_list.0.iter().find(|level| level.number == level_number.0);

    if level.is_none() {
        next_app_state.set(AppState::Credits);
        return;
    } 

    for size in level.unwrap().asteroids_sizes.iter() {
        commands.trigger(SpawnAsteroid {
            size: *size,
            from_point: None,
            ignore_ship_area: true,
        });
    }
}

fn check_pause_system(
    keys: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<InGameState>>,
    mut next_game_state: ResMut<NextState<InGameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_game_state.set(
            if *game_state == InGameState::Running {
                InGameState::Paused
            } else {
                InGameState::Running
            }
        );
    }
}

fn on_break_ship(
    _trigger: On<BreakShip>,
    mut commands: Commands,
    ship_assets: Res<ShipAssets>,
    ship_transform: Single<&Transform, With<Ship>>,
    sound_assets: Res<SoundAssets>,
) {
    let mut rng = rand::rng();

    commands.spawn((
        AudioPlayer::new(sound_assets.ship_hit.clone()),
        PlaybackSettings::DESPAWN,
    ));

    // spawn ship parts
    for part in ship_assets.parts.iter() {
        let random_v: f32 = rng.random_range(20.0..45.0) * if rng.random() { 1.0 } else { -1.0 };
        commands.spawn((
            ShipPart,
            WrapAround,
            Velocity(Vec2::new(
                random_v,
                random_v * -1.,
            )),
            Gizmo { handle: part.clone(), ..default() },
            Visibility::default(),
            Transform::from_translation(ship_transform.translation)
                .with_rotation(ship_transform.rotation),
        ));
    }
}

fn gameover_setup(
    mut commands: Commands,
    ship_entity: Single<Entity, With<Ship>>,
) {
    commands.entity(*ship_entity).despawn();
}

#[derive(Component, Clone, Default)]
struct GameoverMenuUi;

fn gameover_menu_setup() -> impl Scene {
    bsn! {
        GameoverMenuUi
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(15),
            flex_direction: FlexDirection::Column,
        }
        Children [
            @button("Restart")
            on(|
                _event: On<PointerPress>,
                // mut commands: Commands,
                mut next_game_state: ResMut<NextState<InGameState>>,
                // ship_quey: Query<Entity, With<Ship>>,
                // bullets: Query<Entity, With<Bullet>>,
                // asteroids: Query<Entity, With<Asteroid>>,
                // mut level_number: ResMut<LevelNumber>,
            | {
                next_game_state.set(InGameState::Start);
            })
            --
            @button("Menu")
            on(|
                _event: On<PointerPress>,
                mut next_app_state: ResMut<NextState<AppState>>,
                mut next_game_state: ResMut<NextState<InGameState>>,
            | {
                next_game_state.set(InGameState::None);
                next_app_state.set(AppState::Menu);
            })
            --
            @button("Quit")
            on(|_event: On<PointerPress>, mut app_exit_writer: MessageWriter<AppExit>| {
                app_exit_writer.write(AppExit::Success);
            })
            BackgroundColor(Color::srgb(0.3, 0.1, 0.1))
        ]
    }
}

fn gameover_cleanup(
    mut commands: Commands,
    query: Query<Entity, With<GameoverMenuUi>>,
    ship_parts: Query<Entity, With<ShipPart>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }

    for entity in &ship_parts {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
struct BlinkTimer(Timer);

fn respawn_setup_system(
    mut commands: Commands,
    mut ship: Single<(&mut Transform, &mut Velocity), With<Ship>>,
    ship_body: Single<Entity, With<ShipBody>>,
) {
    *ship.0 = Transform::IDENTITY;
    ship.1.0 = Vec2::ZERO;

    commands.entity(*ship_body).insert(
        BlinkTimer(Timer::new(Duration::from_secs_f32(0.5), TimerMode::Repeating)),
    );
}

fn respawn_update_system(
    mut commands: Commands,
    ship_assets: Res<ShipAssets>,
    ship_body: Single<(Entity, &mut BlinkTimer, Option<&Gizmo>), With<ShipBody>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut next_game_state: ResMut<NextState<InGameState>>
) {
    let (entity, mut blink_timer, gizmo) = ship_body.into_inner();
    
    blink_timer.0.tick(time.delta());

    if keys.get_just_pressed().len() > 0 {

        commands.entity(entity).remove::<BlinkTimer>();

        if gizmo.is_none() {
            commands.entity(entity).insert(Gizmo {
                handle: ship_assets.body.clone(),
                ..default()
            });
        }

        next_game_state.set(InGameState::Running);
        return;
    }

    if blink_timer.0.just_finished() {
        match gizmo {
            Some(_) => {
                commands.entity(entity).remove::<Gizmo>();
            },
            None => {
                commands.entity(entity).insert(Gizmo {
                    handle: ship_assets.body.clone(),
                    ..default()
                });
            },
        };
    }
}
