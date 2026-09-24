import CryptoTokenKit
import Foundation

public typealias CTKSmartCardUserInteractionEventCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    Int32
) -> Void

private enum CTKSmartCardUserInteractionEvent: Int32 {
    case characterEntered = 0
    case correctionKeyPressed = 1
    case validationKeyPressed = 2
    case invalidCharacterEntered = 3
    case oldPINRequested = 4
    case newPINRequested = 5
    case newPINConfirmationRequested = 6
}

private final class CTKSmartCardUserInteractionDelegateBox: NSObject, TKSmartCardUserInteractionDelegate {
    private let callback: CTKSmartCardUserInteractionEventCallback
    private let context: CTKCallbackContext

    init(callback: @escaping CTKSmartCardUserInteractionEventCallback, context: CTKCallbackContext) {
        self.callback = callback
        self.context = context
        super.init()
    }

    private func emit(_ event: CTKSmartCardUserInteractionEvent, interaction: TKSmartCardUserInteraction) {
        callback(context.pointer, ctkRetain(interaction), event.rawValue)
    }

    func characterEntered(in interaction: TKSmartCardUserInteraction) {
        emit(.characterEntered, interaction: interaction)
    }

    func correctionKeyPressed(in interaction: TKSmartCardUserInteraction) {
        emit(.correctionKeyPressed, interaction: interaction)
    }

    func validationKeyPressed(in interaction: TKSmartCardUserInteraction) {
        emit(.validationKeyPressed, interaction: interaction)
    }

    func invalidCharacterEntered(in interaction: TKSmartCardUserInteraction) {
        emit(.invalidCharacterEntered, interaction: interaction)
    }

    func oldPINRequested(in interaction: TKSmartCardUserInteraction) {
        emit(.oldPINRequested, interaction: interaction)
    }

    func newPINRequested(in interaction: TKSmartCardUserInteraction) {
        emit(.newPINRequested, interaction: interaction)
    }

    func newPINConfirmationRequested(in interaction: TKSmartCardUserInteraction) {
        emit(.newPINConfirmationRequested, interaction: interaction)
    }

    func emit(_ rawEvent: Int32, interaction: TKSmartCardUserInteraction) {
        guard let event = CTKSmartCardUserInteractionEvent(rawValue: rawEvent) else {
            return
        }
        emit(event, interaction: interaction)
    }
}

private struct CTKMockPinInteractionState {
    var completion = UInt32(TKSmartCardUserInteractionForPINOperation.Completion.key.rawValue)
    var messageIndices: [NSNumber]?
    var localeIdentifier = Locale.current.identifier
    var resultSW: UInt16 = 0
    var resultData: Data?
    var confirmation: UInt32 = 0
}

private let ctkMockPinInteractionStateLock = NSLock()
private var ctkMockPinInteractionStates: [ObjectIdentifier: CTKMockPinInteractionState] = [:]

private func ctkSetMockPinInteractionState(
    for interaction: TKSmartCardUserInteraction,
    _ update: (inout CTKMockPinInteractionState) -> Void
) {
    ctkMockPinInteractionStateLock.lock()
    defer { ctkMockPinInteractionStateLock.unlock() }
    let key = ObjectIdentifier(interaction)
    var state = ctkMockPinInteractionStates[key] ?? CTKMockPinInteractionState()
    update(&state)
    ctkMockPinInteractionStates[key] = state
}

private func ctkMockPinInteractionState(
    for interaction: TKSmartCardUserInteraction
) -> CTKMockPinInteractionState? {
    ctkMockPinInteractionStateLock.lock()
    defer { ctkMockPinInteractionStateLock.unlock() }
    return ctkMockPinInteractionStates[ObjectIdentifier(interaction)]
}

private class CTKMockSmartCardUserInteractionBase: TKSmartCardUserInteraction {
    private var isRunning = false

    override func run(reply: @escaping @Sendable (Bool, (any Error)?) -> Void) {
        isRunning = true
        reply(true, nil)
        isRunning = false
    }

