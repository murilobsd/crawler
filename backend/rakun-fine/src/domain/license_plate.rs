use std::fmt;

/// The code of RENAVAM
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LicensePlate(pub String);

impl fmt::Display for LicensePlate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renavam_display() {
        let license: String = String::from("ABC1234");
        let license_plate = LicensePlate(license);
        let license_plate_fmt: String = format!("{}", license_plate);
        assert_eq!(license_plate_fmt, "ABC1234".to_string())
    }
}
