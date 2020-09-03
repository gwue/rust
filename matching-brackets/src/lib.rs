pub fn brackets_are_balanced(string: &str) -> bool {
    let mut bracket_stack = Vec::new();

    let bracket_pairs = [('(', ')'), ('[', ']'), ('{', '}')];

    for c in string.chars() {
        let found_bracket = bracket_pairs
            .iter()
            .find(|&pair| pair.0 == c || pair.1 == c);

        if let Some(pair) = found_bracket {
            if c == pair.0 {
                bracket_stack.push(c);
            } else {
                let on_stack = bracket_stack.pop();
                match on_stack {
                    Some(opening_bracket) => {
                        if opening_bracket != pair.0 {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                }
            }
        }
    }
    bracket_stack.is_empty()
}
