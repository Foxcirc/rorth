
use enumflags2::BitFlags;
use crate::lexer::*;

#[derive(Debug, PartialEq)]
pub(crate) struct Validator {
    current: (BitFlags<Tokenkind>, bool, bool),
    peek: (BitFlags<Tokenkind>, bool, bool)
}

impl Validator {

    #[inline(always)]
    pub(crate) fn new() -> Self {
        Self {
            current: (BitFlags::empty(), false, false),
            peek: (BitFlags::empty(), false, false),
        }
    }
    
    #[inline(always)]
    pub(crate) fn reset(&mut self) {
        self.peek = (BitFlags::empty(), false, false);
        self.current = (BitFlags::empty(), false, false);
    }

    #[inline(always)]
    pub(crate) fn update(&mut self, chr: char, len: u16) {
        Self::check(chr, len, &self.current.clone() /* can someone please edit this clone awayyy */, &mut self.current);
    }
    
    #[inline(always)]
    pub(crate) fn peek(&mut self, chr: char, len: u16) -> usize {
        Self::check(chr, len, &self.current, &mut self.peek);
        self.peek.0.iter().count()
    }

    /// This is the *magic* function.
    /// It decides for what token(s) the current character is valid for.
    #[inline(always)]
    fn check(chr: char, len: u16, old: &(BitFlags<Tokenkind>, bool, bool), flags: &mut (BitFlags<Tokenkind>, bool, bool)) {

        /* 
            Wenn man Sperma isst, dann ist das ja eigentlich Kanibalismus...
                -- ~~Skeletor~~ LeptiaDieWahre will come back with more disturbing facts tomorrow.
        */

        use Tokenkind::*;

        macro_rules! set {
            ($flags:expr, $kind:ident: $cond:expr) => {
                if $cond { $flags.0.insert($kind) } else { $flags.0.remove($kind) };
            };
        }

        macro_rules! simple {
            ($flags:expr, $chr:ident, $start:expr, $(($cond:pat => $kind:ident)),*,) => {
                $( set!($flags, $kind: $start && matches!($chr, $cond)); )*
            };
        }

        macro_rules! extend {
            ($flags:expr, $chr:ident, $len:ident, $(($start:ident + $second:literal => $kind:ident)),*,) => {
                $( set!($flags, $kind: (old.0.contains($start) && $len == 1 && $chr == $second)); )*
            };
        }

        let start = len == 0;

        //? Single character tokens:

        simple! {
            flags, chr, start,
                (' ' | '\t' | '\r' => Seperator),
                ('\n' => Newline),
                ('+'  => Plus),
                ('-'  => Minus),
                ('*'  => Star),
                ('/'  => Slash),
                ('.'  => Dot),
                (':'  => Colon),
                (','  => Comma),
                ('='  => Equals),
                ('!'  => Bang),
                ('&'  => Ampersand),
                ('%'  => Percent),
                ('~'  => Tilde),
                ('#'  => Hashtag),
                ('?'  => Questionmark),
                ('|'  => Pipe),
                
                ('<'  => OpenSharp),
                ('>'  => CloseSharp),
                ('['  => OpenSquare),
                (']'  => CloseSquare),
                ('{'  => OpenCurly),
                ('}'  => CloseCurly),
                ('('  => OpenNormal),
                (')'  => CloseNormal),
        };

        //? Mutli character tokens:

        extend!( // todo use else-if´s here instead of just if´s (or be more efficient in another way)
            flags, chr, len,
            (Dot + '.' => DoubleDot),
            (Equals + '=' => EqualsEquals),
            (Bang + '=' => BangEquals),
            (OpenSharp + '=' => OpenSharpEquals),
            (CloseSharp + '=' => CloseSharpEquals),
            (Minus + '=' => MinusEquals),
            (Plus + '=' => PlusEquals),
            (Star + '=' => StarEquals),
            (Slash + '=' => SlashEquals),
            (Percent + '=' => PercentEquals),
        );

        //? Dynamic tokens:
        
        set!(flags, Integer: (start                   && matches!(chr, '0'..='9')) || (old.0.contains(Integer) && matches!(chr, '0'..='9' | '_')));
        set!(flags, Float:   (old.0.contains(Integer) && matches!(chr, '.'))             || (old.0.contains(Float)   && matches!(chr, '0'..='9')));
        
        // todo add escaping support (\") HOW??? je ne sais pas :(
        
        set!(flags, Literal: old.1 || (start && chr == '"'));
        set!(flags, Comment: old.2 || (start && chr == '`'));

        if chr == '"' && !old.0.contains(Comment) { flags.1 = !flags.1 };
        if chr == '`' && !old.0.contains(Literal) { flags.2 = !flags.2 };

        set!(flags, Note: (start && chr == '\u{00b4}') || (old.0.contains(Note) && matches!(chr, 'a'..='z' | 'A'..='Z' | '-' | '_')));

        set!(flags, Ident: (start && matches!(chr, 'a'..='z' | 'A'..='Z' | '_')) | (old.0.contains(Ident) && matches!(chr, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_')));
        
    }

    pub(crate) fn decide(&self) -> Option<Tokenkind> {

        if self.current.0.iter().count() == 1 { self.current.0.iter().next() }
        else {
            None
        }

    }

}
