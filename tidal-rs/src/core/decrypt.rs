use aes::cipher::{
    KeyIvInit,
    StreamCipher,
};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;

use crate::core::AppResult;

type Aes128Ctr = ctr::Ctr128BE<aes::Aes128>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

const MASTER_KEY: &str = "UIlTTEMmmLfGowo/UC60x2H45W6MdGgTRfo/umg4754=";

pub struct DecryptionKey {
    pub key: [u8; 16],
    pub nonce: [u8; 8],
}

pub fn decrypt_key_id(key_id: &str) -> AppResult<DecryptionKey> {
    use aes::cipher::BlockDecryptMut;

    let master_key = BASE64.decode(MASTER_KEY)?;
    let key_id_bytes = BASE64.decode(key_id)?;

    let iv: [u8; 16] = key_id_bytes[..16].try_into()?;
    let encrypted = &key_id_bytes[16..];

    let mut decrypted = encrypted.to_vec();
    let decryptor = Aes256CbcDec::new_from_slices(&master_key, &iv)
        .map_err(|e| format!("Invalid key/iv length: {:?}", e))?;
    decryptor
        .decrypt_padded_mut::<aes::cipher::block_padding::NoPadding>(&mut decrypted)
        .map_err(|e| format!("Decryption failed: {:?}", e))?;

    let key: [u8; 16] = decrypted[..16].try_into()?;
    let nonce: [u8; 8] = decrypted[16..24].try_into()?;

    Ok(DecryptionKey { key, nonce })
}

pub struct StreamDecryptor {
    cipher: Aes128Ctr,
}

impl std::fmt::Debug for StreamDecryptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamDecryptor").finish_non_exhaustive()
    }
}

impl StreamDecryptor {
    pub fn new(decryption_key: &DecryptionKey) -> Self {
        let mut iv = [0u8; 16];
        iv[..8].copy_from_slice(&decryption_key.nonce);
        let cipher = Aes128Ctr::new(&decryption_key.key.into(), &iv.into());
        Self { cipher }
    }

    pub fn decrypt(&mut self, data: &mut [u8]) {
        self.cipher.apply_keystream(data);
    }
}
