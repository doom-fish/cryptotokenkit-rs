import CryptoTokenKit
import Foundation
import Security

func ctkObjectIDString(_ objectID: TKToken.ObjectID) -> String {
    if let string = objectID as? String {
        return string
    }
    return String(describing: objectID)
}

func ctkTokenKeychainItemBaseDictionary(_ item: TKTokenKeychainItem) -> [String: Any] {
    [
        "objectId": ctkObjectIDString(item.objectID),
        "label": item.label as Any,
        "constraints": ctkConstraintsDictionary(item.constraints),
    ]
}

func ctkTokenKeychainEntryDictionary(_ item: TKTokenKeychainItem) -> [String: Any] {
    if let key = item as? TKTokenKeychainKey {
        let base = ctkTokenKeychainItemBaseDictionary(key)
        let value: [String: Any] = [
            "item": base,
            "keyType": key.keyType,
            "applicationTag": ctkBytes(key.applicationTag) as Any,
            "keySizeInBits": key.keySizeInBits,
            "publicKeyData": ctkBytes(key.publicKeyData) as Any,
            "publicKeyHash": ctkBytes(key.publicKeyHash) as Any,
            "capabilities": [
                "canDecrypt": key.canDecrypt,
                "canSign": key.canSign,
                "canPerformKeyExchange": key.canPerformKeyExchange,
                "suitableForLogin": key.isSuitableForLogin,
            ],
        ]
        return ["kind": "key", "value": value]
    }
    if let certificate = item as? TKTokenKeychainCertificate {
        let base = ctkTokenKeychainItemBaseDictionary(certificate)
        let value: [String: Any] = [
            "item": base,
            "data": [UInt8](certificate.data),
        ]
        return ["kind": "certificate", "value": value]
    }
    return ["kind": "item", "value": ctkTokenKeychainItemBaseDictionary(item)]
}

func ctkTokenKeychainEntriesDictionary(_ items: [TKTokenKeychainItem]) -> [[String: Any]] {
    items.map(ctkTokenKeychainEntryDictionary)
}

@available(macOS 10.15, *)
func ctkTokenConfigurationDictionary(
    _ configuration: TKToken.Configuration,
    keychainContents: TKTokenKeychainContents?
) -> [String: Any] {
    [
        "instanceId": configuration.instanceID,
        "configurationData": ctkBytes(configuration.configurationData) as Any,
        "keychainItems": ctkTokenKeychainEntriesDictionary(configuration.keychainItems),
        "keychainContentsItems": keychainContents.map { ctkTokenKeychainEntriesDictionary($0.items) } as Any,
    ]
}

