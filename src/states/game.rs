use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    core::math::Vector3,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

use log::info;

use crate::components::boid::Boid;

pub struct GameState;

impl SimpleState for GameState {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        let boid_sprite = load_boid(world);
        init_camera(world, &dimensions);
        init_boids(world, &dimensions, boid_sprite);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_boid(world: &mut World) -> SpriteRender {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/boid.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/boid.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };
    SpriteRender{sprite_sheet: sheet_handle.clone(), sprite_number: 0}
}

fn init_boids(world: &mut World, dimensions: &ScreenDimensions, sprite: SpriteRender) {
    let mut boid_transform = Transform::default();

    let y = dimensions.height() / 2.0;
    boid_transform.set_scale(Vector3::from_element(0.3));
    boid_transform.set_translation_xyz(dimensions.width() / 2.0, y, 0.0);

    world.register::<Boid>();
    // Create a left plank entity.
    world
        .create_entity()
        .with(sprite.clone())
        .with(Boid::new())
        .with(boid_transform)
        .build();



}