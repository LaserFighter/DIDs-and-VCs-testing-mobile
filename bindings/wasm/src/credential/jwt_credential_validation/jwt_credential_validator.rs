// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::core::Object;
use identity_iota::core::Url;
use identity_iota::credential::vc_jwt_validation::CredentialValidator as JwtCredentialValidator;
use identity_iota::credential::StatusCheck;
use identity_iota::did::CoreDID;

use super::options::WasmJwtCredentialValidationOptions;
use crate::common::ImportedDocumentLock;
use crate::common::ImportedDocumentReadGuard;
use crate::common::WasmTimestamp;
use crate::credential::validation_options::WasmStatusCheck;
use crate::credential::WasmCredential;
use crate::credential::WasmDecodedJwtCredential;
use crate::credential::WasmFailFast;
use crate::credential::WasmJwt;
use crate::credential::WasmSubjectHolderRelationship;
use crate::did::ArrayIToCoreDocument;
use crate::did::IToCoreDocument;
use crate::did::WasmCoreDID;
use crate::did::WasmJwsVerificationOptions;
use crate::error::Result;
use crate::error::WasmResult;
use crate::verification::IJwsVerifier;
use crate::verification::WasmJwsVerifier;

use wasm_bindgen::prelude::*;

/// A type for decoding and validating `Credentials`.
// NOTE: The methods that take `&Credential` have been copied over from the old `CredentialValidator`. The old
// `CredentialValidator` either needs to be updated or removed.
#[wasm_bindgen(js_name = JwtCredentialValidator)]
pub struct WasmJwtCredentialValidator(JwtCredentialValidator<WasmJwsVerifier>);

#[wasm_bindgen(js_class = JwtCredentialValidator)]
impl WasmJwtCredentialValidator {
  /// Creates a new `JwtCredentialValidator`. If a `signature_verifier` is provided it will be used when
  /// verifying decoded JWS signatures, otherwise the default which is only capable of handling the `EdDSA`
  /// algorithm will be used.
  #[wasm_bindgen(constructor)]
  pub fn new(signature_verifier: Option<IJwsVerifier>) -> WasmJwtCredentialValidator {
    let signature_verifier = WasmJwsVerifier::new(signature_verifier);
    WasmJwtCredentialValidator(JwtCredentialValidator::with_signature_verifier(signature_verifier))
  }

  /// Decodes and validates a `Credential` issued as a JWS. A `DecodedJwtCredential` is returned upon success.
  ///
  /// The following properties are validated according to `options`:
  /// - the issuer's signature on the JWS,
  /// - the expiration date,
  /// - the issuance date,
  /// - the semantic structure.
  ///
  /// # Warning
  /// The lack of an error returned from this method is in of itself not enough to conclude that the credential can be
  /// trusted. This section contains more information on additional checks that should be carried out before and after
  /// calling this method.
  ///
  /// ## The state of the issuer's DID Document
  /// The caller must ensure that `issuer` represents an up-to-date DID Document.
  ///
  /// ## Properties that are not validated
  ///  There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated, such as:
  /// `proof`, `credentialStatus`, `type`, `credentialSchema`, `refreshService` **and more**.
  /// These should be manually checked after validation, according to your requirements.
  ///
  /// # Errors
  /// An error is returned whenever a validated condition is not satisfied.
  #[wasm_bindgen]
  pub fn validate(
    &self,
    credential_jwt: &WasmJwt,
    issuer: &IToCoreDocument,
    options: &WasmJwtCredentialValidationOptions,
    fail_fast: WasmFailFast,
  ) -> Result<WasmDecodedJwtCredential> {
    let issuer_lock = ImportedDocumentLock::from(issuer);
    let issuer_guard = issuer_lock.blocking_read();

    self
      .0
      .validate(&credential_jwt.0, &issuer_guard, &options.0, fail_fast.into())
      .wasm_result()
      .map(WasmDecodedJwtCredential)
  }

