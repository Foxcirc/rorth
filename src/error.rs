
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
    hints: Vec<String>,
}

impl Diag {

    pub(crate) fn new() -> Self {
        Self {
            level: Level::Info,
            message: String::new(),
            code: None,
            pos: None,
            file: None,
            hints: Vec::new(),
        }
    }

    pub(crate) fn level(&mut self, level: Level) -> &mut Self {
        self.level = level;
        self
    }

    pub(crate) fn code(&mut self, code: &str) -> &mut Self {
        // self.code = Some(code.lines().nth(pos.line).expect("Error position not in the source code.").into());
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

    pub(crate) fn hint(&mut self, message: &str) -> &mut Self {
        self.hints.push(message.into());
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
        //   hint: enable this feature using `#{unstable-feature: enums}`
        //   hint: enumerations aren't stable yet, please consider using `std:Enum` for now

        let mut output = String::with_capacity(
            self.message.len() +
            self.code.clone().map(|s| s.len()).unwrap_or(0) +
            self.hints.iter().map(|s| s.len()).sum::<usize>() +
            16
        );

        let mut push = |text: String| output.push_str(text.as_ref());

        match self.level {
            Level::Info    => push(color!("[blue]info: ", )),
            Level::Warning => push(color!("[yellow]warning: ", )),
            Level::Error   => push(color!("[red]error: ", )),
        }

        push(self.message.clone());
        push("\n".into());

        match (self.file.clone(), self.pos) {
            (Some(path), Some(pos)) => push(color!("[245]    at {}:{}:{}\n", path, pos.line, pos.column)),
            (Some(path), None)      => push(color!("[245]    file {}\n", path)),
            (None, Some(pos))       => push(color!("[245]    at {}:{}\n", pos.line, pos.column)),
            (None, None)            => (),
        }

        for hint in self.hints.clone().into_iter() {
            push(color!("[30]  hint: [245]{}\n", hint));
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
}
