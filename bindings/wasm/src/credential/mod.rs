// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::module_inception)]

pub use self::credential::WasmCredential;
pub use self::credential_builder::*;
pub use self::credential_validator::WasmCredentialValidator;
pub use self::domain_linkage_configuration::WasmDomainLinkageConfiguration;
pub use self::jws::WasmJws;
pub use self::jwt::WasmJwt;
pub use self::jwt_credential_validation::*;
pub use self::presentation::WasmPresentation;
pub use self::presentation_builder::*;
pub use self::presentation_validator::WasmPresentationValidator;
pub use self::types::*;
pub use self::validation_options::WasmCredentialValidationOptions;
pub use self::validation_options::WasmFailFast;
pub use self::validation_options::WasmPresentationValidationOptions;
pub use self::validation_options::WasmSubjectHolderRelationship;

mod credential;
mod credential_builder;
mod credential_validator;
mod domain_linkage_configuration;
mod domain_linkage_credential_builder;
mod domain_linkage_validator;
mod jws;
mod jwt;
mod jwt_credential_validation;
mod linked_domain_service;
mod presentation;
mod presentation_builder;
mod presentation_validator;
mod types;
mod validation_options;
