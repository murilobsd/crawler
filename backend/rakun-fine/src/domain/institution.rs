use crate::domain::Fine;

/// The identification of institution
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct InstitutionId(pub String);

/// Institution generating the fine
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Institution {
    pub active: bool,
    pub captcha: bool,
    pub city: Option<String>,
    pub id: Option<InstitutionId>,
    pub name: String,
    pub uf: Option<String>,
    pub url: Option<String>,
    pub fines: Option<Vec<Fine>>,
}

impl Institution {
    pub fn new(name: String) -> Self {
        Self {
            active: false,
            city: None,
            description: None,
            id: None,
            name,
            uf: None,
            url: None,
            fines: None,
        }
    }

    pub fn new_with_id(id: Option<InstitutionId>, name: String) -> Self {
        Self {
            active: false,
            city: None,
            description: None,
            id,
            name,
            uf: None,
            url: None,
            fines: None,
        }
    }
}
