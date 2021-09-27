use crate::DecimalDegree;
use crate::Direction;
use chrono::{DateTime, Utc};

pub struct SunTimeCalculator {
    pub longitude: DecimalDegree,
    pub time: DateTime<Utc>,
}

impl SunTimeCalculator {
    pub fn new(longitude: DecimalDegree, time: DateTime<Utc>) -> Self {
        SunTimeCalculator { longitude, time }
    }

    fn longitude_diff(&self, longitude: &DecimalDegree) -> i16 {
        let first_longitude = self.longitude.to_coordinates();
        let second_longitude = longitude.to_coordinates();
        let directions = (&first_longitude.direction, &second_longitude.direction);

        let longitude_diff = match directions {
            (Direction::E, Direction::W) | (Direction::W, Direction::E) => {
                first_longitude.degrees.abs() + second_longitude.degrees.abs()
            }
   
            _ => (first_longitude.degrees.abs() - second_longitude.degrees.abs()).abs(),
        };
        
        longitude_diff
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
        assert_eq!(actual, expected)
    }
}
