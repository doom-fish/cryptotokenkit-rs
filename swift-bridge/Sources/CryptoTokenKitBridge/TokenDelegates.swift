import CryptoTokenKit
import Foundation

public typealias CTKTokenSessionBeginAuthCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    Int32,
    UnsafePointer<CChar>?,
    UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32

public typealias CTKTokenSessionSupportsCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    Int32,
    UnsafePointer<CChar>?,
    UnsafeMutableRawPointer?
) -> Bool

public typealias CTKTokenSessionDataCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafePointer<UInt8>?,
    Int,
    UnsafePointer<CChar>?,
    UnsafeMutableRawPointer?,
    UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    UnsafeMutablePointer<Int>?,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32

public typealias CTKTokenSessionKeyExchangeCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafePointer<UInt8>?,
    Int,
    UnsafePointer<CChar>?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    UnsafeMutablePointer<Int>?,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32

public typealias CTKTokenCreateSessionCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32

public typealias CTKTokenTerminateSessionCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?
) -> Void

public typealias CTKTokenDriverCreateTokenCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?,
    UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32

public typealias CTKTokenDriverTerminateTokenCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?
) -> Void

public typealias CTKSmartCardTokenDriverCreateTokenCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafePointer<UInt8>?,
    Int,
    Bool,
    UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32

public typealias CTKSmartCardTokenDriverTerminateTokenCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?
) -> Void

private func ctkConsumeRetained<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as type: T.Type) throws -> T {
    guard let object = Unmanaged<AnyObject>.fromOpaque(ptr).takeRetainedValue() as? T else {
        throw ctkNSError(status: CTK_INVALID_ARGUMENT, message: "the Rust delegate returned a handle that is not a \(type)")
    }
    return object
}

private func ctkConfigurationJSONString(_ configuration: [String: Any]) -> String {
    ctkJSONString(ctkJSONCompatibleValue(configuration))
}

private final class CTKTokenSessionDelegateBox: NSObject, TKTokenSessionDelegate {
    private let beginAuthCallback: CTKTokenSessionBeginAuthCallback
    private let supportsCallback: CTKTokenSessionSupportsCallback
    private let signCallback: CTKTokenSessionDataCallback
    private let decryptCallback: CTKTokenSessionDataCallback
    private let keyExchangeCallback: CTKTokenSessionKeyExchangeCallback
    private let context: CTKCallbackContext

    init(
        beginAuthCallback: @escaping CTKTokenSessionBeginAuthCallback,
        supportsCallback: @escaping CTKTokenSessionSupportsCallback,
        signCallback: @escaping CTKTokenSessionDataCallback,
        decryptCallback: @escaping CTKTokenSessionDataCallback,
        keyExchangeCallback: @escaping CTKTokenSessionKeyExchangeCallback,
        context: CTKCallbackContext
    ) {
        self.beginAuthCallback = beginAuthCallback
        self.supportsCallback = supportsCallback
        self.signCallback = signCallback
        self.decryptCallback = decryptCallback
        self.keyExchangeCallback = keyExchangeCallback
        self.context = context
        super.init()
    }

    func invokeBeginAuth(
        session: TKTokenSession,
        operation: TKTokenOperation,
        constraintJSON: String,
        errorOut: NSErrorPointer = nil
    ) -> UnsafeMutableRawPointer? {
        var rawOperation: UnsafeMutableRawPointer?
        var errorCString: UnsafeMutablePointer<CChar>?
        let status = constraintJSON.withCString { json in
            beginAuthCallback(
                context.pointer,
                ctkRetain(session),
                Int32(operation.rawValue),
                json,
                &rawOperation,
                &errorCString
            )
        }
        guard status == CTK_OK else {
            ctkWriteCallbackError(
                errorOut,
                status: status,
                errorCString: errorCString,
                fallback: "token-session delegate begin-auth failed"
            )
            return nil
        }
        return rawOperation
    }

    func invokeSupports(
        session: TKTokenSession,
        operation: TKTokenOperation,
        objectID: String,
        algorithm: TKTokenKeyAlgorithm
    ) -> Bool {
        objectID.withCString { objectIDCString in
            supportsCallback(
                context.pointer,
                ctkRetain(session),
                Int32(operation.rawValue),
                objectIDCString,
                ctkRetain(algorithm)
            )
        }
    }

    private func invokeDataCallback(
        callback: CTKTokenSessionDataCallback,
        session: TKTokenSession,
        data: Data,
        objectID: String,
        algorithm: TKTokenKeyAlgorithm
    ) throws -> Data {
        var replyBytes: UnsafeMutablePointer<UInt8>?
        var replyLength = 0
        var errorCString: UnsafeMutablePointer<CChar>?
        let status = data.withUnsafeBytes { bytes in
            objectID.withCString { objectIDCString in
                callback(
                    context.pointer,
                    ctkRetain(session),
                    bytes.baseAddress?.assumingMemoryBound(to: UInt8.self),
                    data.count,
                    objectIDCString,
                    ctkRetain(algorithm),
                    &replyBytes,
                    &replyLength,
                    &errorCString
                )
            }
        }
        let reply = ctkTakeSecretBuffer(replyBytes, replyLength)
        guard status == CTK_OK else {
            throw ctkNSError(
                status: status,
                message: ctkTakeCString(errorCString).ifEmpty("token-session delegate data operation failed")
            )
        }
        return reply
    }

