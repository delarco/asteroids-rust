use bevy::prelude::*;

use crate::states::AppState;

#[derive(Component)]
struct IntroScreen;

#[derive(Resource, Deref, DerefMut)]
struct IntroTimer(Timer);

pub fn intro_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Intro), setup)
        .add_systems(Update, update.run_if(in_state(AppState::Intro)));
}

fn setup(mut commands: Commands) {
    commands.spawn((
        IntroScreen,
        DespawnOnExit(AppState::Intro),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        children![(
            Text::new("Asteroids"),
            TextFont { font_size: FontSize::Px(40.), ..default() },
            TextColor(Color::srgb(0.8, 0.8, 1.0)),
            TextShadow {
                offset: Vec2::new(3.0, 3.0),
                color: Color::srgb(0.1, 0.1, 0.5),
            },
        )],
    ));

    commands.insert_resource(IntroTimer(Timer::from_seconds(1.5, TimerMode::Once)));
}

fn update(
    mut game_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<IntroTimer>,
) {
    if timer.tick(time.delta()).is_finished() {
        game_state.set(AppState::Menu);
    }
}
