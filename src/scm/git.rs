// Copyright (c) The Amphitheatre Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::Deserialize;
use super::client::ListOptions;

/// Represents a git reference.
#[derive(Debug, PartialEq)]
pub struct Reference {
    pub name: String,
    pub path: String,
    pub sha: String,
}

/// Represents a repository commit.
#[derive(Debug, Default, PartialEq)]
pub struct Commit {
    pub sha: String,
    pub message: String,
    pub author: Signature,
    pub committer: Signature,
    pub link: String,
}

#[derive(Debug, Default, PartialEq)]
pub struct Signature {
    pub name: String,
    pub email: String,
    pub date: String,
    // Fields are optional. The provider may choose to
    // include account information in the response.
    pub login: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct TreeResponse {
    sha: String,
    url: String,
    tree: Vec<TreeEntry>,
    truncated: bool,
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct TreeEntry {
    path: String,
    mode: String,
    #[serde(rename = "type")]
    entry_type: String,
    size: Option<u64>,
    // Some entries have a "size" field, but it might be absent
    sha: String,
    url: String,
}
/// Provides access to git resources.
pub trait GitService {
    /// Returns a list of git branches.
    fn list_branches(&self, repo: &str, opts: ListOptions) -> anyhow::Result<Vec<Reference>>;

    /// Returns a list of git tags.
    fn list_tags(&self, repo: &str, opts: ListOptions) -> anyhow::Result<Vec<Reference>>;

    /// Finds a git commit by reference
    fn find_commit(&self, repo: &str, reference: &str) -> anyhow::Result<Option<Commit>>;

    /// Returns a single tree using the SHA1 value or ref name for that tree.
    fn git_trees(&self, repo: &str, tree_sha: &str, recursive: &str) -> anyhow::Result<Option<TreeResponse>>;
}
