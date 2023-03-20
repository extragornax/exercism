#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl ResistorColor {
    pub fn to_string(&self) -> String {
        match self {
            ResistorColor::Black => "Black".to_string(),
            ResistorColor::Brown => "Brown".to_string(),
            ResistorColor::Red => "Red".to_string(),
            ResistorColor::Orange => "Orange".to_string(),
            ResistorColor::Yellow => "Yellow".to_string(),
            ResistorColor::Green => "Green".to_string(),
            ResistorColor::Blue => "Blue".to_string(),
            ResistorColor::Violet => "Violet".to_string(),
            ResistorColor::Grey => "Grey".to_string(),
            ResistorColor::White => "White".to_string(),
        }
    }
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color as u32
}

pub fn value_to_color_string(value: u32) -> String {
    match value {
        0 => ResistorColor::Black.to_string(),
        1 => ResistorColor::Brown.to_string(),
        2 => ResistorColor::Red.to_string(),
        3 => ResistorColor::Orange.to_string(),
        4 => ResistorColor::Yellow.to_string(),
        5 => ResistorColor::Green.to_string(),
        6 => ResistorColor::Blue.to_string(),
        7 => ResistorColor::Violet.to_string(),
        8 => ResistorColor::Grey.to_string(),
        9 => ResistorColor::White.to_string(),
        _ => "value out of range".to_owned(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    vec![
        ResistorColor::Black,
        ResistorColor::Brown,
        ResistorColor::Red,
        ResistorColor::Orange,
        ResistorColor::Yellow,
        ResistorColor::Green,
        ResistorColor::Blue,
        ResistorColor::Violet,
        ResistorColor::Grey,
        ResistorColor::White,
    ]
}
