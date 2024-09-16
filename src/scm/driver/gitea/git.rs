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

use crate::{http::Client, scm::git::GitService};
use crate::scm::client::ListOptions;
use crate::scm::git::{Reference, Commit, Tree};

pub struct GiteaGitService {
    pub client: Client,
}

impl GitService for GiteaGitService {
    /// Returns a list of branches for the specified repository.
    /// 
    /// 
    /// 
    fn list_branches(&self, repo: &str, opts: ListOptions) -> anyhow::Result<Vec<Reference>> {
        todo!()
    }

    fn list_tags(&self, repo: &str, opts: ListOptions) -> anyhow::Result<Vec<Reference>> {
        todo!()
    }

    fn find_commit(&self, repo: &str, reference: &str) -> anyhow::Result<Option<Commit>> {
        todo!()
    }

    fn get_tree(&self, repo: &str, tree_sha: &str, recursive: Option<bool>) -> anyhow::Result<Option<Tree>> {
        todo!()
    }
}