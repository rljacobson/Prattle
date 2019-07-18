#![allow(dead_code)]
/// This module defines the grammar of a syntax description DSL.
use std::fs;
extern crate nom;
use nom::{
    bytes::complete::{tag, tag_no_case, is_not},
    character::complete::{alpha1, multispace1, space0, space1,
                          line_ending, not_line_ending, char as character},
    combinator::{map, opt},
    sequence::{preceded, terminated, pair, tuple, delimited},
    IResult,
    Err,
    error::{ErrorKind, ErrorKind::Eof},
    branch::alt,
    number::complete::float,
    multi::{separated_nonempty_list, separated_list, many1, many0_count},
};

use crate::syntax::syntax::*;

// region: Infinite Recursion Guard

// This is how we keep from infinite recursion.
struct RecursingState{
    pub sequence: bool
}
enum RecursiveFn{
    All,
    Sequence
}
static mut RECURSING: RecursingState =
    RecursingState {
        sequence: false
    };
// The code in this module runs in a single thread, so no thread
// safety mechanism is necessary here.
fn recurse_clear(which: RecursiveFn){
    unsafe{
        match which {
            RecursiveFn::Sequence => {
                RECURSING.sequence = false;
            },
            RecursiveFn::All => {
                RECURSING.sequence = false;
            }
        }
    }
}
fn recurse_guard(which: RecursiveFn) -> bool {
    unsafe {
        match which{
            RecursiveFn::Sequence => {
                let original = RECURSING.sequence;
                RECURSING.sequence = true;
                original
            },
            // Unused:
            RecursiveFn::All => RECURSING.sequence,
        }
    }
}
// endregion: Infinite Recursion Guard

/// The same as parse_field, but expects a name field: `"name: value"`.
fn parse_op_name(input: &str) -> IResult<&str, &str> {
    let parse_field_name =
        delimited(
            many0_count(alt((multispace1, comment))),
            tag_no_case("name:"),
            space0
        );

    preceded(
        parse_field_name,
        alpha1
    )(input)
}
fn parse_field_name(input: &str) -> IResult<&str, FieldType> {
    map(
        delimited(
            many0_count(alt((multispace1, comment))),
            alpha1,
            tag(":")
        ),
    field_to_enum
    )(input)
}
fn parse_field_value(input: &str) -> IResult<&str, &str> {
    delimited(
        space0,
        alpha1,
        end_of_line
    )(input)
}
fn parse_bool_option(input: &str) -> IResult<&str, bool> {
    let parse_field_value =
        delimited(
            space0,
            alt((
                tag_no_case("true"),
                tag_no_case("false")
            )),
            end_of_line
        );

    map(
        parse_field_value,
        |instr: &str| match &instr.to_ascii_lowercase()[..] {
            "true"=> true,
            "false"=>false,
            _ => false
        }
    )(input)
}
fn parse_associativity(input: &str) -> IResult<&str, Associativity> {
    let parse_field_value =
        delimited(
            space0,
            alt((
                tag_no_case("left"),
                tag_no_case("right"),
                tag_no_case("non"),
                tag_no_case("none"),
            )),
            end_of_line
        );

    map(
        parse_field_value,
        assoc_to_enum
    )(input)
}
fn parse_number(input: &str) -> IResult<&str, f32>{
    delimited(
        space0,
        float,
        end_of_line
    )(input)
}

// Custom combinators, not used on their own.
fn eoff(inp: &str) -> IResult<&str, &str> {
    let result = many0_count(alt((multispace1, comment)))(inp);
    match result {
        Ok((i, _)) => {
            if i.len() < 1 {
                Ok((i, &""[..]))
            } else {
                Err(Err::Error((inp, ErrorKind::Eof)))
            }
        },
        Err(Err::Error((i, _))) => {
            // For some reason, eof errors manifest as Many0 errors.
            if i.len() < 1 {
                Ok((i, &""[..]))
            } else {
                Err(Err::Error((inp, ErrorKind::Eof)))
            }
        }
//        Err(Err::Error((i, ErrorKind::Eof))) // Shouldn't happen.
//            => Ok((i, &""[..])),
        Err(x) => Err(x) // Some other error, not eof
    }
}
fn end_of_line(inp: &str) -> IResult<&str, Syntax> {
    map(
        tuple((
            space0,
            opt(comment),
            alt((line_ending, eoff))
        )),
        |_| Syntax::Newline
    )(inp)
}
fn comment(inp: &str) -> IResult<&str, &str> {
    map(
        terminated(
            character('#'),
            not_line_ending
        ),
        |_| "" // Must be thrown away.
    )(inp)
}

