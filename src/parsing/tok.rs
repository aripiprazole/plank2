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

#[derive(Debug)]
pub enum Tok {
    Eof(Loc),
    Nl(Loc),
    At(Loc),
    Semi(Loc),
    Comma(Loc),
    Colon(Loc),
    Bar(Loc),
    LParen(Loc),
    RParen(Loc),
    LBrace(Loc),
    RBrace(Loc),
    LBracket(Loc),
    RBracket(Loc),
    Apostrophe(Loc),
    Dot(Loc),
    Ampersand(Loc),
    Add(Loc),
    Sub(Loc),
    Div(Loc),
    Times(Loc),
    Concat(Loc),
    Bang(Loc),
    Equal(Loc),
    Assign(Loc),
    Gt(Loc),
    Gte(Loc),
    Lt(Loc),
    Lte(Loc),
    Eq(Loc),
    Neq(Loc),
    DoubleArrowLeft(Loc),
    ArrowLeft(Loc),
    Return(Loc),
    Fun(Loc),
    Type(Loc),
    Let(Loc),
    If(Loc),
    Then(Loc),
    Else(Loc),
    Mutable(Loc),
    True(Loc),
    False(Loc),
    Use(Loc),
    Sizeof(Loc),
    Module(Loc),
    Match(Loc),
    Enum(Loc),
    Identifier(Identifier),
    String(String, Loc),
    Number(String, Loc),
}
