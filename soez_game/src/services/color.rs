use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Color {
    // Formats
    RGBA(u32),
    RGB(u32),
    // TODO: HSV

    // Standard
    Red,
    Green,
    Blue,
    White,
    Black,

    // X11
    AliceBlue,
    SeaGreen,
    SlateGray,
    PeachPuff,
}

impl Color {

    pub fn rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Red =>       (0xff, 0x00, 0x00),
            Color::Green =>     (0x00, 0xff, 0x00),
            Color::Blue =>      (0x00, 0x00, 0xff),
            Color::White =>     (0xff, 0xff, 0xff),
            Color::Black =>     (0x00, 0x00, 0x00),

            Color::AliceBlue => (0xf0, 0xf8, 0xff),
            Color::SeaGreen =>  (0x2e, 0x8b, 0x57),
            Color::SlateGray =>  (0x70, 0x80, 0x90),
            Color::PeachPuff => (0xff, 0xba, 0xd9),

            Color::RGB(value) => {
                let [_, r, g, b] = value.to_be_bytes();
                (r, g, b)
            }

            Color::RGBA(value) => {
                let [r, g, b, _a] = value.to_be_bytes();
                (r, g, b)
            }
        }
    }

    pub fn rgba(&self) -> (u8, u8, u8, u8) {
        match self {
            Color::RGBA(value) => {
                let [r, g, b, a] = value.to_be_bytes();
                (r, g, b, a)
            },
            other => {
                let (r, g, b) = other.rgb();
                (r, g, b, 0xff)
            }
        }
    }
}