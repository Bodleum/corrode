use std::fmt::Display;

use thiserror::Error;

use crate::parsing::Span;

#[derive(Debug, Error, Clone)]
pub struct ParseError<'a> {
    span: Span<'a>,
    message: String,
    context: Vec<&'static str>,
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (line, col) = (self.span.location_line(), self.span.location_offset());
        let gutter_width = line.checked_ilog10().unwrap_or(0) + 1;
        let bad_line = self.span.fragment().lines().next().unwrap_or_default();

        writeln!(f, "Frontmatter parse error at line {line}, column {col}")?;

        for _ in 0..=gutter_width {
            write!(f, " ")?;
        }
        writeln!(f, "|")?;

        writeln!(f, "{line} | {bad_line}")?;

        for _ in 0..=gutter_width {
            write!(f, " ")?;
        }
        write!(f, "|")?;
        for _ in 0..=col {
            write!(f, " ")?;
        }
        write!(f, "^")?;
        for _ in 1..bad_line.len() {
            write!(f, "^")?;
        }
        writeln!(f, "")?;

        writeln!(f, "{}", self.message)?;
        if !self.context.is_empty() {
            writeln!(f, "while: {}", self.context.join("\nwhile: "))?;
        }

        Ok(())
    }
}

impl<'a> ParseError<'a> {
    pub fn new(message: String, span: Span<'a>, context: Option<&'static str>) -> Self {
        let mut c = Vec::new();
        if let Some(context) = context {
            c.push(context)
        }

        Self {
            span,
            message,
            context: c,
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn line(&self) -> u32 {
        self.span.location_line()
    }

    pub fn offset(&self) -> usize {
        self.span.location_offset()
    }
}

impl<'a> nom::error::ParseError<Span<'a>> for ParseError<'a> {
    fn from_error_kind(input: Span<'a>, kind: nom::error::ErrorKind) -> Self {
        Self {
            span: input,
            message: format!("Parse error: {kind:?}"),
            context: Vec::new(),
        }
    }

    fn append(_input: Span, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }

    fn from_char(input: Span<'a>, c: char) -> Self {
        Self {
            span: input,
            message: format!("Unexpected character: '{c}'"),
            context: Vec::new(),
        }
    }

    fn or(self, other: Self) -> Self {
        other
    }
}

impl<'a> nom::error::ContextError<Span<'a>> for ParseError<'a> {
    fn add_context(_input: Span, ctx: &'static str, other: Self) -> Self {
        let mut s = Self {
            span: other.span,
            message: other.message,
            context: other.context,
        };
        s.context.push(ctx);
        s
    }
}
