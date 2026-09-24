import CryptoTokenKit
import Foundation

@_cdecl("ctk_smart_card_slot_name")
public func ctk_smart_card_slot_name(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return nil }
    return ctkCString(card.slot.name)
}

@_cdecl("ctk_smart_card_valid")
public func ctk_smart_card_valid(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return false }
    return card.isValid
}

@_cdecl("ctk_smart_card_allowed_protocols")
public func ctk_smart_card_allowed_protocols(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UInt32 {
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return 0 }
    return UInt32(card.allowedProtocols.rawValue)
}

@_cdecl("ctk_smart_card_set_allowed_protocols")
public func ctk_smart_card_set_allowed_protocols(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ protocols: UInt32,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return }
    card.allowedProtocols = TKSmartCardProtocol(rawValue: UInt(protocols))
}

@_cdecl("ctk_smart_card_current_protocol")
public func ctk_smart_card_current_protocol(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UInt32 {
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return 0 }
    return UInt32(card.currentProtocol.rawValue)
}

@_cdecl("ctk_smart_card_sensitive")
public func ctk_smart_card_sensitive(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return false }
    return card.isSensitive
}

@_cdecl("ctk_smart_card_set_sensitive")
public func ctk_smart_card_set_sensitive(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ sensitive: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return }
    card.isSensitive = sensitive
}

@_cdecl("ctk_smart_card_cla")
public func ctk_smart_card_cla(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UInt8 {
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return 0 }
    return card.cla
}

@_cdecl("ctk_smart_card_set_cla")
public func ctk_smart_card_set_cla(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ cla: UInt8,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return }
    card.cla = cla
}

@_cdecl("ctk_smart_card_use_extended_length")
public func ctk_smart_card_use_extended_length(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return false }
    return card.useExtendedLength
}

@_cdecl("ctk_smart_card_set_use_extended_length")
public func ctk_smart_card_set_use_extended_length(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ enabled: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return }
    card.useExtendedLength = enabled
}

@_cdecl("ctk_smart_card_use_command_chaining")
public func ctk_smart_card_use_command_chaining(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return false }
    return card.useCommandChaining
}

@_cdecl("ctk_smart_card_set_use_command_chaining")
public func ctk_smart_card_set_use_command_chaining(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ enabled: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return }
    card.useCommandChaining = enabled
}

@_cdecl("ctk_smart_card_context_json")
public func ctk_smart_card_context_json(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return nil }
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
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return CTK_INVALID_ARGUMENT }
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

    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    let pending = CTKPendingReply<(Bool, Error?)>()
    card.beginSession { success, error in
        if !pending.complete((success, error)), success {
            card.endSession()
        }
    }
    guard let (success, error) = pending.wait(seconds: 30) else {
        ctkWriteError(errorOut, "timed out waiting for smart-card session")
        return CTK_TIMED_OUT
    }
    guard success else {
        ctkWriteNSError(errorOut, fallback: "failed to begin smart-card session", error: error)
        return error.map(ctkStatus(from:)) ?? CTK_FRAMEWORK_ERROR
    }
    return CTK_OK
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

    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    let request = Data(bytes: requestPtr, count: requestLen)
    let pending = CTKPendingReply<(Data?, Error?)>()
    card.transmit(request) { response, error in
        _ = pending.complete((response, error))
    }
    guard let (replyData, replyError) = pending.wait(seconds: 30) else {
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
public func ctk_smart_card_end_session(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return }
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

    guard !hasLE || (0...65536).contains(le) else {
        ctkWriteError(errorOut, "le must be between 0 and 65536")
        return CTK_INVALID_ARGUMENT
    }

    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return CTK_INVALID_ARGUMENT }
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
