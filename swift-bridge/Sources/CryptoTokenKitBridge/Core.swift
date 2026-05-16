import CryptoTokenKit
import Foundation

public let CTK_OK: Int32 = 0
public let CTK_INVALID_ARGUMENT: Int32 = -1
public let CTK_FRAMEWORK_ERROR: Int32 = -2
public let CTK_TIMED_OUT: Int32 = -3

@inline(__always)
public func ctkRetain(_ object: some AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
public func ctkBorrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as _: T.Type = T.self) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@_cdecl("ctk_object_release")
public func ctk_object_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

@inline(__always)
public func ctkCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
public func ctkWriteError(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ message: String
) {
    errorOut?.pointee = ctkCString(message)
}

@inline(__always)
public func ctkStatus(from error: Error) -> Int32 {
    Int32((error as NSError).code)
}

func ctkJSONString(_ value: Any) -> String {
    do {
        let data = try JSONSerialization.data(withJSONObject: value, options: [.sortedKeys])
        return String(data: data, encoding: .utf8) ?? "{}"
    } catch {
        return "{}"
    }
}
