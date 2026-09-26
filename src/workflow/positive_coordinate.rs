use super::coordinate_error::coordinate_error;

pub(super) fn positive_coordinate(name: &str, value: usize) -> Result<usize, String> {
    if value == 0 {
        Err(coordinate_error(name))
    } else {
        Ok(value)
    }
}
