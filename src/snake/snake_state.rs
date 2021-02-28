use amethyst::{core::Transform, prelude::*, renderer::Camera};

use crate::utils::get_screen_dimensions;

#[derive(Default)]
pub struct SnakeState;

impl SimpleState for SnakeState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let (width, height) = get_screen_dimensions(world);
        initialise_camera(world, width, height);
    }
}

fn initialise_camera(world: &mut World, width: f32, height: f32) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(width * 0.5, height * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();
}
