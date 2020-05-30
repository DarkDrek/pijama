//! Parsers for binary operators.
//!
//! This module contains a set of functions `bin_op_<n>` where each `n` represents one level of
//! precedence, i.e., the operators in `bin_op_5` have higher precedence than the operators in
//! `bin_op_1`.
//!
//! Each one of this parsers is used inside the [`binary_op`] submodule with the same numeric
//! convention as here.
//!
//! [`binary_op`]: crate::parser::node::binary_op
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, space0},
    combinator::{map, not, peek},
    sequence::terminated,
};

use log::debug;

use pijama_ast::{BinOp, BinOp::*, Span};

use crate::parser::{
    helpers::{log_success, surrounded, with_context},
    IResult,
};

/// Parser for the binary operators with precedence level 1.
///
/// These operators are `&&` and `||`.
///
/// All the binary operators might be surrounded by zero or more spaces.
pub fn bin_op_1(input: Span) -> IResult<BinOp> {
    surrounded(
        with_context(
            "Expected logical operator (&&, ||)",
            log_success(
                alt((map(tag("&&"), |_| And), map(tag("||"), |_| Or))),
                |op, loc| debug!("Parsed logical operator {:?} at {}", op, loc),
            ),
        ),
        space0,
    )(input)
}

/// Parser for the binary operators with precedence level 2.
///
/// These operators are `<=`, `>=`, `<`, `>`, `==` and `!=`.
///
/// An additional check is done for `<` and `>` to be sure they are not the beginning of the `>>`
/// and `<<` operators.
///
/// All the binary operators might be surrounded by zero or more spaces.
pub fn bin_op_2(input: Span) -> IResult<BinOp> {
    surrounded(
        with_context(
            "Expected comparision operator (<=, >=, <, >, ==, !=)",
            log_success(
                alt((
                    map(tag("<="), |_| Lte),
                    map(tag(">="), |_| Gte),
                    map(terminated(char('<'), peek(not(char('<')))), |_| Lt),
                    map(terminated(char('>'), peek(not(char('>')))), |_| Gt),
                    map(tag("=="), |_| Eq),
                    map(tag("!="), |_| Neq),
                )),
                |op, loc| debug!("Parsed comparison operator {:?} at {}", op, loc),
            ),
        ),
        space0,
    )(input)
}

/// Parser for the binary operators with precedence level 3.
///
/// These operators are `&`, `|`, `^`, `>>` and `<<`.
///
/// An additional check is done for `&` and `|` to be sure they are not the beginning of the `&&`
/// and `||` operators.
///
/// All the binary operators might be surrounded by zero or more spaces.
pub fn bin_op_3(input: Span) -> IResult<BinOp> {
    surrounded(
        with_context(
            "Expected binary operator (&, |, ^, <<, >>)",
            log_success(
                alt((
                    map(terminated(char('&'), peek(not(char('&')))), |_| BitAnd),
                    map(terminated(char('|'), peek(not(char('|')))), |_| BitOr),
                    map(char('^'), |_| BitXor),
                    map(tag(">>"), |_| Shr),
                    map(tag("<<"), |_| Shl),
                )),
                |op, loc| debug!("Parsed binary operator {:?} at {}", op, loc),
            ),
        ),
        space0,
    )(input)
}

/// Parser for the binary operators with precedence level 4.
///
/// These operators are `+` and `-`.
///
/// All the binary operators might be surrounded by zero or more spaces.
pub fn bin_op_4(input: Span) -> IResult<BinOp> {
    surrounded(
        with_context(
            "Expected binary operator (+, -)",
            log_success(
                alt((map(char('+'), |_| Add), map(char('-'), |_| Sub))),
                |op, loc| debug!("Parsed binary operator {:?} at {}", op, loc),
            ),
        ),
        space0,
    )(input)
}

/// Parser for the binary operators with precedence level 5.
///
/// These operators are `*`, `/` and `%`.
///
/// All the binary operators might be surrounded by zero or more spaces.
pub fn bin_op_5(input: Span) -> IResult<BinOp> {
    surrounded(
        with_context(
            "Expected binary operator (*, /, %)",
            log_success(
                alt((
                    map(char('*'), |_| Mul),
                    map(char('/'), |_| Div),
                    map(char('%'), |_| Rem),
                )),
                |op, loc| debug!("Parsed binary operator {:?} at {}", op, loc),
            ),
        ),
        space0,
    )(input)
}
