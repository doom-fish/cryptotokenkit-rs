use cryptotokenkit::{
    Token, TokenDriver, TokenKeychainCertificate, TokenKeychainEntry, TokenKeychainKey,
};

const SAMPLE_CERT_DER: &[u8] = include_bytes!("fixtures/sample-cert.der");
const RSA_KEY_TYPE: &str = "42";

#[test]
fn keychain_entries_round_trip_through_token_configuration(
) -> Result<(), Box<dyn std::error::Error>> {
    let driver = TokenDriver::new();
    let token = Token::new(&driver, "com.example.cryptotokenkit.keychain-test")?;

    let certificate =
        TokenKeychainCertificate::new("cert-1", SAMPLE_CERT_DER.to_vec()).with_label("Certificate");
    let key = TokenKeychainKey::new("key-1", RSA_KEY_TYPE)
        .with_label("Signing key")
        .with_key_size_in_bits(2048);

    token.set_keychain_items(&[
        TokenKeychainEntry::from(certificate.clone()),
        TokenKeychainEntry::from(key.clone()),
    ])?;

    let resolved_key = token.key_for_object_id(&key.item.object_id)?;
    let resolved_certificate = token.certificate_for_object_id(&certificate.item.object_id)?;
    assert_eq!(resolved_key.item.label.as_deref(), Some("Signing key"));
    assert_eq!(resolved_key.key_size_in_bits, 2048);
    assert_eq!(resolved_certificate.data, SAMPLE_CERT_DER);
    Ok(())
}
