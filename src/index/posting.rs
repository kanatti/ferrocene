use std::rc::Rc;

use super::Term;

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
