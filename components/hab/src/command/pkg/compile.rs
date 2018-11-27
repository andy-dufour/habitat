// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::Path;

use common::ui::{UIWriter, UI};
use hcore::package::{PackageIdent, PackageInstall};
use hcore::templating;

use error::Result;

pub fn start(ui: &mut UI, ident: &PackageIdent, fs_root_path: &Path) -> Result<()> {
    let pkg_install = PackageInstall::load(ident, Some(fs_root_path))?;
    templating::compile_from_package_install(&pkg_install)?;
    ui.end(format!("Compiling templates for '{}' completed", ident,))?;
    Ok(())
}