#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Term {
    pub field: String,
    pub text: String,
}

impl Ord for Term {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.field.cmp(&other.field) {
            std::cmp::Ordering::Equal => self.text.cmp(&other.text),
            ord => ord,
        }
    }
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Term {
    pub fn new(field: String, text: String) -> Self {
        Self { field, text }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term_ord() {
        let term1 = Term::new("a".to_string(), "a".to_string());
        let term2 = Term::new("a".to_string(), "b".to_string());
        let term3 = Term::new("b".to_string(), "a".to_string());

        assert!(term1 < term2);
        assert!(term2 < term3);
        assert!(term1 < term3);
    }

    #[test]
    fn test_term_sorting() {
        let mut terms = vec![
            Term::new("a".to_string(), "b".to_string()),
            Term::new("a".to_string(), "a".to_string()),
            Term::new("b".to_string(), "a".to_string()),
        ];

        terms.sort();

        assert_eq!(
            terms,
            vec![
                Term::new("a".to_string(), "a".to_string()),
                Term::new("a".to_string(), "b".to_string()),
                Term::new("b".to_string(), "a".to_string()),
            ]
        );
    }
}
