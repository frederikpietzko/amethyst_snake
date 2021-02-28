use amethyst::{
    core::Transform,
    prelude::*,
    renderer::{Camera, SpriteRender},
};

use crate::utils::{get_screen_dimensions, load_sprite_sheet};

pub const ARENA_CELLS: u8 = 20;
pub const LINE_LENGHT: f32 = 32.;

type Dimensions = (f32, f32);

#[derive(Default)]
pub struct SnakeState;

impl SimpleState for SnakeState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let dimensions: Dimensions = get_screen_dimensions(world);
        initialise_camera(world, dimensions);
        initialise_grid(world, dimensions);
    }
}

fn initialise_camera(world: &mut World, (width, height): Dimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(width * 0.5, height * 0.5, 1.);
    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();
}

fn initialise_grid(world: &mut World, (width, height): Dimensions) {
    let padding_y = (height - ARENA_CELLS as f32 * LINE_LENGHT) / 2.;
    let padding_x = width * 0.05;
    let sprite_handle = load_sprite_sheet(world, "GridLine");
    let white_line_ver = SpriteRender::new(sprite_handle.clone(), 0);
    let white_line_hor = SpriteRender::new(sprite_handle.clone(), 1);

    for y in 0..ARENA_CELLS {
        for x in 0..ARENA_CELLS {
            // Left
            if x == 0 {
                let mut transform = Transform::default();
                transform.set_translation_xyz(
                    padding_x,
                    padding_y + (y as f32 * LINE_LENGHT) + LINE_LENGHT / 2.,
                    0.,
                );

                world
                    .create_entity()
                    .with(white_line_ver.clone())
                    .with(transform)
                    .build();
            }

            //Top
            if y == ARENA_CELLS - 1 {
                let mut transform = Transform::default();
                transform.set_translation_xyz(
                    padding_x + (x as f32 * LINE_LENGHT) + LINE_LENGHT / 2.,
                    padding_y + (y as f32 * LINE_LENGHT) + LINE_LENGHT,
                    0.,
                );
                world
                    .create_entity()
                    .with(white_line_hor.clone())
                    .with(transform)
                    .build();
            }

            // Bottom
            {
                let mut transform = Transform::default();
                transform.set_translation_xyz(
                    padding_x + (x as f32 * LINE_LENGHT) + LINE_LENGHT / 2.,
                    padding_y + (y as f32 * LINE_LENGHT),
                    0.,
                );
                world
                    .create_entity()
                    .with(white_line_hor.clone())
                    .with(transform)
                    .build();
            }

            // Right
            {
                let mut transform = Transform::default();
                transform.set_translation_xyz(
                    padding_x + (x as f32 * LINE_LENGHT) + LINE_LENGHT,
                    padding_y + (y as f32 * LINE_LENGHT) + LINE_LENGHT / 2.,
                    0.,
                );

                world
                    .create_entity()
                    .with(white_line_ver.clone())
                    .with(transform)
                    .build();
            }
        }
    }
}
