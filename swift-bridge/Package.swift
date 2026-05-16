// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "MapKitBridge",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .library(
            name: "MapKitBridge",
            type: .static,
            targets: ["MapKitBridge"])
    ],
    targets: [
        .target(
            name: "MapKitBridge",
            path: "Sources/MapKitBridge",
            publicHeadersPath: "include")
    ]
)
