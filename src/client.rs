use serde::*;
use crate::utils::*;
use std::ffi::*;
use botan::{base64_encode, base64_decode};

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    what: String,
    signature: String,
    content: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    signature: String,
    boxed_content: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Insertion {
    pub name: String,
    pub content: String
}

pub fn query(what: String, content: String, password: &CStr) -> Vec<String> {
    let pubkey = botan::Pubkey::load_pem(crate::SERVER_PUBLIC).unwrap();
    let rng = botan::RandomNumberGenerator::new().unwrap();
    let key = get_private(password);
    if key.is_none() { return vec![String::from("PASSWORD ERROR!")]; }
    let prikey = key.unwrap();
    let signer = botan::Signer::new(&prikey, "PKCS1v15(SHA-256)").unwrap();
    signer.update(what.as_bytes()).unwrap();
    let signature = signer.finish(&rng).and_then(|x| base64_encode(x.as_slice())).unwrap();
    let encrypter = botan::Encryptor::new(&pubkey, "OAEP(SHA-256)").unwrap();
    let content = encrypter.encrypt(content.as_bytes(), &rng)
        .and_then(|x| base64_encode(x.as_slice())).unwrap();
    let message = Request {
        what: what.clone(),
        signature,
        content
    };
    let client = reqwest::blocking::Client::new();
    client.post(SERVER)
        .json(&message)
        .send()
        .ok()
        .and_then(|x| x.json::<Response>().ok())
        .and_then(|x| {
            let verifer = botan::Verifier::new(&pubkey, "PKCS1v15(SHA-256)")
                .unwrap();
            verifer.update(what.as_bytes()).unwrap();
            let sign = base64_decode(x.signature.as_str()).unwrap();
            if let Ok(true) = verifer.finish(sign.as_slice()) {
                Some(x.boxed_content)
            } else {
                None
            }
        }).map(|x| {
            let decrypter = botan::Decryptor::new(&prikey, "OAEP(SHA-256)")
                .unwrap();
            x.iter()
                .map(|x| base64_decode(x.as_str()).unwrap())
                .map(|x|decrypter.decrypt(x.as_slice()).unwrap())
                .map(|x|String::from_utf8(x).unwrap())
                .collect()
        }).unwrap_or(vec![String::from("QUERY ERROR!")])
}
