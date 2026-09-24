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
    ]
}

private func ctkCopySecret(
    _ secret: String?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>,
    _ outLen: UnsafeMutablePointer<Int>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outBytes.pointee = nil
    outLen.pointee = 0
    guard var secret else { return CTK_OK }
    return secret.withUTF8 { utf8 in
        guard let buffer = malloc(max(utf8.count, 1))?.assumingMemoryBound(to: UInt8.self) else {
            ctkWriteError(errorOut, "failed to allocate a secret buffer")
            return CTK_FRAMEWORK_ERROR
        }
        if let base = utf8.baseAddress {
            memcpy(buffer, base, utf8.count)
        }
        outBytes.pointee = buffer
        outLen.pointee = utf8.count
        return CTK_OK
    }
}

private func ctkSecretString(_ secretPtr: UnsafePointer<UInt8>?, _ secretLen: Int) -> String? {
    guard secretLen >= 0 else { return nil }
    guard let secretPtr else { return secretLen == 0 ? "" : nil }
    return String(decoding: UnsafeBufferPointer(start: secretPtr, count: secretLen), as: UTF8.self)
}

@_cdecl("ctk_token_session_new")
public func ctk_token_session_new(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let token = ctkBorrow(tokenPtr, as: TKToken.self, errorOut) else { return nil }
    return ctkRetain(TKTokenSession(token: token))
}

@_cdecl("ctk_smart_card_token_session_new")
public func ctk_smart_card_token_session_new(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let token = ctkBorrow(tokenPtr, as: TKSmartCardToken.self, errorOut) else { return nil }
    return ctkRetain(TKSmartCardTokenSession(token: token))
}

@_cdecl("ctk_token_session_token_instance_id")
public func ctk_token_session_token_instance_id(
    _ sessionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let session = ctkBorrow(sessionPtr, as: TKTokenSession.self, errorOut) else { return nil }
    guard #available(macOS 10.15, *) else {
        return nil
    }
    return ctkCString(session.token.configuration.instanceID)
}

@_cdecl("ctk_smart_card_token_session_smart_card")
public func ctk_smart_card_token_session_smart_card(
    _ sessionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let session = ctkBorrow(sessionPtr, as: TKSmartCardTokenSession.self, errorOut) else { return nil }
    return ctkRetain(session.smartCard)
}

@_cdecl("ctk_smart_card_token_session_get_smart_card")
public func ctk_smart_card_token_session_get_smart_card(
    _ sessionPtr: UnsafeMutableRawPointer?,
    _ outSmartCard: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outSmartCard.pointee = nil
    guard let sessionPtr else {
        ctkWriteError(errorOut, "missing smart-card token session handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let session = ctkBorrow(sessionPtr, as: TKSmartCardTokenSession.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    guard #available(macOS 26.0, *) else {
        ctkWriteError(errorOut, "getSmartCard() requires macOS 26.0 or newer")
        return CTK_UNSUPPORTED
    }
    do {
        outSmartCard.pointee = ctkRetain(try session.getSmartCard())
        return CTK_OK
    } catch {
        ctkWriteNSError(errorOut, fallback: "failed to retrieve smart card", error: error)
        return ctkStatus(from: error)
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
    guard let operation = ctkBorrow(operationPtr, as: TKTokenAuthOperation.self, errorOut) else { return CTK_INVALID_ARGUMENT }
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
    _ operationPtr: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>,
    _ outLen: UnsafeMutablePointer<Int>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outBytes.pointee = nil
    outLen.pointee = 0
    guard let operation = ctkBorrow(operationPtr, as: TKTokenPasswordAuthOperation.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    return ctkCopySecret(operation.password, outBytes, outLen, errorOut)
}

@_cdecl("ctk_token_password_auth_operation_set_password")
public func ctk_token_password_auth_operation_set_password(
    _ operationPtr: UnsafeMutableRawPointer?,
    _ passwordPtr: UnsafePointer<UInt8>?,
    _ passwordLen: Int,
    _ hasPassword: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let operationPtr else {
        ctkWriteError(errorOut, "missing token password auth operation handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let operation = ctkBorrow(operationPtr, as: TKTokenPasswordAuthOperation.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    guard hasPassword else {
        operation.password = nil
        return CTK_OK
    }
    guard let password = ctkSecretString(passwordPtr, passwordLen) else {
        ctkWriteError(errorOut, "invalid password buffer")
        return CTK_INVALID_ARGUMENT
    }
    operation.password = password
    return CTK_OK
}

@_cdecl("ctk_token_smart_card_pin_auth_operation_new")
public func ctk_token_smart_card_pin_auth_operation_new() -> UnsafeMutableRawPointer? {
    ctkRetain(TKTokenSmartCardPINAuthOperation())
}

@_cdecl("ctk_token_smart_card_pin_auth_operation_json")
public func ctk_token_smart_card_pin_auth_operation_json(
    _ operationPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let operation = ctkBorrow(operationPtr, as: TKTokenSmartCardPINAuthOperation.self, errorOut) else { return nil }
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
    guard let operation = ctkBorrow(operationPtr, as: TKTokenSmartCardPINAuthOperation.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    if let pinFormat = value["pinFormat"] as? [String: Any] {
        ctkApplySmartCardPINFormat(pinFormat, to: operation.pinFormat)
    }
    operation.apduTemplate = ctkData(from: value["apduTemplate"])
    if let pinByteOffset = ctkNumber(from: value["pinByteOffset"]) {
        operation.pinByteOffset = pinByteOffset.intValue
    }
    return CTK_OK
}

@_cdecl("ctk_token_smart_card_pin_auth_operation_pin")
public func ctk_token_smart_card_pin_auth_operation_pin(
    _ operationPtr: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>,
    _ outLen: UnsafeMutablePointer<Int>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outBytes.pointee = nil
    outLen.pointee = 0
    guard let operation = ctkBorrow(operationPtr, as: TKTokenSmartCardPINAuthOperation.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    return ctkCopySecret(operation.pin, outBytes, outLen, errorOut)
}

@_cdecl("ctk_token_smart_card_pin_auth_operation_set_pin")
public func ctk_token_smart_card_pin_auth_operation_set_pin(
    _ operationPtr: UnsafeMutableRawPointer?,
    _ pinPtr: UnsafePointer<UInt8>?,
    _ pinLen: Int,
    _ hasPIN: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let operationPtr else {
        ctkWriteError(errorOut, "missing smart-card PIN auth operation handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let operation = ctkBorrow(operationPtr, as: TKTokenSmartCardPINAuthOperation.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    guard hasPIN else {
        operation.pin = nil
        return CTK_OK
    }
    guard let pin = ctkSecretString(pinPtr, pinLen) else {
        ctkWriteError(errorOut, "invalid PIN buffer")
        return CTK_INVALID_ARGUMENT
    }
    operation.pin = pin
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
    guard let operation = ctkBorrow(operationPtr, as: TKTokenSmartCardPINAuthOperation.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    if let smartCardPtr {
        guard let smartCard = ctkBorrow(smartCardPtr, as: TKSmartCard.self, errorOut) else { return CTK_INVALID_ARGUMENT }
        operation.smartCard = smartCard
    } else {
        operation.smartCard = nil
    }
    return CTK_OK
}