    override func cancel() -> Bool {
        let wasRunning = isRunning
        isRunning = false
        return wasRunning
    }
}

private class CTKMockSmartCardUserInteractionForPINOperation: TKSmartCardUserInteractionForPINOperation {
    private var isRunning = false

    override func run(reply: @escaping @Sendable (Bool, (any Error)?) -> Void) {
        isRunning = true
        reply(true, nil)
        isRunning = false
    }

    override func cancel() -> Bool {
        let wasRunning = isRunning
        isRunning = false
        return wasRunning
    }
}

private final class CTKMockSmartCardUserInteractionForSecurePINVerification: TKSmartCardUserInteractionForSecurePINVerification {
    private var isRunning = false
    private var pinCompletionValue: Completion = .key
    private var pinMessageIndicesValue: [NSNumber]?
    private var localeValue: Locale? = Locale.current
    private var resultSWValue: UInt16 = 0
    private var resultDataValue: Data?

    override var pinCompletion: Completion {
        get { pinCompletionValue }
        set { pinCompletionValue = newValue }
    }

    override var pinMessageIndices: [NSNumber]? {
        get { pinMessageIndicesValue }
        set { pinMessageIndicesValue = newValue }
    }

    override var locale: Locale? {
        get { localeValue }
        set { localeValue = newValue }
    }

    override var resultSW: UInt16 {
        get { resultSWValue }
        set { resultSWValue = newValue }
    }

    override var resultData: Data? {
        get { resultDataValue }
        set { resultDataValue = newValue }
    }

    override func run(reply: @escaping @Sendable (Bool, (any Error)?) -> Void) {
        isRunning = true
        reply(true, nil)
        isRunning = false
    }

    override func cancel() -> Bool {
        let wasRunning = isRunning
        isRunning = false
        return wasRunning
    }
}

private final class CTKMockSmartCardUserInteractionForSecurePINChange: TKSmartCardUserInteractionForSecurePINChange {
    private var isRunning = false
    private var pinCompletionValue: Completion = .key
    private var pinMessageIndicesValue: [NSNumber]?
    private var localeValue: Locale? = Locale.current
    private var resultSWValue: UInt16 = 0
    private var resultDataValue: Data?
    private var pinConfirmationValue: Confirmation = []

    override var pinCompletion: Completion {
        get { pinCompletionValue }
        set { pinCompletionValue = newValue }
    }

    override var pinMessageIndices: [NSNumber]? {
        get { pinMessageIndicesValue }
        set { pinMessageIndicesValue = newValue }
    }

    override var locale: Locale? {
        get { localeValue }
        set { localeValue = newValue }
    }

    override var resultSW: UInt16 {
        get { resultSWValue }
        set { resultSWValue = newValue }
    }

    override var resultData: Data? {
        get { resultDataValue }
        set { resultDataValue = newValue }
    }

    override var pinConfirmation: Confirmation {
        get { pinConfirmationValue }
        set { pinConfirmationValue = newValue }
    }

    override func run(reply: @escaping @Sendable (Bool, (any Error)?) -> Void) {
        isRunning = true
        reply(true, nil)
        isRunning = false
    }

    override func cancel() -> Bool {
        let wasRunning = isRunning
        isRunning = false
        return wasRunning
    }
}

private final class CTKMockSmartCardSlot: TKSmartCardSlot {
    weak var mockCard: CTKMockSmartCard?
    let mockName: String

    init(name: String) {
        self.mockName = name
        super.init()
    }

    override var state: TKSmartCardSlot.State {
        .validCard
    }

    override var atr: TKSmartCardATR? {
        nil
    }

    override var name: String {
        mockName
    }

    override var maxInputLength: Int {
        261
    }

    override var maxOutputLength: Int {
        261
    }

    override func makeSmartCard() -> TKSmartCard? {
        mockCard
    }
}