    func invokeKeyExchange(
        session: TKTokenSession,
        publicKeyData: Data,
        objectID: String,
        algorithm: TKTokenKeyAlgorithm,
        parameters: TKTokenKeyExchangeParameters
    ) throws -> Data {
        var replyBytes: UnsafeMutablePointer<UInt8>?
        var replyLength = 0
        var errorCString: UnsafeMutablePointer<CChar>?
        let status = publicKeyData.withUnsafeBytes { bytes in
            objectID.withCString { objectIDCString in
                keyExchangeCallback(
                    context.pointer,
                    ctkRetain(session),
                    bytes.baseAddress?.assumingMemoryBound(to: UInt8.self),
                    publicKeyData.count,
                    objectIDCString,
                    ctkRetain(algorithm),
                    ctkRetain(parameters),
                    &replyBytes,
                    &replyLength,
                    &errorCString
                )
            }
        }
        let reply = ctkTakeSecretBuffer(replyBytes, replyLength)
        guard status == CTK_OK else {
            throw ctkNSError(
                status: status,
                message: ctkTakeCString(errorCString).ifEmpty("token-session delegate key exchange failed")
            )
        }
        return reply
    }

    func tokenSession(
        _ session: TKTokenSession,
        beginAuthFor operation: TKTokenOperation,
        constraint: Any
    ) throws -> TKTokenAuthOperation {
        var error: NSError?
        let json = ctkJSONString(ctkJSONCompatibleValue(constraint))
        guard let rawOperation = invokeBeginAuth(
            session: session,
            operation: operation,
            constraintJSON: json,
            errorOut: &error
        ) else {
            throw error ?? ctkNSError(status: CTK_FRAMEWORK_ERROR, message: "token-session delegate returned no auth operation")
        }
        return try ctkConsumeRetained(rawOperation, as: TKTokenAuthOperation.self)
    }

    func tokenSession(
        _ session: TKTokenSession,
        supports operation: TKTokenOperation,
        keyObjectID: TKToken.ObjectID,
        algorithm: TKTokenKeyAlgorithm
    ) -> Bool {
        invokeSupports(
            session: session,
            operation: operation,
            objectID: ctkObjectIDString(keyObjectID),
            algorithm: algorithm
        )
    }

    func tokenSession(
        _ session: TKTokenSession,
        sign dataToSign: Data,
        keyObjectID: TKToken.ObjectID,
        algorithm: TKTokenKeyAlgorithm
    ) throws -> Data {
        try invokeDataCallback(
            callback: signCallback,
            session: session,
            data: dataToSign,
            objectID: ctkObjectIDString(keyObjectID),
            algorithm: algorithm
        )
    }

    func tokenSession(
        _ session: TKTokenSession,
        decrypt ciphertext: Data,
        keyObjectID: TKToken.ObjectID,
        algorithm: TKTokenKeyAlgorithm
    ) throws -> Data {
        try invokeDataCallback(
            callback: decryptCallback,
            session: session,
            data: ciphertext,
            objectID: ctkObjectIDString(keyObjectID),
            algorithm: algorithm
        )
    }

    func tokenSession(
        _ session: TKTokenSession,
        performKeyExchange otherPartyPublicKeyData: Data,
        keyObjectID objectID: TKToken.ObjectID,
        algorithm: TKTokenKeyAlgorithm,
        parameters: TKTokenKeyExchangeParameters
    ) throws -> Data {
        try invokeKeyExchange(
            session: session,
            publicKeyData: otherPartyPublicKeyData,
            objectID: ctkObjectIDString(objectID),
            algorithm: algorithm,
            parameters: parameters
        )
    }
}

private final class CTKTokenDelegateBox: NSObject, TKTokenDelegate {
    private let createSessionCallback: CTKTokenCreateSessionCallback
    private let terminateSessionCallback: CTKTokenTerminateSessionCallback
    private let context: CTKCallbackContext

    init(
        createSessionCallback: @escaping CTKTokenCreateSessionCallback,
        terminateSessionCallback: @escaping CTKTokenTerminateSessionCallback,
        context: CTKCallbackContext
    ) {
        self.createSessionCallback = createSessionCallback
        self.terminateSessionCallback = terminateSessionCallback
        self.context = context
        super.init()
    }

    func invokeCreateSession(
        token: TKToken,
        errorOut: NSErrorPointer = nil
    ) -> UnsafeMutableRawPointer? {
        var rawSession: UnsafeMutableRawPointer?
        var errorCString: UnsafeMutablePointer<CChar>?
        let status = createSessionCallback(context.pointer, ctkRetain(token), &rawSession, &errorCString)
        guard status == CTK_OK else {
            ctkWriteCallbackError(
                errorOut,
                status: status,
                errorCString: errorCString,
                fallback: "token delegate failed to create session"
            )
            return nil
        }
        return rawSession
    }

