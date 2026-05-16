use cryptotokenkit::{
    SmartCardPinCharset, SmartCardPinEncoding, SmartCardPinFormat, SmartCardPinJustification,
    SmartCardProtocol,
};

#[test]
fn smart_card_constants_match_header_defaults() {
    let format = SmartCardPinFormat::default();
    assert_eq!(format.charset, SmartCardPinCharset::Numeric);
    assert_eq!(format.encoding, SmartCardPinEncoding::Ascii);
    assert_eq!(format.min_pin_length, 4);
    assert_eq!(format.max_pin_length, 8);
    assert_eq!(format.pin_block_byte_length, 8);
    assert_eq!(format.pin_justification, SmartCardPinJustification::Left);
    assert_eq!(SmartCardProtocol::ANY.bits(), (1 << 16) - 1);
}
