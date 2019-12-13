use std::str::Chars;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bigram {
    Full(char, char),
    Half(char),
}

pub struct Bigrams<'a> {
    element: Option<Bigram>,
    underlying: Chars<'a>,
}

impl<'a> Bigrams<'a> {
    pub fn new(string: &'a str) -> Self {
        let mut bigrams = Bigrams {
            element: None,
            underlying: string.chars(),
        };

        bigrams.refill();
        bigrams
    }

    fn refill(&mut self) {
        assert!(self.element.is_none());

        let mut i = 0;
        'outer: while i < 2 {
            // Find next suitable character
            let next = loop {
                // Get character from string
                let next = match self.underlying.next() {
                    Some(c) => c,
                    None => break 'outer, // break if EOF
                };

                match next {
                    'а'..='я' | 'ё' => break next, // found suitable character
                    _ if i > 0 => break 'outer,
                    _ => continue,
                }
            };

            // Add character to bigram
            self.element = match self.element {
                None => Some(Bigram::Half(next)),
                Some(Bigram::Half(c)) => Some(Bigram::Full(c, next)),
                _ => unreachable!(),
            };

            i += 1;
        }
    }
}

impl<'a> Iterator for Bigrams<'a> {
    type Item = Bigram;

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
