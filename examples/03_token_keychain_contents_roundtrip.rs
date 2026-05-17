use cryptotokenkit::{
    Token, TokenDriver, TokenKeychainCertificate, TokenKeychainEntry, TokenKeychainKey,
};

const SAMPLE_CERT_DER: &[u8] = include_bytes!("../tests/fixtures/sample-cert.der");
const RSA_KEY_TYPE: &str = "42";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let driver = TokenDriver::new();
    let token = Token::new(&driver, "com.example.cryptotokenkit.keychain")?;

    let certificate = TokenKeychainCertificate::new("cert-1", SAMPLE_CERT_DER.to_vec())
        .with_label("Example certificate");
    let key = TokenKeychainKey::new("key-1", RSA_KEY_TYPE)
        .with_label("Example key")
        .with_key_size_in_bits(2048);

    token.set_keychain_items(&[
        TokenKeychainEntry::from(certificate.clone()),
        TokenKeychainEntry::from(key.clone()),
    ])?;

    let resolved_key = token.key_for_object_id(&key.item.object_id)?;
    let resolved_certificate = token.certificate_for_object_id(&certificate.item.object_id)?;
    println!(
        "key: {} bits={}",
        resolved_key.item.object_id.0, resolved_key.key_size_in_bits
    );
    println!(
        "certificate: {} bytes={}",
        resolved_certificate.item.object_id.0,
        resolved_certificate.data.len()
    );
    println!("✅ token keychain contents OK");
    Ok(())
}