private final class CTKMockSmartCard: TKSmartCard {
    private let mockSlot: CTKMockSmartCardSlot
    private var validValue = true
    private var allowedProtocolsValue = TKSmartCardProtocol(rawValue: UInt((1 << 16) - 1))
    private var currentProtocolValue = TKSmartCardProtocol.t1
    private var sensitiveValue = false
    private var contextValue: Any?
    private var claValue: UInt8 = 0
    private var useExtendedLengthValue = false
    private var useCommandChainingValue = false
    private var sessionDepth = 0

    init(slotName: String) {
        let slot = CTKMockSmartCardSlot(name: slotName)
        self.mockSlot = slot
        super.init()
        slot.mockCard = self
    }

    override var slot: TKSmartCardSlot {
        mockSlot
    }

    override var isValid: Bool {
        validValue
    }

    override var allowedProtocols: TKSmartCardProtocol {
        get { allowedProtocolsValue }
        set { allowedProtocolsValue = newValue }
    }

    override var currentProtocol: TKSmartCardProtocol {
        currentProtocolValue
    }

    override var isSensitive: Bool {
        get { sensitiveValue }
        set { sensitiveValue = newValue }
    }

    override var context: Any? {
        get { contextValue }
        set { contextValue = newValue }
    }

    override func beginSession(reply: @escaping @Sendable (Bool, (any Error)?) -> Void) {
        sessionDepth += 1
        reply(true, nil)
    }

    override func transmit(_ request: Data, reply: @escaping @Sendable (Data?, (any Error)?) -> Void) {
        reply(request, nil)
    }

    override func endSession() {
        sessionDepth = max(0, sessionDepth - 1)
    }

    var currentSessionDepth: Int {
        sessionDepth
    }

    override func userInteractionForSecurePINVerification(
        _ pinFormat: TKSmartCardPINFormat,
        apdu: Data,
        pinByteOffset: Int
    ) -> TKSmartCardUserInteractionForSecurePINVerification? {
        let interaction = CTKMockSmartCardUserInteractionForSecurePINVerification()
        interaction.initialTimeout = 0
        interaction.interactionTimeout = 0
        interaction.pinCompletion = .key
        interaction.pinMessageIndices = nil
        interaction.resultSW = 0
        interaction.resultData = nil
        ctkSetMockPinInteractionState(for: interaction) { state in
            state.completion = UInt32(TKSmartCardUserInteractionForPINOperation.Completion.key.rawValue)
            state.messageIndices = nil
            state.localeIdentifier = Locale.current.identifier
            state.resultSW = 0
            state.resultData = nil
            state.confirmation = 0
        }
        return interaction
    }

    override func userInteractionForSecurePINChange(
        _ pinFormat: TKSmartCardPINFormat,
        apdu: Data,
        currentPINByteOffset: Int,
        newPINByteOffset: Int
    ) -> TKSmartCardUserInteractionForSecurePINChange? {
        let interaction = CTKMockSmartCardUserInteractionForSecurePINChange()
        interaction.initialTimeout = 0
        interaction.interactionTimeout = 0
        interaction.pinCompletion = .key
        interaction.pinMessageIndices = nil
        interaction.pinConfirmation = []
        interaction.resultSW = 0
        interaction.resultData = nil
        ctkSetMockPinInteractionState(for: interaction) { state in
            state.completion = UInt32(TKSmartCardUserInteractionForPINOperation.Completion.key.rawValue)
            state.messageIndices = nil
            state.localeIdentifier = Locale.current.identifier
            state.resultSW = 0
            state.resultData = nil
            state.confirmation = 0
        }
        return interaction
    }

    override var cla: UInt8 {
        get { claValue }
        set { claValue = newValue }
    }

    override var useExtendedLength: Bool {
        get { useExtendedLengthValue }
        set { useExtendedLengthValue = newValue }
    }

    override var useCommandChaining: Bool {
        get { useCommandChainingValue }
        set { useCommandChainingValue = newValue }
    }
}

@_cdecl("ctk_smart_card_slot")
public func ctk_smart_card_slot(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return nil }
    return ctkRetain(card.slot)
}

