
use std::str::Chars;
use crate::lexer::*;

pub(crate) struct Lexer<'a> {
    iter: Tracked<Chars<'a>>,
    text: &'a str,
    length: usize,
}

impl <'a>Lexer<'a> {
    
    pub(crate) fn new(text: &'a str) -> Self {
        Self {
            iter: text.chars().track(),
            text,
            length: text.len(),
        }
    }

    pub(crate) fn build(mut self) -> Result<Tokenstream<'a>, LexError> {

        let mut stream = Tokenstream::new(self.length, self.text);
        
        let mut start = 0;
        let mut valids = Validator::new();
        let mut finished = false;

        loop {

            let chr = match self.iter.next() {
                Some(chr) => chr,
                None => {
                    if finished { break };
                    finished = true;
                    ' '
                },
            };

            if self.pos() != 1 && valids.peek(chr, self.len(start)) == 0 {

                let target = valids.decide().ok_or(LexError::InvalidChar { chr, pos: self.pos() })?;
                
                if target != Tokenkind::Seperator {

                    let token = self.make(target, start);
                    stream.push(token);

                };
                
                start = self.pos() - 1;
                valids.reset();
                
            };

            valids.update(chr, self.len(start));
                
        }

        return Ok(stream)

    }

    fn make(&self, kind: Tokenkind, start: u32) -> Token {

        use Tokenkind::*;

        match kind {

            DoubleDot        => Token::new(kind, start, 2),
            EqualsEquals     => Token::new(kind, start, 2),
            BangEquals       => Token::new(kind, start, 2),
            OpenSharpEquals  => Token::new(kind, start, 2),
            CloseSharpEquals => Token::new(kind, start, 2),
            MinusEquals      => Token::new(kind, start, 2),
            PlusEquals       => Token::new(kind, start, 2),
            StarEquals       => Token::new(kind, start, 2),
            SlashEquals      => Token::new(kind, start, 2),
            PercentEquals    => Token::new(kind, start, 2),

            Integer          => Token::new(kind, start, self.len(start)),
            Float            => Token::new(kind, start, self.len(start)),
            Ident            => Token::new(kind, start, self.len(start)),
            KeyLet            => Token::new(kind, start, self.len(start)),
            KeyIn             => Token::new(kind, start, self.len(start)),
            KeyEnd            => Token::new(kind, start, self.len(start)),
            KeyProc           => Token::new(kind, start, self.len(start)),

            Literal          => Token::new(kind, start + 1, self.len(start) - 2), //? the literal doesn't include the enclosing double quotes
            Comment          => Token::new(kind, start + 1, self.len(start) - 2), //? the comment doesn't include the enclosing accents
            
            Note             => Token::new(kind, start + 1, self.len(start) - 1), //? the note doesn't include opening accent

            other            => Token::new(other, start, 1),

        }

    }

    #[inline(always)]
    fn pos(&self) -> u32 {
        self.iter.pos() as u32
    }

    #[inline(always)]
    fn len(&self, start: u32) -> u16 {
        (self.pos() - 1 - start) as u16
    }

}
