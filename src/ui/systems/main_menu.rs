use std::process;

use amethyst::{
    ecs::{Read, ReadStorage, ReaderId, System, SystemData, Write, WriteStorage},
    prelude::SystemDesc,
    shred::ReadExpect,
    shrev::EventChannel,
    ui::{UiEvent, UiEventType, UiText, UiTransform},
};

use crate::ui::MenuItems;

use super::super::{BUTTON_COLOR, BUTTON_HOVER_COLOR, MENU_FONT_SIZE, MENU_HOVER_FONT_SIZE};
pub struct MainMenuSystem {
    reader_id: ReaderId<UiEvent>,
}

impl MainMenuSystem {
    pub fn new(reader_id: ReaderId<UiEvent>) -> Self {
        Self { reader_id }
    }
}

impl<'s> System<'s> for MainMenuSystem {
    type SystemData = (
        Read<'s, EventChannel<UiEvent>>,
        ReadStorage<'s, UiTransform>,
        WriteStorage<'s, UiText>,
    );

    fn run(&mut self, (events, _transform, mut texts): Self::SystemData) {
        for event in (events).read(&mut self.reader_id) {
            let button = texts.get_mut(event.target).unwrap();
            match event.event_type {
                UiEventType::HoverStart => {
                    button.font_size = MENU_HOVER_FONT_SIZE;
                    button.color = BUTTON_HOVER_COLOR;
                }
                UiEventType::HoverStop => {
                    button.font_size = MENU_FONT_SIZE;
                    button.color = BUTTON_COLOR;
                }
                _ => {}
            }
            println!("{:?}", event);
        }
    }
}

pub struct MainMenuSystemDesc;
impl<'a, 'b> SystemDesc<'a, 'b, MainMenuSystem> for MainMenuSystemDesc {
    fn build(self, world: &mut amethyst::shred::World) -> MainMenuSystem {
        let mut event_channel = <Write<EventChannel<UiEvent>>>::fetch(world);
        let reader_id = event_channel.register_reader();
        MainMenuSystem::new(reader_id)
    }
}
