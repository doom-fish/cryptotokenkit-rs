use cryptotokenkit::{SmartCardAtr, TlvRecord};

fn main() {
    let atr = SmartCardAtr::parse(&[0x3B, 0x00]).expect("expected valid ATR sample");
    println!("protocol-count: {}", atr.protocols.len());
    println!("historical-bytes: {}", atr.historical_bytes.len());

    let tlv = TlvRecord::compact(0x1, &[0xAA, 0xBB]).expect("expected compact TLV encoding");
    println!("compact-tlv-tag: {} bytes={}", tlv.tag, tlv.data.len());
    println!("✅ smart-card ATR parse OK");
}