@_cdecl("ctk_mock_smart_card_session_depth")
public func ctk_mock_smart_card_session_depth(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int {
    guard let card = ctkBorrow(cardPtr, as: CTKMockSmartCard.self, errorOut) else { return -1 }
    return card.currentSessionDepth
}

@_cdecl("ctk_mock_smart_card_new")
public func ctk_mock_smart_card_new(_ slotName: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let slotName else { return nil }
    return ctkRetain(CTKMockSmartCard(slotName: String(cString: slotName)))
}

@_cdecl("ctk_smart_card_user_interaction_set_delegate")
public func ctk_smart_card_user_interaction_set_delegate(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ callback: CTKSmartCardUserInteractionEventCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ release: CTKContextRelease?,
    _ outDelegate: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let context = CTKCallbackContext(userInfo, release: release)
    outDelegate.pointee = nil
    guard let interactionPtr else {
        ctkWriteError(errorOut, "missing smart-card user-interaction handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let callback else {
        ctkWriteError(errorOut, "missing smart-card user-interaction delegate callback")
        return CTK_INVALID_ARGUMENT
    }
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteraction.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    let box = CTKSmartCardUserInteractionDelegateBox(callback: callback, context: context)
    interaction.delegate = box
    outDelegate.pointee = ctkRetain(box)
    return CTK_OK
}

@_cdecl("ctk_smart_card_user_interaction_clear_delegate")
public func ctk_smart_card_user_interaction_clear_delegate(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteraction.self, errorOut) else { return }
    interaction.delegate = nil
}

@_cdecl("ctk_smart_card_user_interaction_has_delegate")
public func ctk_smart_card_user_interaction_has_delegate(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteraction.self, errorOut) else { return false }
    return interaction.delegate != nil
}

@_cdecl("ctk_smart_card_user_interaction_emit_delegate_event")
public func ctk_smart_card_user_interaction_emit_delegate_event(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ event: Int32,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteraction.self, errorOut) else { return }
    guard let delegate = interaction.delegate as? CTKSmartCardUserInteractionDelegateBox else {
        return
    }
    delegate.emit(event, interaction: interaction)
}

@_cdecl("ctk_smart_card_user_interaction_initial_timeout")
public func ctk_smart_card_user_interaction_initial_timeout(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Double {
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteraction.self, errorOut) else { return 0 }
    return interaction.initialTimeout
}

@_cdecl("ctk_smart_card_user_interaction_set_initial_timeout")
public func ctk_smart_card_user_interaction_set_initial_timeout(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ timeout: Double,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteraction.self, errorOut) else { return }
    interaction.initialTimeout = timeout
}

@_cdecl("ctk_smart_card_user_interaction_interaction_timeout")
public func ctk_smart_card_user_interaction_interaction_timeout(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Double {
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteraction.self, errorOut) else { return 0 }
    return interaction.interactionTimeout
}

@_cdecl("ctk_smart_card_user_interaction_set_interaction_timeout")
public func ctk_smart_card_user_interaction_set_interaction_timeout(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ timeout: Double,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteraction.self, errorOut) else { return }
    interaction.interactionTimeout = timeout
}

