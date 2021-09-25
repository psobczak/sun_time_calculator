mod geopoint;
use geopoint::{Coordinate, Direction};
use std::fmt;

fn main() {
    let lat = Coordinate::new(122, 13, 56, Direction::W);
    let long = Coordinate::new(21, 00, 30, Direction::E);
    let warsaw = Place::new(lat, long, String::from("Warszawa"));
    println!("{}", warsaw)
}

#[derive(Debug)]
struct Place {
    latitiude: Coordinate,
    longitude: Coordinate,
    name: String,
}

impl Place {
    fn new(latitiude: Coordinate, longitude: Coordinate, name: String) -> Self {
        Place {
            latitiude,
            longitude,
            name,
        }
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | Lat: {}, Long: {}",
            self.name, self.latitiude, self.longitude
        )
    }
}
