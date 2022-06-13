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
    syntax::{id::Identifier, loc::Located},
};

pub fn lexer() -> impl Parser<char, Vec<Located<Tok>>, Error = Simple<char>> {
    let number = text::int::<_, Simple<char>>(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect::<String>()
        .map(Tok::Number);

    let string = just::<_, _, Simple<char>>('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Tok::String);

    let simple_string = just::<_, _, Simple<char>>('\'')
        .ignore_then(filter(|c| *c != '\'').repeated())
        .then_ignore(just('\''))
        .collect::<String>()
        .map(Tok::String);

    let ident = text::ident::<_, Simple<char>>().map_with_span(|ident: String, range| match ident
        .as_str()
    {
        "return" => Tok::Return,
        "fun" => Tok::Fun,
        "type" => Tok::Type,
        "let" => Tok::Let,
        "if" => Tok::If,
        "then" => Tok::Then,
        "else" => Tok::Else,
        "mutable" => Tok::Mutable,
        "true" => Tok::True,
        "false" => Tok::False,
        "use" => Tok::Use,
        "sizeof" => Tok::Sizeof,
        "module" => Tok::Module,
        "match" => Tok::Match,
        "enum" => Tok::Enum,
        _ => Tok::Identifier(Identifier::new(ident, range.into())),
    });

    let ctrl = just::<_, _, Simple<char>>("{")
        .map(|_| Tok::LBrace)
        .or(just("}").map(|_| Tok::RBrace))
        .or(just("[").map(|_| Tok::LBracket))
        .or(just("]").map(|_| Tok::RBracket))
        .or(just("(").map(|_| Tok::LParen))
        .or(just(")").map(|_| Tok::RParen))
        .or(just("->").map(|_| Tok::ArrowLeft))
        .or(just("=>").map(|_| Tok::DoubleArrowLeft))
        .or(just("+").map(|_| Tok::Add))
        .or(just("-").map(|_| Tok::Sub))
        .or(just("*").map(|_| Tok::Times))
        .or(just("/").map(|_| Tok::Div))
        .or(just("++").map(|_| Tok::Concat))
        .or(just(".").map(|_| Tok::Dot))
        .or(just("&").map(|_| Tok::Ampersand))
        .or(just("<").map(|_| Tok::Lt))
        .or(just("<=").map(|_| Tok::Lte))
        .or(just(">").map(|_| Tok::Gt))
        .or(just(">=").map(|_| Tok::Gte))
        .or(just("!").map(|_| Tok::Bang))
        .or(just(":=").map(|_| Tok::Assign))
        .or(just("==").map(|_| Tok::Eq))
        .or(just("!=").map(|_| Tok::Neq));

    let tok = ident
        .or(number)
        .or(string)
        .or(simple_string)
        .or(ctrl)
        .recover_with(skip_then_retry_until([]));

    tok.map_with_span(|tok, span| (tok, span.into())).repeated()
}
