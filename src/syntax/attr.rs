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
pub struct Attribute {
    pub name: Identifier,
    pub arguments: AttributeTok,
    pub loc: Loc,
}

impl Attribute {
    pub fn loc(&self) -> &Loc {
        &self.loc
    }
}

#[derive(Debug)]
pub enum AttributeTok {
    Int(i32, Loc),
    String(String, Loc),
    Ref(Identifier, Loc),
    Float(f32, Loc),
    True(Loc),
    False(Loc),
}

impl AttributeTok {
    pub fn loc(&self) -> &Loc {
        match self {
            AttributeTok::Int(_, loc) => loc,
            AttributeTok::String(_, loc) => loc,
            AttributeTok::Ref(_, loc) => loc,
            AttributeTok::Float(_, loc) => loc,
            AttributeTok::True(loc) => loc,
            AttributeTok::False(loc) => loc,
        }
    }
}
