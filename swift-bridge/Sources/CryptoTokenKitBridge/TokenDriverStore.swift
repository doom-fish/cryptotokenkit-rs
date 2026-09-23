import CryptoTokenKit
import Foundation

func ctkDriverConfigurationsSnapshotDictionary() -> [String: Any] {
    var snapshot: [String: Any] = [:]
    if #available(macOS 10.15, *) {
        for (classID, configuration) in TKTokenDriver.Configuration.driverConfigurations {
            snapshot[classID] = ctkTokenDriverConfigurationDictionary(configuration)
        }
    }
    return snapshot
}

@available(macOS 10.15, *)
private func ctkHostedDriverConfiguration(
    classID: String,
    errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> TKTokenDriver.Configuration? {
    guard let configuration = TKTokenDriver.Configuration.driverConfigurations[classID] else {
        ctkWriteError(
            errorOut,
            "no token driver with class ID \(classID) is hosted by this process; only the app that contains the token extension can change its token configurations"
        )
        return nil
    }
    return configuration
}

@_cdecl("ctk_token_driver_add_token_configuration_json")
public func ctk_token_driver_add_token_configuration_json(
    _ classID: UnsafePointer<CChar>?,
    _ instanceID: UnsafePointer<CChar>?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outJSON.pointee = nil
    guard let classID, let instanceID else {
        ctkWriteError(errorOut, "missing token-driver configuration identifiers")
        return CTK_INVALID_ARGUMENT
    }
    guard #available(macOS 10.15, *) else {
        ctkWriteError(errorOut, "token-driver configurations require macOS 10.15 or newer")
        return CTK_UNSUPPORTED
    }
    guard let driverConfiguration = ctkHostedDriverConfiguration(
        classID: String(cString: classID),
        errorOut: errorOut
    ) else {
        return CTK_UNSUPPORTED
    }
    let configuration = driverConfiguration.addTokenConfiguration(for: String(cString: instanceID))
    outJSON.pointee = ctkCString(
        ctkJSONString(ctkTokenConfigurationDictionary(configuration, keychainContents: nil))
    )
    return CTK_OK
}

@_cdecl("ctk_token_driver_remove_token_configuration")
public func ctk_token_driver_remove_token_configuration(
    _ classID: UnsafePointer<CChar>?,
    _ instanceID: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let classID, let instanceID else {
        ctkWriteError(errorOut, "missing token-driver configuration identifiers")
        return CTK_INVALID_ARGUMENT
    }
    guard #available(macOS 10.15, *) else {
        ctkWriteError(errorOut, "token-driver configurations require macOS 10.15 or newer")
        return CTK_UNSUPPORTED
    }
    guard let driverConfiguration = ctkHostedDriverConfiguration(
        classID: String(cString: classID),
        errorOut: errorOut
    ) else {
        return CTK_UNSUPPORTED
    }
    driverConfiguration.removeTokenConfiguration(for: String(cString: instanceID))
    return CTK_OK
}
