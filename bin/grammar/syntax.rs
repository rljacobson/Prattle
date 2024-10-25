#![allow(dead_code)]
/// This module defines types for the grammar description DSL.

use itertools::{join};
use std::fmt;

pub enum FieldType {
    Name,
    Associativity,
    Precedence,
    Meaningful,
    Syntax,
    Parse,
    Fullform,
    Error,
}

/// Given a field name as a FieldName enum value, returns its &str representation.
pub fn enum_to_field(name: FieldType) -> &'static str{
    match name {
        FieldType::Name => "name",
        FieldType::Associativity => "associativity",
        FieldType::Meaningful => "meaningful",
        FieldType::Syntax => "grammar",
        FieldType::Parse => "parse",
        FieldType::Fullform => "fullform",
        _ => "Error"
    }
}
/// Given a field name as a &str string, returns a FieldName.
pub fn field_to_enum(name: &str) -> FieldType {
    match &name.to_ascii_lowercase()[..] {
        "name" => FieldType::Name,
        "associativity" => FieldType::Associativity,
        "meaningful" => FieldType::Meaningful,
        "syntax" => FieldType::Syntax,
        "parse" => FieldType::Parse,
        "fullform" => FieldType::Fullform,
        _ => FieldType::Error,
    }
}

pub enum Associativity{
    Left,
    Right,
    Non,
    Full
}

pub fn assoc_to_enum(name: &str) ->Associativity{
    match &name.to_ascii_lowercase()[..] {
        "left" => Associativity::Left,
        "right" => Associativity::Right,
        "full" => Associativity::Full,
        "non" => Associativity::Non,
        _ => Associativity::Non,
    }
}

pub fn enum_to_assoc(name: &Associativity) -> &str{
    match name {
        Associativity::Left => "Left" ,
        Associativity::Right => "Right",
        Associativity::Full => "Full",
        Associativity::Non => "Non",
    }
}

pub enum TokenDenotation{
    Left,
    Null,
    Other
}

#[derive(Clone)]
pub enum Syntax {
    Expr1,
    Expr2,
    Expr3,
    Expr4,
    N,
    NegN,
    Symbol,
    Comma,
    NoSpace,
    Newline,
    Word(String),
    NamedChar(String),
    Number(i32),
    OnePlus(Box<Syntax>),
    OnePlusDelim(Box<Syntax>, Box<Syntax>), // OnePlusDelim(inner, delim)
    ZeroPlus(Box<Syntax>),
    ZeroPlusDelim(Box<Syntax>, Box<Syntax>), // ZeroPlusDelim(inner, delim)
    Optional(Box<Syntax>),
    Literal(String),
    Cons(Box<Syntax>, Vec<Syntax>),
    SequenceDelim(Vec<Syntax>),
    Sequence(Vec<Syntax>),
    Alternative(Vec<Syntax>),
    Empty // The absense of a grammar value.
}

// Display functions
impl fmt::Display for Syntax{
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        let out = match self{
            Syntax::Expr1 => "Expr1: expr1".to_string(),
            Syntax::Expr2 => "Expr2: expr2".to_string(),
            Syntax::Expr3 => "Expr3: expr3".to_string(),
            Syntax::Expr4 => "Expr4: expr4".to_string(),
            Syntax::N => "N: n".to_string(),
            Syntax::NegN => "NegN: -n".to_string(),
            Syntax::Symbol => "Symbol: symb".to_string(),
            Syntax::Word(word) => format!("Word: {}", &word),
            Syntax::Comma => ",".to_string(),
            Syntax::NoSpace => "NoSpace: nospace".to_string(),
            Syntax::Newline => "\n\tNewline\n".to_string(),
            Syntax::Number(n) => format!("Number: {}",  &n),
            Syntax::OnePlus(expr)
                => format!("Plus: ({})+", &expr),
            Syntax::OnePlusDelim(expr, delim)
                => format!("Plus Delim: ({} {})+", &expr, &delim),
            Syntax::ZeroPlus(expr)
                => format!("Star: ({})*", &expr),
            Syntax::ZeroPlusDelim(expr, delim)
                => format!("Star Delim: ({} {})*", &expr, &delim),
            Syntax::Optional(expr)
                => format!("Opt: ({})?", &expr),
            Syntax::Literal(expr)
                => format!("Literal: \"{}\"", &expr),
            Syntax::Cons(head, tail)
                => format!("Cons: {}[{}]", head, join(tail, ", ")),
            Syntax::SequenceDelim(list)
                => format!("Sequence Delim: {}", join(list, ", ")),
            Syntax::Sequence(list)
                => format!("Sequence: {}", join(list, " ")),
            Syntax::Alternative(list)
                => format!("Alt: ({})", join(list, " | ")),
            Syntax::NamedChar(name)
                =>  format!("NamedChar: {}", name),
            Syntax::Empty => "Empty".to_string(),
        };
        write!(dest, "<{} >", out)
    }
}
impl fmt::Debug for Syntax{
    fn fmt(&self, dest: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(dest, "{}", self)
    }
}

pub fn syntax_to_enum(name: &str) -> Syntax{
    match &name.to_lowercase()[..]{
        "expr1" => Syntax::Expr1,
        "expr2" => Syntax::Expr2,
        "expr3" => Syntax::Expr3,
        "expr4" => Syntax::Expr4,
        "n" => Syntax::N,
        "-n" => Syntax::NegN,
        "symb" => Syntax::Symbol,
        "," => Syntax::Comma,
        "nospace" => Syntax::NoSpace,
        _ => Syntax::Word(String::from(name)),
    }
}

pub struct OpRecord {
    pub name: String,
    pub associativity: Associativity,
    pub precedence: u32,
    pub meaningful: bool,
    pub syntax: Syntax,
    pub parse: Syntax,
    pub fullform: Syntax
}

impl fmt::Display for OpRecord {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::from("OpRecord{");
        out.push_str("\n\tname: ");
        out.push_str(&self.name);
        out.push_str("\n\tassociativity: ");
        out.push_str(&format!("{}", enum_to_assoc(&self.associativity)));
        out.push_str("\n\tprecedence: ");
        out.push_str(&format!("{}", self.precedence));
        out.push_str("\n\tmeaningful: ");
        out.push_str(&format!("{}", self.meaningful));
        out.push_str("\n\tgrammar: ");
        out.push_str(&format!("{}", &self.syntax));
        out.push_str("\n\tparse: ");
        out.push_str(&format!("{}", &self.parse));
        out.push_str("\n\tfullform: ");
        out.push_str(&format!("{}", &self.fullform));
        out.push_str("\n}");
        write!(dest, "<{} >", out)
    }
}
impl fmt::Debug for OpRecord{
    fn fmt(&self, dest: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(dest, "{}", self)
    }
}
