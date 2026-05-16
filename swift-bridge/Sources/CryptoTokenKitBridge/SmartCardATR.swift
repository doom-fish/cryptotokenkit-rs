import CryptoTokenKit
import Foundation

public typealias CTKAtrSourceCallback = @convention(c) (UnsafeMutableRawPointer?) -> Int32

func ctkProtocolValue(_ value: NSNumber?) -> UInt32? {
    guard let value else { return nil }
    return UInt32(truncating: value)
}

func ctkTlvRecordDictionary(_ record: TKTLVRecord, encoding: String) -> [String: Any] {
    [
        "encoding": encoding,
        "tag": NSNumber(value: record.tag),
        "value": [UInt8](record.value),
        "data": [UInt8](record.data),
    ]
}

func ctkATRDictionary(_ atr: TKSmartCardATR) -> [String: Any] {
    var interfaceGroups: [[String: Any]] = []
    var index = 1
    while let group = atr.interfaceGroup(at: index) {
        interfaceGroups.append([
            "index": index,
            "ta": group.ta?.uint8Value as Any,
            "tb": group.tb?.uint8Value as Any,
            "tc": group.tc?.uint8Value as Any,
            "protocol": ctkProtocolValue(group.protocol) as Any,
        ])
        index += 1
    }

    let historicalRecords = atr.historicalRecords?.map { record in
        ctkTlvRecordDictionary(record, encoding: "compact")
    }

    return [
        "bytes": [UInt8](atr.bytes),
        "protocols": atr.protocols.map { UInt32(truncating: $0) },
        "interfaceGroups": interfaceGroups,
        "historicalBytes": [UInt8](atr.historicalBytes),
        "historicalRecords": historicalRecords as Any,
    ]
}

@_cdecl("ctk_smart_card_atr_parse_bytes_json")
public func ctk_smart_card_atr_parse_bytes_json(
    _ dataPtr: UnsafePointer<UInt8>?,
    _ dataLen: Int
) -> UnsafeMutablePointer<CChar>? {
    guard let dataPtr, dataLen > 0 else { return nil }
    let data = Data(bytes: dataPtr, count: dataLen)
    guard let atr = TKSmartCardATR(bytes: data) else {
        return nil
    }
    return ctkCString(ctkJSONString(ctkATRDictionary(atr)))
}

@_cdecl("ctk_smart_card_atr_parse_source_json")
public func ctk_smart_card_atr_parse_source_json(
    _ callback: CTKAtrSourceCallback?,
    _ userInfo: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let callback else { return nil }
    guard let atr = TKSmartCardATR(source: { callback(userInfo) }) else {
        return nil
    }
    return ctkCString(ctkJSONString(ctkATRDictionary(atr)))
}

@_cdecl("ctk_ber_tlv_record_json")
public func ctk_ber_tlv_record_json(
    _ tag: UInt64,
    _ valuePtr: UnsafePointer<UInt8>?,
    _ valueLen: Int
) -> UnsafeMutablePointer<CChar>? {
    let value = valuePtr.map { Data(bytes: $0, count: valueLen) } ?? Data()
    let record = TKBERTLVRecord(tag: tag, value: value)
    return ctkCString(ctkJSONString(ctkTlvRecordDictionary(record, encoding: "ber")))
}

@_cdecl("ctk_simple_tlv_record_json")
public func ctk_simple_tlv_record_json(
    _ tag: UInt8,
    _ valuePtr: UnsafePointer<UInt8>?,
    _ valueLen: Int
) -> UnsafeMutablePointer<CChar>? {
    let value = valuePtr.map { Data(bytes: $0, count: valueLen) } ?? Data()
    let record = TKSimpleTLVRecord(tag: tag, value: value)
    return ctkCString(ctkJSONString(ctkTlvRecordDictionary(record, encoding: "simple")))
}

@_cdecl("ctk_compact_tlv_record_json")
public func ctk_compact_tlv_record_json(
    _ tag: UInt8,
    _ valuePtr: UnsafePointer<UInt8>?,
    _ valueLen: Int
) -> UnsafeMutablePointer<CChar>? {
    let value = valuePtr.map { Data(bytes: $0, count: valueLen) } ?? Data()
    let record = TKCompactTLVRecord(tag: tag, value: value)
    return ctkCString(ctkJSONString(ctkTlvRecordDictionary(record, encoding: "compact")))
}
