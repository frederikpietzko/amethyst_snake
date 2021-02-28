use amethyst::{
    assets::{Handle, Loader},
    prelude::*,
    ui::{FontAsset, TtfFormat},
};

const PREFIX: &str = "font/";

pub fn load_font(world: &mut World, name: &str) -> Handle<FontAsset> {
    world.read_resource::<Loader>().load(
        PREFIX.to_string() + name,
        TtfFormat,
        (),
        &world.read_resource(),
    )
}
