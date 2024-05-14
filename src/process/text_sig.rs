use std::collections::HashMap;
use std::io::Read;

use anyhow::Result;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::{process_passwd_gen, TextSignFormat};

pub trait TextSigner {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

pub struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;

        Ok(Self { key })
    }

    pub fn ken_gen() -> Result<HashMap<&'static str, Vec<u8>>> {
        let key = process_passwd_gen(32, false, false, false, false)?;
        let mut map = HashMap::new();

        map.insert("blake3.txt", key.as_bytes().to_vec());
        Ok(map)
    }
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let sig = blake3::keyed_hash(&self.key, &buf);
        Ok(sig.as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let new_sig = blake3::keyed_hash(&self.key, &buf);
        Ok(new_sig.as_bytes() == sig)
    }
}

pub struct Ed25519Signer {
    key: SigningKey,
}

impl Ed25519Signer {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        let key = SigningKey::from_bytes(key);

        Ok(Self { key })
    }

    fn key_gen() -> Result<HashMap<&'static str, Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let verifying_key: VerifyingKey = (&signing_key).into();

        let mut map = HashMap::new();

        map.insert("ed25519.pri", signing_key.to_bytes().to_vec());
        map.insert("ed25519.pub", verifying_key.to_bytes().to_vec());

        Ok(map)
    }
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl Ed25519Verifier {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        let key = VerifyingKey::from_bytes(key)?;

        Ok(Self { key })
    }
}

impl TextVerifier for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let sig = (&sig[..64]).try_into()?;
        let sig = Signature::from_bytes(sig);

        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

pub fn process_text_sign(
    reader: &mut dyn Read,
    key: &[u8],
    format: TextSignFormat,
) -> Result<Vec<u8>> {
    let signer: Box<dyn TextSigner> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
    };

    signer.sign(reader)
}

pub fn process_text_verify(
    reader: &mut dyn Read,
    key: &[u8],
    sig: &[u8],
    format: TextSignFormat,
) -> Result<bool> {
    let verifier: Box<dyn TextVerifier> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Verifier::try_new(key)?),
    };

    verifier.verify(reader, sig)
}

pub fn process_key_gen(format: TextSignFormat) -> Result<HashMap<&'static str, Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::ken_gen(),
        TextSignFormat::Ed25519 => Ed25519Signer::key_gen(),
    }
}
