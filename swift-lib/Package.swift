// swift-tools-version: 6.1
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "PasskeyBridge",
    platforms: [
        .macOS(.v15)
    ],
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "PasskeyBridge",
            type: .static,
            targets: ["PasskeyBridge"]),
    ],
    dependencies: [
        .package(url: "https://github.com/Brendonovich/swift-rs", from: "1.0.5")
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "PasskeyBridge",
            dependencies: [
                .product(name: "SwiftRs", package: "swift-rs")
            ]
        )
    ]
)
