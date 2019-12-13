use std::str::Chars;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bigram {
    Full(char, char),
    Half(char),
}

impl Bigram {
    pub fn is_half(&self) -> bool {
        match self {
            Bigram::Half(_) => true,
            _ => false,
        }
    }

    pub fn is_full(&self) -> bool {
        match self {
            Bigram::Full(_, _) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum ParsingItem {
    Parsed(Bigram),
    Unparsed(Vec<char>),
}

pub struct Parser<'a> {
    element: Option<ParsingItem>,
    underlying: Chars<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(string: &'a str) -> Self {
        let mut parser = Parser {
            element: None,
            underlying: string.chars(),
        };

        parser.refill();
        parser
    }

    fn refill(&mut self) {
        use ParsingItem::*;

        assert!(self.element.is_none());

        let mut i = 0;
        while i < 2 {
            // Find next suitable character
            let next = loop {
                // Get character from string
                let peeked_next = match self.underlying.clone().next() {
                    Some(c) => c,
                    None => return,
                };

                match peeked_next {
                    'а'..='я' | 'ё' => {
                        // return if unparsed sequence finishes
                        if let Some(Unparsed(_)) = self.element {
                            return;
                        }

                        self.underlying.next(); // shift iterator

                        break peeked_next;
                    }
                    c => {
                        // return if parsed sequence finishes
                        if let Some(Parsed(_)) = self.element {
                            return;
                        }

                        self.underlying.next(); // shift iterator

                        self.add_unparsed_character(c);

                        continue;
                    }
                }
            };

            // Add character to bigram
            self.add_bigram_character(next);
            i += 1;
        }
    }

    pub fn add_bigram_character(&mut self, c: char) {
        use ParsingItem::*;

        assert!(match self.element {
            Some(Unparsed(_)) => false,
            _ => true,
        });

        self.element = match self.element {
            None => Some(Parsed(Bigram::Half(c))),
            Some(Parsed(Bigram::Half(l))) => Some(Parsed(Bigram::Full(l, c))),
            _ => unreachable!(),
        };
    }

    pub fn add_unparsed_character(&mut self, c: char) {
        use ParsingItem::*;

        assert!(match self.element {
            Some(Parsed(_)) => false,
            _ => true,
        });

        if let None = self.element {
            self.element = Some(Unparsed(vec![c]));
        } else if let Some(Unparsed(unparsed)) = &mut self.element {
            unparsed.push(c);
        } else {
            unreachable!();
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = ParsingItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.element.is_none() {
            return None;
        }

        let new_element = None;
        let result = std::mem::replace(&mut self.element, new_element);

        self.refill();

        result
    }
}
