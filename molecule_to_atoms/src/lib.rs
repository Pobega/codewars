// type Atom = (String, usize);
// type Molecule = Vec<Atom>;

#[derive(Debug)]
pub struct ParseError {}

pub struct Molecule {}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    Err(ParseError{})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_molecules() {
        assert_eq!(parse_molecule("H"), [("H",1)]);
        assert_eq!(parse_molecule("O2"), [("O",2)]);
        assert_eq!(parse_molecule("H2O"), [("H",2),("O",1)]);
        assert_eq!(parse_molecule("Mg(OH)2"), [("Mg",1),("O",2),("H",2)]);
        assert_eq!(parse_molecule("K4[ON(SO3)2]2"), [("K",4),("O",14),("N",2),("S",4)]);
    }

    #[test]
    #[should_panic(expected = "Not a valid molecule")]
    fn test_invalid_molecule() {
        parse_molecule("pie");
    }

    #[test]
    #[should_panic(expected = "Mismatched parenthesis")]
    fn test_mismatched_parens() {
        parse_molecule("Mg(OH");
        parse_molecule("Mg(OH}2");
    }
}
