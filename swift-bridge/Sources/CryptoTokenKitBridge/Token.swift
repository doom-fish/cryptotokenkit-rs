import CryptoTokenKit
import Foundation

@_cdecl("ctk_token_new")
public func ctk_token_new(
    _ driverPtr: UnsafeMutableRawPointer?,
    _ instanceID: UnsafePointer<CChar>?,
    _ outToken: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outToken.pointee = nil
    guard let driverPtr else {
        ctkWriteError(errorOut, "missing token driver handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let instanceID else {
        ctkWriteError(errorOut, "missing token instance identifier")
        return CTK_INVALID_ARGUMENT
    }

    guard let driver = ctkBorrow(driverPtr, as: TKTokenDriver.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    let token = TKToken(tokenDriver: driver, instanceID: String(cString: instanceID))
    outToken.pointee = ctkRetain(token)
    return CTK_OK
}

@_cdecl("ctk_smart_card_token_new")
public func ctk_smart_card_token_new(
    _ smartCardPtr: UnsafeMutableRawPointer?,
    _ aidPtr: UnsafePointer<UInt8>?,
    _ aidLen: Int,
    _ hasAID: Bool,
    _ instanceID: UnsafePointer<CChar>?,
    _ driverPtr: UnsafeMutableRawPointer?,
    _ outToken: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outToken.pointee = nil
    guard let smartCardPtr else {
        ctkWriteError(errorOut, "missing smart-card handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let instanceID else {
        ctkWriteError(errorOut, "missing smart-card token instance identifier")
        return CTK_INVALID_ARGUMENT
    }
    guard let driverPtr else {
        ctkWriteError(errorOut, "missing smart-card token driver handle")
        return CTK_INVALID_ARGUMENT
    }

    guard let smartCard = ctkBorrow(smartCardPtr, as: TKSmartCard.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    guard let driver = ctkBorrow(driverPtr, as: TKSmartCardTokenDriver.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    let aidData = hasAID ? aidPtr.map { Data(bytes: $0, count: aidLen) } : nil
    let token = TKSmartCardToken(
        smartCard: smartCard,
        aid: aidData,
        instanceID: String(cString: instanceID),
        tokenDriver: driver
    )
    outToken.pointee = ctkRetain(token)
    return CTK_OK
}

@_cdecl("ctk_smart_card_token_aid_json")
public func ctk_smart_card_token_aid_json(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let token = ctkBorrow(tokenPtr, as: TKSmartCardToken.self, errorOut) else { return nil }
    guard let aid = token.aid else {
        return nil
    }
    return ctkCString(ctkJSONString([UInt8](aid)))
}

@_cdecl("ctk_token_configuration_json")
public func ctk_token_configuration_json(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outJSON.pointee = nil
    guard let tokenPtr else {
        ctkWriteError(errorOut, "missing token handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let token = ctkBorrow(tokenPtr, as: TKToken.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    guard #available(macOS 10.15, *) else {
        ctkWriteError(errorOut, "token.configuration requires macOS 10.15 or newer")
        return CTK_UNSUPPORTED
    }
    let dictionary = ctkTokenConfigurationDictionary(
        token.configuration,
        keychainContents: token.keychainContents
    )
    outJSON.pointee = ctkCString(ctkJSONString(dictionary))
    return CTK_OK
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
    guard let token = ctkBorrow(tokenPtr, as: TKToken.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    guard #available(macOS 10.15, *) else {
        ctkWriteError(errorOut, "token.configuration requires macOS 10.15 or newer")
        return CTK_UNSUPPORTED
    }
    let data = hasData ? dataPtr.map { Data(bytes: $0, count: dataLen) } ?? Data() : nil
    let configuration = token.configuration
    configuration.configurationData = data
    guard configuration.configurationData == data else {
        ctkWriteError(
            errorOut,
            "CryptoTokenKit did not store the configuration data; only the app that contains the token extension can change its token configurations"
        )
        return CTK_UNSUPPORTED
    }
    return CTK_OK
}
