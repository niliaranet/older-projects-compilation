use crate::keyboard::{self, key};
use crate::{Arrow, Message};

pub fn handle_menu_keys(key: keyboard::Key, _modifiers: keyboard::Modifiers) -> Option<Message> {

    match key.as_ref() {
        keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::ArrowPress(Arrow::Right)),
        keyboard::Key::Named(key::Named::ArrowLeft) => Some(Message::ArrowPress(Arrow::Left)),
        keyboard::Key::Named(key::Named::ArrowDown) => Some(Message::ArrowPress(Arrow::Down)),
        keyboard::Key::Named(key::Named::ArrowUp) => Some(Message::ArrowPress(Arrow::Up)),

        keyboard::Key::Named(key::Named::Enter) => Some(Message::Start),
        _ => None,
    }
}

pub fn handle_running_keys(key: keyboard::Key, _modifiers: keyboard::Modifiers) -> Option<Message> {
    match key.as_ref() {
        keyboard::Key::Character("p") => Some(Message::Pause),
        keyboard::Key::Character("s") | keyboard::Key::Character("q") => Some(Message::Stop),
        keyboard::Key::Character("n") => Some(Message::NextStage),
        keyboard::Key::Named(key::Named::Backspace) => Some(Message::ResetStage),
        _ => None,
    }
}

pub fn handle_arrow_key_release(
    key: keyboard::Key,
    _modifiers: keyboard::Modifiers,
) -> Option<Message> {
    use keyboard::key;

    match key.as_ref() {
        keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::ArrowRelease(Arrow::Right)),
        keyboard::Key::Named(key::Named::ArrowLeft) => Some(Message::ArrowRelease(Arrow::Left)),
        keyboard::Key::Named(key::Named::ArrowDown) => Some(Message::ArrowRelease(Arrow::Down)),
        keyboard::Key::Named(key::Named::ArrowUp) => Some(Message::ArrowRelease(Arrow::Up)),
        _ => None,
    }
}
