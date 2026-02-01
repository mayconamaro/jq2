use std::collections::LinkedList;

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenObject,
    CloseObject,
    Colon,
    Comma,
    OpenArray,
    CloseArray,
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
        ('{', State::NotString) => tokens.push_back(Token::OpenObject),
        ('}', State::NotString) => tokens.push_back(Token::CloseObject),
        (':', State::NotString) => tokens.push_back(Token::Colon),
        (',', State::NotString) => tokens.push_back(Token::Comma),
        ('[', State::NotString) => tokens.push_back(Token::OpenArray),
        (']', State::NotString) => tokens.push_back(Token::CloseArray),

        // Whitespace
        (' ', State::NotString) => (),
        ('\n', State::NotString) => (),
        ('\r', State::NotString) => (),
        ('\t', State::NotString) => (),

        // String
        // TODO: \u hex sequence escape validation
        ('"', State::NotString) => *state = State::String,
        ('"', State::String) => add_string_token_from_buffer(tokens, state, buffer),
        (c, State::String) => buffer.push(c),

        // Error
        (c, State::NotString) => panic!("Unsupported caracter: {}", c),
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
        let expected = LinkedList::from([Token::OpenObject, Token::CloseObject]);
        let result = tokenize("{ }");

        assert_eq!(expected, result);
    }

    #[test]
    fn test_tokenize_simple_object() {
        let expected = LinkedList::from([
            Token::OpenObject,
            Token::String(String::from("a?")),
            Token::Colon,
            Token::String(String::from("a")),
            Token::CloseObject,
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

    #[test]
    fn test_tokenize_multi_field_object_and_whitespaces() {
        /*
         * {
         *  field1:  value1,
         *   field2:  [value2, value3]
         * }
         */
        let expected = LinkedList::from([
            Token::OpenObject,
            Token::String(String::from("field1")),
            Token::Colon,
            Token::String(String::from("value1")),
            Token::Comma,
            Token::String(String::from("field2")),
            Token::Colon,
            Token::OpenArray,
            Token::String(String::from("value2")),
            Token::Comma,
            Token::String(String::from("value3")),
            Token::CloseArray,
            Token::CloseObject,
        ]);
        let result = tokenize(
            "{\n\r\t\"field1\": \t\"value1\",\n\r\t \"field2\": \t[\"value2\", \"value3\"]\n\r}",
        );

        assert_eq!(expected, result);
    }
}
