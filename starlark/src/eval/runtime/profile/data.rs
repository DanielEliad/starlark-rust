/*
 * Copyright 2019 The Starlark in Rust Authors.
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::fs;
use std::path::Path;

use anyhow::Context;

/// Collected profiling data.
#[derive(Clone, Debug)]
pub struct ProfileData {
    /// Serialized to text (e.g. CSV or flamegraph).
    profile: String,
}

impl ProfileData {
    pub(crate) fn new(profile: String) -> ProfileData {
        ProfileData { profile }
    }

    /// Generate a string with profile data (e.g. CSV or flamegraph, depending on profile type).
    pub fn gen(&self) -> String {
        self.profile.clone()
    }

    /// Write to a file.
    pub fn write(&self, path: &Path) -> anyhow::Result<()> {
        fs::write(path, &self.profile)
            .with_context(|| format!("write profile data to `{}`", path.display()))?;
        Ok(())
    }
}
