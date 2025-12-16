use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use block_padding::Pkcs7;
use blowfish::Blowfish;
use blowfish::cipher::{BlockDecryptMut, KeyInit};
use ecb::Decryptor;
use swf::avm2::types::Op;
use swf::extensions::ReadSwfExt;
use swf::{SwfBuf, avm2::read::Reader, parse_swf};

const DECRYPT_KEY: &str = "pub1isher1l0O";

pub fn check_process(buf: &SwfBuf) -> Result<Option<String>> {
    let parsed = parse_swf(buf)?;

    for tag in &parsed.tags {
        let swf::Tag::DoAbc2(tag) = tag else { continue };

        let mut reader = Reader::new(tag.data);
        let abc = reader.read()?;

        for body in &abc.method_bodies {
            let mut r = Reader::new(&body.code);

            while !r.as_slice().is_empty() {
                let op = r.read_op()?;

                if let Op::PushString { value } = op {
                    let raw = &abc.constant_pool.strings[value.0 as usize];

                    if raw.len() > 50 {
                        if let Ok(s) = std::str::from_utf8(raw) {
                            let decrypted = KKDecryptor
                                .decrypt(s, DECRYPT_KEY)
                                .map_err(|e| anyhow!(e))?;

                            return Ok(Some(decrypted));
                        }
                    }
                }
            }
        }
    }

    Ok(None)
}

// ============================================================

pub struct KKDecryptor;

impl KKDecryptor {
    pub fn decrypt(&self, data: &str, key: &str) -> Result<String> {
        let transformed = self.fd2_transform(data, key)?;

        let cleaned: String = transformed.chars().filter(|c| !c.is_whitespace()).collect();

        let mut encrypted = BASE64.decode(cleaned)?;

        type BlowfishEcb = Decryptor<Blowfish>;

        let cipher = BlowfishEcb::new_from_slice(key.as_bytes())?;

        let decrypted = cipher
            .decrypt_padded_mut::<Pkcs7>(&mut encrypted)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {:?}", e))?;
        Ok(String::from_utf8_lossy(decrypted).into_owned())
    }

    fn fd2_transform(&self, input: &str, key: &str) -> Result<String> {
        let reversed: String = input.chars().rev().collect();
        let cleaned: String = reversed.chars().filter(|c| !c.is_whitespace()).collect();

        let decoded = BASE64.decode(cleaned)?;

        let xored = self.apply_xor(&decoded, key);
        Ok(String::from_utf8_lossy(&xored).into_owned())
    }

    fn apply_xor(&self, data: &[u8], key: &str) -> Vec<u8> {
        let key = key.as_bytes();
        let key_len = key.len();

        data.iter()
            .enumerate()
            .map(|(i, &b)| b ^ key[i % key_len])
            .collect()
    }
}
