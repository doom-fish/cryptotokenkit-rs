import CryptoTokenKit
import Foundation

private let ctkTokenConfigurationDataLock = NSLock()
private var ctkTokenConfigurationDataStore: [ObjectIdentifier: Data] = [:]

private func ctkStoredConfigurationData(for token: TKToken) -> Data? {
    ctkTokenConfigurationDataLock.lock()
    defer { ctkTokenConfigurationDataLock.unlock() }
    return ctkTokenConfigurationDataStore[ObjectIdentifier(token)]
}

private func ctkSetStoredConfigurationData(_ data: Data?, for token: TKToken) {
    ctkTokenConfigurationDataLock.lock()
    defer { ctkTokenConfigurationDataLock.unlock() }
    let key = ObjectIdentifier(token)
    if let data {
        ctkTokenConfigurationDataStore[key] = data
    } else {
        ctkTokenConfigurationDataStore.removeValue(forKey: key)
    }
}

@_cdecl("ctk_token_new")
public func ctk_token_new(
    _ driverPtr: UnsafeMutableRawPointer?,
    _ instanceID: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let driverPtr else {
        ctkWriteError(errorOut, "missing token driver handle")
        return nil
    }
    guard let instanceID else {
        ctkWriteError(errorOut, "missing token instance identifier")
        return nil
    }

    let driver: TKTokenDriver = ctkBorrow(driverPtr)
    let token = TKToken(tokenDriver: driver, instanceID: String(cString: instanceID))
    return ctkRetain(token)
}

@_cdecl("ctk_smart_card_token_new")
public func ctk_smart_card_token_new(
    _ smartCardPtr: UnsafeMutableRawPointer?,
    _ aidPtr: UnsafePointer<UInt8>?,
    _ aidLen: Int,
    _ hasAID: Bool,
    _ instanceID: UnsafePointer<CChar>?,
    _ driverPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let smartCardPtr else {
        ctkWriteError(errorOut, "missing smart-card handle")
        return nil
    }
    guard let instanceID else {
        ctkWriteError(errorOut, "missing smart-card token instance identifier")
        return nil
    }
    guard let driverPtr else {
        ctkWriteError(errorOut, "missing smart-card token driver handle")
        return nil
    }

    let smartCard: TKSmartCard = ctkBorrow(smartCardPtr)
    let driver: TKSmartCardTokenDriver = ctkBorrow(driverPtr)
    let aidData = hasAID ? aidPtr.map { Data(bytes: $0, count: aidLen) } : nil
    let token = TKSmartCardToken(
        smartCard: smartCard,
        aid: aidData,
        instanceID: String(cString: instanceID),
        tokenDriver: driver
    )
    return ctkRetain(token)
}

@_cdecl("ctk_smart_card_token_aid_json")
public func ctk_smart_card_token_aid_json(
    _ tokenPtr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let tokenPtr else { return nil }
    let token: TKSmartCardToken = ctkBorrow(tokenPtr)
    guard let aid = token.aid else {
        return nil
    }
    return ctkCString(ctkJSONString([UInt8](aid)))
}

@_cdecl("ctk_token_configuration_json")
public func ctk_token_configuration_json(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let tokenPtr else {
        ctkWriteError(errorOut, "missing token handle")
        return nil
    }
    let token: TKToken = ctkBorrow(tokenPtr)
    guard #available(macOS 10.15, *) else {
        ctkWriteError(errorOut, "token.configuration requires macOS 10.15 or newer")
        return nil
    }
    var dictionary = ctkTokenConfigurationDictionary(
        token.configuration,
        keychainContents: token.keychainContents
    )
    dictionary["configurationData"] = ctkBytes(ctkStoredConfigurationData(for: token)) ?? NSNull()
    return ctkCString(ctkJSONString(dictionary))
}

@_cdecl("ctk_token_set_configuration_data")
public func ctk_token_set_configuration_data(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ dataPtr: UnsafePointer<UInt8>?,
    _ dataLen: Int,
    _ hasData: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let tokenPtr else {
        ctkWriteError(errorOut, "missing token handle")
        return CTK_INVALID_ARGUMENT
    }
    let token: TKToken = ctkBorrow(tokenPtr)
    guard #available(macOS 10.15, *) else {
        ctkWriteError(errorOut, "token.configuration requires macOS 10.15 or newer")
        return CTK_FRAMEWORK_ERROR
    }
    let data = hasData ? dataPtr.map { Data(bytes: $0, count: dataLen) } : nil
    ctkSetStoredConfigurationData(data, for: token)
    return CTK_OK
}
