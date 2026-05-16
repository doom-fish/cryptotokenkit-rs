// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "CryptoTokenKitBridge",
    platforms: [
        .macOS(.v10_13)
    ],
    products: [
        .library(
            name: "CryptoTokenKitBridge",
            type: .static,
            targets: ["CryptoTokenKitBridge"]
        )
    ],
    targets: [
        .target(
            name: "CryptoTokenKitBridge",
            path: "Sources/CryptoTokenKitBridge",
            publicHeadersPath: "include"
        )
    ]
)
