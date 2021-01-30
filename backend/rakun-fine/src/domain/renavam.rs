use std::fmt;

/// The code of RENAVAM
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RenavamCode(pub u32);

impl fmt::Display for RenavamCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renavam_display() {
        let renavam_code = RenavamCode(1);
        let renavam_code_fmt: String = format!("{}", renavam_code);
        assert_eq!(renavam_code_fmt, "1".to_string())
    }
}
