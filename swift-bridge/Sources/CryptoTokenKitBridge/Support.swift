import CryptoTokenKit
import Foundation
import ObjectiveC.runtime
import Security

func ctkTakeCString(_ ptr: UnsafeMutablePointer<CChar>?) -> String {
    guard let ptr else { return "" }
    let string = String(cString: ptr)
    free(ptr)
    return string
}

func ctkTakeJSONValue(_ ptr: UnsafeMutablePointer<CChar>?) -> Any? {
    let string = ctkTakeCString(ptr)
    guard !string.isEmpty, let data = string.data(using: .utf8) else {
        return nil
    }
    return try? JSONSerialization.jsonObject(with: data)
}

func ctkNSError(status: Int32, message: String) -> NSError {
    NSError(
        domain: TKErrorDomain,
        code: Int(status),
        userInfo: [NSLocalizedDescriptionKey: message]
    )
}

func ctkWriteCallbackError(
    _ errorOut: NSErrorPointer,
    status: Int32,
    errorCString: UnsafeMutablePointer<CChar>?,
    fallback: String
) {
    guard let errorOut else {
        _ = ctkTakeCString(errorCString)
        return
    }
    let message = ctkTakeCString(errorCString)
    errorOut.pointee = ctkNSError(status: status, message: message.isEmpty ? fallback : message)
}

func ctkJSONCompatibleValue(_ value: Any?) -> Any {
    guard let value else {
        return NSNull()
    }
    switch value {
    case let string as String:
        return string
    case let number as NSNumber:
        return number
    case let data as Data:
        return [UInt8](data)
    case is NSNull:
        return NSNull()
    case let array as [Any]:
        return array.map(ctkJSONCompatibleValue)
    case let dictionary as [String: Any]:
        var result: [String: Any] = [:]
        for (key, nestedValue) in dictionary {
            result[key] = ctkJSONCompatibleValue(nestedValue)
        }
        return result
    case let dictionary as [AnyHashable: Any]:
        var result: [String: Any] = [:]
        for (key, nestedValue) in dictionary {
            result[String(describing: key)] = ctkJSONCompatibleValue(nestedValue)
        }
        return result
    default:
        return String(describing: value)
    }
}

func ctkPlistValue(fromJSONCompatible value: Any?) -> Any? {
    guard let value else {
        return nil
    }
    switch value {
    case is NSNull:
        return nil
    case let string as String:
        return string
    case let number as NSNumber:
        return number
    case let array as [Any]:
        return array.compactMap(ctkPlistValue(fromJSONCompatible:))
    case let dictionary as [String: Any]:
        var result: [String: Any] = [:]
        for (key, nestedValue) in dictionary {
            result[key] = ctkPlistValue(fromJSONCompatible: nestedValue)
        }
        return result
    default:
        return value
    }
}

func ctkSecKeyAlgorithmName(_ algorithm: SecKeyAlgorithm) -> String {
    unsafeBitCast(algorithm, to: CFString.self) as String
}

func ctkSecKeyAlgorithm(from string: String) -> SecKeyAlgorithm {
    unsafeBitCast(string as CFString, to: SecKeyAlgorithm.self)
}

private var ctkMockTokenKeyAlgorithmStorageKey: UInt8 = 0

private final class CTKMockTokenKeyAlgorithmStorage: NSObject {
    let baseAlgorithm: String
    let supportedAlgorithms: Set<String>

    init(baseAlgorithm: String, supportedAlgorithms: Set<String>) {
        self.baseAlgorithm = baseAlgorithm
        self.supportedAlgorithms = supportedAlgorithms
    }
}

final class CTKMockTokenKeyAlgorithm: TKTokenKeyAlgorithm {
    override func isAlgorithm(_ algorithm: SecKeyAlgorithm) -> Bool {
        storage.baseAlgorithm == ctkSecKeyAlgorithmName(algorithm)
    }

    override func supportsAlgorithm(_ algorithm: SecKeyAlgorithm) -> Bool {
        storage.supportedAlgorithms.contains(ctkSecKeyAlgorithmName(algorithm))
    }

    static func make(baseAlgorithm: String, supportedAlgorithms: [String]) -> CTKMockTokenKeyAlgorithm {
        let instance = class_createInstance(CTKMockTokenKeyAlgorithm.self, 0) as! CTKMockTokenKeyAlgorithm
        let storage = CTKMockTokenKeyAlgorithmStorage(
            baseAlgorithm: baseAlgorithm,
            supportedAlgorithms: Set(supportedAlgorithms).union([baseAlgorithm])
        )
        objc_setAssociatedObject(
            instance,
            &ctkMockTokenKeyAlgorithmStorageKey,
            storage,
            .OBJC_ASSOCIATION_RETAIN_NONATOMIC
        )
        return instance
    }

    private var storage: CTKMockTokenKeyAlgorithmStorage {
        (objc_getAssociatedObject(self, &ctkMockTokenKeyAlgorithmStorageKey) as? CTKMockTokenKeyAlgorithmStorage)
            ?? CTKMockTokenKeyAlgorithmStorage(baseAlgorithm: "", supportedAlgorithms: [])
    }
}

final class CTKMockTokenKeyExchangeParameters: TKTokenKeyExchangeParameters {
    private let requestedSizeValue: Int
    private let sharedInfoValue: Data?

    init(requestedSize: Int, sharedInfo: Data?) {
        self.requestedSizeValue = requestedSize
        self.sharedInfoValue = sharedInfo
        super.init()
    }

    override var requestedSize: Int {
        requestedSizeValue
    }

    override var sharedInfo: Data? {
        sharedInfoValue
    }
}

func ctkMockTokenKeyAlgorithm(
    baseAlgorithm: UnsafePointer<CChar>?,
    supportedAlgorithmsJSON: UnsafePointer<CChar>?
) -> TKTokenKeyAlgorithm? {
    guard let baseAlgorithm else {
        return nil
    }
    let supportedAlgorithms = (ctkJSONValue(from: supportedAlgorithmsJSON) as? [String]) ?? []
    return CTKMockTokenKeyAlgorithm.make(
        baseAlgorithm: String(cString: baseAlgorithm),
        supportedAlgorithms: supportedAlgorithms
    )
}

func ctkMockTokenKeyExchangeParameters(
    requestedSize: Int,
    sharedInfoPtr: UnsafePointer<UInt8>?,
    sharedInfoLen: Int,
    hasSharedInfo: Bool
) -> TKTokenKeyExchangeParameters {
    let sharedInfo = hasSharedInfo ? sharedInfoPtr.map { Data(bytes: $0, count: sharedInfoLen) } : nil
    return CTKMockTokenKeyExchangeParameters(requestedSize: requestedSize, sharedInfo: sharedInfo)
}
