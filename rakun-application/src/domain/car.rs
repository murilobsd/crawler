use std::fmt;

/// LicenseNumber
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LicenseNumber(pub u32);

impl fmt::Display for LicenseNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// LicensePlate
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LicensePlate(pub u32);

impl fmt::Display for LicensePlate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// CarId
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CarId(pub String);

impl fmt::Display for CarId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Car
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Car {
    /// The Car ID
    pub id: Option<CarId>,
    /// The license_number (RENAVAM)...
    pub license_number: LicenseNumber,
    /// The license_plate ...
    pub license_plate: LicensePlate,
}

impl fmt::Display for Car {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Car License: {}", self.license_number)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn hi_test() {
        assert_eq!(2, 1 + 1);
    }
}
