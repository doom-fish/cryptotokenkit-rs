use cryptotokenkit::{SmartCardAtr, TlvEncoding, TlvRecord};

#[test]
fn ber_length_that_overflows_usize_is_rejected() {
    let data = [
        0x5A, 0x88, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01,
    ];
    assert_eq!(
        TlvRecord::parse_with_encoding(TlvEncoding::Ber, &data),
        None
    );
    assert_eq!(
        TlvRecord::parse_sequence_with_encoding(TlvEncoding::Ber, &data),
        None
    );
    let fallback = TlvRecord::parse(&data).expect("compact interpretation");
    assert_eq!(fallback.encoding, TlvEncoding::Compact);
    assert_eq!(fallback.value, data[1..]);
}

#[test]
fn ber_length_just_below_overflow_is_rejected() {
    let data = [
        0x1F, 0x81, 0x01, 0x88, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xF0,
    ];
    assert_eq!(
        TlvRecord::parse_with_encoding(TlvEncoding::Ber, &data),
        None
    );
}

#[test]
fn ber_length_longer_than_the_input_is_rejected() {
    assert_eq!(
        TlvRecord::parse_with_encoding(TlvEncoding::Ber, &[0x5A, 0x82, 0x01, 0x00, 0xAA]),
        None
    );
    assert_eq!(
        TlvRecord::parse_with_encoding(TlvEncoding::Ber, &[0x5A, 0x03, 0x01, 0x02]),
        None
    );
}

#[test]
fn ber_length_forms_without_enough_bytes_are_rejected() {
    for data in [
        &[][..],
        &[0x5A][..],
        &[0x5F][..],
        &[0x5F, 0x81][..],
        &[0x5A, 0x81][..],
        &[0x5A, 0x84, 0x00, 0x00][..],
        &[0x5A, 0x80][..],
        &[0x5A, 0x89, 0, 0, 0, 0, 0, 0, 0, 0, 0][..],
    ] {
        assert_eq!(
            TlvRecord::parse_with_encoding(TlvEncoding::Ber, data),
            None,
            "{data:02X?}"
        );
    }
}

#[test]
fn ber_tags_longer_than_eight_bytes_are_rejected() {
    let nine_byte_tag = [0x1F, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x01, 0x00];
    assert_eq!(
        TlvRecord::parse_with_encoding(TlvEncoding::Ber, &nine_byte_tag),
        None
    );

    let unterminated_tag = [0x1F, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81];
    assert_eq!(
        TlvRecord::parse_with_encoding(TlvEncoding::Ber, &unterminated_tag),
        None
    );
}

#[test]
fn ber_long_form_length_is_decoded() {
    let mut data = vec![0x5A, 0x82, 0x01, 0x00];
    data.extend(std::iter::repeat_n(0xAB, 0x100));
    let record =
        TlvRecord::parse_with_encoding(TlvEncoding::Ber, &data).expect("valid long-form TLV");
    assert_eq!(record.tag, 0x5A);
    assert_eq!(record.value.len(), 0x100);
    assert!(record.value.iter().all(|byte| *byte == 0xAB));
}

#[test]
fn simple_tlv_truncated_lengths_are_rejected() {
    for data in [
        &[0x01][..],
        &[0x01, 0xFF][..],
        &[0x01, 0xFF, 0x00][..],
        &[0x01, 0xFF, 0x00, 0x02, 0xAA][..],
        &[0x01, 0x02, 0xAA][..],
        &[0x00, 0x00][..],
        &[0xFF, 0x00][..],
    ] {
        assert_eq!(
            TlvRecord::parse_with_encoding(TlvEncoding::Simple, data),
            None,
            "{data:02X?}"
        );
    }
}

#[test]
fn compact_tlv_truncated_values_are_rejected() {
    assert_eq!(
        TlvRecord::parse_with_encoding(TlvEncoding::Compact, &[0x13, 0xAA]),
        None
    );
    assert_eq!(
        TlvRecord::parse_with_encoding(TlvEncoding::Compact, &[]),
        None
    );
}

