
use fancy::colorize as color;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::error::Error as ErrorTrait;
use crate::lexer::*;

#[derive(Debug, Clone)]
pub(crate) struct Diag {
    level: Level,
    message: String,
    code: Option<String>,
    pos: Option<Location>,
    file: Option<String>,
    notes: Vec<String>,
}

impl Diag {

    pub(crate) fn new() -> Self {
        Self {
            level: Level::Info,
            message: String::new(),
            code: None,
            pos: None,
            file: None,
            notes: Vec::new(),
        }
    }

    pub(crate) fn level(&mut self, level: Level) -> &mut Self {
        self.level = level;
        self
    }

    pub(crate) fn code(&mut self, code: &str) -> &mut Self {
        self.code = Some(code.into());
        self
    }

    pub(crate) fn pos(&mut self, pos: Location) -> &mut Self {
        self.pos = Some(pos);
        self
    }

    pub(crate) fn file(&mut self, path: &str) -> &mut Self {
        self.file = Some(path.into());
        self
    }

    pub(crate) fn note(&mut self, message: &str) -> &mut Self {
        self.notes.push(message.into());
        self
    }

    pub(crate) fn say(&mut self, message: &str) -> &mut Self {
        self.message = message.into();
        self
    }

    pub(crate) fn emit(&self) {

        // info: compiling foo.rh
        // error: use of unstable feature `enums`
        //     at lexer/token.rh:240:96
        //     in `let Tokenkind enum {`
        //   note: enable this feature using `#{unstable-feature: enums}`
        //   note: enumerations aren't stable yet, please consider using `std:Enum` for now

        let mut output = String::with_capacity(
            self.message.len() +
            self.code.clone().map(|s| s.len()).unwrap_or(0) +
            self.notes.iter().map(|s| s.len()).sum::<usize>() +
            16
        );

        let mut push = |text: String| output.push_str(text.as_ref());

        match self.level {
            Level::Info    => push(color!("[blue]info: ", )),
            Level::Warning => push(color!("[yellow]warning: ", )),
            Level::Error   => push(color!("[red]error: ", )),
            Level::Fatal   => push(color!("[red]fatal: ", )),
        }

        push(self.message.clone());
        push("\n".into());

        match (self.file.clone(), self.pos) {
            (Some(path), Some(pos)) => push(color!("[245]    at {}:{}:{}\n", path, pos.line, pos.column)),
            (Some(path), None)      => push(color!("[245]    file {}\n", path)),
            (None, Some(pos))       => push(color!("[245]    at {}:{}\n", pos.line, pos.column)),
            (None, None)            => (),
        }

        for note in self.notes.clone().into_iter() {
            push(color!("[30]  note: [245]{}\n", note));
        }

        eprint!("{}", output);

    }

    pub(crate) fn abort(&self) -> ! {
        self.emit();
        std::process::exit(1);
    }

    pub(crate) fn info(text: &str) {
        Self::new()
            .level(Level::Info)
            .say(text)
            .emit();
    }

    pub(crate) fn warning(text: &str) {
        Self::new()
            .level(Level::Warning)
            .say(text)
            .emit();
    }

    pub(crate) fn error(text: &str) -> ! {
        Self::new()
            .level(Level::Error)
            .say(text)
            .abort();
    }

    #[track_caller]
    pub(crate) fn fatal(text: &str) -> ! {
        macro_rules! loc { () => { std::panic::Location::caller() }; }
        Self::new()
            .level(Level::Fatal)
            .say(&color!("[red][[{}:{}]] {}", loc!().file(), loc!().line(), text))
            .abort();
    }

}

impl Display for Diag {
    fn fmt(&self, _fmt: &mut Formatter) -> FmtResult {
        Ok(())
    }
}

impl ErrorTrait for Diag {}

#[derive(Debug, Clone)]
pub(crate) enum Level {
    Info,
    Warning,
    Error,
    Fatal,
}

pub(crate) trait DiagPanic<T> {
    fn aborts(self, msg: &str) -> T;
    fn abortsby(self, diag: &Diag) -> T;
}

impl<T, E> DiagPanic<T> for Result<T, E> {
    fn aborts(self, msg: &str) -> T {
        match self {
            Ok(v) => v,
            Err(_) => Diag::fatal(msg),
        }
    }
    fn abortsby(self, diag: &Diag) -> T {
        match self {
            Ok(v) => v,
            Err(_) => diag.abort(),
        }
    }
}

impl<T> DiagPanic<T> for Option<T> {
    fn aborts(self, msg: &str) -> T {
        match self {
            Some(v) => v,
            None => Diag::fatal(msg),
        }
    }
    fn abortsby(self, diag: &Diag) -> T {
        match self {
            Some(v) => v,
            None => diag.abort(),
        }
    }
}

#[macro_export]
macro_rules! fatal {
    ($message:literal, $($arg:expr),+) => {
        Diag::fatal(&format!($message, $($arg)+))
    };
    ($message:literal) => {
        Diag::fatal($message)
    };
}

