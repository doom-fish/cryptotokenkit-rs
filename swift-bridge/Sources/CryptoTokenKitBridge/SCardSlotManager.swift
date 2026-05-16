import CryptoTokenKit
import Foundation

public typealias CTKSlotStateCallback = @convention(c) (UnsafeMutableRawPointer?, Int32) -> Void

private final class CTKSlotStateObserverBox: NSObject {
    let observation: NSKeyValueObservation

    init(
        slot: TKSmartCardSlot,
        callback: @escaping CTKSlotStateCallback,
        userInfo: UnsafeMutableRawPointer?
    ) {
        self.observation = slot.observe(\.state, options: [.initial, .new]) { slot, _ in
            callback(userInfo, Int32(slot.state.rawValue))
        }
        super.init()
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
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let managerPtr else {
        ctkWriteError(errorOut, "missing smart-card slot manager")
        return nil
    }
    let manager: TKSmartCardSlotManager = ctkBorrow(managerPtr)
    return ctkCString(ctkJSONString(manager.slotNames))
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

    let manager: TKSmartCardSlotManager = ctkBorrow(managerPtr)
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

    let manager: TKSmartCardSlotManager = ctkBorrow(managerPtr)
    let semaphore = DispatchSemaphore(value: 0)
    manager.getSlot(withName: String(cString: name)) { slot in
        if let slot {
            outSlot.pointee = ctkRetain(slot)
        }
        semaphore.signal()
    }
    if semaphore.wait(timeout: .now() + .seconds(30)) == .timedOut {
        ctkWriteError(errorOut, "timed out waiting for smart-card slot lookup")
        return CTK_TIMED_OUT
    }
    return CTK_OK
}

@_cdecl("ctk_slot_name")
public func ctk_slot_name(_ slotPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let slotPtr else { return nil }
    let slot: TKSmartCardSlot = ctkBorrow(slotPtr)
    return ctkCString(slot.name)
}

@_cdecl("ctk_slot_max_input_length")
public func ctk_slot_max_input_length(_ slotPtr: UnsafeMutableRawPointer?) -> Int {
    guard let slotPtr else { return 0 }
    let slot: TKSmartCardSlot = ctkBorrow(slotPtr)
    return slot.maxInputLength
}

@_cdecl("ctk_slot_max_output_length")
public func ctk_slot_max_output_length(_ slotPtr: UnsafeMutableRawPointer?) -> Int {
    guard let slotPtr else { return 0 }
    let slot: TKSmartCardSlot = ctkBorrow(slotPtr)
    return slot.maxOutputLength
}

@_cdecl("ctk_slot_state")
public func ctk_slot_state(_ slotPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let slotPtr else { return Int32(TKSmartCardSlot.State.missing.rawValue) }
    let slot: TKSmartCardSlot = ctkBorrow(slotPtr)
    return Int32(slot.state.rawValue)
}

@_cdecl("ctk_slot_atr_json")
public func ctk_slot_atr_json(_ slotPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let slotPtr else { return nil }
    let slot: TKSmartCardSlot = ctkBorrow(slotPtr)
    guard let atr = slot.atr else {
        return nil
    }
    return ctkCString(ctkJSONString(ctkATRDictionary(atr)))
}

@_cdecl("ctk_slot_make_smart_card")
public func ctk_slot_make_smart_card(_ slotPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let slotPtr else { return nil }
    let slot: TKSmartCardSlot = ctkBorrow(slotPtr)
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
    _ outObserver: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outObserver.pointee = nil
    guard let slotPtr else {
        ctkWriteError(errorOut, "missing smart-card slot")
        return CTK_INVALID_ARGUMENT
    }
    guard let callback else {
        ctkWriteError(errorOut, "missing smart-card slot state callback")
        return CTK_INVALID_ARGUMENT
    }

    let slot: TKSmartCardSlot = ctkBorrow(slotPtr)
    let observer = CTKSlotStateObserverBox(slot: slot, callback: callback, userInfo: userInfo)
    outObserver.pointee = ctkRetain(observer)
    return CTK_OK
}
