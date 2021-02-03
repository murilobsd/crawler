use std::fmt;

/// FineId
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FineId(pub String);

impl fmt::Display for FineId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Fine ...
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Fine {
    /// The Fine ID
    pub id: Option<FineId>,
    /// The ait ...
    pub ait: u32,
    /// The description ...
    pub description: Option<String>,
}

impl Fine {
    pub fn new(ait: u32, description: Option<String>) -> Self {
        Self {
            id: None,
            ait,
            description,
        }
    }
}

/// FineBuilder ...
pub struct FineBuilder {
    /// The Fine ID
    pub id: Option<FineId>,
    /// The ait ...
    pub ait: u32,
    /// The description ...
    pub description: Option<String>,
}

impl FineBuilder {
    /// new
    pub fn new(ait: u32) -> Self {
        Self {
            id: None,
            ait,
            description: None,
        }
    }

    /// with_id
    pub fn with_id(&mut self, id: FineId) -> &Self {
        self.id = Some(id);
        self
    }

    /// with description
    pub fn with_description(&mut self, description: String) -> &Self {
        self.description = Some(description);
        self
    }

    /// build
    pub fn build(&self) -> Fine {
        Fine {
            id: None,
            ait: self.ait,
            description: self.description.clone(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn build_test() {
        let fine: Fine = Fine::new(1, Some(String::from("description")));
        let mut fine_builder = FineBuilder::new(1);
        fine_builder
            .with_description(String::from("description"))
            .build();

        assert_eq!(fine.ait, fine_builder.ait);
        assert_eq!(fine.description, fine_builder.description);
    }
}
