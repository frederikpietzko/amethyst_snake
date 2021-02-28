mod ui;
mod utils;
use ui::{MainMenuSystem, MainMenuSystemDesc, MenuState};

use amethyst::{
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    start_logger,
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    Application, GameDataBuilder, Result,
};

fn main() -> Result<()> {
    start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");
    let bindings_path = app_root.join("config").join("bindings.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0., 0., 0., 1.]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_path)?)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_system_desc(MainMenuSystemDesc, "main_menu_system_desc", &[]);

    let mut game = Application::new(assets_dir, MenuState::default(), game_data)?;
    game.run();

    Ok(())
}
