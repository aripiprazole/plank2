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

use crate::syntax::block::Block;
use crate::syntax::exp::Exp;
use crate::syntax::loc::Loc;

#[derive(Debug)]
pub enum IfBranch {
    Then(Exp, Loc),
    Block(Block),
}

impl IfBranch {
    pub fn loc(&self) -> &Loc {
        match self {
            IfBranch::Then(_, loc) => loc,
            IfBranch::Block(block) => block.loc(),
        }
    }
}
