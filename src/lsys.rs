//
// Uses
//
use std::collections::HashMap;

//
// Structs
//
pub struct Lsys {
    line: Vec<char>,
    grammar: HashMap<char, Vec<char>>,
    depth: i32,
    max_depth: i32
}

//
// Implementations
//
impl Lsys {

    pub fn new(seed: Vec<char>, grammar: HashMap<char, Vec<char>>, max_depth: i32) -> Lsys {
        Lsys {
            line: seed,
            grammar: grammar,
            depth: 0,
            max_depth: max_depth
        }
    }

    pub fn next(&mut self) -> Option<char> {
        // Loop
        loop {
            // Fetch next
            let next = self.line.pop();
            // If no next abort.
            if next.is_none() {
                // Abort!
                return next;
            }
            // Unwrap next.
            let c = next.unwrap();
            // If next is pop character, pop.
            if c == '\0' {
                // Pop
                self.depth -= 1;
                continue;
            }
            // If at max depth, abort.
            if self.depth == self.max_depth {
                // Abort
                return next;
            }
            // Attempt to find expansion.
            let expansion = self.grammar.get(&c);
            // Expand if possible.
            if expansion.is_some() {
                // Increase depth.
                self.depth += 1;
                // Push pop token.
                self.line.push('\0');
                // Push expansion.
                for e in expansion.unwrap() {
                    self.line.push(*e);
                }
            } else {
                // Abort!
                return next;
            }
        }
    }

}

//
// Creates a basic grammar hashmap.
//
#[allow(dead_code)]
pub fn basic_grammar() -> HashMap<char, Vec<char>> {
    let mut grammar = HashMap::new();
    grammar.insert('f', str_char_vec("f+f-f-f+f"));
    return grammar;
}

//
// Creates a char vec from a String.
//
#[allow(dead_code)]
pub fn str_char_vec(line: &str) -> Vec<char> {
    let mut chars = Vec::with_capacity(line.len() as usize);
    for c in line.chars() {
        chars.push(c);
    }
    return chars;
}

//
// Creates a char vec from a String.
//
#[allow(dead_code)]
pub fn string_char_vec(line: &String) -> Vec<char> {
    let mut chars = Vec::with_capacity(line.len() as usize);
    for c in line.chars() {
        chars.push(c);
    }
    return chars;
}
