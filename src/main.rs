mod coordinate;
use coordinate::{Coordinate, Direction};

fn main() {
    // let lat = Coordinate::new(122, 13, 56, Direction::W);
    // let long = Coordinate::new(21, 00, 30, Direction::E);
    // // let warsaw = Place::new(lat, long, String::from("Warszawa"));
    // // println!("{}", warsaw)

    let degree = 338.8671;
    let xd = DecimalDegree::new(degree, CoordinateType::Longitude).to_coordinates();
    println!("{}", xd);
}

// struct Place {
//     latitude: f32,
//     longitude: f32,
//     name: String,
// }

// #[derive(Debug)]
// struct Place {
//     latitiude: Coordinate,
//     longitude: Coordinate,
//     name: String,
// }

// impl Place {
//     fn new(latitiude: Coordinate, longitude: Coordinate, name: String) -> Self {
//         Place {
//             latitiude,
//             longitude,
//             name,
//         }
//     }
// }

// impl fmt::Display for Place {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{} | Lat: {}, Long: {}",
//             self.name, self.latitiude, self.longitude
//         )
//     }
// }

#[derive(Debug)]
enum CoordinateType {
    Latitiude,
    Longitude,
}

#[derive(Debug)]
struct DecimalDegree {
    degree: f32,
    coordinate_type: CoordinateType,
}

impl DecimalDegree {
    fn new(degree: f32, coordinate_type: CoordinateType) -> Self {
        DecimalDegree {
            degree,
            coordinate_type,
        }
    }

    fn to_coordinates(&self) -> Coordinate {
        let degrees = self.degree.trunc();
        let minutes = (self.degree - degrees) * 60.0;
        let minutes_diff = minutes - minutes.floor();
        let seconds = (minutes_diff * 60.0).floor();

        let direction = match self.coordinate_type {
            CoordinateType::Latitiude => {
                if degrees < 0.0 {
                    Direction::S
                } else {
                    Direction::N
                }
            }
            CoordinateType::Longitude => {
                if degrees < 0.0 {
                    Direction::E
                } else {
                    Direction::N
                }
            }
        };

        let coordinate = Coordinate::new(degrees as i16, minutes as u16, seconds as u16, direction);
        coordinate
    }
}
