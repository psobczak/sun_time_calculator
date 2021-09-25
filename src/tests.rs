use coordinate::{Coordinate, Direction}

#[cfg(test)]

#[test]
fn should_map_from_decimal_to_coordinates() {
    let expected = Coordinate::new(156, 44, 31, Direction::E)
    let actual = DecimalDegree::new(156.742).to_coordinates();
    assert_nq!(expected, actual);
}