import CryptoTokenKit
import Foundation

@_cdecl("ctk_smart_card_slot_name")
public func ctk_smart_card_slot_name(
    _ cardPtr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let cardPtr else { return nil }
    let card: TKSmartCard = ctkBorrow(cardPtr)
    return ctkCString(card.slot.name)
}

@_cdecl("ctk_smart_card_valid")
public func ctk_smart_card_valid(_ cardPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let cardPtr else { return false }
    let card: TKSmartCard = ctkBorrow(cardPtr)
    return card.isValid
}

@_cdecl("ctk_smart_card_allowed_protocols")
public func ctk_smart_card_allowed_protocols(_ cardPtr: UnsafeMutableRawPointer?) -> UInt32 {
    guard let cardPtr else { return 0 }
    let card: TKSmartCard = ctkBorrow(cardPtr)
    return UInt32(card.allowedProtocols.rawValue)
}

@_cdecl("ctk_smart_card_set_allowed_protocols")
public func ctk_smart_card_set_allowed_protocols(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ protocols: UInt32
) {
    guard let cardPtr else { return }
    let card: TKSmartCard = ctkBorrow(cardPtr)
    card.allowedProtocols = TKSmartCardProtocol(rawValue: UInt(protocols))
}

@_cdecl("ctk_smart_card_current_protocol")
public func ctk_smart_card_current_protocol(_ cardPtr: UnsafeMutableRawPointer?) -> UInt32 {
    guard let cardPtr else { return 0 }
    let card: TKSmartCard = ctkBorrow(cardPtr)
    return UInt32(card.currentProtocol.rawValue)
}

@_cdecl("ctk_smart_card_sensitive")
public func ctk_smart_card_sensitive(_ cardPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let cardPtr else { return false }
    let card: TKSmartCard = ctkBorrow(cardPtr)
    return card.isSensitive
}

@_cdecl("ctk_smart_card_set_sensitive")
public func ctk_smart_card_set_sensitive(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ sensitive: Bool
) {
    guard let cardPtr else { return }
    let card: TKSmartCard = ctkBorrow(cardPtr)
    card.isSensitive = sensitive
}

@_cdecl("ctk_smart_card_cla")
public func ctk_smart_card_cla(_ cardPtr: UnsafeMutableRawPointer?) -> UInt8 {
    guard let cardPtr else { return 0 }
    let card: TKSmartCard = ctkBorrow(cardPtr)
    return card.cla
}

@_cdecl("ctk_smart_card_set_cla")
public func ctk_smart_card_set_cla(_ cardPtr: UnsafeMutableRawPointer?, _ cla: UInt8) {
    guard let cardPtr else { return }
    let card: TKSmartCard = ctkBorrow(cardPtr)
    card.cla = cla
}

@_cdecl("ctk_smart_card_use_extended_length")
public func ctk_smart_card_use_extended_length(_ cardPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let cardPtr else { return false }
    let card: TKSmartCard = ctkBorrow(cardPtr)
    return card.useExtendedLength
}

@_cdecl("ctk_smart_card_set_use_extended_length")
public func ctk_smart_card_set_use_extended_length(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ enabled: Bool
) {
    guard let cardPtr else { return }
    let card: TKSmartCard = ctkBorrow(cardPtr)
    card.useExtendedLength = enabled
}

@_cdecl("ctk_smart_card_use_command_chaining")
public func ctk_smart_card_use_command_chaining(_ cardPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let cardPtr else { return false }
    let card: TKSmartCard = ctkBorrow(cardPtr)
    return card.useCommandChaining
}

@_cdecl("ctk_smart_card_set_use_command_chaining")
public func ctk_smart_card_set_use_command_chaining(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ enabled: Bool
) {
    guard let cardPtr else { return }
    let card: TKSmartCard = ctkBorrow(cardPtr)
    card.useCommandChaining = enabled
}

@_cdecl("ctk_smart_card_context_json")
public func ctk_smart_card_context_json(
    _ cardPtr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let cardPtr else { return nil }
    let card: TKSmartCard = ctkBorrow(cardPtr)
    if let context = card.context as? CTKRustContextBox {
        return ctkCString(context.json)
    }
    if let context = card.context as? String {
        return ctkCString(context)
    }
    return nil
}