#[test]
fn sequences_with_trailing_garbage_are_rejected() {
    let first = TlvRecord::ber(0x5A, &[0x01]).expect("BER record");
    let mut data = first.data;
    data.extend_from_slice(&[0x5A, 0x88, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    assert_eq!(
        TlvRecord::parse_sequence_with_encoding(TlvEncoding::Ber, &data),
        None
    );
}

#[test]
fn malformed_atr_bytes_are_rejected() {
    assert_eq!(SmartCardAtr::parse(&[]), None);
    assert_eq!(SmartCardAtr::parse(&[0x00]), None);
    assert_eq!(SmartCardAtr::parse(&[0x3B, 0xFF]), None);
}

#[test]
fn pseudo_random_inputs_never_panic() {
    let mut state = 0x2545_F491_4F6C_DD1Du64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for _ in 0..512 {
        let len = usize::try_from(next() % 24).expect("small length");
        let data: Vec<u8> = (0..len).map(|_| next().to_le_bytes()[0]).collect();
        for encoding in [TlvEncoding::Ber, TlvEncoding::Simple, TlvEncoding::Compact] {
            if let Some(record) = TlvRecord::parse_with_encoding(encoding, &data) {
                assert_eq!(record.encoding, encoding);
                assert!(record.value.len() <= data.len());
            }
        }
        if let Some(records) = TlvRecord::parse_sequence(&data) {
            let total: usize = records.iter().map(|record| record.value.len()).sum();
            assert!(total <= data.len());
        }
        let _ = SmartCardAtr::parse(&data);
    }
}

#[test]
fn constructors_reject_values_the_framework_would_raise_on() {
    assert_eq!(TlvRecord::ber(0, &[]), None);
    assert_eq!(TlvRecord::ber_tag_data(0), None);
    assert_eq!(TlvRecord::ber_constructed(0, &[]), None);
    assert_eq!(TlvRecord::compact(0x10, &[]), None);
    assert_eq!(TlvRecord::compact(0x01, &[0; 16]), None);
    assert_eq!(TlvRecord::simple(0x01, &vec![0; 0x1_0000]), None);

    assert!(TlvRecord::compact(0x0F, &[0; 15]).is_some());
    assert_eq!(
        TlvRecord::simple(0x01, &vec![0; 0xFFFF]).map(|record| record.value.len()),
        Some(0xFFFF)
    );
}

#[test]
fn constructed_records_reject_invalid_children() {
    let invalid_children = [
        TlvRecord {
            encoding: TlvEncoding::Compact,
            tag: 0x10,
            value: Vec::new(),
            data: Vec::new(),
        },
        TlvRecord {
            encoding: TlvEncoding::Compact,
            tag: 0x01,
            value: vec![0; 16],
            data: Vec::new(),
        },
        TlvRecord {
            encoding: TlvEncoding::Simple,
            tag: 0x100,
            value: Vec::new(),
            data: Vec::new(),
        },
        TlvRecord {
            encoding: TlvEncoding::Ber,
            tag: 0,
            value: Vec::new(),
            data: Vec::new(),
        },
    ];
    for child in invalid_children {
        assert_eq!(
            TlvRecord::ber_constructed(0x70, std::slice::from_ref(&child)),
            None,
            "{child:?}"
        );
    }

    let valid = TlvRecord::compact(0x01, &[0xAA]).expect("compact record");
    assert!(TlvRecord::ber_constructed(0x70, &[valid]).is_some());
}

#[test]
fn zero_ber_tag_in_input_is_rejected_without_aborting() {
    assert_eq!(
        TlvRecord::parse_with_encoding(TlvEncoding::Ber, &[0x00, 0x00]),
        None
    );
    assert_eq!(
        TlvRecord::parse_sequence_with_encoding(TlvEncoding::Ber, &[0x5A, 0x00, 0x00, 0x00]),
        None
    );
}
