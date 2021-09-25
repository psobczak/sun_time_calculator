use std::fmt;

#[derive(Debug)]
pub struct Coordinate {
    pub degrees: i16,
    pub minutes: u16,
    pub seconds: u16,
    pub direction: Direction,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}°{}′{}′′{:?}",
            self.degrees, self.minutes, self.seconds, self.direction
        )
    }
}

impl Coordinate {
    pub fn new(degrees: i16, minutes: u16, seconds: u16, direction: Direction) -> Self {
        let degrees = match direction {
            Direction::E | Direction::W => match degrees {
                -180..=180 => degrees,
                _ => panic!("Degrees must be between -180 and 180"),
            },
            Direction::N | Direction::S => match degrees {
                0..=90 => degrees,
                _ => panic!("Degrees must be between 0 and 90"),
            },
        };

        let minutes = match minutes {
            0..=60 => minutes,
            _ => panic!("Minutes must be between 0 and 59"),
        };

        let seconds = match seconds {
            0..=60 => seconds,
            _ => panic!("Seconds must be between 0 and 59"),
        };

        Coordinate {
            degrees,
            minutes,
            seconds,
            direction,
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    N,
    S,
    E,
    W,
}
