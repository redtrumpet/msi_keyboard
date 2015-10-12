extern crate hidapi_rust;

use hidapi_rust::{HidApi, HidDevice};

#[derive(Clone, Copy)]
pub enum Region {
    Left,
    Middle,
    Right,
}

#[derive(Clone, Copy)]
pub enum Color {
    Off,
    Red,
    Orange,
    Yellow,
    Green,
    Sky,
    Blue,
    Purple,
    White,
}

#[derive(Clone, Copy)]
pub enum Level {
    Light,
    Low,
    Med,
    High,
}

#[derive(Clone, Copy)]
pub enum Mode {
    Disabled,
    Normal,
    Gaming,
    // TODO: implement Breathe and Wave
    Breathe,
    Demo,
    Wave,
}

#[derive(Clone, Copy)]
enum Operation {
    Commit,
    Set,
    SpecialModeColorInput,
}

impl Region {
    fn to_u8(self) -> u8 {
        match self {
            Region::Left   => 0x01,
            Region::Middle => 0x02,
            Region::Right  => 0x03,
        }
    }
}

impl Color {
    fn to_u8(self) -> u8 {
        match self {
            Color::Off    => 0x00,
            Color::Red    => 0x01,
            Color::Orange => 0x02,
            Color::Yellow => 0x03,
            Color::Green  => 0x04,
            Color::Sky    => 0x05,
            Color::Blue   => 0x06,
            Color::Purple => 0x07,
            Color::White  => 0x08,
        }
    }
}

impl Level {
    fn to_u8(self) -> u8 {
        match self {
            Level::Light => 0x03,
            Level::Low   => 0x02,
            Level::Med   => 0x01,
            Level::High  => 0x00,
        }
    }
}

impl Mode {
    fn to_u8(self) -> u8 {
        match self {
            Mode::Disabled => 0x00,
            Mode::Normal   => 0x01,
            Mode::Gaming   => 0x02,
            Mode::Breathe  => 0x03,
            Mode::Demo     => 0x04,
            Mode::Wave     => 0x05,
        }
    }
}

impl Operation {
    fn to_u8(self) -> u8 {
        match self {
            Operation::Commit                => 0x41,
            Operation::Set                   => 0x42,
            Operation::SpecialModeColorInput => 0x43,
        }
    }
}

pub struct MSIKeyboard<'a> {
    device: HidDevice<'a>,
    current_mode: Option<Mode>,
}

impl<'a> MSIKeyboard<'a> {
    pub fn new(api: &'a HidApi) -> MSIKeyboard<'a> {
        MSIKeyboard {
            device: api.open(6000, 65280).unwrap(),
            current_mode: None,
        }
    }
    pub fn set_mode(&mut self, mode: Mode) {
        let commit: [u8; 8] = [
            0x01,
            0x02,
            Operation::Commit.to_u8(),
            mode.to_u8(), // mode
            0x00,
            0x00,
            0x00,
            0xec // End Of Request
        ];
        self.device.send_feature_report(&commit);
        self.current_mode = Some(mode);
    }
    pub fn set_color(&mut self, region: Region, color: Color, level: Level) {
        let activate : [u8; 8] = [
            0x01,
            0x02,
            Operation::Set.to_u8(),
            region.to_u8(),
            color.to_u8(),
            level.to_u8(),
            0x00,
            0xec
        ];
        self.device.send_feature_report(&activate);
        // commit
        match self.current_mode {
            None => {
                self.set_mode(Mode::Normal);
            }
            Some(m) => {
                self.set_mode(m);
            }
        }
    }
}

