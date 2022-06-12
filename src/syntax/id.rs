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
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq)]
pub struct Identifier {
    pub text: String,
    pub loc: Loc,
}

impl Identifier {
    pub fn new(text: String, loc: Loc) -> Self {
        Identifier { text, loc }
    }

    pub fn from_string(text: String) -> Identifier {
        Identifier {
            text,
            loc: Loc::Generated,
        }
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text.hash(state)
    }
}
