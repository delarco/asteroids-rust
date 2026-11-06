use bevy::{
    post_process::bloom::Bloom, prelude::*, scene::Ready,
    ui_widgets::Button,
    audio,
};
use rand::RngExt;

use crate::{
    assets::SoundAssets,
    components::Asteroid,
    events::SpawnAsteroid,
    resources::{BloomEffect, DebugTool, Volume},
    states::{AppState, MenuState},
    systems::{
        movement::movement_system,
        wrap_objects::wrap_around_system,
    },
    utils::{BUTTON_BORDER_COLOR, button, on_hover_enter, on_hover_exit, on_pointer_press},
};

#[derive(Component, Clone, Default)]
pub struct MenuUi;

#[derive(Component, Clone, Default)]
pub struct SettingsMenuUi;

#[derive(Component, Clone, Default)]
struct BloomToggleButton;

#[derive(Component, Clone, Default)]
struct VolumeText;

#[derive(Component, Clone, Default)]
struct DebugToggleButton;

pub fn menu_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(AppState::Menu),
        (
            // asteroid_spawn_system,
            asteroids_spawn,
            menu_ui_setup.spawn(),
        ),
    );

    app.add_systems(OnExit(AppState::Menu), menu_ui_despawn);

    app.add_systems(
        Update,
        (movement_system, wrap_around_system).run_if(in_state(AppState::Menu)),
    );

    app.add_systems(
        OnEnter(MenuState::Settings),
        (settings_ui_setup.spawn(), menu_ui_hide),
    )
    .add_systems(
        OnExit(MenuState::Settings),
        (settings_ui_despawn, menu_ui_show),
    );
}

fn asteroids_spawn(mut commands: Commands) {
    let mut rng = rand::rng();

    for _ in 0..7 {
        commands.trigger(SpawnAsteroid {
            size: rng.random_range(1..=3),
            from_point: None,
            ignore_ship_area: false,
        });
    }
}

fn menu_ui_setup() -> impl SceneList {
    bsn_list! {
        @menu_ui_buttons()
        --
        @controls_text()
    }
}

fn menu_ui_buttons() -> impl Scene {
    bsn! {
        MenuUi
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(15),
            flex_direction: FlexDirection::Column,
        }
        Children [
            @button("Start")
            on(|_event: On<PointerPress>, mut next_app_state: ResMut<NextState<AppState>>| {
                next_app_state.set(AppState::InGame);
            })
            --
            @button("Settings")
            on(|_event: On<PointerPress>, mut next_menu_state: ResMut<NextState<MenuState>>| {
                next_menu_state.set(MenuState::Settings);
            })
            --
            @button("Exit")
            on(|_event: On<PointerPress>, mut app_exit_writer: MessageWriter<AppExit>| {
                app_exit_writer.write(AppExit::Success);
            })
            BackgroundColor(Color::srgb(0.3, 0.1, 0.1))
            exit_button_visibility()
        ]
    }
}

#[cfg(target_arch = "wasm32")]
pub fn exit_button_visibility() -> Visibility {
    Visibility::Hidden
}

#[cfg(not(target_arch = "wasm32"))]
pub fn exit_button_visibility() -> Visibility {
    Visibility::Inherited
}

fn menu_ui_hide(mut menu_ui_query: Query<&mut Visibility, With<MenuUi>>) {
    for mut visibility in &mut menu_ui_query {
        *visibility = Visibility::Hidden;
    }
}

fn menu_ui_show(mut menu_ui_query: Query<&mut Visibility, With<MenuUi>>) {
    for mut visibility in &mut menu_ui_query {
        *visibility = Visibility::Visible;
    }
}

fn menu_ui_despawn(
    mut commands: Commands,
    query: Query<Entity, With<MenuUi>>,
    asteroids_query: Query<Entity, With<Asteroid>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    for entity in &asteroids_query {
        commands.entity(entity).despawn();
    }
}

