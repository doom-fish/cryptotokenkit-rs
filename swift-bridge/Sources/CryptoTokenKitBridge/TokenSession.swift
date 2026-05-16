import CryptoTokenKit
import Foundation

func ctkTokenSmartCardPinAuthOperationDictionary(
    _ operation: TKTokenSmartCardPINAuthOperation
) -> [String: Any] {
    [
        "pinFormat": ctkSmartCardPINFormatDictionary(operation.pinFormat),
        "apduTemplate": ctkBytes(operation.apduTemplate) as Any,
        "pinByteOffset": operation.pinByteOffset,
        "hasSmartCard": operation.smartCard != nil,
        "pin": operation.pin as Any,
    ]
}

@_cdecl("ctk_token_session_new")
public func ctk_token_session_new(_ tokenPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let tokenPtr else { return nil }
    let token: TKToken = ctkBorrow(tokenPtr)
    return ctkRetain(TKTokenSession(token: token))
}

@_cdecl("ctk_smart_card_token_session_new")
public func ctk_smart_card_token_session_new(_ tokenPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let tokenPtr else { return nil }
    let token: TKSmartCardToken = ctkBorrow(tokenPtr)
    return ctkRetain(TKSmartCardTokenSession(token: token))
}

@_cdecl("ctk_token_session_token_instance_id")
public func ctk_token_session_token_instance_id(
    _ sessionPtr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let sessionPtr else { return nil }
    let session: TKTokenSession = ctkBorrow(sessionPtr)
    guard #available(macOS 10.15, *) else {
        return nil
    }
    return ctkCString(session.token.configuration.instanceID)
}

@_cdecl("ctk_smart_card_token_session_smart_card")
public func ctk_smart_card_token_session_smart_card(
    _ sessionPtr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let sessionPtr else { return nil }
    let session: TKSmartCardTokenSession = ctkBorrow(sessionPtr)
    return ctkRetain(session.smartCard)
}

@_cdecl("ctk_smart_card_token_session_get_smart_card")
public func ctk_smart_card_token_session_get_smart_card(
    _ sessionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let sessionPtr else {
        ctkWriteError(errorOut, "missing smart-card token session handle")
        return nil
    }
    let session: TKSmartCardTokenSession = ctkBorrow(sessionPtr)
    guard #available(macOS 26.0, *) else {
        ctkWriteError(errorOut, "getSmartCard() requires macOS 26.0 or newer")
        return nil
    }
    do {
        return ctkRetain(try session.getSmartCard())
    } catch {
        ctkWriteNSError(errorOut, fallback: "failed to retrieve smart card", error: error)
        return nil
    }
}

@_cdecl("ctk_token_auth_operation_new")
public func ctk_token_auth_operation_new() -> UnsafeMutableRawPointer? {
    ctkRetain(TKTokenAuthOperation())
}

@_cdecl("ctk_token_auth_operation_finish")
public func ctk_token_auth_operation_finish(
    _ operationPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let operationPtr else {
        ctkWriteError(errorOut, "missing token auth operation handle")
        return CTK_INVALID_ARGUMENT
    }
    let operation: TKTokenAuthOperation = ctkBorrow(operationPtr)
    do {
        try operation.finish()
        return CTK_OK
    } catch {
        ctkWriteNSError(errorOut, fallback: "token auth operation failed", error: error)
        return ctkStatus(from: error)
    }
}

@_cdecl("ctk_token_password_auth_operation_new")
public func ctk_token_password_auth_operation_new() -> UnsafeMutableRawPointer? {
    ctkRetain(TKTokenPasswordAuthOperation())
}

@_cdecl("ctk_token_password_auth_operation_password")
public func ctk_token_password_auth_operation_password(
    _ operationPtr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let operationPtr else { return nil }
    let operation: TKTokenPasswordAuthOperation = ctkBorrow(operationPtr)
    guard let password = operation.password else {
        return nil
    }
    return ctkCString(password)
}

@_cdecl("ctk_token_password_auth_operation_set_password")
public func ctk_token_password_auth_operation_set_password(
    _ operationPtr: UnsafeMutableRawPointer?,
    _ password: UnsafePointer<CChar>?,
    _ hasPassword: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let operationPtr else {
        ctkWriteError(errorOut, "missing token password auth operation handle")
        return CTK_INVALID_ARGUMENT
    }
    let operation: TKTokenPasswordAuthOperation = ctkBorrow(operationPtr)
    operation.password = hasPassword ? password.map { String(cString: $0) } : nil
    return CTK_OK
}

@_cdecl("ctk_token_smart_card_pin_auth_operation_new")
public func ctk_token_smart_card_pin_auth_operation_new() -> UnsafeMutableRawPointer? {
    ctkRetain(TKTokenSmartCardPINAuthOperation())
}

@_cdecl("ctk_token_smart_card_pin_auth_operation_json")
public func ctk_token_smart_card_pin_auth_operation_json(
    _ operationPtr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let operationPtr else { return nil }
    let operation: TKTokenSmartCardPINAuthOperation = ctkBorrow(operationPtr)
    return ctkCString(ctkJSONString(ctkTokenSmartCardPinAuthOperationDictionary(operation)))
}

@_cdecl("ctk_token_smart_card_pin_auth_operation_update_json")
public func ctk_token_smart_card_pin_auth_operation_update_json(
    _ operationPtr: UnsafeMutableRawPointer?,
    _ json: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let operationPtr else {
        ctkWriteError(errorOut, "missing smart-card PIN auth operation handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let json, let value = ctkJSONValue(from: json) as? [String: Any] else {
        ctkWriteError(errorOut, "invalid smart-card PIN auth operation JSON payload")
        return CTK_INVALID_ARGUMENT
    }
    let operation: TKTokenSmartCardPINAuthOperation = ctkBorrow(operationPtr)
    if let pinFormat = value["pinFormat"] as? [String: Any] {
        ctkApplySmartCardPINFormat(pinFormat, to: operation.pinFormat)
    }
    operation.apduTemplate = ctkData(from: value["apduTemplate"])
    if let pinByteOffset = ctkNumber(from: value["pinByteOffset"]) {
        operation.pinByteOffset = pinByteOffset.intValue
    }
    operation.pin = ctkString(from: value["pin"])
    return CTK_OK
}

@_cdecl("ctk_token_smart_card_pin_auth_operation_set_smart_card")
public func ctk_token_smart_card_pin_auth_operation_set_smart_card(
    _ operationPtr: UnsafeMutableRawPointer?,
    _ smartCardPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let operationPtr else {
        ctkWriteError(errorOut, "missing smart-card PIN auth operation handle")
        return CTK_INVALID_ARGUMENT
    }
    let operation: TKTokenSmartCardPINAuthOperation = ctkBorrow(operationPtr)
    operation.smartCard = smartCardPtr.map { ctkBorrow($0, as: TKSmartCard.self) }
    return CTK_OK
}
