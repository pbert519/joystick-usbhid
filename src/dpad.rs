#[derive(Debug, defmt::Format, Default)]
pub enum DPadDirection {
    #[default]
    None,
    Up,
    RightUp,
    Right,
    RightDown,
    Down,
    LeftDown,
    Left,
    LeftUp,
}

impl DPadDirection {
    pub fn from_pins(up: bool, right: bool, down: bool, left: bool) -> DPadDirection {
        if up && !right && !down && !left {
            DPadDirection::Up
        } else if !up && right && !down && !left {
            DPadDirection::Right
        } else if !up && !right && down && !left {
            DPadDirection::Down
        } else if !up && !right && !down && left {
            DPadDirection::Left
        } else if up && right && !down && !left {
            DPadDirection::RightUp
        } else if !up && right && down && !left {
            DPadDirection::RightDown
        } else if up && !right && !down && left {
            DPadDirection::LeftUp
        } else if !up && !right && down && left {
            DPadDirection::LeftDown
        } else {
            DPadDirection::None
        }
    }

    pub fn as_bitfield(&self) -> u8 {
        match self {
            DPadDirection::None => 0b0000,
            DPadDirection::Up => 0b0001,
            DPadDirection::RightUp => 0b0011,
            DPadDirection::Right => 0b0010,
            DPadDirection::RightDown => 0b0110,
            DPadDirection::Down => 0b0100,
            DPadDirection::LeftDown => 0b1100,
            DPadDirection::Left => 0b1000,
            DPadDirection::LeftUp => 0b1001,
        }
    }
}