fn settings_ui_setup() -> impl Scene {
    bsn! {
        SettingsMenuUi
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(15),
            flex_direction: FlexDirection::Column,
        }
        Children [
            @bloom_button()
            --
            @volume_widget()
            --
            @debug_button()
            --
            @button("Back")
            on(|_event: On<PointerPress>, mut next_menu_state: ResMut<NextState<MenuState>>| {
                    next_menu_state.set(MenuState::Main);
            })
            BackgroundColor(Color::srgb(0.3, 0.1, 0.1))
        ]
    }
}

fn settings_ui_despawn(mut commands: Commands, query: Query<Entity, With<SettingsMenuUi>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn bloom_button() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Row, // Place children side-by-side
            align_items: AlignItems::Center,    // Center them vertically along the row
            column_gap: Val::Px(20.0),          // Space between label and button
        }
        Children [
            Text("Bloom")
            TextFont { font_size: px(16.0), }
            TextColor(Color::srgb(0.9, 0.9, 0.9))
            TextShadow
            --
            BloomToggleButton
            Button
            Node {
                width: px(50),
                height: px(45),
                border: px(5),
                border_radius: BorderRadius::MAX,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
            }
            BorderColor::from(BUTTON_BORDER_COLOR)
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15))
            on(on_hover_enter)
            on(on_hover_exit)
            on(on_pointer_press)
            on(|
                _event: On<PointerPress>,
                mut commands: Commands,
                mut bloom: ResMut<BloomEffect>,
                camera_entity: Single<Entity, With<Camera2d>>,
                mut button_q: Single<&mut BackgroundColor, With<BloomToggleButton>>,
            | {
                bloom.0 = !bloom.0;

                if bloom.0 {
                    commands.entity(*camera_entity).insert(Bloom::default());
                    **button_q = BackgroundColor(Color::srgb(0., 0.5, 0.));
                } else {
                    commands.entity(*camera_entity).remove::<Bloom>();
                    **button_q = BackgroundColor(Color::srgb(0.5, 0., 0.));
                }
            })
            on(|_event: On<Ready>, mut button_q: Single<&mut BackgroundColor, With<BloomToggleButton>>, bloom: Res<BloomEffect>| {
                **button_q = BackgroundColor(if bloom.0 {
                    Color::srgb(0., 0.5, 0.)
                } else {
                    Color::srgb(0.5, 0., 0.)
                });
            })
        ]
    }
}

