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
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let watcherPtr else {
        ctkWriteError(errorOut, "missing token watcher handle")
        return nil
    }
    let watcher: TKTokenWatcher = ctkBorrow(watcherPtr)
    return ctkCString(ctkJSONString(watcher.tokenIDs))
}

@_cdecl("ctk_token_watcher_set_insertion_handler")
public func ctk_token_watcher_set_insertion_handler(
    _ watcherPtr: UnsafeMutableRawPointer?,
    _ callback: CTKTokenWatcherCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let watcherPtr else {
        ctkWriteError(errorOut, "missing token watcher handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let callback else {
        ctkWriteError(errorOut, "missing token watcher insertion callback")
        return CTK_INVALID_ARGUMENT
    }
    let watcher: TKTokenWatcher = ctkBorrow(watcherPtr)
    watcher.setInsertionHandler { tokenID in
        tokenID.withCString { callback(userInfo, $0) }
    }
    return CTK_OK
}

@_cdecl("ctk_token_watcher_add_removal_handler")
public func ctk_token_watcher_add_removal_handler(
    _ watcherPtr: UnsafeMutableRawPointer?,
    _ tokenID: UnsafePointer<CChar>?,
    _ callback: CTKTokenWatcherCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
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
    let watcher: TKTokenWatcher = ctkBorrow(watcherPtr)
    watcher.addRemovalHandler({ tokenID in
        tokenID.withCString { callback(userInfo, $0) }
    }, forTokenID: String(cString: tokenID))
    return CTK_OK
}

@_cdecl("ctk_token_watcher_token_info_json")
public func ctk_token_watcher_token_info_json(
    _ watcherPtr: UnsafeMutableRawPointer?,
    _ tokenID: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let watcherPtr else {
        ctkWriteError(errorOut, "missing token watcher handle")
        return nil
    }
    guard let tokenID else {
        ctkWriteError(errorOut, "missing token identifier")
        return nil
    }
    let watcher: TKTokenWatcher = ctkBorrow(watcherPtr)
    guard #available(macOS 12.0, *) else {
        ctkWriteError(errorOut, "tokenInfo(forTokenID:) requires macOS 12.0 or newer")
        return nil
    }
    guard let info = watcher.tokenInfo(forTokenID: String(cString: tokenID)) else {
        return nil
    }
    return ctkCString(ctkJSONString(ctkTokenWatcherTokenInfoDictionary(info)))
}