func ctkTokenKeychainItem(
    from entry: [String: Any],
    errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> TKTokenKeychainItem? {
    guard let kind = ctkString(from: entry["kind"]),
          let value = entry["value"] as? [String: Any] else {
        ctkWriteError(errorOut, "invalid token keychain entry payload")
        return nil
    }

    let baseValue = kind == "item" ? value : (value["item"] as? [String: Any] ?? [:])
    guard let objectID = ctkString(from: baseValue["objectId"]) else {
        ctkWriteError(errorOut, "invalid token keychain object identifier")
        return nil
    }

    let label = ctkString(from: baseValue["label"])
    let constraints = ctkConstraints(from: baseValue["constraints"])

    switch kind {
    case "item":
        let item = TKTokenKeychainItem(objectID: objectID)
        item.label = label
        item.constraints = constraints
        return item

    case "certificate":
        guard let data = ctkData(from: value["data"]),
              let certificateRef = SecCertificateCreateWithData(nil, data as CFData),
              let certificate = TKTokenKeychainCertificate(certificate: certificateRef, objectID: objectID) else {
            ctkWriteError(errorOut, "failed to build TKTokenKeychainCertificate from DER data")
            return nil
        }
        certificate.label = label
        certificate.constraints = constraints
        return certificate

    case "key":
        guard let key = TKTokenKeychainKey(certificate: nil, objectID: objectID) else {
            ctkWriteError(errorOut, "failed to create TKTokenKeychainKey")
            return nil
        }
        key.label = label
        key.constraints = constraints
        key.keyType = ctkString(from: value["keyType"]) ?? ""
        key.applicationTag = ctkData(from: value["applicationTag"])
        key.keySizeInBits = ctkNumber(from: value["keySizeInBits"])?.intValue ?? 0
        key.publicKeyData = ctkData(from: value["publicKeyData"])
        key.publicKeyHash = ctkData(from: value["publicKeyHash"])
        if let capabilities = value["capabilities"] as? [String: Any] {
            key.canDecrypt = (capabilities["canDecrypt"] as? Bool) ?? false
            key.canSign = (capabilities["canSign"] as? Bool) ?? false
            key.canPerformKeyExchange = (capabilities["canPerformKeyExchange"] as? Bool) ?? false
            key.isSuitableForLogin = (capabilities["suitableForLogin"] as? Bool) ?? false
        }
        return key

    default:
        ctkWriteError(errorOut, "unknown token keychain entry kind")
        return nil
    }
}

func ctkTokenKeychainItems(
    from json: UnsafePointer<CChar>?,
    errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> [TKTokenKeychainItem]? {
    guard let array = ctkJSONValue(from: json) as? [[String: Any]] else {
        ctkWriteError(errorOut, "invalid token keychain JSON payload")
        return nil
    }
    return array.compactMap { ctkTokenKeychainItem(from: $0, errorOut: errorOut) }
}

@_cdecl("ctk_token_set_keychain_items_json")
public func ctk_token_set_keychain_items_json(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ json: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tokenPtr else {
        ctkWriteError(errorOut, "missing token handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let json else {
        ctkWriteError(errorOut, "missing token keychain JSON payload")
        return CTK_INVALID_ARGUMENT
    }

    let token: TKToken = ctkBorrow(tokenPtr)
    guard #available(macOS 10.15, *) else {
        ctkWriteError(errorOut, "token.configuration requires macOS 10.15 or newer")
        return CTK_FRAMEWORK_ERROR
    }
    guard let items = ctkTokenKeychainItems(from: json, errorOut: errorOut) else {
        return CTK_INVALID_ARGUMENT
    }
    token.configuration.keychainItems = items
    return CTK_OK
}

@_cdecl("ctk_token_key_for_object_id_json")
public func ctk_token_key_for_object_id_json(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ objectID: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let tokenPtr else {
        ctkWriteError(errorOut, "missing token handle")
        return nil
    }
    guard let objectID else {
        ctkWriteError(errorOut, "missing token object identifier")
        return nil
    }

    let token: TKToken = ctkBorrow(tokenPtr)
    guard #available(macOS 10.15, *) else {
        ctkWriteError(errorOut, "token.configuration requires macOS 10.15 or newer")
        return nil
    }
    do {
        let key = try token.configuration.key(for: String(cString: objectID))
        return ctkCString(ctkJSONString(ctkTokenKeychainEntryDictionary(key)["value"] ?? [:]))
    } catch {
        ctkWriteNSError(errorOut, fallback: "failed to resolve token key", error: error)
        return nil
    }
}

@_cdecl("ctk_token_certificate_for_object_id_json")
public func ctk_token_certificate_for_object_id_json(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ objectID: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let tokenPtr else {
        ctkWriteError(errorOut, "missing token handle")
        return nil
    }
    guard let objectID else {
        ctkWriteError(errorOut, "missing token object identifier")
        return nil
    }

    let token: TKToken = ctkBorrow(tokenPtr)
    guard #available(macOS 10.15, *) else {
        ctkWriteError(errorOut, "token.configuration requires macOS 10.15 or newer")
        return nil
    }
    do {
        let certificate = try token.configuration.certificate(for: String(cString: objectID))
        return ctkCString(ctkJSONString(ctkTokenKeychainEntryDictionary(certificate)["value"] ?? [:]))
    } catch {
        ctkWriteNSError(errorOut, fallback: "failed to resolve token certificate", error: error)
        return nil
    }
}

@_cdecl("ctk_token_keychain_contents_items_json")
public func ctk_token_keychain_contents_items_json(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let tokenPtr else {
        ctkWriteError(errorOut, "missing token handle")
        return nil
    }
    let token: TKToken = ctkBorrow(tokenPtr)
    guard let keychainContents = token.keychainContents else {
        return nil
    }
    return ctkCString(ctkJSONString(ctkTokenKeychainEntriesDictionary(keychainContents.items)))
}
