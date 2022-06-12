/*
 *    Copyright 2022 Gabrielle Guimarães de Oliveira
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */
use chumsky::prelude::*;

use crate::{
    parsing::tok::Tok,
    syntax::{id::Identifier, loc::Loc},
};

macro_rules! ranged {
    ($f:expr) => {
        |value, range: std::ops::Range<usize>| {
            $f(value, Loc::Range(range.start as i32, range.end as i32))
        }
    };
}

macro_rules! ranged_void {
    ($f:expr) => {
        |value, range: std::ops::Range<usize>| $f(Loc::Range(range.start as i32, range.end as i32))
    };
}

pub fn lexer() -> impl Parser<char, Vec<Tok>, Error = Simple<char>> {
    let number = text::int::<_, Simple<char>>(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect::<String>()
        .map_with_span(ranged!(Tok::Number));

    let string = just::<_, _, Simple<char>>('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map_with_span(ranged!(Tok::String));

    let simple_string = just::<_, _, Simple<char>>('\'')
        .ignore_then(filter(|c| *c != '\'').repeated())
        .then_ignore(just('\''))
        .collect::<String>()
        .map_with_span(ranged!(Tok::String));

    let ident = text::ident::<_, Simple<char>>().map_with_span(|ident: String, range| match ident
        .as_str()
    {
        "return" => Tok::Return(range.into()),
        "fun" => Tok::Fun(range.into()),
        "type" => Tok::Type(range.into()),
        "let" => Tok::Let(range.into()),
        "if" => Tok::If(range.into()),
        "then" => Tok::Then(range.into()),
        "else" => Tok::Else(range.into()),
        "mutable" => Tok::Mutable(range.into()),
        "true" => Tok::True(range.into()),
        "false" => Tok::False(range.into()),
        "use" => Tok::Use(range.into()),
        "sizeof" => Tok::Sizeof(range.into()),
        "module" => Tok::Module(range.into()),
        "match" => Tok::Match(range.into()),
        "enum" => Tok::Enum(range.into()),
        _ => Tok::Identifier(Identifier::new(ident, range.into())),
    });

    let ctrl = just::<_, _, Simple<char>>("{")
        .map_with_span(ranged_void!(Tok::LBrace))
        .or(just("}").map_with_span(ranged_void!(Tok::RBrace)))
        .or(just("[").map_with_span(ranged_void!(Tok::LBracket)))
        .or(just("]").map_with_span(ranged_void!(Tok::RBracket)))
        .or(just("(").map_with_span(ranged_void!(Tok::LParen)))
        .or(just(")").map_with_span(ranged_void!(Tok::RParen)))
        .or(just("->").map_with_span(ranged_void!(Tok::ArrowLeft)))
        .or(just("=>").map_with_span(ranged_void!(Tok::DoubleArrowLeft)))
        .or(just("+").map_with_span(ranged_void!(Tok::Add)))
        .or(just("-").map_with_span(ranged_void!(Tok::Sub)))
        .or(just("*").map_with_span(ranged_void!(Tok::Times)))
        .or(just("/").map_with_span(ranged_void!(Tok::Div)))
        .or(just("++").map_with_span(ranged_void!(Tok::Concat)))
        .or(just(".").map_with_span(ranged_void!(Tok::Dot)))
        .or(just("&").map_with_span(ranged_void!(Tok::Ampersand)))
        .or(just("<").map_with_span(ranged_void!(Tok::Lt)))
        .or(just("<=").map_with_span(ranged_void!(Tok::Lte)))
        .or(just(">").map_with_span(ranged_void!(Tok::Gt)))
        .or(just(">=").map_with_span(ranged_void!(Tok::Gte)))
        .or(just("!").map_with_span(ranged_void!(Tok::Bang)))
        .or(just(":=").map_with_span(ranged_void!(Tok::Assign)))
        .or(just("==").map_with_span(ranged_void!(Tok::Eq)))
        .or(just("!=").map_with_span(ranged_void!(Tok::Neq)));

    let tok = ident
        .or(number)
        .or(string)
        .or(simple_string)
        .or(ctrl)
        .recover_with(skip_then_retry_until([]));

    tok.repeated()
}
