
use std::str::CharIndices;
use crate::lexer::*;

pub(crate) struct Lexer<'a> {
    iter: CharIndices<'a>,
    text: &'a str,
    length: usize,
}

impl <'a>Lexer<'a> {
    
    pub(crate) fn new(text: &'a str) -> Self {
        Self {
            iter: text.char_indices(),
            text,
            length: text.len(),
        }
    }

    pub(crate) fn build(mut self) -> Result<Tokenstream<'a>, LexError> {

        let mut stream = Tokenstream::new(self.length, self.text);
        
        let mut start: usize = 0;
        let mut valids = Validator::new();
        let mut finished = false;

        loop {

            let (pos, chr) = match self.iter.next() {
                Some((pos, chr)) => (pos, chr),
                None => {
                    if finished { break };
                    finished = true;
                    (self.text.len(), ' ')
                },
            };

            if pos != 0 && valids.peek(chr, pos - start) == 0 {

                let target = valids.decide().ok_or(LexError::InvalidChar { chr, pos: pos})?;
                
                if target != Tokenkind::Seperator {

                    let token = self.make(target, start, pos);
                    stream.push(token);

                };
                
                start = pos;
                valids.reset();
                
            };

            valids.update(chr, pos - start);
                
        }

        return Ok(stream)

    }

    fn make(&self, kind: Tokenkind, start: usize, pos: usize) -> Token<'a> {

        use Tokenkind::*;
        
        let slice = self.rawr(start, pos);

        match kind {

            DoubleDot        => Token::new(kind, slice, start),
            EqualsEquals     => Token::new(kind, slice, start),
            BangEquals       => Token::new(kind, slice, start),
            OpenSharpEquals  => Token::new(kind, slice, start),
            CloseSharpEquals => Token::new(kind, slice, start),
            MinusEquals      => Token::new(kind, slice, start),
            PlusEquals       => Token::new(kind, slice, start),
            StarEquals       => Token::new(kind, slice, start),
            SlashEquals      => Token::new(kind, slice, start),
            PercentEquals    => Token::new(kind, slice, start),

            Integer          => Token::new(kind, slice, start),
            Float            => Token::new(kind, slice, start),
            Ident            => Token::new(kind, slice, start),
            KeyLet            => Token::new(kind, slice, start),
            KeyIn             => Token::new(kind, slice, start),
            KeyEnd            => Token::new(kind, slice, start),
            KeyProc           => Token::new(kind, slice, start),

            Literal          => Token::new(kind, self.rawr(start + 1, pos - 1), start), //? the literal doesn't include the enclosing double quotes
            Comment          => Token::new(kind, self.rawr(start + 1, pos - 1), start), //? the comment doesn't include the enclosing accents
            
            Note             => Token::new(kind, self.rawr(start + 2 /* 'start + 2 bc '´' has a len of 2 bytes */, pos), start + 1), //? the note doesn't include opening accent

            other            => Token::new(other, slice, 1),

        }
        
    }

    #[inline]
    fn rawr(&self, start: usize, end: usize) -> &'a str {
        &self.text[start..end]
    }

}
