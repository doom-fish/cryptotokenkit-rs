import CryptoTokenKit
import Foundation
import Security

public let CTK_OK: Int32 = 0
public let CTK_INVALID_ARGUMENT: Int32 = -1
public let CTK_FRAMEWORK_ERROR: Int32 = -2
public let CTK_TIMED_OUT: Int32 = -3

@inline(__always)
public func ctkRetain(_ object: some AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
public func ctkBorrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as _: T.Type = T.self) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@_cdecl("ctk_object_release")
public func ctk_object_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

@inline(__always)
public func ctkCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
public func ctkWriteError(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ message: String
) {
    errorOut?.pointee = ctkCString(message)
}

@inline(__always)
public func ctkWriteNSError(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    fallback: String,
    error: Error?
) {
    ctkWriteError(errorOut, (error as NSError?)?.localizedDescription ?? fallback)
}

@inline(__always)
public func ctkStatus(from error: Error) -> Int32 {
    Int32((error as NSError).code)
}

func ctkJSONString(_ value: Any) -> String {
    do {
        let data = try JSONSerialization.data(withJSONObject: value, options: [.sortedKeys])
        return String(data: data, encoding: .utf8) ?? "{}"
    } catch {
        return "{}"
    }
}

func ctkJSONValue(from cString: UnsafePointer<CChar>?) -> Any? {
    guard let cString else { return nil }
    let data = Data(bytes: cString, count: strlen(cString))
    return try? JSONSerialization.jsonObject(with: data)
}

@inline(__always)
func ctkBytes(_ data: Data?) -> [UInt8]? {
    data.map { [UInt8]($0) }
}

@inline(__always)
func ctkData(from value: Any?) -> Data? {
    guard let bytes = value as? [UInt8] else { return nil }
    return Data(bytes)
}

@inline(__always)
func ctkString(from value: Any?) -> String? {
    value as? String
}

@inline(__always)
func ctkNumber(from value: Any?) -> NSNumber? {
    value as? NSNumber
}

func ctkTokenOperationName(_ operation: TKTokenOperation) -> String {
    switch operation {
    case .none:
        return "None"
    case .readData:
        return "ReadData"
    case .signData:
        return "SignData"
    case .decryptData:
        return "DecryptData"
    case .performKeyExchange:
        return "PerformKeyExchange"
    @unknown default:
        return "None"
    }
}

func ctkTokenOperation(from value: String) -> TKTokenOperation? {
    switch value {
    case "None":
        return TKTokenOperation.none
    case "ReadData":
        return .readData
    case "SignData":
        return .signData
    case "DecryptData":
        return .decryptData
    case "PerformKeyExchange":
        return .performKeyExchange
    default:
        return nil
    }
}

func ctkConstraintsDictionary(_ constraints: [NSNumber: Any]?) -> [String: Any] {
    guard let constraints else { return [:] }
    var result: [String: Any] = [:]
    for (key, value) in constraints {
        if let operation = TKTokenOperation(rawValue: key.intValue) {
            result[ctkTokenOperationName(operation)] = value
        }
    }
    return result
}

func ctkConstraints(from value: Any?) -> [NSNumber: Any]? {
    guard let dictionary = value as? [String: Any] else { return nil }
    var result: [NSNumber: Any] = [:]
    for (key, value) in dictionary {
        guard let operation = ctkTokenOperation(from: key) else { continue }
        result[NSNumber(value: operation.rawValue)] = value
    }
    return result
}

final class CTKRustContextBox: NSObject {
    let json: String

    init(json: String) {
        self.json = json
    }
}

func ctkSmartCardPINCharsetName(_ value: TKSmartCardPINFormat.Charset) -> String {
    switch value {
    case .numeric:
        return "Numeric"
    case .alphanumeric:
        return "Alphanumeric"
    case .upperAlphanumeric:
        return "UpperAlphanumeric"
    @unknown default:
        return "Numeric"
    }
}

