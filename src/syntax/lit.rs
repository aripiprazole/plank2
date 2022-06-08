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

use crate::syntax::loc::Loc;

#[derive(Debug, PartialEq)]
pub enum Lit {
    String(String, Loc),
    Int(i32, Loc),
    Float(f32, Loc),
    True(Loc),
    False(Loc),
}

impl Lit {
    pub fn loc(&self) -> &Loc {
        match self {
            Lit::String(_, loc) => loc,
            Lit::Int(_, loc) => loc,
            Lit::Float(_, loc) => loc,
            Lit::True(loc) => loc,
            Lit::False(loc) => loc,
        }
    }
}