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

use crate::syntax::attr::Attribute;
use crate::syntax::block::FunctionBody;
use crate::syntax::exp::Exp;
use crate::syntax::id::Identifier;
use crate::syntax::loc::Loc;
use crate::syntax::path::QualifiedPath;
use crate::syntax::typ::TypRef;
use std::collections::HashMap;

#[derive(Debug)]
pub struct EnumDescriptor {
    pub name: Identifier,
    pub variants: Vec<EnumVariantDescriptor>,
    pub abstractions: Vec<Identifier>,
    pub loc: Loc,
}

#[derive(Debug)]
pub struct EnumVariantDescriptor {
    pub name: Identifier,
    pub parameters: Vec<TypRef>,
    pub loc: Loc,
}

#[derive(Debug)]
pub struct StructDescriptor {
    pub name: Identifier,
    pub properties: Vec<StructPropertyDescriptor>,
    pub abstractions: Vec<Identifier>,
    pub loc: Loc,
}

#[derive(Debug)]
pub struct StructPropertyDescriptor {
    pub mutable: bool,
    pub name: Identifier,
    pub typ: TypRef,
    pub loc: Loc,
}

#[derive(Debug)]
pub struct ModuleDescriptor {
    pub path: QualifiedPath,
    pub content: Vec<Decl>,
    pub loc: Loc,
}

#[derive(Debug)]
pub struct FunctionDescriptor {
    pub name: Identifier,
    pub parameters: HashMap<Identifier, TypRef>,
    pub return_typ: Option<TypRef>,
    pub body: FunctionBody,
    pub attributes: Vec<Attribute>,
    pub loc: Loc,
}

#[derive(Debug)]
pub struct LetDescriptor {
    pub name: Identifier,
    pub value: Exp,
    pub typ: Option<TypRef>,
    pub mutable: bool,
    pub loc: Loc,
}

#[derive(Debug)]
pub enum Decl {
    Enum(EnumDescriptor),
    Struct(StructDescriptor),
    Module(ModuleDescriptor),
    Function(FunctionDescriptor),
    Let(LetDescriptor),
}

impl Decl {
    pub fn loc(&self) -> &Loc {
        match self {
            Decl::Enum(EnumDescriptor { loc, .. }) => loc,
            Decl::Struct(StructDescriptor { loc, .. }) => loc,
            Decl::Module(ModuleDescriptor { loc, .. }) => loc,
            Decl::Function(FunctionDescriptor { loc, .. }) => loc,
            Decl::Let(LetDescriptor { loc, .. }) => loc,
        }
    }
}
