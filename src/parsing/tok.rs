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

use crate::syntax::id::Identifier;
use crate::syntax::loc::Loc;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Tok {
    Eof,
    Nl,
    At,
    Semi,
    Comma,
    Colon,
    Bar,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Apostrophe,
    Dot,
    Ampersand,
    Add,
    Sub,
    Div,
    Times,
    Concat,
    Bang,
    Equal,
    Assign,
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
    Neq,
    DoubleArrowLeft,
    ArrowLeft,
    Return,
    Fun,
    Type,
    Let,
    If,
    Then,
    Else,
    Mutable,
    True,
    False,
    Use,
    Sizeof,
    Module,
    Match,
    Enum,
    Identifier(Identifier),
    String(String),
    Number(String),
}
