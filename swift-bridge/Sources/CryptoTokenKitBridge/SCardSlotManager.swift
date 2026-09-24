import CryptoTokenKit
import Foundation

public typealias CTKSlotStateCallback = @convention(c) (UnsafeMutableRawPointer?, Int32) -> Void

private final class CTKSlotStateObserverBox: NSObject {
    let observation: NSKeyValueObservation

    init(
        slot: TKSmartCardSlot,
        callback: @escaping CTKSlotStateCallback,
        context: CTKCallbackContext
    ) {
        self.observation = slot.observe(\.state, options: [.initial, .new]) { slot, _ in
            callback(context.pointer, Int32(slot.state.rawValue))
        }
        super.init()
    }

    deinit {
        observation.invalidate()
    }
}

@_cdecl("ctk_slot_manager_default")
public func ctk_slot_manager_default() -> UnsafeMutableRawPointer? {
    guard let manager = TKSmartCardSlotManager.default else {
        return nil
    }
    return ctkRetain(manager)
}

@_cdecl("ctk_slot_manager_slot_names_json")
public func ctk_slot_manager_slot_names_json(
    _ managerPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outJSON.pointee = nil
    guard let managerPtr else {
        ctkWriteError(errorOut, "missing smart-card slot manager")
        return CTK_INVALID_ARGUMENT
    }
    guard let manager = ctkBorrow(managerPtr, as: TKSmartCardSlotManager.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    outJSON.pointee = ctkCString(ctkJSONString(manager.slotNames))
    return CTK_OK
}

@_cdecl("ctk_slot_manager_slot_named")
public func ctk_slot_manager_slot_named(
    _ managerPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ outSlot: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outSlot.pointee = nil
    guard let managerPtr else {
        ctkWriteError(errorOut, "missing smart-card slot manager")
        return CTK_INVALID_ARGUMENT
    }
    guard let name else {
        ctkWriteError(errorOut, "missing smart-card slot name")
        return CTK_INVALID_ARGUMENT
    }

    guard let manager = ctkBorrow(managerPtr, as: TKSmartCardSlotManager.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    if let slot = manager.slotNamed(String(cString: name)) {
        outSlot.pointee = ctkRetain(slot)
    }
    return CTK_OK
}

@_cdecl("ctk_slot_manager_get_slot_with_name")
public func ctk_slot_manager_get_slot_with_name(
    _ managerPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ outSlot: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outSlot.pointee = nil
    guard let managerPtr else {
        ctkWriteError(errorOut, "missing smart-card slot manager")
        return CTK_INVALID_ARGUMENT
    }
    guard let name else {
        ctkWriteError(errorOut, "missing smart-card slot name")
        return CTK_INVALID_ARGUMENT
    }

    guard let manager = ctkBorrow(managerPtr, as: TKSmartCardSlotManager.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    let pending = CTKPendingReply<TKSmartCardSlot?>()
    manager.getSlot(withName: String(cString: name)) { slot in
        _ = pending.complete(slot)
    }
    guard let slot = pending.wait(seconds: 30) else {
        ctkWriteError(errorOut, "timed out waiting for smart-card slot lookup")
        return CTK_TIMED_OUT
    }
    if let slot {
        outSlot.pointee = ctkRetain(slot)
    }
    return CTK_OK
}

@_cdecl("ctk_slot_name")
public func ctk_slot_name(
    _ slotPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let slot = ctkBorrow(slotPtr, as: TKSmartCardSlot.self, errorOut) else { return nil }
    return ctkCString(slot.name)
}

@_cdecl("ctk_slot_max_input_length")
public func ctk_slot_max_input_length(
    _ slotPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int {
    guard let slot = ctkBorrow(slotPtr, as: TKSmartCardSlot.self, errorOut) else { return 0 }
    return slot.maxInputLength
}

@_cdecl("ctk_slot_max_output_length")
public func ctk_slot_max_output_length(
    _ slotPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int {
    guard let slot = ctkBorrow(slotPtr, as: TKSmartCardSlot.self, errorOut) else { return 0 }
    return slot.maxOutputLength
}

@_cdecl("ctk_slot_state")
public func ctk_slot_state(
    _ slotPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let slot = ctkBorrow(slotPtr, as: TKSmartCardSlot.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    return Int32(slot.state.rawValue)
}

@_cdecl("ctk_slot_atr_json")
public func ctk_slot_atr_json(
    _ slotPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let slot = ctkBorrow(slotPtr, as: TKSmartCardSlot.self, errorOut) else { return nil }
    guard let atr = slot.atr else {
        return nil
    }
    return ctkCString(ctkJSONString(ctkATRDictionary(atr)))
}

@_cdecl("ctk_slot_make_smart_card")
public func ctk_slot_make_smart_card(
    _ slotPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let slot = ctkBorrow(slotPtr, as: TKSmartCardSlot.self, errorOut) else { return nil }
    guard let card = slot.makeSmartCard() else {
        return nil
    }
    return ctkRetain(card)
}

@_cdecl("ctk_slot_observe_state")
public func ctk_slot_observe_state(
    _ slotPtr: UnsafeMutableRawPointer?,
    _ callback: CTKSlotStateCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ release: CTKContextRelease?,
    _ outObserver: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let context = CTKCallbackContext(userInfo, release: release)
    outObserver.pointee = nil
    guard let slotPtr else {
        ctkWriteError(errorOut, "missing smart-card slot")
        return CTK_INVALID_ARGUMENT
    }
    guard let callback else {
        ctkWriteError(errorOut, "missing smart-card slot state callback")
        return CTK_INVALID_ARGUMENT
    }

    guard let slot = ctkBorrow(slotPtr, as: TKSmartCardSlot.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    let observer = CTKSlotStateObserverBox(slot: slot, callback: callback, context: context)
    outObserver.pointee = ctkRetain(observer)
    return CTK_OK
}
