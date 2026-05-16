import CryptoTokenKit
import Foundation

private let ctkTokenDriverConfigurationStoreLock = NSLock()
private var ctkTokenDriverConfigurationStore: [String: [String: [String: Any]]] = [:]

private func ctkTokenConfigurationSnapshotDictionary(
    instanceID: String,
    configurationData: Data? = nil,
    keychainItems: [[String: Any]] = [],
    keychainContentsItems: [[String: Any]]? = nil
) -> [String: Any] {
    [
        "instanceId": instanceID,
        "configurationData": ctkBytes(configurationData) as Any,
        "keychainItems": keychainItems,
        "keychainContentsItems": keychainContentsItems as Any,
    ]
}

private func ctkStoredTokenDriverConfigurations() -> [String: [String: [String: Any]]] {
    ctkTokenDriverConfigurationStoreLock.lock()
    defer { ctkTokenDriverConfigurationStoreLock.unlock() }
    return ctkTokenDriverConfigurationStore
}

private func ctkAddStoredTokenConfiguration(classID: String, instanceID: String) -> [String: Any] {
    ctkTokenDriverConfigurationStoreLock.lock()
    defer { ctkTokenDriverConfigurationStoreLock.unlock() }
    let snapshot = ctkTokenConfigurationSnapshotDictionary(instanceID: instanceID)
    var configurations = ctkTokenDriverConfigurationStore[classID] ?? [:]
    configurations[instanceID] = snapshot
    ctkTokenDriverConfigurationStore[classID] = configurations
    return snapshot
}

private func ctkRemoveStoredTokenConfiguration(classID: String, instanceID: String) {
    ctkTokenDriverConfigurationStoreLock.lock()
    defer { ctkTokenDriverConfigurationStoreLock.unlock() }
    guard var configurations = ctkTokenDriverConfigurationStore[classID] else {
        return
    }
    configurations.removeValue(forKey: instanceID)
    if configurations.isEmpty {
        ctkTokenDriverConfigurationStore.removeValue(forKey: classID)
    } else {
        ctkTokenDriverConfigurationStore[classID] = configurations
    }
}

func ctkDriverConfigurationsSnapshotDictionary() -> [String: Any] {
    var merged: [String: Any] = [:]
    if #available(macOS 10.15, *) {
        for (classID, configuration) in TKTokenDriver.Configuration.driverConfigurations {
            merged[classID] = ctkTokenDriverConfigurationDictionary(configuration)
        }
    }
    for (classID, configurations) in ctkStoredTokenDriverConfigurations() {
        let frameworkSnapshot = (merged[classID] as? [String: Any])?["tokenConfigurations"] as? [String: Any] ?? [:]
        var tokenConfigurations = frameworkSnapshot
        for (instanceID, snapshot) in configurations {
            tokenConfigurations[instanceID] = snapshot
        }
        merged[classID] = [
            "classId": classID,
            "tokenConfigurations": tokenConfigurations,
        ]
    }
    return merged
}

@_cdecl("ctk_token_driver_add_token_configuration_json")
public func ctk_token_driver_add_token_configuration_json(
    _ classID: UnsafePointer<CChar>?,
    _ instanceID: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let classID, let instanceID else {
        ctkWriteError(errorOut, "missing token-driver configuration identifiers")
        return nil
    }
    let snapshot = ctkAddStoredTokenConfiguration(
        classID: String(cString: classID),
        instanceID: String(cString: instanceID)
    )
    return ctkCString(ctkJSONString(snapshot))
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
    ctkRemoveStoredTokenConfiguration(
        classID: String(cString: classID),
        instanceID: String(cString: instanceID)
    )
    return CTK_OK
}