func ctkSmartCardPINEncodingName(_ value: TKSmartCardPINFormat.Encoding) -> String {
    switch value {
    case .binary:
        return "Binary"
    case .ascii:
        return "Ascii"
    case .bcd:
        return "Bcd"
    @unknown default:
        return "Ascii"
    }
}

func ctkSmartCardPINJustificationName(_ value: TKSmartCardPINFormat.Justification) -> String {
    switch value {
    case .left:
        return "Left"
    case .right:
        return "Right"
    @unknown default:
        return "Left"
    }
}

func ctkSmartCardPINCharset(from value: Any?) -> TKSmartCardPINFormat.Charset? {
    if let string = ctkString(from: value) {
        switch string {
        case "Numeric":
            return .numeric
        case "Alphanumeric":
            return .alphanumeric
        case "UpperAlphanumeric":
            return .upperAlphanumeric
        default:
            return nil
        }
    }
    if let number = ctkNumber(from: value) {
        return TKSmartCardPINFormat.Charset(rawValue: number.intValue)
    }
    return nil
}

func ctkSmartCardPINEncoding(from value: Any?) -> TKSmartCardPINFormat.Encoding? {
    if let string = ctkString(from: value) {
        switch string {
        case "Binary":
            return .binary
        case "Ascii":
            return .ascii
        case "Bcd":
            return .bcd
        default:
            return nil
        }
    }
    if let number = ctkNumber(from: value) {
        return TKSmartCardPINFormat.Encoding(rawValue: number.intValue)
    }
    return nil
}

func ctkSmartCardPINJustification(from value: Any?) -> TKSmartCardPINFormat.Justification? {
    if let string = ctkString(from: value) {
        switch string {
        case "Left":
            return .left
        case "Right":
            return .right
        default:
            return nil
        }
    }
    if let number = ctkNumber(from: value) {
        return TKSmartCardPINFormat.Justification(rawValue: number.intValue)
    }
    return nil
}

func ctkSmartCardPINFormatDictionary(_ format: TKSmartCardPINFormat) -> [String: Any] {
    [
        "charset": ctkSmartCardPINCharsetName(format.charset),
        "encoding": ctkSmartCardPINEncodingName(format.encoding),
        "minPinLength": format.minPINLength,
        "maxPinLength": format.maxPINLength,
        "pinBlockByteLength": format.pinBlockByteLength,
        "pinJustification": ctkSmartCardPINJustificationName(format.pinJustification),
        "pinBitOffset": format.pinBitOffset,
        "pinLengthBitOffset": format.pinLengthBitOffset,
        "pinLengthBitSize": format.pinLengthBitSize,
    ]
}

func ctkApplySmartCardPINFormat(_ value: [String: Any], to format: TKSmartCardPINFormat) {
    if let resolved = ctkSmartCardPINCharset(from: value["charset"]) {
        format.charset = resolved
    }
    if let resolved = ctkSmartCardPINEncoding(from: value["encoding"]) {
        format.encoding = resolved
    }
    if let min = ctkNumber(from: value["minPinLength"]) {
        format.minPINLength = min.intValue
    }
    if let max = ctkNumber(from: value["maxPinLength"]) {
        format.maxPINLength = max.intValue
    }
    if let block = ctkNumber(from: value["pinBlockByteLength"]) {
        format.pinBlockByteLength = block.intValue
    }
    if let resolved = ctkSmartCardPINJustification(from: value["pinJustification"]) {
        format.pinJustification = resolved
    }
    if let offset = ctkNumber(from: value["pinBitOffset"]) {
        format.pinBitOffset = offset.intValue
    }
    if let offset = ctkNumber(from: value["pinLengthBitOffset"]) {
        format.pinLengthBitOffset = offset.intValue
    }
    if let size = ctkNumber(from: value["pinLengthBitSize"]) {
        format.pinLengthBitSize = size.intValue
    }
}
