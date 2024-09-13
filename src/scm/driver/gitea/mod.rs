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

pub mod constants;
pub mod content;
pub mod driver;
pub mod git;
pub mod pr;
pub mod repo;
mod utils;

use self::constants::GITEA_ENDPOINT;
use driver::GiteaDriver;

use super::Driver;
use crate::http::Client;

/// Returns a new Gitea driver using the default xxxxxxx address.
#[inline]
pub fn default() -> Driver {
    from(Client::new(GITEA_ENDPOINT, None))
}

/// Returns a new Gitea driver.
#[inline]
pub fn new(url: &str, token: Option<String>) -> Driver {
    from(Client::new(url, token))
}

/// Returns a new Gitea driver using the given client.
pub fn from(client: Client) -> Driver {
    Driver::Gitea(GiteaDriver { client })
}