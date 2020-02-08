#[derive(Debug, Clone)]
pub enum Color {
    RGBA(u32),

    Red,
    Green,
    Blue,
    White,
    Black,

    AliceBlue,
}

impl Color {
    pub fn rgba(&self) -> (u8, u8, u8, u8) {
        match self {
            Color::Red => (0xff, 0, 0, 0xff),
            Color::Green => (0, 0xff, 0, 0xff),
            Color::Blue => (0, 0, 0xff, 0xff),
            Color::White => (0xff, 0xff, 0xff, 0xff),
            Color::AliceBlue => (0xf0, 0xf8, 0xff, 0xff),
            Color::Black => (0, 0, 0, 0xff),
            Color::RGBA(value) => {
                let [r, g, b, a] = value.to_be_bytes();
                (r, g, b, a)
            }
        }
    }
}