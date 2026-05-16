use cryptotokenkit::{SmartCardAtr, SmartCardProtocol, TlvEncoding, TlvRecord};

#[test]
fn atr_and_tlv_helpers_round_trip() {
    let atr = SmartCardAtr::parse(&[0x3B, 0x00]).expect("expected valid ATR");
    assert_eq!(atr.protocols, vec![SmartCardProtocol::T0]);
    assert!(atr.historical_bytes.is_empty());

    let source_bytes = [0x3B, 0x00];
    let mut index = 0;
    let from_source = SmartCardAtr::parse_from_source(|| {
        let next = source_bytes.get(index).copied();
        index += 1;
        next
    })
    .expect("expected ATR from source");
    assert_eq!(from_source.bytes, atr.bytes);

    let ber = TlvRecord::ber(0x5A, &[0x01, 0x02]).expect("expected BER TLV");
    assert_eq!(ber.encoding, TlvEncoding::Ber);
    let simple = TlvRecord::simple(0x10, &[0xCC]).expect("expected simple TLV");
    assert_eq!(simple.encoding, TlvEncoding::Simple);
    let compact = TlvRecord::compact(0x1, &[0xAA, 0xBB]).expect("expected compact TLV");
    assert_eq!(compact.encoding, TlvEncoding::Compact);
}
