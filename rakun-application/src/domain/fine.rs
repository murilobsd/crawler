/// Fine ...
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Fine {
    /// The ait ...
    pub ait: u32,
    /// The description ...
    pub description: Option<String>,
}

impl Fine {
    pub fn new(ait: u32, description: Option<String>) -> Self {
        Self { ait, description }
    }
}

/// FineBuilder ...
pub struct FineBuilder {
    /// The ait ...
    pub ait: u32,
    /// The description ...
    pub description: Option<String>,
}

impl FineBuilder {
    pub fn new(ait: u32) -> Self {
        Self {
            ait,
            description: None,
        }
    }

    pub fn with_description(&mut self, description: String) -> &Self {
        self.description = Some(description);
        self
    }

    pub fn build(&self) -> Fine {
        Fine {
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