  /// Decode and verify the JWS signature of a `Credential` issued as a JWT using the DID Document of a trusted
  /// issuer.
  ///
  /// A `DecodedJwtCredential` is returned upon success.
  ///
  /// # Warning
  /// The caller must ensure that the DID Documents of the trusted issuers are up-to-date.
  ///
  /// ## Proofs
  ///  Only the JWS signature is verified. If the `Credential` contains a `proof` property this will not be verified
  /// by this method.
  ///
  /// # Errors
  /// This method immediately returns an error if
  /// the credential issuer' url cannot be parsed to a DID belonging to one of the trusted issuers. Otherwise an attempt
  /// to verify the credential's signature will be made and an error is returned upon failure.
  #[wasm_bindgen(js_name = verifySignature)]
  #[allow(non_snake_case)]
  pub fn verify_signature(
    &self,
    credential: &WasmJwt,
    trustedIssuers: &ArrayIToCoreDocument,
    options: &WasmJwsVerificationOptions,
  ) -> Result<WasmDecodedJwtCredential> {
    let issuer_locks: Vec<ImportedDocumentLock> = trustedIssuers.into();
    let trusted_issuers: Vec<ImportedDocumentReadGuard<'_>> =
      issuer_locks.iter().map(ImportedDocumentLock::blocking_read).collect();
    self
      .0
      .verify_signature(&credential.0, &trusted_issuers, &options.0)
      .wasm_result()
      .map(WasmDecodedJwtCredential)
  }

  /// Validate that the credential expires on or after the specified timestamp.
  #[wasm_bindgen(js_name = checkExpiresOnOrAfter)]
  pub fn check_expires_on_or_after(credential: &WasmCredential, timestamp: &WasmTimestamp) -> Result<()> {
    JwtCredentialValidator::check_expires_on_or_after(&credential.0, timestamp.0).wasm_result()
  }

  /// Validate that the credential is issued on or before the specified timestamp.
  #[wasm_bindgen(js_name = checkIssuedOnOrBefore)]
  pub fn check_issued_on_or_before(credential: &WasmCredential, timestamp: &WasmTimestamp) -> Result<()> {
    JwtCredentialValidator::check_issued_on_or_before(&credential.0, timestamp.0).wasm_result()
  }

  /// Validate that the relationship between the `holder` and the credential subjects is in accordance with
  /// `relationship`. The `holder` parameter is expected to be the URL of the holder.
  #[wasm_bindgen(js_name = checkSubjectHolderRelationship)]
  pub fn check_subject_holder_relationship(
    credential: &WasmCredential,
    holder: &str,
    relationship: WasmSubjectHolderRelationship,
  ) -> Result<()> {
    let holder: Url = Url::parse(holder).wasm_result()?;
    JwtCredentialValidator::check_subject_holder_relationship(&credential.0, &holder, relationship.into()).wasm_result()
  }

  /// Checks whether the credential status has been revoked.
  ///
  /// Only supports `BitmapRevocation2022`.
  #[wasm_bindgen(js_name = checkStatus)]
  #[allow(non_snake_case)]
  pub fn check_status(
    credential: &WasmCredential,
    trustedIssuers: &ArrayIToCoreDocument,
    statusCheck: WasmStatusCheck,
  ) -> Result<()> {
    let issuer_locks: Vec<ImportedDocumentLock> = trustedIssuers.into();
    let trusted_issuers: Vec<ImportedDocumentReadGuard<'_>> =
      issuer_locks.iter().map(ImportedDocumentLock::blocking_read).collect();
    let status_check: StatusCheck = statusCheck.into();
    JwtCredentialValidator::check_status(&credential.0, &trusted_issuers, status_check).wasm_result()
  }

  /// Utility for extracting the issuer field of a `Credential` as a DID.
  ///
  /// ### Errors
  ///
  /// Fails if the issuer field is not a valid DID.
  #[wasm_bindgen(js_name = extractIssuer)]
  pub fn extract_issuer(credential: &WasmCredential) -> Result<WasmCoreDID> {
    JwtCredentialValidator::extract_issuer::<CoreDID, Object>(&credential.0)
      .map(WasmCoreDID::from)
      .wasm_result()
  }
}