fn volume_widget() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Row, // Place children side-by-side
            align_items: AlignItems::Center,    // Center them vertically along the row
            column_gap: Val::Px(20.0),          // Space between label and button
        }
        Children [
            Text("Volume")
            TextFont { font_size: px(16.0), }
            TextColor(Color::srgb(0.9, 0.9, 0.9))
            TextShadow
            --
            Button
            Node {
                width: px(50),
                height: px(45),
                border: px(5),
                border_radius: BorderRadius::MAX,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
            }
            BorderColor::from(BUTTON_BORDER_COLOR)
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15))
            on(on_hover_enter)
            on(on_hover_exit)
            on(|
                _event: On<PointerPress>,
                mut commands: Commands,
                mut volume: ResMut<Volume>,
                mut volume_text: Single<&mut Text, With<VolumeText>>,
                mut global_volume: ResMut<GlobalVolume>,
                sound_assets: Res<SoundAssets>,
            | {
                volume.0 = if volume.0 > 0 { volume.0 - 1 } else { 0 };
                volume_text.0 = format!("{}", volume.0);
                global_volume.volume = audio::Volume::Linear(volume.0 as f32 / 10.); 
                
                commands.spawn((
                    AudioPlayer::new(sound_assets.click.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            })
            Children [
                Text("-")
                TextFont {
                    font_size: px(16.0),
                }
                TextColor(Color::srgb(0.9, 0.9, 0.9))
                TextShadow
                Pickable::IGNORE
            ]
            --
            VolumeText
            Text::default()
            TextFont {
                font_size: px(16.0),
            }
            Node { width: px(30.0), }
            TextColor(Color::srgb(0.9, 0.9, 0.9))
            TextShadow
            on(|_event: On<Ready>, volume: Res<Volume>, mut volume_text: Single<&mut Text, With<VolumeText>>| {
                volume_text.0 = format!("{}", volume.0);
            })
            --
            Button
            Node {
                width: px(50),
                height: px(45),
                border: px(5),
                border_radius: BorderRadius::MAX,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
            }
            BorderColor::from(BUTTON_BORDER_COLOR)
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15))
            on(on_hover_enter)
            on(on_hover_exit)
            on(|
                _event: On<PointerPress>,
                mut commands: Commands,
                mut volume: ResMut<Volume>,
                mut volume_text: Single<&mut Text, With<VolumeText>>,
                mut global_volume: ResMut<GlobalVolume>,
                sound_assets: Res<SoundAssets>,
            | {
                volume.0 = if volume.0 < 10 { volume.0 + 1 } else { 10 };
                volume_text.0 = format!("{}", volume.0);
                global_volume.volume = audio::Volume::Linear(volume.0 as f32 / 10.);

                commands.spawn((
                    AudioPlayer::new(sound_assets.click.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            })
            Children [
                Text("+")
                TextFont {
                    font_size: px(16.0),
                }
                TextColor(Color::srgb(0.9, 0.9, 0.9))
                TextShadow
                Pickable::IGNORE
            ]
        ]
    }
}

fn debug_button() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Row, // Place children side-by-side
            align_items: AlignItems::Center,    // Center them vertically along the row
            column_gap: Val::Px(20.0),          // Space between label and button
        }
        Children [
            Text("Debug tool")
            TextFont {
                font_size: px(16.0),
            }
            TextColor(Color::srgb(0.9, 0.9, 0.9))
            TextShadow
            --
            DebugToggleButton
            Button
            Node {
                width: px(50),
                height: px(45),
                border: px(5),
                border_radius: BorderRadius::MAX,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
            }
            BorderColor::from(BUTTON_BORDER_COLOR)
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15))
            on(on_hover_enter)
            on(on_hover_exit)
            on(on_pointer_press)
            on(|
                _event: On<PointerPress>,
                mut debug_tool: ResMut<DebugTool>,
                mut button_q: Single<&mut BackgroundColor, With<DebugToggleButton>>,
            | {
                debug_tool.0 = !debug_tool.0;

                **button_q = if debug_tool.0 {
                    BackgroundColor(Color::srgb(0., 0.5, 0.))
                } else {
                    BackgroundColor(Color::srgb(0.5, 0., 0.))
                }
            })
            on(|_event: On<Ready>, mut button_q: Single<&mut BackgroundColor, With<DebugToggleButton>>, debug_tool: Res<DebugTool>| {
                **button_q = BackgroundColor(if debug_tool.0 {
                    Color::srgb(0., 0.5, 0.)
                } else {
                    Color::srgb(0.5, 0., 0.)
                });
            })
        ]
    }
}

fn controls_text() -> impl Scene {
     bsn! {
        MenuUi
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
        }
        Children [
            Text(r#"
Pause:        ESC
Accelerate:   W, Up arrow
Rotate left:  A, Left arrow
Rotate right: D, Right arrow
Fire bullet:  Q, Space
Fire bomb:    E, Enter
            "#)
            // Text("Rotate left: A, Left arrow\n")
            // Text("Rotate right: D, Right arrow\n")
            // Text("Fire bullet: Q, Space\n")
            // Text("Fire bomb: E, Enter")
            TextFont { font_size: px(16.0), }
            TextColor(Color::srgb(0.9, 0.9, 0.9))
            TextShadow 
        ]
    }
}