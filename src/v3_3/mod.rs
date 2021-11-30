// Copyright 2021 Red Hat, Inc.
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

use serde::{Deserialize, Serialize};

include!("schema.rs");

impl Default for Config {
    fn default() -> Self {
        Self {
            ignition: Ignition {
                version: Some("3.3.0".into()),
                ..Default::default()
            },
            kernel_arguments: None,
            passwd: None,
            storage: None,
            systemd: None,
        }
    }
}
