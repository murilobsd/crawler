use std::fmt;

/// The identification of institution
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct InstitutionId(pub String);

impl fmt::Display for InstitutionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A struct which represents institution generating the fine.
///
/// Institution store data of Detrans, DER's.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Institution {
    pub city: Option<String>,
    pub id: Option<InstitutionId>,
    pub name: &'static str,
    pub uf: Option<String>,
    pub url: Option<String>,
}

impl fmt::Display for Institution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.url {
            Some(u) => write!(f, "Institution: {} - {}", self.name, u),
            None => write!(f, "Institution: {}", self.name),
        }
    }
}

impl Institution {
    /// Returns a new institution
    pub fn new(name: &'static str) -> Self {
        Self {
            city: None,
            id: None,
            name,
            uf: None,
            url: None,
        }
    }

    /// Returns a new institution with id
    pub fn new_with_id(id: Option<InstitutionId>, name: &'static str) -> Self {
        Self {
            city: None,
            id,
            name,
            uf: None,
            url: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn institution_without_id() {
        let institution = Institution::new("DER - SP");
        assert_eq!(institution.name, "DER - SP");
    }

    #[test]
    fn institution_with_id() {
        let institution_id = InstitutionId("1".to_string());
        let institution = Institution::new_with_id(Option::from(institution_id), "DER - SP");
        assert_eq!(institution.name, "DER - SP");
        match institution.id {
            Some(v) => assert_eq!(v.0, "1".to_string()),
            None => panic!("institution not has id"),
        }
    }
}