@_cdecl("ctk_smart_card_set_context_json")
public func ctk_smart_card_set_context_json(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ json: UnsafePointer<CChar>?,
    _ hasJSON: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let cardPtr else {
        ctkWriteError(errorOut, "missing smart-card handle")
        return CTK_INVALID_ARGUMENT
    }
    let card: TKSmartCard = ctkBorrow(cardPtr)
    if hasJSON {
        guard let json else {
            ctkWriteError(errorOut, "missing smart-card context JSON")
            return CTK_INVALID_ARGUMENT
        }
        card.context = CTKRustContextBox(json: String(cString: json))
    } else {
        card.context = nil
    }
    return CTK_OK
}

@_cdecl("ctk_smart_card_begin_session")
public func ctk_smart_card_begin_session(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let cardPtr else {
        ctkWriteError(errorOut, "missing smart-card handle")
        return CTK_INVALID_ARGUMENT
    }

    let card: TKSmartCard = ctkBorrow(cardPtr)
    let semaphore = DispatchSemaphore(value: 0)
    var status = CTK_OK
    var callbackError: Error?
    card.beginSession { success, error in
        if !success {
            status = error.map(ctkStatus(from:)) ?? CTK_FRAMEWORK_ERROR
            callbackError = error
        }
        semaphore.signal()
    }
    if semaphore.wait(timeout: .now() + .seconds(30)) == .timedOut {
        ctkWriteError(errorOut, "timed out waiting for smart-card session")
        return CTK_TIMED_OUT
    }
    if status != CTK_OK {
        ctkWriteNSError(errorOut, fallback: "failed to begin smart-card session", error: callbackError)
    }
    return status
}

@_cdecl("ctk_smart_card_transmit_request_json")
public func ctk_smart_card_transmit_request_json(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ requestPtr: UnsafePointer<UInt8>?,
    _ requestLen: Int,
    _ outReplyJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outReplyJSON.pointee = nil
    guard let cardPtr else {
        ctkWriteError(errorOut, "missing smart-card handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let requestPtr else {
        ctkWriteError(errorOut, "missing smart-card request")
        return CTK_INVALID_ARGUMENT
    }

    let card: TKSmartCard = ctkBorrow(cardPtr)
    let request = Data(bytes: requestPtr, count: requestLen)
    let semaphore = DispatchSemaphore(value: 0)
    var replyData: Data?
    var replyError: Error?
    card.transmit(request) { response, error in
        replyData = response
        replyError = error
        semaphore.signal()
    }
    if semaphore.wait(timeout: .now() + .seconds(30)) == .timedOut {
        ctkWriteError(errorOut, "timed out waiting for smart-card transmit")
        return CTK_TIMED_OUT
    }
    if let replyError {
        ctkWriteNSError(errorOut, fallback: "smart-card transmit failed", error: replyError)
        return ctkStatus(from: replyError)
    }
    outReplyJSON.pointee = ctkCString(ctkJSONString([UInt8](replyData ?? Data())))
    return CTK_OK
}

@_cdecl("ctk_smart_card_end_session")
public func ctk_smart_card_end_session(_ cardPtr: UnsafeMutableRawPointer?) {
    guard let cardPtr else { return }
    let card: TKSmartCard = ctkBorrow(cardPtr)
    card.endSession()
}

@_cdecl("ctk_smart_card_send_ins")
public func ctk_smart_card_send_ins(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ ins: UInt8,
    _ p1: UInt8,
    _ p2: UInt8,
    _ dataPtr: UnsafePointer<UInt8>?,
    _ dataLen: Int,
    _ hasLE: Bool,
    _ le: Int,
    _ outReplyJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outReplyJSON.pointee = nil
    guard let cardPtr else {
        ctkWriteError(errorOut, "missing smart-card handle")
        return CTK_INVALID_ARGUMENT
    }

    let card: TKSmartCard = ctkBorrow(cardPtr)
    let requestData = dataPtr.map { Data(bytes: $0, count: dataLen) }
    let expectedLength: Int? = hasLE ? le : nil

    let sessionStatus = ctk_smart_card_begin_session(cardPtr, errorOut)
    if sessionStatus != CTK_OK {
        return sessionStatus
    }
    defer { card.endSession() }

    do {
        let reply = try card.send(ins: ins, p1: p1, p2: p2, data: requestData, le: expectedLength)
        outReplyJSON.pointee = ctkCString(ctkJSONString([
            "data": [UInt8](reply.response),
            "statusWord": reply.sw,
        ]))
        return CTK_OK
    } catch {
        ctkWriteNSError(errorOut, fallback: "CryptoTokenKit APDU exchange failed", error: error)
        return ctkStatus(from: error)
    }
}
