extern crate msi_keyboard;
extern crate hidapi_rust;

use hidapi_rust::HidApi;
use msi_keyboard::{MSIKeyboard, Mode, Region, Color, Level};


fn main() {
    let api = HidApi::new().unwrap();
    let mut keyboard = MSIKeyboard::new(&api);

    keyboard.set_color(Region::Left, Color::Green, Level::High);
    keyboard.set_color(Region::Middle, Color::Yellow, Level::High);
    keyboard.set_color(Region::Right, Color::Red, Level::High);
}
