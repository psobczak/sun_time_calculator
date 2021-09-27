mod coordinate;
mod time_calculator;

use chrono::{DateTime, Utc};
use coordinate::{Coordinate, Direction};
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use time_calculator::SunTimeCalculator;

fn main() -> Result<(), Box<dyn Error>> {
    // let rdr = csv::ReaderBuilder::new()
    //     .has_headers(true)
    //     .from_path("data/worldcities.csv");

    // for result in rdr?.deserialize() {
    //     let record: Record = result?;
    //     let city = record.map_to_city();
    //     println!("{}", city);
    // }

    // let tokyo = City::new(
    //     DecimalDegree::new(35.6897, CoordinateType::Latitude),
    //     DecimalDegree::new(20.78, CoordinateType::Longitude),
    //     String::from("Tokyo"),
    // );

    // let jakarta = City::new(
    //     DecimalDegree::new(-6.2146, CoordinateType::Latitude),
    //     DecimalDegree::new(-35.5, CoordinateType::Longitude),
    //     String::from("Jakarta"),
    // );

    let first_longitude = DecimalDegree::new(10.23, CoordinateType::Longitude);
    let second_longitude = DecimalDegree::new(-23.44, CoordinateType::Longitude);

    let actual = SunTimeCalculator::new(first_longitude, Utc::now())
        .sun_time_diff(&second_longitude.to_coordinates());

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Record {
    city: String,
    lat: f32,
    lng: f32,
    country: String,
}

impl Record {
    fn map_to_city(&self) -> City {
        City::new(
            DecimalDegree::new(self.lat, CoordinateType::Latitude),
            DecimalDegree::new(self.lng, CoordinateType::Longitude),
            self.city.clone(),
        )
    }
}

pub struct City {
    latitude: DecimalDegree,
    longitude: DecimalDegree,
    name: String,
}

impl City {
    fn new(latitude: DecimalDegree, longitude: DecimalDegree, name: String) -> Self {
        City {
            latitude,
            longitude,
            name,
        }
    }
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | Lat: {}, Long: {}",
            self.name, self.latitude.degree, self.longitude.degree
        )
    }
}

#[derive(Debug)]
pub enum CoordinateType {
    Latitude,
    Longitude,
}

#[derive(Debug)]
pub struct DecimalDegree {
    pub degree: f32,
    pub coordinate_type: CoordinateType,
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
        let minutes = (self.degree - degrees).abs() * 60.0;
        let minutes_diff = minutes - minutes.floor();
        let seconds = (minutes_diff * 60.0).floor();

        let direction = match self.coordinate_type {
            CoordinateType::Latitude => {
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
                    Direction::W
                }
            }
        };

        Coordinate::new(degrees as i16, minutes as u16, seconds as u16, direction)
    }
}