    func createSession(_ token: TKToken) throws -> TKTokenSession {
        var error: NSError?
        guard let rawSession = invokeCreateSession(token: token, errorOut: &error) else {
            throw error ?? ctkNSError(status: CTK_FRAMEWORK_ERROR, message: "token delegate returned no session")
        }
        return try ctkConsumeRetained(rawSession, as: TKTokenSession.self)
    }

    func token(_ token: TKToken, terminateSession session: TKTokenSession) {
        terminateSessionCallback(context.pointer, ctkRetain(token), ctkRetain(session))
    }
}

private final class CTKTokenDriverDelegateBox: NSObject, TKTokenDriverDelegate {
    private let createTokenCallback: CTKTokenDriverCreateTokenCallback
    private let terminateTokenCallback: CTKTokenDriverTerminateTokenCallback
    private let context: CTKCallbackContext

    init(
        createTokenCallback: @escaping CTKTokenDriverCreateTokenCallback,
        terminateTokenCallback: @escaping CTKTokenDriverTerminateTokenCallback,
        context: CTKCallbackContext
    ) {
        self.createTokenCallback = createTokenCallback
        self.terminateTokenCallback = terminateTokenCallback
        self.context = context
        super.init()
    }

    func invokeTokenForConfiguration(
        driver: TKTokenDriver,
        configurationJSON: String,
        errorOut: NSErrorPointer = nil
    ) -> UnsafeMutableRawPointer? {
        var rawToken: UnsafeMutableRawPointer?
        var errorCString: UnsafeMutablePointer<CChar>?
        let status = configurationJSON.withCString { json in
            createTokenCallback(context.pointer, ctkRetain(driver), json, &rawToken, &errorCString)
        }
        guard status == CTK_OK else {
            ctkWriteCallbackError(
                errorOut,
                status: status,
                errorCString: errorCString,
                fallback: "token-driver delegate failed to create token"
            )
            return nil
        }
        return rawToken
    }

    @available(macOS 10.15, *)
    func tokenDriver(
        _ driver: TKTokenDriver,
        tokenFor configuration: TKToken.Configuration
    ) throws -> TKToken {
        var error: NSError?
        let configurationJSON = ctkConfigurationJSONString(
            ctkTokenConfigurationDictionary(configuration, keychainContents: nil)
        )
        guard let rawToken = invokeTokenForConfiguration(
            driver: driver,
            configurationJSON: configurationJSON,
            errorOut: &error
        ) else {
            throw error ?? ctkNSError(status: CTK_FRAMEWORK_ERROR, message: "token-driver delegate returned no token")
        }
        return try ctkConsumeRetained(rawToken, as: TKToken.self)
    }

    func tokenDriver(_ driver: TKTokenDriver, terminateToken token: TKToken) {
        terminateTokenCallback(context.pointer, ctkRetain(driver), ctkRetain(token))
    }
}

private final class CTKSmartCardTokenDriverDelegateBox: NSObject, TKSmartCardTokenDriverDelegate {
    private let createTokenCallback: CTKSmartCardTokenDriverCreateTokenCallback
    private let terminateTokenCallback: CTKSmartCardTokenDriverTerminateTokenCallback
    private let context: CTKCallbackContext

    init(
        createTokenCallback: @escaping CTKSmartCardTokenDriverCreateTokenCallback,
        terminateTokenCallback: @escaping CTKSmartCardTokenDriverTerminateTokenCallback,
        context: CTKCallbackContext
    ) {
        self.createTokenCallback = createTokenCallback
        self.terminateTokenCallback = terminateTokenCallback
        self.context = context
        super.init()
    }

    func invokeCreateToken(
        driver: TKSmartCardTokenDriver,
        smartCard: TKSmartCard,
        aid: Data?,
        errorOut: NSErrorPointer = nil
    ) -> UnsafeMutableRawPointer? {
        var rawToken: UnsafeMutableRawPointer?
        var errorCString: UnsafeMutablePointer<CChar>?
        let status = aid.map({ aid in
            aid.withUnsafeBytes { bytes in
                createTokenCallback(
                    context.pointer,
                    ctkRetain(driver),
                    ctkRetain(smartCard),
                    bytes.baseAddress?.assumingMemoryBound(to: UInt8.self),
                    aid.count,
                    true,
                    &rawToken,
                    &errorCString
                )
            }
        }) ?? createTokenCallback(
            context.pointer,
            ctkRetain(driver),
            ctkRetain(smartCard),
            nil,
            0,
            false,
            &rawToken,
            &errorCString
        )
        guard status == CTK_OK else {
            ctkWriteCallbackError(
                errorOut,
                status: status,
                errorCString: errorCString,
                fallback: "smart-card token-driver delegate failed to create token"
            )
            return nil
        }
        return rawToken
    }

