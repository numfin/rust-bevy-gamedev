use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(WindowDescriptor {
            title: "Kill numfin".into(),
            mode: bevy::window::WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(gun_input.system())
        .add_system(animate_pistol.system())
        .run()
}

#[derive(Default)]
struct Pistol {
    state: i32,
}

#[derive(Eq, PartialEq)]
enum GameStateVariants {
    Idle,
    Shooting,
}
impl Default for GameStateVariants {
    fn default() -> Self {
        GameStateVariants::Idle
    }
}

#[derive(Default)]
struct GameState {
    state: GameStateVariants,
}

#[derive(Bundle, Default)]
struct Game {
    state: GameState,
    pistol: Pistol,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let pistol_texture: Handle<Texture> = asset_server.load("gun_sprite.png");
    let pistol_atlas = TextureAtlas::from_grid(pistol_texture, Vec2::new(30.0, 30.0), 4, 1);
    let pistol_atlas_handle = texture_atlases.add(pistol_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: pistol_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.075, true));

    commands.spawn_bundle(Game::default());
}

fn animate_pistol(
    time: Res<Time>,
    atlas_collection: Res<Assets<TextureAtlas>>,
    query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
    query_game: Query<&GameState>,
) {
    query.for_each_mut(|(mut timer, mut sprite, texture_atlas_handle)| {
        timer.tick(time.delta());

        match query_game.single() {
            Ok(game_state) => {
                if game_state.state == GameStateVariants::Shooting {
                    if timer.finished() {
                        if let Some(texture_atlas) = atlas_collection.get(texture_atlas_handle) {
                            sprite.index =
                                ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32
                        }
                    }
                } else {
                    sprite.index = 0
                }
            }
            Err(_) => sprite.index = 0,
        }
    })
}

fn gun_input(mouse: Res<Input<MouseButton>>, mut query: Query<&mut GameState>) {
    if mouse.just_pressed(MouseButton::Left) {
        query
            .single_mut()
            .map(|mut game_state| {
                game_state.state = GameStateVariants::Shooting;
            })
            .ok();
    } else if mouse.just_released(MouseButton::Left) {
        query
            .single_mut()
            .map(|mut game_state| {
                game_state.state = GameStateVariants::Idle;
            })
            .ok();
    }
}
