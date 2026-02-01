use std::{collections::LinkedList, panic};

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenObject,
    CloseObject,
    Colon,
    Comma,
    OpenArray,
    CloseArray,
    String(String),
    Number(f64),
}

#[derive(PartialEq, Clone, Copy)]
enum State {
    Default,
    String,
    NumberWholePart,
    NumberZeroPrefix,
    NumberNegativePrefix,
    NumberFractionPart,
}

pub fn tokenize(input: &str) -> LinkedList<Token> {
    let mut tokens = LinkedList::new();
    let mut state = State::Default;
    let mut buffer = Vec::new();

    for c in input.chars() {
        consume(c, &mut tokens, &mut state, &mut buffer);
    }

    if state != State::Default {
        panic!("Expected something else but found EOF")
    }

    tokens
}

fn consume(c: char, tokens: &mut LinkedList<Token>, state: &mut State, buffer: &mut Vec<char>) {
    match (c, *state) {
        ('{', State::Default) => tokens.push_back(Token::OpenObject),
        ('}', State::Default) => tokens.push_back(Token::CloseObject),
        (':', State::Default) => tokens.push_back(Token::Colon),
        (',', State::Default) => tokens.push_back(Token::Comma),
        ('[', State::Default) => tokens.push_back(Token::OpenArray),
        (']', State::Default) => tokens.push_back(Token::CloseArray),

        // Whitespace
        (' ', State::Default) => (),
        ('\n', State::Default) => (),
        ('\r', State::Default) => (),
        ('\t', State::Default) => (),

        // String
        // TODO: \u hex sequence escape validation
        ('"', State::Default) => *state = State::String,
        ('"', State::String) => add_string_token_from_buffer(tokens, state, buffer),
        (c, State::String) => buffer.push(c),

        // Number
        ('-', State::Default) => add_negative_prefix(state, buffer),
        ('0', State::Default) => add_zero_prefix(state, buffer),
        ('1', State::Default) => add_number_whole_part_symbol('1', state, buffer),
        ('2', State::Default) => add_number_whole_part_symbol('2', state, buffer),
        ('3', State::Default) => add_number_whole_part_symbol('3', state, buffer),
        ('4', State::Default) => add_number_whole_part_symbol('4', state, buffer),
        ('5', State::Default) => add_number_whole_part_symbol('5', state, buffer),
        ('6', State::Default) => add_number_whole_part_symbol('6', state, buffer),
        ('7', State::Default) => add_number_whole_part_symbol('7', state, buffer),
        ('8', State::Default) => add_number_whole_part_symbol('8', state, buffer),
        ('9', State::Default) => add_number_whole_part_symbol('9', state, buffer),

        // Number negative prefix
        ('0', State::NumberNegativePrefix) => add_zero_prefix(state, buffer),
        ('1', State::NumberNegativePrefix) => add_number_whole_part_symbol('1', state, buffer),
        ('2', State::NumberNegativePrefix) => add_number_whole_part_symbol('2', state, buffer),
        ('3', State::NumberNegativePrefix) => add_number_whole_part_symbol('3', state, buffer),
        ('4', State::NumberNegativePrefix) => add_number_whole_part_symbol('4', state, buffer),
        ('5', State::NumberNegativePrefix) => add_number_whole_part_symbol('5', state, buffer),
        ('6', State::NumberNegativePrefix) => add_number_whole_part_symbol('6', state, buffer),
        ('7', State::NumberNegativePrefix) => add_number_whole_part_symbol('7', state, buffer),
        ('8', State::NumberNegativePrefix) => add_number_whole_part_symbol('8', state, buffer),
        ('9', State::NumberNegativePrefix) => add_number_whole_part_symbol('9', state, buffer),

        // Number zero prefix
        ('.', State::NumberZeroPrefix) => add_number_fraction_symbol(state, buffer),
        ('e', State::NumberZeroPrefix) => panic!("Not implemented"),
        ('E', State::NumberZeroPrefix) => panic!("Not implemented"),
        (c, State::NumberZeroPrefix) => add_number_from_buffer(c, tokens, state, buffer),

        // Number whole part
        ('0', State::NumberWholePart) => buffer.push('0'),
        ('1', State::NumberWholePart) => buffer.push('1'),
        ('2', State::NumberWholePart) => buffer.push('2'),
        ('3', State::NumberWholePart) => buffer.push('3'),
        ('4', State::NumberWholePart) => buffer.push('4'),
        ('5', State::NumberWholePart) => buffer.push('5'),
        ('6', State::NumberWholePart) => buffer.push('6'),
        ('7', State::NumberWholePart) => buffer.push('7'),
        ('8', State::NumberWholePart) => buffer.push('8'),
        ('9', State::NumberWholePart) => buffer.push('9'),
        ('.', State::NumberWholePart) => add_number_fraction_symbol(state, buffer),
        (c, State::NumberWholePart) => add_number_from_buffer(c, tokens, state, buffer),

        // Number fraction part
        ('0', State::NumberFractionPart) => buffer.push('0'),
        ('1', State::NumberFractionPart) => buffer.push('1'),
        ('2', State::NumberFractionPart) => buffer.push('2'),
        ('3', State::NumberFractionPart) => buffer.push('3'),
        ('4', State::NumberFractionPart) => buffer.push('4'),
        ('5', State::NumberFractionPart) => buffer.push('5'),
        ('6', State::NumberFractionPart) => buffer.push('6'),
        ('7', State::NumberFractionPart) => buffer.push('7'),
        ('8', State::NumberFractionPart) => buffer.push('8'),
        ('9', State::NumberFractionPart) => buffer.push('9'),
        ('e', State::NumberFractionPart) => panic!("Not implemented"),
        ('E', State::NumberFractionPart) => panic!("Not implemented"),
        (c, State::NumberFractionPart) => add_number_from_buffer(c, tokens, state, buffer),

        // Errors
        (c, State::Default) => panic!("Unsupported caracter: {}", c),
        (_, State::NumberNegativePrefix) => panic!("Number error"),
    }
}

