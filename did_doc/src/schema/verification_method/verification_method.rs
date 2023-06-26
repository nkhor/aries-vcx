use did_parser::{Did, DidUrl};
use serde::{Deserialize, Serialize};

use crate::schema::types::jsonwebkey::JsonWebKey;

use super::{public_key::PublicKeyField, VerificationMethodType};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VerificationMethod {
    id: DidUrl,
    controller: Did,
    #[serde(rename = "type")]
    verification_method_type: VerificationMethodType,
    #[serde(flatten)]
    public_key: PublicKeyField,
}

impl VerificationMethod {
    pub fn builder(
        id: DidUrl,
        controller: Did,
        verification_method_type: VerificationMethodType,
    ) -> IncompleteVerificationMethodBuilder {
        IncompleteVerificationMethodBuilder::new(id, controller, verification_method_type)
    }

    pub fn id(&self) -> &DidUrl {
        &self.id
    }

    pub fn controller(&self) -> &Did {
        &self.controller
    }

    pub fn verification_method_type(&self) -> &VerificationMethodType {
        &self.verification_method_type
    }

    pub fn public_key(&self) -> &PublicKeyField {
        &self.public_key
    }
}

#[derive(Debug, Clone)]
pub struct IncompleteVerificationMethodBuilder {
    id: DidUrl,
    controller: Did,
    verification_method_type: VerificationMethodType,
}

#[derive(Debug, Clone)]
pub struct CompleteVerificationMethodBuilder {
    id: DidUrl,
    controller: Did,
    verification_method_type: VerificationMethodType,
    public_key: Option<PublicKeyField>,
}

impl IncompleteVerificationMethodBuilder {
    pub fn new(
        id: DidUrl,
        controller: Did,
        verification_method_type: VerificationMethodType,
    ) -> Self {
        Self {
            id,
            verification_method_type,
            controller,
        }
    }

    pub fn add_public_key_multibase(
        self,
        public_key_multibase: String,
    ) -> CompleteVerificationMethodBuilder {
        CompleteVerificationMethodBuilder {
            id: self.id,
            controller: self.controller,
            verification_method_type: self.verification_method_type,
            public_key: Some(PublicKeyField::Multibase {
                public_key_multibase,
            }),
        }
    }

    pub fn add_public_key_jwk(
        self,
        public_key_jwk: JsonWebKey,
    ) -> CompleteVerificationMethodBuilder {
        CompleteVerificationMethodBuilder {
            id: self.id,
            controller: self.controller,
            verification_method_type: self.verification_method_type,
            public_key: Some(PublicKeyField::Jwk { public_key_jwk }),
        }
    }

    pub fn add_public_key_base58(
        self,
        public_key_base58: String,
    ) -> CompleteVerificationMethodBuilder {
        CompleteVerificationMethodBuilder {
            id: self.id,
            controller: self.controller,
            verification_method_type: self.verification_method_type,
            public_key: Some(PublicKeyField::Base58 { public_key_base58 }),
        }
    }

    pub fn add_public_key_base64(
        self,
        public_key_base64: String,
    ) -> CompleteVerificationMethodBuilder {
        CompleteVerificationMethodBuilder {
            id: self.id,
            controller: self.controller,
            verification_method_type: self.verification_method_type,
            public_key: Some(PublicKeyField::Base64 { public_key_base64 }),
        }
    }

    pub fn add_public_key_hex(self, public_key_hex: String) -> CompleteVerificationMethodBuilder {
        CompleteVerificationMethodBuilder {
            id: self.id,
            controller: self.controller,
            verification_method_type: self.verification_method_type,
            public_key: Some(PublicKeyField::Hex { public_key_hex }),
        }
    }

    pub fn add_public_key_pem(self, public_key_pem: String) -> CompleteVerificationMethodBuilder {
        CompleteVerificationMethodBuilder {
            id: self.id,
            controller: self.controller,
            verification_method_type: self.verification_method_type,
            public_key: Some(PublicKeyField::Pem { public_key_pem }),
        }
    }
}

