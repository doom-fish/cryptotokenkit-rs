import CryptoTokenKit
import Foundation

@available(macOS 10.15, *)
func ctkTokenDriverConfigurationDictionary(
    _ configuration: TKTokenDriver.Configuration
) -> [String: Any] {
    var tokenConfigurations: [String: Any] = [:]
    for (instanceID, tokenConfiguration) in configuration.tokenConfigurations {
        tokenConfigurations[instanceID] = ctkTokenConfigurationDictionary(tokenConfiguration, keychainContents: nil)
    }
    return [
        "classId": configuration.classID,
        "tokenConfigurations": tokenConfigurations,
    ]
}

@_cdecl("ctk_token_driver_new")
public func ctk_token_driver_new() -> UnsafeMutableRawPointer? {
    ctkRetain(TKTokenDriver())
}

@_cdecl("ctk_smart_card_token_driver_new")
public func ctk_smart_card_token_driver_new() -> UnsafeMutableRawPointer? {
    ctkRetain(TKSmartCardTokenDriver())
}

@_cdecl("ctk_driver_configurations_json")
public func ctk_driver_configurations_json() -> UnsafeMutablePointer<CChar>? {
    ctkCString(ctkJSONString(ctkDriverConfigurationsSnapshotDictionary()))
}