fn add_number_from_buffer(
    c: char,
    tokens: &mut LinkedList<Token>,
    state: &mut State,
    buffer: &mut Vec<char>,
) {
    *state = State::Default;
    let buffer_as_string: String = buffer.iter().cloned().collect();
    let number: f64 = buffer_as_string
        .parse()
        .expect("Should be able to create number. There's a bug in the compiler!");
    tokens.push_back(Token::Number(number));
    buffer.clear();
    consume(c, tokens, state, buffer);
}

fn add_number_fraction_symbol(state: &mut State, buffer: &mut Vec<char>) {
    *state = State::NumberFractionPart;
    buffer.push('.');
}

fn add_negative_prefix(state: &mut State, buffer: &mut Vec<char>) {
    *state = State::NumberNegativePrefix;
    buffer.push('-');
}

fn add_zero_prefix(state: &mut State, buffer: &mut Vec<char>) {
    *state = State::NumberZeroPrefix;
    buffer.push('0');
}

fn add_number_whole_part_symbol(c: char, state: &mut State, buffer: &mut Vec<char>) {
    *state = State::NumberWholePart;
    buffer.push(c);
}

fn add_string_token_from_buffer(
    tokens: &mut LinkedList<Token>,
    state: &mut State,
    buffer: &mut Vec<char>,
) {
    let value: String = buffer.iter().cloned().collect();
    tokens.push_back(Token::String(value));
    buffer.clear();
    *state = State::Default
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
    fn test_tokenize_numbers() {
        assert_eq!(tokenize("0 "), LinkedList::from([Token::Number(0.0)]));
        assert_eq!(tokenize("1 "), LinkedList::from([Token::Number(1.0)]));
        assert_eq!(tokenize("-1 "), LinkedList::from([Token::Number(-1.0)]));
        assert_eq!(tokenize("103 "), LinkedList::from([Token::Number(103.0)]));
        assert_eq!(tokenize("-108 "), LinkedList::from([Token::Number(-108.0)]));
        assert_eq!(tokenize("0.34 "), LinkedList::from([Token::Number(0.34)]));
        assert_eq!(tokenize("-0.35 "), LinkedList::from([Token::Number(-0.35)]));
        assert_eq!(
            tokenize("125.0304 "),
            LinkedList::from([Token::Number(125.0304)])
        );
        assert_eq!(
            tokenize("-1205.0304 "),
            LinkedList::from([Token::Number(-1205.0304)])
        );
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
            Token::Number(456.0),
            Token::Comma,
            Token::String(String::from("3")),
            Token::CloseArray,
            Token::CloseObject,
        ]);
        let result = tokenize(
            "{\n\r\t\"field1\": \t\"value1\",\n\r\t \"field2\": \t[\"value2\", 456, \"3\"]\n\r}",
        );

        assert_eq!(expected, result);
    }
}
