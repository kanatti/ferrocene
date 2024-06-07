pub mod document_writer;
pub mod field_info;
pub mod fields_writer;
pub mod index_writer;

use std::rc::Rc;

pub use index_writer::IndexWriter;

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

/// Information about a term in a doc
#[derive(Debug)]
pub struct Posting {
    pub term: Rc<Term>,
    pub freq: u32,
    pub positions: Vec<u32>,
}

impl Posting {
    pub fn new(term: Rc<Term>, position: u32) -> Self {
        Self {
            term,
            freq: 1,
            positions: vec![position],
        }
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

    #[test]
    fn test_posting_sorting() {
        let mut postings = vec![
            Posting::new(Rc::new(Term::new("b".to_string(), "a".to_string())), 3),
            Posting::new(Rc::new(Term::new("a".to_string(), "b".to_string())), 1),
            Posting::new(Rc::new(Term::new("a".to_string(), "a".to_string())), 2),
            Posting::new(Rc::new(Term::new("a".to_string(), "c".to_string())), 4),
        ];

        postings.sort_by_key(|p| p.term.clone());

        assert_eq!(
            postings.get(0).unwrap().term,
            Term::new("a".to_string(), "a".to_string()).into()
        );
        assert_eq!(
            postings.get(1).unwrap().term,
            Term::new("a".to_string(), "b".to_string()).into()
        );
        assert_eq!(
            postings.get(2).unwrap().term,
            Term::new("a".to_string(), "c".to_string()).into()
        );
        assert_eq!(
            postings.get(3).unwrap().term,
            Term::new("b".to_string(), "a".to_string()).into()
        );
    }
}