fn parse_syntax_token(input: &str) -> IResult<&str, Syntax> {
    fn string_literal(inp: &str) -> IResult<&str, Syntax> {
        map(
            delimited(
                tag("\""),
                is_not("\""),
                tag("\""),
            ),
            |inner: &str| {
                Syntax::Literal(inner.to_string())
            }
        )(inp)
    }
    fn metavariable(inp: &str) -> IResult<&str, Syntax> {
        map(
            alt((
                tag_no_case("expr1"),
                tag_no_case("expr2"),
                tag_no_case("expr3"),
                tag_no_case("expr4"),
                tag_no_case("nospace"),
                tag_no_case("n"),
                tag_no_case("-n"),
                tag_no_case("symb"),
                tag(","),
                alpha1
            )),
            syntax_to_enum
        )(inp)
    }
    fn one_plus_delim(inp: &str) -> IResult<&str, Syntax> {
        map(
            delimited(
                tag("("),
                pair(parse_syntax_token, parse_syntax_token),
                preceded(space0, tag(")+"))
            ),
            |inner|
                Syntax::OnePlusDelim(Box::from(inner.0), Box::from(inner.1))
        )(inp)
    }
    fn one_plus(inp: &str) -> IResult<&str, Syntax> {
        map(
            delimited(
                tag("("),
                parse_syntax_token,
                tag(")+")
            ),
            |inner| Syntax::OnePlus(Box::from(inner))
        )(inp)
    }
    fn zero_plus_delim(inp: &str) -> IResult<&str, Syntax> {
        map(
            delimited(
                tag("("),
                pair(parse_syntax_token, parse_syntax_token),
                preceded(space0, tag(")*"))
            ),
            |inner|
                Syntax::ZeroPlusDelim(Box::from(inner.0), Box::from(inner.1))
        )(inp)
    }
    fn zero_plus(inp: &str) -> IResult<&str, Syntax> {
        map(
            delimited(
                tag("("),
                parse_syntax_token,
                tag(")*")
            ),
            |inner| Syntax::ZeroPlus(Box::from(inner))
        )(inp)
    }
    fn cons(inp: &str) -> IResult<&str, Syntax> {
        let result =
            map(
                pair(
                    metavariable,
                    delimited(
                        character('['),
                        separated_list(terminated(character(','), space0), parse_syntax_token),
                        character(']')
                    )
                ),
                | (head, tail) | {
                    Syntax::Cons(Box::from(head), tail)
                }
            )(inp);
        result
    }
    fn number(inp: &str) -> IResult<&str, Syntax> {
        map(float,
            |n| {Syntax::Number(n as i32)}
        )(inp)
    }
    fn optional(inp: &str) -> IResult<&str, Syntax> {
        map(
            delimited(
                tag("("),
                parse_syntax_token,
                tag(")?")
            ),
            |inner| Syntax::Optional(Box::from(inner))
        )(inp)
    }
    fn sequence(inp: &str) -> IResult<&str, Syntax>{
        if recurse_guard(RecursiveFn::Sequence){
            return Err(Err::Error((inp, nom::error::ErrorKind::Not)));
        }
        let result =
            map(
                many1(parse_syntax_token),
                |mut seq| {
                    match seq.as_slice(){
                        [_] => seq.pop().unwrap(),
                        [_, Syntax::Sequence(ref y)] => {
                            let mut newvec: Vec<Syntax> = Vec::new();
                            newvec.reserve(y.len()+1);
                            // This match is unnecessary, as we already know what the second
                            // vector element is. Not using unsafe for the zealots.
                            match seq.pop().unwrap(){
                                Syntax::Sequence(v) => Syntax::Sequence(v),
                                _ => panic!("Unreachable code!")
                            }
                        },
                        _ => Syntax::Sequence(seq)
                    }
                }
            )(inp);
        result
    }
    fn alternative(inp: &str) -> IResult<&str, Syntax>{
        map(
            delimited(
                character('('),
                separated_nonempty_list(character('|'), parse_syntax_token),
                character(')')
            ),
            |inner| Syntax::Alternative(inner)
        )(inp)
    }
    fn named_char(inp: &str) -> IResult<&str, Syntax>{
        map(
            delimited(
                tag("\\["),
                alpha1,
                character(']')
            ),
            |inner| Syntax::NamedChar(String::from(inner))
        )(inp)
    }

    // Things we skip:
    fn skip(inp: &str) -> IResult<&str, Syntax> {
        preceded(
            space1,
            parse_syntax_token
        )(inp)
    }

    map(
        alt((
            cons,
            metavariable,
            string_literal,
            optional,
            alternative,
            number,
            one_plus_delim,
            zero_plus_delim,
            one_plus,
            zero_plus,
            skip,
            sequence,
        )),
        | inner | {
            recurse_clear(RecursiveFn::All);
            inner
        }
    )(input)
}

fn parse_syntax(input: &str) -> IResult<&str, Syntax> {
    map(
        terminated(
            many1(parse_syntax_token),
            end_of_line
        ),
        |mut inner| {
            if inner.len() == 1{
                inner.pop().unwrap()
            } else {
                Syntax::Sequence(inner)
            }
        }
    )(input)
}