    func tokenDriver(
        _ driver: TKSmartCardTokenDriver,
        createTokenFor smartCard: TKSmartCard,
        aid AID: Data?
    ) throws -> TKSmartCardToken {
        var error: NSError?
        guard let rawToken = invokeCreateToken(
            driver: driver,
            smartCard: smartCard,
            aid: AID,
            errorOut: &error
        ) else {
            throw error ?? ctkNSError(status: CTK_FRAMEWORK_ERROR, message: "smart-card token-driver delegate returned no token")
        }
        return try ctkConsumeRetained(rawToken, as: TKSmartCardToken.self)
    }

    func tokenDriver(_ driver: TKTokenDriver, terminateToken token: TKToken) {
        terminateTokenCallback(context.pointer, ctkRetain(driver), ctkRetain(token))
    }
}

@_cdecl("ctk_token_session_set_delegate")
public func ctk_token_session_set_delegate(
    _ sessionPtr: UnsafeMutableRawPointer?,
    _ beginAuthCallback: CTKTokenSessionBeginAuthCallback?,
    _ supportsCallback: CTKTokenSessionSupportsCallback?,
    _ signCallback: CTKTokenSessionDataCallback?,
    _ decryptCallback: CTKTokenSessionDataCallback?,
    _ keyExchangeCallback: CTKTokenSessionKeyExchangeCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ release: CTKContextRelease?,
    _ outDelegate: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let context = CTKCallbackContext(userInfo, release: release)
    outDelegate.pointee = nil
    guard let sessionPtr else {
        ctkWriteError(errorOut, "missing token-session handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let beginAuthCallback,
          let supportsCallback,
          let signCallback,
          let decryptCallback,
          let keyExchangeCallback else {
        ctkWriteError(errorOut, "missing token-session delegate callbacks")
        return CTK_INVALID_ARGUMENT
    }
    guard let session = ctkBorrow(sessionPtr, as: TKTokenSession.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    let box = CTKTokenSessionDelegateBox(
        beginAuthCallback: beginAuthCallback,
        supportsCallback: supportsCallback,
        signCallback: signCallback,
        decryptCallback: decryptCallback,
        keyExchangeCallback: keyExchangeCallback,
        context: context
    )
    session.delegate = box
    outDelegate.pointee = ctkRetain(box)
    return CTK_OK
}

@_cdecl("ctk_token_session_clear_delegate")
public func ctk_token_session_clear_delegate(
    _ sessionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let session = ctkBorrow(sessionPtr, as: TKTokenSession.self, errorOut) else { return }
    session.delegate = nil
}

@_cdecl("ctk_token_session_has_delegate")
public func ctk_token_session_has_delegate(
    _ sessionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let session = ctkBorrow(sessionPtr, as: TKTokenSession.self, errorOut) else { return false }
    return session.delegate != nil
}

@_cdecl("ctk_token_session_token")
public func ctk_token_session_token(
    _ sessionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let session = ctkBorrow(sessionPtr, as: TKTokenSession.self, errorOut) else { return nil }
    return ctkRetain(session.token)
}

@_cdecl("ctk_token_session_invoke_delegate_begin_auth")
public func ctk_token_session_invoke_delegate_begin_auth(
    _ sessionPtr: UnsafeMutableRawPointer?,
    _ operation: Int32,
    _ constraintJSON: UnsafePointer<CChar>?,
    _ outOperation: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outOperation.pointee = nil
    guard let sessionPtr else {
        ctkWriteError(errorOut, "missing token-session handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let constraintJSON else {
        ctkWriteError(errorOut, "missing token-session constraint JSON")
        return CTK_INVALID_ARGUMENT
    }
    guard let session = ctkBorrow(sessionPtr, as: TKTokenSession.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    guard let delegate = session.delegate as? CTKTokenSessionDelegateBox else {
        ctkWriteError(errorOut, "token-session delegate is not managed by the Rust bridge")
        return CTK_FRAMEWORK_ERROR
    }
    var callbackError: NSError?
    let raw = delegate.invokeBeginAuth(
        session: session,
        operation: TKTokenOperation(rawValue: Int(operation)) ?? .none,
        constraintJSON: String(cString: constraintJSON),
        errorOut: &callbackError
    )
    if let callbackError {
        ctkWriteNSError(errorOut, fallback: "token-session delegate begin-auth failed", error: callbackError)
        return ctkStatus(from: callbackError)
    }
    outOperation.pointee = raw
    return CTK_OK
}

@_cdecl("ctk_token_session_invoke_delegate_supports")
public func ctk_token_session_invoke_delegate_supports(
    _ sessionPtr: UnsafeMutableRawPointer?,
    _ operation: Int32,
    _ objectID: UnsafePointer<CChar>?,
    _ baseAlgorithm: UnsafePointer<CChar>?,
    _ supportedAlgorithmsJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let session = ctkBorrow(sessionPtr, as: TKTokenSession.self, errorOut) else { return false }
    guard let objectID,
          let algorithm = ctkMockTokenKeyAlgorithm(
              baseAlgorithm: baseAlgorithm,
              supportedAlgorithmsJSON: supportedAlgorithmsJSON
          ) else {
        ctkWriteError(errorOut, "invalid token-session delegate invocation")
        return false
    }
    guard let delegate = session.delegate as? CTKTokenSessionDelegateBox else {
        return false
    }
    return delegate.invokeSupports(
        session: session,
        operation: TKTokenOperation(rawValue: Int(operation)) ?? .none,
        objectID: String(cString: objectID),
        algorithm: algorithm
    )
}

private func ctkInvokeTokenSessionDataDelegate(
    sessionPtr: UnsafeMutableRawPointer?,
    operation: Int32,
    requestPtr: UnsafePointer<UInt8>?,
    requestLen: Int,
    objectID: UnsafePointer<CChar>?,
    baseAlgorithm: UnsafePointer<CChar>?,
    supportedAlgorithmsJSON: UnsafePointer<CChar>?,
    outReplyBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>,
    outReplyLength: UnsafeMutablePointer<Int>,
    errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    callback: (CTKTokenSessionDelegateBox, TKTokenSession, Data, String, TKTokenKeyAlgorithm) throws -> Data
) -> Int32 {
    outReplyBytes.pointee = nil
    outReplyLength.pointee = 0
    guard let sessionPtr, let requestPtr, let objectID,
          let algorithm = ctkMockTokenKeyAlgorithm(
              baseAlgorithm: baseAlgorithm,
              supportedAlgorithmsJSON: supportedAlgorithmsJSON
          ) else {
        ctkWriteError(errorOut, "invalid token-session delegate invocation")
        return CTK_INVALID_ARGUMENT
    }
    guard let session = ctkBorrow(sessionPtr, as: TKTokenSession.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    guard let delegate = session.delegate as? CTKTokenSessionDelegateBox else {
        ctkWriteError(errorOut, "token-session delegate is not managed by the Rust bridge")
        return CTK_FRAMEWORK_ERROR
    }
    do {
        var reply = try callback(
            delegate,
            session,
            Data(bytes: requestPtr, count: requestLen),
            String(cString: objectID),
            algorithm
        )
        return ctkCopySecretBuffer(&reply, outReplyBytes, outReplyLength, errorOut)
    } catch {
        ctkWriteNSError(errorOut, fallback: "token-session delegate invocation failed", error: error)
        return ctkStatus(from: error)
    }
}

@_cdecl("ctk_token_session_invoke_delegate_sign")
public func ctk_token_session_invoke_delegate_sign(
    _ sessionPtr: UnsafeMutableRawPointer?,
    _ operation: Int32,
    _ requestPtr: UnsafePointer<UInt8>?,
    _ requestLen: Int,
    _ objectID: UnsafePointer<CChar>?,
    _ baseAlgorithm: UnsafePointer<CChar>?,
    _ supportedAlgorithmsJSON: UnsafePointer<CChar>?,
    _ outReplyBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>,
    _ outReplyLength: UnsafeMutablePointer<Int>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ctkInvokeTokenSessionDataDelegate(
        sessionPtr: sessionPtr,
        operation: operation,
        requestPtr: requestPtr,
        requestLen: requestLen,
        objectID: objectID,
        baseAlgorithm: baseAlgorithm,
        supportedAlgorithmsJSON: supportedAlgorithmsJSON,
        outReplyBytes: outReplyBytes,
        outReplyLength: outReplyLength,
        errorOut: errorOut
    ) { delegate, session, data, objectID, algorithm in
        try delegate.tokenSession(
            session,
            sign: data,
            keyObjectID: objectID,
            algorithm: algorithm
        )
    }
}

@_cdecl("ctk_token_session_invoke_delegate_decrypt")
public func ctk_token_session_invoke_delegate_decrypt(
    _ sessionPtr: UnsafeMutableRawPointer?,
    _ operation: Int32,
    _ requestPtr: UnsafePointer<UInt8>?,
    _ requestLen: Int,
    _ objectID: UnsafePointer<CChar>?,
    _ baseAlgorithm: UnsafePointer<CChar>?,
    _ supportedAlgorithmsJSON: UnsafePointer<CChar>?,
    _ outReplyBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>,
    _ outReplyLength: UnsafeMutablePointer<Int>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ctkInvokeTokenSessionDataDelegate(
        sessionPtr: sessionPtr,
        operation: operation,
        requestPtr: requestPtr,
        requestLen: requestLen,
        objectID: objectID,
        baseAlgorithm: baseAlgorithm,
        supportedAlgorithmsJSON: supportedAlgorithmsJSON,
        outReplyBytes: outReplyBytes,
        outReplyLength: outReplyLength,
        errorOut: errorOut
    ) { delegate, session, data, objectID, algorithm in
        try delegate.tokenSession(
            session,
            decrypt: data,
            keyObjectID: objectID,
            algorithm: algorithm
        )
    }
}

@_cdecl("ctk_token_session_invoke_delegate_key_exchange")
public func ctk_token_session_invoke_delegate_key_exchange(
    _ sessionPtr: UnsafeMutableRawPointer?,
    _ publicKeyPtr: UnsafePointer<UInt8>?,
    _ publicKeyLen: Int,
    _ objectID: UnsafePointer<CChar>?,
    _ baseAlgorithm: UnsafePointer<CChar>?,
    _ supportedAlgorithmsJSON: UnsafePointer<CChar>?,
    _ requestedSize: Int,
    _ sharedInfoPtr: UnsafePointer<UInt8>?,
    _ sharedInfoLen: Int,
    _ hasSharedInfo: Bool,
    _ outReplyBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>,
    _ outReplyLength: UnsafeMutablePointer<Int>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outReplyBytes.pointee = nil
    outReplyLength.pointee = 0
    guard let sessionPtr, let publicKeyPtr, let objectID,
          let algorithm = ctkMockTokenKeyAlgorithm(
              baseAlgorithm: baseAlgorithm,
              supportedAlgorithmsJSON: supportedAlgorithmsJSON
          ) else {
        ctkWriteError(errorOut, "invalid token-session delegate key-exchange invocation")
        return CTK_INVALID_ARGUMENT
    }
    guard let session = ctkBorrow(sessionPtr, as: TKTokenSession.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    guard let delegate = session.delegate as? CTKTokenSessionDelegateBox else {
        ctkWriteError(errorOut, "token-session delegate is not managed by the Rust bridge")
        return CTK_FRAMEWORK_ERROR
    }
    let parameters = ctkMockTokenKeyExchangeParameters(
        requestedSize: requestedSize,
        sharedInfoPtr: sharedInfoPtr,
        sharedInfoLen: sharedInfoLen,
        hasSharedInfo: hasSharedInfo
    )
    do {
        var reply = try delegate.tokenSession(
            session,
            performKeyExchange: Data(bytes: publicKeyPtr, count: publicKeyLen),
            keyObjectID: String(cString: objectID),
            algorithm: algorithm,
            parameters: parameters
        )
        return ctkCopySecretBuffer(&reply, outReplyBytes, outReplyLength, errorOut)
    } catch {
        ctkWriteNSError(errorOut, fallback: "token-session delegate key exchange failed", error: error)
        return ctkStatus(from: error)
    }
}

@_cdecl("ctk_token_set_delegate")
public func ctk_token_set_delegate(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ createSessionCallback: CTKTokenCreateSessionCallback?,
    _ terminateSessionCallback: CTKTokenTerminateSessionCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ release: CTKContextRelease?,
    _ outDelegate: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let context = CTKCallbackContext(userInfo, release: release)
    outDelegate.pointee = nil
    guard let tokenPtr else {
        ctkWriteError(errorOut, "missing token handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let createSessionCallback, let terminateSessionCallback else {
        ctkWriteError(errorOut, "missing token delegate callbacks")
        return CTK_INVALID_ARGUMENT
    }
    guard let token = ctkBorrow(tokenPtr, as: TKToken.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    let box = CTKTokenDelegateBox(
        createSessionCallback: createSessionCallback,
        terminateSessionCallback: terminateSessionCallback,
        context: context
    )
    token.delegate = box
    outDelegate.pointee = ctkRetain(box)
    return CTK_OK
}

@_cdecl("ctk_token_clear_delegate")
public func ctk_token_clear_delegate(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let token = ctkBorrow(tokenPtr, as: TKToken.self, errorOut) else { return }
    token.delegate = nil
}

@_cdecl("ctk_token_has_delegate")
public func ctk_token_has_delegate(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let token = ctkBorrow(tokenPtr, as: TKToken.self, errorOut) else { return false }
    return token.delegate != nil
}

@_cdecl("ctk_token_token_driver")
public func ctk_token_token_driver(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let token = ctkBorrow(tokenPtr, as: TKToken.self, errorOut) else { return nil }
    return ctkRetain(token.tokenDriver)
}

@_cdecl("ctk_token_invoke_delegate_create_session")
public func ctk_token_invoke_delegate_create_session(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ outSession: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outSession.pointee = nil
    guard let tokenPtr else {
        ctkWriteError(errorOut, "missing token handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let token = ctkBorrow(tokenPtr, as: TKToken.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    guard let delegate = token.delegate as? CTKTokenDelegateBox else {
        ctkWriteError(errorOut, "token delegate is not managed by the Rust bridge")
        return CTK_FRAMEWORK_ERROR
    }
    var callbackError: NSError?
    outSession.pointee = delegate.invokeCreateSession(token: token, errorOut: &callbackError)
    if let callbackError {
        ctkWriteNSError(errorOut, fallback: "token delegate create-session failed", error: callbackError)
        return ctkStatus(from: callbackError)
    }
    return CTK_OK
}

@_cdecl("ctk_token_invoke_delegate_terminate_session")
public func ctk_token_invoke_delegate_terminate_session(
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ sessionPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let token = ctkBorrow(tokenPtr, as: TKToken.self, errorOut) else { return }
    guard let session = ctkBorrow(sessionPtr, as: TKTokenSession.self, errorOut) else { return }
    guard let delegate = token.delegate as? CTKTokenDelegateBox else {
        return
    }
    delegate.token(token, terminateSession: session)
}

@_cdecl("ctk_token_driver_set_delegate")
public func ctk_token_driver_set_delegate(
    _ driverPtr: UnsafeMutableRawPointer?,
    _ createTokenCallback: CTKTokenDriverCreateTokenCallback?,
    _ terminateTokenCallback: CTKTokenDriverTerminateTokenCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ release: CTKContextRelease?,
    _ outDelegate: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let context = CTKCallbackContext(userInfo, release: release)
    outDelegate.pointee = nil
    guard let driverPtr else {
        ctkWriteError(errorOut, "missing token-driver handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let createTokenCallback, let terminateTokenCallback else {
        ctkWriteError(errorOut, "missing token-driver delegate callbacks")
        return CTK_INVALID_ARGUMENT
    }
    guard let driver = ctkBorrow(driverPtr, as: TKTokenDriver.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    let box = CTKTokenDriverDelegateBox(
        createTokenCallback: createTokenCallback,
        terminateTokenCallback: terminateTokenCallback,
        context: context
    )
    driver.delegate = box
    outDelegate.pointee = ctkRetain(box)
    return CTK_OK
}

@_cdecl("ctk_token_driver_clear_delegate")
public func ctk_token_driver_clear_delegate(
    _ driverPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let driver = ctkBorrow(driverPtr, as: TKTokenDriver.self, errorOut) else { return }
    driver.delegate = nil
}

@_cdecl("ctk_token_driver_has_delegate")
public func ctk_token_driver_has_delegate(
    _ driverPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let driver = ctkBorrow(driverPtr, as: TKTokenDriver.self, errorOut) else { return false }
    return driver.delegate != nil
}

@_cdecl("ctk_token_driver_invoke_delegate_token_for_configuration_json")
public func ctk_token_driver_invoke_delegate_token_for_configuration_json(
    _ driverPtr: UnsafeMutableRawPointer?,
    _ configurationJSON: UnsafePointer<CChar>?,
    _ outToken: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outToken.pointee = nil
    guard let driverPtr, let configurationJSON else {
        ctkWriteError(errorOut, "missing token-driver delegate invocation arguments")
        return CTK_INVALID_ARGUMENT
    }
    guard let driver = ctkBorrow(driverPtr, as: TKTokenDriver.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    guard let delegate = driver.delegate as? CTKTokenDriverDelegateBox else {
        ctkWriteError(errorOut, "token-driver delegate is not managed by the Rust bridge")
        return CTK_FRAMEWORK_ERROR
    }
    var callbackError: NSError?
    outToken.pointee = delegate.invokeTokenForConfiguration(
        driver: driver,
        configurationJSON: String(cString: configurationJSON),
        errorOut: &callbackError
    )
    if let callbackError {
        ctkWriteNSError(errorOut, fallback: "token-driver delegate token creation failed", error: callbackError)
        return ctkStatus(from: callbackError)
    }
    return CTK_OK
}

@_cdecl("ctk_token_driver_invoke_delegate_terminate_token")
public func ctk_token_driver_invoke_delegate_terminate_token(
    _ driverPtr: UnsafeMutableRawPointer?,
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let driver = ctkBorrow(driverPtr, as: TKTokenDriver.self, errorOut) else { return }
    guard let token = ctkBorrow(tokenPtr, as: TKToken.self, errorOut) else { return }
    guard let delegate = driver.delegate as? CTKTokenDriverDelegateBox else {
        return
    }
    delegate.tokenDriver(driver, terminateToken: token)
}

@_cdecl("ctk_smart_card_token_driver_set_delegate")
public func ctk_smart_card_token_driver_set_delegate(
    _ driverPtr: UnsafeMutableRawPointer?,
    _ createTokenCallback: CTKSmartCardTokenDriverCreateTokenCallback?,
    _ terminateTokenCallback: CTKSmartCardTokenDriverTerminateTokenCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ release: CTKContextRelease?,
    _ outDelegate: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let context = CTKCallbackContext(userInfo, release: release)
    outDelegate.pointee = nil
    guard let driverPtr else {
        ctkWriteError(errorOut, "missing smart-card token-driver handle")
        return CTK_INVALID_ARGUMENT
    }
    guard let createTokenCallback, let terminateTokenCallback else {
        ctkWriteError(errorOut, "missing smart-card token-driver delegate callbacks")
        return CTK_INVALID_ARGUMENT
    }
    guard let driver = ctkBorrow(driverPtr, as: TKSmartCardTokenDriver.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    let box = CTKSmartCardTokenDriverDelegateBox(
        createTokenCallback: createTokenCallback,
        terminateTokenCallback: terminateTokenCallback,
        context: context
    )
    driver.delegate = box
    outDelegate.pointee = ctkRetain(box)
    return CTK_OK
}

@_cdecl("ctk_smart_card_token_driver_invoke_delegate_create_token")
public func ctk_smart_card_token_driver_invoke_delegate_create_token(
    _ driverPtr: UnsafeMutableRawPointer?,
    _ smartCardPtr: UnsafeMutableRawPointer?,
    _ aidPtr: UnsafePointer<UInt8>?,
    _ aidLen: Int,
    _ hasAid: Bool,
    _ outToken: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outToken.pointee = nil
    guard let driverPtr, let smartCardPtr else {
        ctkWriteError(errorOut, "missing smart-card token-driver delegate invocation arguments")
        return CTK_INVALID_ARGUMENT
    }
    guard let driver = ctkBorrow(driverPtr, as: TKSmartCardTokenDriver.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    guard let smartCard = ctkBorrow(smartCardPtr, as: TKSmartCard.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    guard let delegate = driver.delegate as? CTKSmartCardTokenDriverDelegateBox else {
        ctkWriteError(errorOut, "smart-card token-driver delegate is not managed by the Rust bridge")
        return CTK_FRAMEWORK_ERROR
    }
    let aid = hasAid ? aidPtr.map { Data(bytes: $0, count: aidLen) } : nil
    var callbackError: NSError?
    outToken.pointee = delegate.invokeCreateToken(
        driver: driver,
        smartCard: smartCard,
        aid: aid,
        errorOut: &callbackError
    )
    if let callbackError {
        ctkWriteNSError(errorOut, fallback: "smart-card token-driver delegate token creation failed", error: callbackError)
        return ctkStatus(from: callbackError)
    }
    return CTK_OK
}

@_cdecl("ctk_smart_card_token_driver_invoke_delegate_terminate_token")
public func ctk_smart_card_token_driver_invoke_delegate_terminate_token(
    _ driverPtr: UnsafeMutableRawPointer?,
    _ tokenPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) {
    guard let driver = ctkBorrow(driverPtr, as: TKTokenDriver.self, errorOut) else { return }
    guard let token = ctkBorrow(tokenPtr, as: TKToken.self, errorOut) else { return }
    guard let delegate = driver.delegate as? CTKSmartCardTokenDriverDelegateBox else {
        return
    }
    delegate.tokenDriver(driver, terminateToken: token)
}

@_cdecl("ctk_token_key_algorithm_is_algorithm")
public func ctk_token_key_algorithm_is_algorithm(
    _ algorithmPtr: UnsafeMutableRawPointer?,
    _ algorithmName: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let algorithm = ctkBorrow(algorithmPtr, as: TKTokenKeyAlgorithm.self, errorOut) else { return false }
    guard let algorithmName else {
        ctkWriteError(errorOut, "missing algorithm name")
        return false
    }
    return algorithm.isAlgorithm(ctkSecKeyAlgorithm(from: String(cString: algorithmName)))
}

@_cdecl("ctk_token_key_algorithm_supports_algorithm")
public func ctk_token_key_algorithm_supports_algorithm(
    _ algorithmPtr: UnsafeMutableRawPointer?,
    _ algorithmName: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let algorithm = ctkBorrow(algorithmPtr, as: TKTokenKeyAlgorithm.self, errorOut) else { return false }
    guard let algorithmName else {
        ctkWriteError(errorOut, "missing algorithm name")
        return false
    }
    return algorithm.supportsAlgorithm(ctkSecKeyAlgorithm(from: String(cString: algorithmName)))
}

@_cdecl("ctk_token_key_exchange_parameters_requested_size")
public func ctk_token_key_exchange_parameters_requested_size(
    _ parametersPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int {
    guard let parameters = ctkBorrow(parametersPtr, as: TKTokenKeyExchangeParameters.self, errorOut) else { return 0 }
    return parameters.requestedSize
}

@_cdecl("ctk_token_key_exchange_parameters_shared_info_json")
public func ctk_token_key_exchange_parameters_shared_info_json(
    _ parametersPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let parameters = ctkBorrow(parametersPtr, as: TKTokenKeyExchangeParameters.self, errorOut) else { return nil }
    guard let sharedInfo = parameters.sharedInfo else {
        return nil
    }
    return ctkCString(ctkJSONString([UInt8](sharedInfo)))
}

@_cdecl("ctk_token_auth_operation_kind")
public func ctk_token_auth_operation_kind(
    _ operationPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let operation = ctkBorrow(operationPtr, as: TKTokenAuthOperation.self, errorOut) else { return CTK_INVALID_ARGUMENT }
    if operation is TKTokenSmartCardPINAuthOperation {
        return 2
    }
    if operation is TKTokenPasswordAuthOperation {
        return 1
    }
    return 0
}

private extension String {
    func ifEmpty(_ fallback: String) -> String {
        isEmpty ? fallback : self
    }
}
