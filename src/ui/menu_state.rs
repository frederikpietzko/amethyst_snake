use amethyst::{
    assets::Handle,
    ecs::Entity,
    prelude::*,
    ui::{Anchor, FontAsset, Interactable, LineMode, UiEventType, UiText, UiTransform},
};

use crate::utils::load_font;

pub const MENU_FONT_SIZE: f32 = 25.;
pub const MENU_HOVER_FONT_SIZE: f32 = 30.;
pub const BUTTON_HEIGHT: f32 = 30.;
pub const BUTTON_WIDTH: f32 = 200.;
pub const BUTTON_COLOR: [f32; 4] = [1., 1., 1., 0.5];
pub const BUTTON_HOVER_COLOR: [f32; 4] = [0.246, 0.746, 0.246, 0.7];
pub const TOP_PADDING: f32 = 200.;

#[derive(Default)]
pub struct MenuState {
    start_button: Option<Entity>,
    exit_button: Option<Entity>,
}

impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let font = load_font(world, "sagan.ttf");

        let start_button = create_start_button(world, font.clone());
        let exit_button = create_exit_button(world, font);

        self.start_button = Some(start_button);
        self.exit_button = Some(exit_button);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Ui(ui_event) = event {
            let start_button = self.start_button.unwrap();
            let exit_button = self.exit_button.unwrap();
            if ui_event.event_type == UiEventType::Click {
                if ui_event.target == exit_button {
                    return Trans::Quit;
                }
                if ui_event.target == start_button {
                    // TODO: Push the Game state.
                    return SimpleTrans::None;
                }
            }
        }
        SimpleTrans::None
    }
}

fn create_start_button(world: &mut World, font: Handle<FontAsset>) -> Entity {
    let ui_transform = UiTransform::new(
        String::from("start_button"),
        Anchor::TopMiddle,
        Anchor::Middle,
        0.,
        -BUTTON_HEIGHT / 2. - TOP_PADDING,
        0.,
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
    );

    let ui_text = UiText::new(
        font,
        String::from("Start"),
        BUTTON_COLOR,
        MENU_FONT_SIZE,
        LineMode::Single,
        Anchor::Middle,
    );

    let start_button = world
        .create_entity()
        .with(ui_transform)
        .with(ui_text)
        .with(Interactable)
        .build();
    return start_button;
}

fn create_exit_button(world: &mut World, font: Handle<FontAsset>) -> Entity {
    let ui_transform = UiTransform::new(
        String::from("exit_button"),
        Anchor::TopMiddle,
        Anchor::Middle,
        0.,
        -TOP_PADDING - BUTTON_HEIGHT * 1.8,
        0.,
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
    );

    let ui_text = UiText::new(
        font,
        String::from("Quit"),
        BUTTON_COLOR,
        MENU_FONT_SIZE,
        LineMode::Single,
        Anchor::Middle,
    );

    let exit_button = world
        .create_entity()
        .with(ui_transform)
        .with(ui_text)
        .with(Interactable)
        .build();

    return exit_button;
}
