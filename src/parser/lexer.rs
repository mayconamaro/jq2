use std::collections::LinkedList;

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenBracket,
    CloseBracket,
    Colon,
    String(String),
}

#[derive(PartialEq, Clone, Copy)]
enum State {
    NotString,
    String,
}

pub fn tokenize(input: &str) -> LinkedList<Token> {
    let mut tokens = LinkedList::new();
    let mut state = State::NotString;
    let mut buffer = Vec::new();

    for c in input.chars() {
        consume(c, &mut tokens, &mut state, &mut buffer);
    }

    if state == State::String {
        panic!("Expected end of string but found EOF")
    }

    tokens
}

fn consume(c: char, tokens: &mut LinkedList<Token>, state: &mut State, buffer: &mut Vec<char>) {
    match (c, *state) {
        ('{', State::NotString) => tokens.push_back(Token::OpenBracket),
        ('}', State::NotString) => tokens.push_back(Token::CloseBracket),
        (':', State::NotString) => tokens.push_back(Token::Colon),
        ('"', State::NotString) => *state = State::String,
        (' ', State::NotString) => (),
        (c, State::NotString) => panic!("Unsupported caracter: {}", c),
        ('"', State::String) => add_string_token_from_buffer(tokens, state, buffer),
        (c, State::String) => buffer.push(c),
    }
}

fn add_string_token_from_buffer(
    tokens: &mut LinkedList<Token>,
    state: &mut State,
    buffer: &mut Vec<char>,
) {
    let value: String = buffer.iter().cloned().collect();
    tokens.push_back(Token::String(value));
    buffer.clear();
    *state = State::NotString
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_empty_object() {
        let expected = LinkedList::from([Token::OpenBracket, Token::CloseBracket]);
        let result = tokenize("{ }");

        assert_eq!(expected, result);
    }

    #[test]
    fn test_tokenize_simple_object() {
        let expected = LinkedList::from([
            Token::OpenBracket,
            Token::String(String::from("a?")),
            Token::Colon,
            Token::String(String::from("a")),
            Token::CloseBracket,
        ]);
        let result = tokenize("{ \"a?\": \"a\" }");

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn test_tokenize_unsupported_char() {
        tokenize("{ \"a\"?: \"a\" }");
    }

    #[test]
    #[should_panic]
    fn test_tokenize_incomplete_string() {
        tokenize(" { \"ab\": \"thisstringisnotgoingtoend } ");
    }
}
