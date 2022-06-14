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

use crate::syntax::exp::Exp;
use crate::syntax::loc::Loc;
use crate::syntax::stmt::Stmt;

#[derive(Debug)]
pub struct Block {
    pub value: Option<Box<Exp>>,
    pub stmts: Vec<Stmt>,
    pub loc: Loc,
}

impl Block {
    pub fn loc(&self) -> &Loc {
        &self.loc
    }
}

#[derive(Debug)]
pub enum FunctionBody {
    Empty(Loc),
    Value(Exp, Loc),
    Block(Block),
}

impl FunctionBody {
    pub fn loc(&self) -> &Loc {
        match self {
            FunctionBody::Empty(loc) => loc,
            FunctionBody::Value(_, loc) => loc,
            FunctionBody::Block(block) => block.loc(),
        }
    }
}
