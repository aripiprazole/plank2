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
use crate::syntax::id::Identifier;
use crate::syntax::if_branch::IfBranch;
use crate::syntax::lit::Lit;
use crate::syntax::loc::Loc;
use crate::syntax::pat::Pat;
use crate::syntax::path::QualifiedPath;
use crate::syntax::typ::TypRef;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Exp {
    Lit(Lit),
    Group(Box<Exp>),
    Block(Block),
    Sizeof(TypRef, Loc),
    Ref(Box<Exp>, Loc),
    Deref(Box<Exp>, Loc),
    Match {
        subject: Box<Exp>,
        patterns: HashMap<Pat, Exp>,
        loc: Loc,
    },
    If {
        cond: Box<Exp>,
        then_branch: Box<IfBranch>,
        else_branch: Box<Option<IfBranch>>,
        loc: Loc,
    },
    Assign {
        name: Identifier,
        value: Box<Exp>,
        module: Option<QualifiedPath>,
        loc: Loc,
    },
    Set {
        receiver: Box<Exp>,
        property: Identifier,
        value: Box<Exp>,
        loc: Loc,
    },
    Get {
        receiver: Box<Exp>,
        property: Identifier,
        loc: Loc,
    },
    Call {
        callee: Box<Exp>,
        arguments: Vec<Exp>,
        loc: Loc,
    },
    Instance {
        typ: TypRef,
        arguments: HashMap<Identifier, Exp>,
        loc: Loc,
    },
}

impl Exp {
    pub fn loc(&self) -> &Loc {
        match self {
            Exp::Lit(lit) => lit.loc(),
            Exp::Group(exp) => exp.loc(),
            Exp::Block(block) => block.loc(),
            Exp::Sizeof(_, loc) => loc,
            Exp::Ref(_, loc) => loc,
            Exp::Deref(_, loc) => loc,
            Exp::Match { loc, .. } => loc,
            Exp::If { loc, .. } => loc,
            Exp::Assign { loc, .. } => loc,
            Exp::Set { loc, .. } => loc,
            Exp::Get { loc, .. } => loc,
            Exp::Call { loc, .. } => loc,
            Exp::Instance { loc, .. } => loc,
        }
    }
}