@_cdecl("ctk_smart_card_user_interaction_run")
public func ctk_smart_card_user_interaction_run(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let interactionPtr else {
        ctkWriteError(errorOut, "missing smart-card user-interaction handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteraction.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    let configured = interaction.initialTimeout + interaction.interactionTimeout
    let limit = configured.isFinite && configured > 25 ? min(configured, 3600) + 5 : 30
    let pending = CTKPendingReply<(Bool, Error?)>()
    interaction.run { success, error in
        _ = pending.complete((success, error))
    }
    guard let (success, error) = pending.wait(seconds: limit) else {
        _ = interaction.cancel()
        ctkWriteError(errorOut, "timed out waiting for smart-card user interaction")
        return CTK_TIMED_OUT
    }
    guard success else {
        ctkWriteNSError(errorOut, fallback: "smart-card user interaction failed", error: error)
        return error.map(ctkStatus(from:)) ?? CTK_FRAMEWORK_ERROR
    }
    return CTK_OK
}

@_cdecl("ctk_smart_card_user_interaction_cancel")
public func ctk_smart_card_user_interaction_cancel(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteraction.self, errorOut) else { return false }
    return interaction.cancel()
}

@_cdecl("ctk_smart_card_pin_interaction_completion")
public func ctk_smart_card_pin_interaction_completion(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UInt32 {
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteractionForPINOperation.self, errorOut) else { return 0 }
    if let state = ctkMockPinInteractionState(for: interaction) {
        return state.completion
    }
    return UInt32(interaction.pinCompletion.rawValue)
}

@_cdecl("ctk_smart_card_pin_interaction_set_completion")
public func ctk_smart_card_pin_interaction_set_completion(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ completion: UInt32,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteractionForPINOperation.self, errorOut) else { return }
    ctkSetMockPinInteractionState(for: interaction) { $0.completion = completion }
    interaction.pinCompletion = TKSmartCardUserInteractionForPINOperation.Completion(rawValue: UInt(completion))
}

@_cdecl("ctk_smart_card_pin_interaction_message_indices_json")
public func ctk_smart_card_pin_interaction_message_indices_json(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteractionForPINOperation.self, errorOut) else { return nil }
    let messageIndices = ctkMockPinInteractionState(for: interaction)?.messageIndices ?? interaction.pinMessageIndices
    guard let messageIndices else {
        return nil
    }
    return ctkCString(ctkJSONString(messageIndices))
}

@_cdecl("ctk_smart_card_pin_interaction_set_message_indices_json")
public func ctk_smart_card_pin_interaction_set_message_indices_json(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ json: UnsafePointer<CChar>?,
    _ hasJSON: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let interactionPtr else {
        ctkWriteError(errorOut, "missing smart-card PIN interaction handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteractionForPINOperation.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    if hasJSON {
        guard let json,
              let indices = ctkJSONValue(from: json) as? [NSNumber] else {
            ctkWriteError(errorOut, "invalid smart-card PIN message-indices JSON")
            return CTK_INVALID_ARGUMENT
        }
        ctkSetMockPinInteractionState(for: interaction) { $0.messageIndices = indices }
        interaction.pinMessageIndices = indices
    } else {
        ctkSetMockPinInteractionState(for: interaction) { $0.messageIndices = nil }
        interaction.pinMessageIndices = nil
    }
    return CTK_OK
}

@_cdecl("ctk_smart_card_pin_interaction_locale_identifier")
public func ctk_smart_card_pin_interaction_locale_identifier(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteractionForPINOperation.self, errorOut) else { return nil }
    let identifier = ctkMockPinInteractionState(for: interaction)?.localeIdentifier
        ?? interaction.locale?.identifier
        ?? Locale.current.identifier
    return ctkCString(identifier)
}

