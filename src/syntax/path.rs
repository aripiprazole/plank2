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

#[derive(Debug, Eq, PartialEq)]
pub struct QualifiedPath {
    pub full_path: Vec<Identifier>,
    pub loc: Loc,
}

impl QualifiedPath {
    pub fn new(path: String, loc: Loc) -> QualifiedPath {
        QualifiedPath {
            full_path: path
                .split(".")
                .map(|text| Identifier::from_string(text.to_string()))
                .collect(),
            loc,
        }
    }
}
