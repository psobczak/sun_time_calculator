use crate::Coordinate;
use crate::CoordinateType;
use crate::DecimalDegree;
use crate::Direction;
use std::str::FromStr;
use std::ops::Sub;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

pub struct SunTimeCalculator {
    pub longitude: DecimalDegree,
    pub time: DateTime<Utc>,
}

impl SunTimeCalculator {
    pub fn new(longitude: DecimalDegree, time: DateTime<Utc>) -> Self {
        SunTimeCalculator { longitude, time }
    }

    fn longitude_diff(&self, longitude: &DecimalDegree) -> Coordinate {
        let first_longitude = self.longitude.to_coordinates();
        let second_longitude = longitude.to_coordinates();
        let directions = (&first_longitude.direction, &second_longitude.direction);

        let longitude_diff = match directions {
            (Direction::E, Direction::W) | (Direction::W, Direction::E) => {
                first_longitude.degrees.abs() as f32 + second_longitude.degrees.abs() as f32
            }

            _ => {
                (first_longitude.degrees.abs() as f32 - second_longitude.degrees.abs() as f32).abs()
            }
        };
        let longitude_diff = DecimalDegree::new(longitude_diff, CoordinateType::Longitude);
        longitude_diff.to_coordinates()
    }

    pub fn sun_time_diff(&self, coordinate: &Coordinate) -> DateTime<Utc> {
        let mut minutes: u32 = coordinate.degrees.abs() as u32 * 4;
        minutes += coordinate.minutes as u32;
        minutes += coordinate.seconds as u32 * 4;

        let hours = minutes / 60;
        let minutes_remaining = minutes % 60;
        let minutes_from_seconds = (coordinate.seconds / 60) as u32;
        let seconds_remaining = (coordinate.seconds % 60) as u32;

        let time = NaiveTime::from_hms(
            hours,
            minutes_remaining + minutes_from_seconds,
            coordinate.seconds as u32 + seconds_remaining,
        );
        let date = NaiveDate::from_ymd(2021, 9, 01);
        let time_diff = NaiveDateTime::new(date, time);

        let dt = DateTime::<Utc>::from_utc(time_diff, Utc);

        self.time.sub(dt);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::CoordinateType;
    use crate::DecimalDegree;
    use test_case::test_case;

    #[test_case(-10.0, -43.5, 33; "when both longitudes are on western side")]
    #[test_case(100.0, 43.5, 57; "when both longitudes are on eastern side")]
    #[test_case(-10.0, 43.5, 53; "when one longitudes is on western and other on eastern side")]
    #[test_case(10.0, -43.5, 53; "when one longitudes is on eastern and other on western side")]
    #[test_case(11.0, 11.0, 0; "when both longitudes are on the same meridian")]
    fn should_correctly_calculate_difference(first_long: f32, second_long: f32, expected: i16) {
        let first_longitude = DecimalDegree::new(first_long, CoordinateType::Longitude);
        let second_longitude = DecimalDegree::new(second_long, CoordinateType::Longitude);

        let actual =
            SunTimeCalculator::new(first_longitude, Utc::now()).longitude_diff(&second_longitude);
        assert_eq!(actual.degrees, expected)
    }

    #[test]
    fn should_return_date_time() {
        let rzeszow = DecimalDegree::new(22.00, CoordinateType::Longitude);
        let toronto = DecimalDegree::new(-79.44, CoordinateType::Longitude);

        let time = NaiveTime::from_hms(09, 00, 00);
        let date = NaiveDate::from_ymd(2021, 9, 01);
        let time_diff = NaiveDateTime::new(date, time);

        let dt = DateTime::<Utc>::from_utc(time_diff, Utc);

        let actual = SunTimeCalculator::new(rzeszow, dt).sun_time_diff(&toronto.to_coordinates());

        assert_eq!(actual, DateTime::from_str("2021-09-01:02:16:00"))
    }
}