@_cdecl("ctk_smart_card_pin_interaction_set_locale_identifier")
public func ctk_smart_card_pin_interaction_set_locale_identifier(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ hasIdentifier: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let interactionPtr else {
        ctkWriteError(errorOut, "missing smart-card PIN interaction handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteractionForPINOperation.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    let locale = hasIdentifier && identifier != nil
        ? Locale(identifier: String(cString: identifier!))
        : Locale.current
    ctkSetMockPinInteractionState(for: interaction) { $0.localeIdentifier = locale.identifier }
    interaction.locale = locale
    return CTK_OK
}

@_cdecl("ctk_smart_card_pin_interaction_result_sw")
public func ctk_smart_card_pin_interaction_result_sw(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UInt16 {
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteractionForPINOperation.self, errorOut) else { return 0 }
    return ctkMockPinInteractionState(for: interaction)?.resultSW ?? interaction.resultSW
}

@_cdecl("ctk_smart_card_pin_interaction_result_data_json")
public func ctk_smart_card_pin_interaction_result_data_json(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteractionForPINOperation.self, errorOut) else { return nil }
    let resultData = ctkMockPinInteractionState(for: interaction)?.resultData ?? interaction.resultData
    guard let resultData else {
        return nil
    }
    return ctkCString(ctkJSONString([UInt8](resultData)))
}

@_cdecl("ctk_smart_card_pin_change_interaction_confirmation")
public func ctk_smart_card_pin_change_interaction_confirmation(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UInt32 {
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteractionForSecurePINChange.self, errorOut) else { return 0 }
    return ctkMockPinInteractionState(for: interaction)?.confirmation
        ?? UInt32(interaction.pinConfirmation.rawValue)
}

@_cdecl("ctk_smart_card_pin_change_interaction_set_confirmation")
public func ctk_smart_card_pin_change_interaction_set_confirmation(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ confirmation: UInt32,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let interaction = ctkBorrow(interactionPtr, as: TKSmartCardUserInteractionForSecurePINChange.self, errorOut) else { return }
    ctkSetMockPinInteractionState(for: interaction) { $0.confirmation = confirmation }
    interaction.pinConfirmation = TKSmartCardUserInteractionForSecurePINChange.Confirmation(rawValue: UInt(confirmation))
}

@_cdecl("ctk_smart_card_user_interaction_for_secure_pin_verification")
public func ctk_smart_card_user_interaction_for_secure_pin_verification(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ pinFormatJSON: UnsafePointer<CChar>?,
    _ apduPtr: UnsafePointer<UInt8>?,
    _ apduLen: Int,
    _ pinByteOffset: Int,
    _ outInteraction: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outInteraction.pointee = nil
    guard let cardPtr else {
        ctkWriteError(errorOut, "missing smart-card handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let pinFormatJSON,
          let pinFormatValue = ctkJSONValue(from: pinFormatJSON) as? [String: Any],
          let apduPtr else {
        ctkWriteError(errorOut, "invalid secure-PIN verification arguments")
        return CTK_INVALID_ARGUMENT
    }
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    let pinFormat = TKSmartCardPINFormat()
    ctkApplySmartCardPINFormat(pinFormatValue, to: pinFormat)
    let interaction = card.userInteractionForSecurePINVerification(
        pinFormat,
        apdu: Data(bytes: apduPtr, count: apduLen),
        pinByteOffset: pinByteOffset
    )
    outInteraction.pointee = interaction.map(ctkRetain)
    return CTK_OK
}

@_cdecl("ctk_smart_card_user_interaction_for_secure_pin_change")
public func ctk_smart_card_user_interaction_for_secure_pin_change(
    _ cardPtr: UnsafeMutableRawPointer?,
    _ pinFormatJSON: UnsafePointer<CChar>?,
    _ apduPtr: UnsafePointer<UInt8>?,
    _ apduLen: Int,
    _ currentPINByteOffset: Int,
    _ newPINByteOffset: Int,
    _ outInteraction: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outInteraction.pointee = nil
    guard let cardPtr else {
        ctkWriteError(errorOut, "missing smart-card handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let pinFormatJSON,
          let pinFormatValue = ctkJSONValue(from: pinFormatJSON) as? [String: Any],
          let apduPtr else {
        ctkWriteError(errorOut, "invalid secure-PIN change arguments")
        return CTK_INVALID_ARGUMENT
    }
    guard let card = ctkBorrow(cardPtr, as: TKSmartCard.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    let pinFormat = TKSmartCardPINFormat()
    ctkApplySmartCardPINFormat(pinFormatValue, to: pinFormat)
    let interaction = card.userInteractionForSecurePINChange(
        pinFormat,
        apdu: Data(bytes: apduPtr, count: apduLen),
        currentPINByteOffset: currentPINByteOffset,
        newPINByteOffset: newPINByteOffset
    )
    outInteraction.pointee = interaction.map(ctkRetain)
    return CTK_OK
}
