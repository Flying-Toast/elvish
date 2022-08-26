use std::borrow::Cow;
use std::collections::HashMap;

enum Node<'a> {
    Text(Cow<'a, str>),
    Element(Element<'a>),
}

struct Element<'a> {
    /// e.g. "div"
    tag: &'a str,
    children: Vec<Node<'a>>,
    attributes: HashMap<&'a str, Cow<'a, str>>,
}

struct Parser<'a> {
    /// The unparsed remainder of the input
    tail: &'a str,
}

impl<'a> Parser<'a> {
    fn empty(&self) -> bool {
        self.tail == ""
    }

    fn eat_while<F>(&mut self, pred: F) -> &'a str
    where
        F: Fn(&char) -> bool,
    {
        let cnt = self.tail
            .chars()
            .take_while(pred)
            .count();
        let eaten = &self.tail[..cnt];
        self.tail = &self.tail[cnt..];
        eaten
    }

    fn peek(&self) -> Option<char> {
        self.tail.chars().next()
    }

    fn eat_whitespace(&mut self) {
        self.eat_while(char::is_ascii_whitespace);
    }

    fn eat_char(&mut self) -> Option<char> {
        if self.empty() {
            None
        } else {
            let ch = self.tail.chars().next();
            self.tail = &self.tail[1..];
            Some(ch)
        }
    }

    fn parse_tag_name(&mut self) -> &'a str {
        self.eat_while(|ch| ch.is_ascii_alphanumeric() || ch == '-')
    }

    fn parse_tag(&mut self) -> 
}

impl<'a> Iterator for Parser<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.eat_whitespace();
        if self.empty() {
            return None;
        }

        if let Some('<') = self.peek() {
            let tag = self.parse_tag_name();
            let mut attrs = HashMap::new();
            self.eat_whitespace();
            while self.peek().unwrap() != '>' {
                let attr_name = self.parse_tag_name();
                assert!(self.eat_char().unwrap() == '=');
                assert!(self.eat_char().unwrap() == '"');
                let attr_value = self.eat_while(|ch| ch != '"');
                assert!(self.eat_char().unwrap() == '"');
                assert!(!attrs.contains_key(attr_name));
                attrs.insert(attr_name, attr_value.into());
                self.eat_whitespace();
            }
            assert!(self.eat_char().unwrap() == '>');
        }
    }
}

fn parse_file(src: &str) -> impl Iterator<Item = Node> {
    Parser { tail: src }
}

fn main() {}
