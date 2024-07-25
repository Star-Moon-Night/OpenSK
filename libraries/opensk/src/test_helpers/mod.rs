// Copyright 2022-2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::api::persist::{Attestation, AttestationId, Persist};
use crate::ctap::command::{AuthenticatorConfigParameters, Command};
use crate::ctap::data_formats::ConfigSubCommand;
use crate::ctap::secret::Secret;
use crate::ctap::status_code::CtapResult;
use crate::ctap::{Channel, CtapState};
use crate::env::Env;

// In tests where we define a dummy user-presence check that immediately returns, the channel
// ID is irrelevant, so we pass this (dummy but valid) value.
const DUMMY_CHANNEL: Channel = Channel::MainHid([0x12, 0x34, 0x56, 0x78]);

pub fn enable_enterprise_attestation<E: Env>(
    state: &mut CtapState<E>,
    env: &mut E,
) -> CtapResult<Attestation> {
    let attestation = Attestation {
        private_key: Secret::from_exposed_secret([0x41; 32]),
        certificate: vec![0xdd; 20],
    };
    env.persist()
        .set_attestation(AttestationId::Enterprise, Some(&attestation))?;

    let config_params = AuthenticatorConfigParameters {
        sub_command: ConfigSubCommand::EnableEnterpriseAttestation,
        sub_command_params: None,
        pin_uv_auth_param: None,
        pin_uv_auth_protocol: None,
    };
    let config_command = Command::AuthenticatorConfig(config_params);
    state.process_parsed_command(env, config_command, DUMMY_CHANNEL)?;

    Ok(attestation)
}
