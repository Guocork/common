// Copyright 2023 The Amphitheatre Authors.
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

use std::collections::HashMap;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine as _;

use crate::config::Credential;
use crate::docker::{AuthConfig, DockerConfig};

/// Build a configuration that conforms to the `.dockerconfigjson` specification.
pub fn build_docker_config(entries: &HashMap<String, Credential>) -> DockerConfig {
    let mut auths = HashMap::new();

    for (endpoint, credential) in entries.iter() {
        let auth = BASE64.encode(format!(
            "{}:{}",
            credential.username_any(),
            credential.password_any()
        ));
        auths.insert(
            endpoint.clone(),
            AuthConfig {
                username: Some(credential.username_any()),
                password: Some(credential.password_any()),
                auth: Some(auth),
            },
        );
    }

    DockerConfig { auths: Some(auths) }
}
