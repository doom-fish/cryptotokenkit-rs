use cryptotokenkit::{TKErrorCode, TK_ERROR_DOMAIN, TlvEncoding, TlvRecord};

#[test]
fn error_constants_and_tlv_helpers_work() {
    assert_eq!(TK_ERROR_DOMAIN, "CryptoTokenKit");
    assert_eq!(TKErrorCode::try_from(-4).unwrap(), TKErrorCode::CanceledByUser);

    assert_eq!(TlvRecord::ber_tag_data(0x5F2D).unwrap(), vec![0x5F, 0x2D]);

    let compact = TlvRecord::compact(0x1, &[0xAA, 0xBB]).unwrap();
    assert_eq!(
        TlvRecord::parse_with_encoding(TlvEncoding::Compact, &compact.data).unwrap(),
        compact
    );

    let simple = TlvRecord::simple(0x01, &[0xCA, 0xFE]).unwrap();
    assert_eq!(
        TlvRecord::parse_with_encoding(TlvEncoding::Simple, &simple.data).unwrap(),
        simple
    );

    let child_one = TlvRecord::ber(0x5F2D, b"en").unwrap();
    let child_two = TlvRecord::ber(0x9F33, &[0x01, 0x02, 0x03]).unwrap();
    let parent = TlvRecord::ber_constructed(0xE1, &[child_one.clone(), child_two.clone()]).unwrap();
    assert_eq!(TlvRecord::parse(&parent.data).unwrap(), parent);

    let sequence_data = [child_one.data.clone(), child_two.data.clone()].concat();
    assert_eq!(
        TlvRecord::parse_sequence(&sequence_data).unwrap(),
        vec![child_one, child_two]
    );
}
