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

use crate::parsing::tok::Tok;
use crate::syntax::decl::Decl;
use crate::syntax::file::PlankFile;
use crate::syntax::loc::Loc;
use crate::syntax::path::QualifiedPath;

// fn decl_parser() -> impl Parser<Tok, Decl, Error = Simple<Tok>> {
//     todo!()
// }

// pub fn parser() -> impl Parser<Tok, PlankFile, Error = Simple<Tok>> {
//     just::<_, _, Simple<Tok>>(Tok::Eof(Loc::Generated)).map(|_| PlankFile {
//         content: vec![],
//         path: QualifiedPath::new("main.Main".into(), Loc::Generated),
//         text: "".into(),
//     })
// }
