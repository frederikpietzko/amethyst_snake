use amethyst::{prelude::*, window::ScreenDimensions};

pub fn get_screen_dimensions(world: &mut World) -> (f32, f32) {
    let screen_dimensions = world.read_resource::<ScreenDimensions>();
    let width = screen_dimensions.width();
    let height = screen_dimensions.height();
    drop(screen_dimensions);

    (width, height)
}