pub fn parse_entry(input: &str) -> IResult<&str, OpRecord>{
    let mut op_name: String;
    let mut prior_rest: &str; // Remaining input of prior iteration over fields
    let mut rest: &str; // Remaining input
    let mut field: FieldType;

    match parse_op_name(input) {
        Ok((rest_input, name)) => {
            op_name = String::from(name);
            rest = rest_input;
            prior_rest = rest_input;
        },
        Err(_e)=> return Err(_e)
    }

    let mut op_record = OpRecord{
        name: op_name,
        associativity: Associativity::None,
        precedence: 0,
        meaningful: false,
        syntax: Syntax::Empty,
        parse: Syntax::Empty,
        fullform: Syntax::Empty
    };

    // Keep fetching fields until we either hit eof, a name field, or an error.
    loop {
        match parse_field_name(rest) {
            Ok((r, f)) => {
                rest = r;
                field = f;
            },
            Err(Err::Error((r, e))) => {
                if r.len() < 1 || Eof == e{
                    return Ok((rest, op_record))
                } else {
                    return Err(Err::Error((r, e)))
                }
            },
            Err(_e) => return Err(_e)
        }

        match field {
            FieldType::Name => {
                // Put the name back on the input
                rest = prior_rest;
                return Ok((rest, op_record));
            }
            FieldType::Associativity => {
                match parse_associativity(rest){
                    Ok((r, a)) => {
                        rest = r;
                        op_record.associativity = a;
                    }
                    Err(_e) => return Err(_e)
                }
            }
            FieldType::Precedence => {
                match parse_number(rest) {
                    Ok((r, a)) => {
                        rest = r;
                        op_record.precedence = a as u32;
                    }
                    Err(_e) => return Err(_e)
                }
            }
            FieldType::Meaningful => {
                match parse_bool_option(rest){
                    Ok((r, a)) => {
                        rest = r;
                        op_record.meaningful = a;
                    }
                    Err(_e) => return Err(_e)
                }
            }
            FieldType::Syntax => {
                match parse_syntax(rest){
                    Ok((r, a)) => {
                        rest = r;
                        op_record.syntax = a;
                    }
                    Err(_e) => return Err(_e)
                }
            }
            FieldType::Parse => {
                match parse_syntax(rest){
                    Ok((r, a)) => {
                        rest = r;
                        op_record.parse = a;
                    }
                    Err(_e) => return Err(_e)
                }
            }
            FieldType::Fullform => {
                match parse_syntax(rest){
                    Ok((r, a)) => {
                        rest = r;
                        op_record.fullform = a;
                    }
                    Err(_e) => return Err(_e)
                }
            }
            FieldType::Error => {
                return Err(Err::Error((prior_rest, nom::error::ErrorKind::Not)));
            }
        }
        prior_rest = rest;
    }
    // Unreachable.
}

pub fn parse_grammar_file(filename: &str) -> Result<Vec<OpRecord>, String>{
    // Read in the file contents.
    let contents: String = fs::read_to_string(filename)
        .expect("Could not read from the file.");

    let result =
        many1(parse_entry)(&contents[..]);
    match result {
        Ok((rest, entries)) => {
            if rest.len() > 0 {
                Err(String::from(format!("Failed to parse the entire file:\n{}", rest)))
            } else {
                Ok(entries)
            }
        },
        Err(_e) => {
            Err(String::from(format!("Received error: {:?}", _e)))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_field_name_test() {
        // Test parse_field:
        let result = parse_field_name("name:\tMultiplyMatrix\n");

        assert_eq!(result, Ok(("\tMultiplyMatrix\n", FieldType::Name) ));
    }

    #[test]
    fn parse_op_name_test1() {
        // Test parse_field:
        let result = parse_op_name("name:\tMultiplyMatrix\n");

        assert_eq!(result, Ok(("", "MultiplyMatrix") ));
    }

    #[test]
    fn parse_op_name_test2() {
        // Test parse_op_name:
        let result = parse_op_name("NAME: Divide\n");

        assert_eq!(result, Ok(("", "Divide") ));
    }

    #[test]
    fn parse_syntax_test1() {
        // Test parse_syntax:
        let test_input = "expr1\"[[\"(expr2\",\")*\"]]\"\n";
        result = parse_syntax(test_input);
        assert_eq!(result, Ok(("", _)));
    }

    #[test]
    fn parse_syntax_test2() {
        // Test parse_syntax:
        let test_input = "SubsuperscriptBox[expr1,expr2,expr3]\n";
        result = parse_syntax(test_input);
        assert_eq!(result, Ok(("", _)));
    }

    #[test]
    fn parse_syntax_test3() {
        // Test parse_syntax:
        let test_input = " expr1 \"\\^\" expr2 \"\\%\" expr3";
        result = parse_syntax(test_input);
        assert_eq!(result, Ok(("", _)));
    }

    #[test]
    fn parse_entry_test() {
        let test_input = r#"name: SubsuperscriptBox
            associativity: right
            meaningful: true
            syntax: expr1 "\^" expr2 "\%" expr3
            parse: SubsuperscriptBox[expr1, expr2, expr3]"#;
        result = parse_entry(test_input);
        assert_eq!(result, Ok(("", _)));
    }
}
