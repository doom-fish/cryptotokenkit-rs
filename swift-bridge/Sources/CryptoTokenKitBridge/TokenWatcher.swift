import CryptoTokenKit
import Foundation

public typealias CTKTokenWatcherCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void

@available(macOS 12.0, *)
func ctkTokenWatcherTokenInfoDictionary(_ info: TKTokenWatcher.TokenInfo) -> [String: Any] {
    [
        "tokenId": info.tokenID,
        "slotName": info.slotName as Any,
        "driverName": info.driverName as Any,
    ]
}

@_cdecl("ctk_token_watcher_new")
public func ctk_token_watcher_new() -> UnsafeMutableRawPointer? {
    ctkRetain(TKTokenWatcher())
}

@_cdecl("ctk_token_watcher_token_ids_json")
public func ctk_token_watcher_token_ids_json(
    _ watcherPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outJSON.pointee = nil
    guard let watcherPtr else {
        ctkWriteError(errorOut, "missing token watcher handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let watcher = ctkBorrow(watcherPtr, as: TKTokenWatcher.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    outJSON.pointee = ctkCString(ctkJSONString(watcher.tokenIDs))
    return CTK_OK
}

@_cdecl("ctk_token_watcher_set_insertion_handler")
public func ctk_token_watcher_set_insertion_handler(
    _ watcherPtr: UnsafeMutableRawPointer?,
    _ callback: CTKTokenWatcherCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ release: CTKContextRelease?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let context = CTKCallbackContext(userInfo, release: release)
    guard let watcherPtr else {
        ctkWriteError(errorOut, "missing token watcher handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let callback else {
        ctkWriteError(errorOut, "missing token watcher insertion callback")
        return CTK_INVALID_ARGUMENT
    }
    guard let watcher = ctkBorrow(watcherPtr, as: TKTokenWatcher.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    watcher.setInsertionHandler { tokenID in
        tokenID.withCString { callback(context.pointer, $0) }
    }
    return CTK_OK
}

@_cdecl("ctk_token_watcher_add_removal_handler")
public func ctk_token_watcher_add_removal_handler(
    _ watcherPtr: UnsafeMutableRawPointer?,
    _ tokenID: UnsafePointer<CChar>?,
    _ callback: CTKTokenWatcherCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ release: CTKContextRelease?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let context = CTKCallbackContext(userInfo, release: release)
    guard let watcherPtr else {
        ctkWriteError(errorOut, "missing token watcher handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let tokenID else {
        ctkWriteError(errorOut, "missing token identifier for removal handler")
        return CTK_INVALID_ARGUMENT
    }
    guard let callback else {
        ctkWriteError(errorOut, "missing token watcher removal callback")
        return CTK_INVALID_ARGUMENT
    }
    guard let watcher = ctkBorrow(watcherPtr, as: TKTokenWatcher.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    watcher.addRemovalHandler({ tokenID in
        tokenID.withCString { callback(context.pointer, $0) }
    }, forTokenID: String(cString: tokenID))
    return CTK_OK
}

@_cdecl("ctk_token_watcher_token_info_json")
public func ctk_token_watcher_token_info_json(
    _ watcherPtr: UnsafeMutableRawPointer?,
    _ tokenID: UnsafePointer<CChar>?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outJSON.pointee = nil
    guard let watcherPtr else {
        ctkWriteError(errorOut, "missing token watcher handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let tokenID else {
        ctkWriteError(errorOut, "missing token identifier")
        return CTK_INVALID_ARGUMENT
    }
    guard let watcher = ctkBorrow(watcherPtr, as: TKTokenWatcher.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    guard #available(macOS 12.0, *) else {
        ctkWriteError(errorOut, "tokenInfo(forTokenID:) requires macOS 12.0 or newer")
        return CTK_UNSUPPORTED
    }
    guard let info = watcher.tokenInfo(forTokenID: String(cString: tokenID)) else {
        return CTK_OK
    }
    outJSON.pointee = ctkCString(ctkJSONString(ctkTokenWatcherTokenInfoDictionary(info)))
    return CTK_OK
}