impl CompleteVerificationMethodBuilder {
    pub fn build(self) -> VerificationMethod {
        VerificationMethod {
            id: self.id,
            controller: self.controller,
            verification_method_type: self.verification_method_type,
            public_key: self.public_key.unwrap(), // SAFETY: The builder will always set the public key
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn create_valid_did() -> Did {
        Did::parse("did:example:123456789abcdefghi".to_string()).unwrap()
    }

    fn create_valid_did_url() -> DidUrl {
        DidUrl::parse("did:example:123456789abcdefghi#fragment".to_string()).unwrap()
    }

    fn create_valid_multibase() -> String {
        "zQmWvQxTqbG2Z9HPJgG57jjwR154cKhbtJenbyYTWkjgF3e".to_string()
    }

    fn create_valid_verification_key_type() -> VerificationMethodType {
        VerificationMethodType::Ed25519VerificationKey2018
    }

    fn create_valid_verification_method_value() -> Value {
        serde_json::json!({
            "id": "did:example:123456789abcdefghi#key-1",
            "type": "Ed25519VerificationKey2018",
            "controller": "did:example:123456789abcdefghi",
            "publicKeyMultibase": "zQmWvQxTqbG2Z9HPJgG57jjwR154cKhbtJenbyYTWkjgF3e"
        })
    }

    fn create_verification_method_multiple_keys() -> Value {
        serde_json::json!({
            "id": "did:example:123456789abcdefghi#key-1",
            "type": "Ed25519VerificationKey2018",
            "controller": "did:example:123456789abcdefghi",
            "publicKeyMultibase": "zQmWvQxTqbG2Z9HPJgG57jjwR154cKhbtJenbyYTWkjgF3e",
            "publicKeyJwk": {
                "kty": "OKP",
                "crv": "Ed25519",
                "x": "zQmWvQxTqbG2Z9HPJgG57jjwR154cKhbtJenbyYTWkjgF3e"
            }
        })
    }

    #[test]
    fn test_verification_method_id() {
        let id = create_valid_did_url();
        let controller = create_valid_did();
        let verification_method_type = create_valid_verification_key_type();
        let verification_method =
            VerificationMethod::builder(id.clone(), controller, verification_method_type)
                .add_public_key_multibase(create_valid_multibase())
                .build();
        assert_eq!(verification_method.id(), &id);
    }

    #[test]
    fn test_verification_method_builder() {
        let id = create_valid_did_url();
        let controller = create_valid_did();
        let verification_method_type = create_valid_verification_key_type();
        let public_key_multibase = create_valid_multibase();

        let vm = VerificationMethod::builder(
            id.clone(),
            controller.clone(),
            verification_method_type.clone(),
        )
        .add_public_key_multibase(public_key_multibase.clone())
        .build();

        assert_eq!(vm.id(), &id);
        assert_eq!(vm.controller(), &controller);
        assert_eq!(vm.verification_method_type(), &verification_method_type);
        match vm.public_key() {
            PublicKeyField::Multibase {
                public_key_multibase,
            } => {
                assert_eq!(public_key_multibase, public_key_multibase)
            }
            _ => panic!("Expected public key to be multibase"),
        }
    }

    #[test]
    fn test_verification_method_builder_complete() {
        let id = create_valid_did_url();
        let controller = create_valid_did();
        let verification_method_type = create_valid_verification_key_type();
        let public_key_multibase = create_valid_multibase();

        let vm = VerificationMethod::builder(
            id.clone(),
            controller.clone(),
            verification_method_type.clone(),
        )
        .add_public_key_multibase(public_key_multibase.clone())
        .build();

        assert_eq!(vm.id(), &id);
        assert_eq!(vm.controller(), &controller);
        assert_eq!(vm.verification_method_type(), &verification_method_type);
        match vm.public_key() {
            PublicKeyField::Multibase {
                public_key_multibase,
            } => {
                assert_eq!(public_key_multibase, public_key_multibase)
            }
            _ => panic!("Expected public key to be multibase"),
        }
    }

    #[test]
    fn test_verification_method_deserialization() {
        let vm: Result<VerificationMethod, _> = serde_json::from_str(
            create_valid_verification_method_value()
                .to_string()
                .as_str(),
        );
        assert!(vm.is_ok());
    }

    #[test]
    fn test_verification_method_deserialization_fails_with_multiple_keys() {
        let vm: Result<VerificationMethod, _> = serde_json::from_str(
            create_verification_method_multiple_keys()
                .to_string()
                .as_str(),
        );
        assert!(vm.is_err());
    }
}