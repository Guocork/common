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
use std::fmt::Display;

use convert_case::{Case, Casing};
use k8s_openapi::api::core::v1::{ContainerPort, EnvVar, ServicePort};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{Condition, Time};
use k8s_openapi::chrono::Utc;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::build::Build;
use super::service::Service;
use super::source::Source;
use super::Manifest;
use crate::utils::kubernetes::to_env_var;

#[derive(Clone, CustomResource, Debug, Default, Deserialize, Serialize, JsonSchema, Validate, PartialEq)]
#[kube(
    group = "amphitheatre.app",
    version = "v1",
    kind = "Actor",
    status = "ActorStatus",
    namespaced
)]
pub struct ActorSpec {
    /// The name of the actor.
    pub name: String,

    /// The description of the actor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The source of the actor.
    pub source: Source,

    /// Specifies the image to launch the container. The image must follow
    /// the Open Container Specification addressable image format.
    /// such as: [<registry>/][<project>/]<image>[:<tag>|@<digest>].
    pub image: String,

    /// overrides the default command declared by the container image
    /// (i.e. by Dockerfile’s CMD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    /// Defines environment variables set in the container. Any boolean values:
    /// true, false, yes, no, SHOULD be enclosed in quotes to ensure they are
    /// not converted to True or False by the YAML parser.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<HashMap<String, String>>,

    /// Depend on other partners from other repositories, or subdirectories on
    /// your local file system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partners: Option<HashMap<String, Source>>,

    /// Defines the behavior of a service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,

    /// sync mode, if enabled, pulls the latest code from source version
    /// control in real time via Webhook, etc. and then rebuilds and deploys it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,

    /// Describes how images are built.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
}

/// Helpers for building jobs.
impl ActorSpec {
    #[inline]
    pub fn build_name(&self) -> String {
        format!("{}-{}", self.name, self.source.rev())
    }

    #[inline]
    pub fn docker_tag(&self) -> String {
        format!("{}:{}", self.image, self.source.rev())
    }
}

/// Helpers for Kubernetes resources.
impl ActorSpec {
    pub fn environments(&self) -> Option<Vec<EnvVar>> {
        self.environments.as_ref().map(to_env_var)
    }

    pub fn container_ports(&self) -> Option<Vec<ContainerPort>> {
        let services = self.services.as_ref()?;
        let mut ports: Vec<ContainerPort> = vec![];

        for service in services {
            let mut items = service.ports.iter().map(|p| p.into()).collect();
            ports.append(&mut items);
        }

        Some(ports).filter(|v| v.is_empty())
    }

    pub fn service_ports(&self) -> Option<Vec<ServicePort>> {
        let services = self.services.as_ref()?;
        let mut ports: Vec<ServicePort> = vec![];

        for service in services {
            let mut items = service
                .ports
                .iter()
                .filter(|p| p.expose.unwrap_or_default())
                .map(|p| p.into())
                .collect();
            ports.append(&mut items);
        }

        Some(ports).filter(|v| v.is_empty())
    }
}

/// Helpers for building.
impl ActorSpec {
    #[inline]
    pub fn has_dockerfile(&self) -> bool {
        self.build.is_some() && self.build.as_ref().unwrap().dockerfile.is_some()
    }

    pub fn dockerfile(&self) -> String {
        if let Some(build) = &self.build {
            if let Some(dockerfile) = &build.dockerfile {
                return dockerfile.clone();
            }
        }

        String::from("Dockerfile")
    }

    pub fn builder(&self) -> String {
        if let Some(build) = &self.build {
            if let Some(builder) = &build.builder {
                return builder.clone();
            }
        }

        String::from("amp-default-cluster-builder")
    }

    pub fn context(&self) -> String {
        if let Some(build) = &self.build {
            if let Some(context) = &build.context {
                return context.clone();
            }
        }

        String::from("")
    }

    pub fn build_env(&self) -> Option<Vec<EnvVar>> {
        if let Some(build) = &self.build {
            return build.environments.as_ref().map(to_env_var);
        }

        None
    }
}

impl From<&Manifest> for ActorSpec {
    fn from(manifest: &Manifest) -> Self {
        Self {
            name: manifest.character.name.clone(),
            description: manifest.character.description.clone(),
            source: Source::new(manifest.character.repository.clone()),
            image: manifest.character.image.clone().unwrap_or_default(),
            command: manifest.character.command.clone(),
            environments: manifest.environments.clone(),
            partners: manifest.partners.clone(),
            services: manifest.services.clone(),
            sync: None,
            build: manifest.build.clone(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, PartialEq)]
pub struct ActorStatus {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    conditions: Vec<Condition>,
}

impl ActorStatus {
    pub fn pending(&self) -> bool {
        self.state(ActorState::Pending, true)
    }

    pub fn building(&self) -> bool {
        self.state(ActorState::Building, true)
    }

    pub fn running(&self) -> bool {
        self.state(ActorState::Running, true)
    }

    pub fn failed(&self) -> bool {
        self.state(ActorState::Failed, true)
    }

    fn state(&self, s: ActorState, status: bool) -> bool {
        self.conditions.iter().any(|condition| {
            condition.type_ == s.to_string() && condition.status == status.to_string().to_case(Case::Pascal)
        })
    }
}

pub enum ActorState {
    Pending,
    Building,
    Running,
    Failed,
}

impl ActorState {
    pub fn pending() -> Condition {
        ActorState::create(ActorState::Pending, true, "Created", None)
    }

    pub fn building() -> Condition {
        ActorState::create(ActorState::Building, true, "Build", None)
    }

    pub fn running(status: bool, reason: &str, message: Option<String>) -> Condition {
        ActorState::create(ActorState::Running, status, reason, message)
    }

    pub fn failed(status: bool, reason: &str, message: Option<String>) -> Condition {
        ActorState::create(ActorState::Failed, status, reason, message)
    }

    #[inline]
    fn create(state: ActorState, status: bool, reason: &str, message: Option<String>) -> Condition {
        Condition {
            type_: state.to_string(),
            status: status.to_string().to_case(Case::Pascal),
            last_transition_time: Time(Utc::now()),
            reason: reason.to_case(Case::Pascal),
            observed_generation: None,
            message: match message {
                Some(message) => message,
                None => "".to_string(),
            },
        }
    }
}

impl Display for ActorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActorState::Pending => f.write_str("Pending"),
            ActorState::Building => f.write_str("Building"),
            ActorState::Running => f.write_str("Running"),
            ActorState::Failed => f.write_str("Failed"),
        }
    }
}
