# FileFire Document SDK

[![Crates.io](https://img.shields.io/crates/v/filefire.svg)](https://crates.io/crates/filefire)
[![Documentation](https://docs.rs/filefire/badge.svg)](https://docs.rs/filefire)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/mubarakhammed/fileFire#license)
[![Build Status](https://github.com/mubarakhammed/fileFire/workflows/CI/badge.svg)](https://github.com/mubarakhammed/fileFire/actions)

**FileFire** is a cross-platform, open-source Document SDK built with Rust, designed for high-performance document processing across mobile, desktop, and web platforms. It follows a **Hexagonal Architecture** with a **Microkernel/Plugin system** for maximum extensibility.

## 🔥 Features

- **Cross-Platform**: Native bindings for Flutter, iOS, Android, Web (WASM), and .NET
- **High Performance**: Built with Rust for memory safety and speed
- **Plugin Architecture**: Extensible microkernel design with hot-swappable plugins
- **Document Support**: PDF, DOCX, XLSX, PPTX processing and rendering
- **Rich Functionality**: Annotations, OCR, digital signatures, watermarking, AI-powered features
- **Cloud Ready**: Headless cloud API for batch processing and SaaS integration

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Applications                              │
├─────────────────────────────────────────────────────────────────┤
│  Flutter  │   iOS    │  Android │   Web    │  .NET   │  Cloud   │
├─────────────────────────────────────────────────────────────────┤
│                      Platform Adapters                          │
├─────────────────────────────────────────────────────────────────┤
│                     Core Engine (Rust)                          │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │   PDF Engine    │  │  Office Engine  │  │  Image Engine   │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
├─────────────────────────────────────────────────────────────────┤
│                      Plugin System                              │
│    OCR   │  AI/ML   │ Signatures │ Watermark │ Converters      │
└─────────────────────────────────────────────────────────────────┘
```

## 🚀 Quick Start

### Flutter
```dart
import 'package:filefire/filefire.dart';

final doc = await FileFire.open("document.pdf");
await doc.annotate(page: 1, text: "Hello World", x: 100, y: 200);
await doc.save("output.pdf");
```

### iOS (Swift)
```swift
import FileFire

let doc = try await FileFire.open("document.pdf")
try await doc.annotate(page: 1, text: "Hello World", x: 100, y: 200)
try await doc.save("output.pdf")
```

### Web (JavaScript)
```javascript
import { FileFire } from 'filefire-wasm';

const doc = await FileFire.open("document.pdf");
await doc.annotate({ page: 1, text: "Hello World", x: 100, y: 200 });
await doc.save("output.pdf");
```

## 📁 Project Structure

```
/filefire
├── core/              # Rust core engine
├── plugins/           # Plugin implementations
│   ├── ocr/          # OCR plugin (Tesseract)
│   ├── ai/           # AI/ML plugins
│   ├── watermark/    # Watermarking plugin
│   └── signature/    # Digital signature plugin
├── bindings/         # Platform-specific adapters
│   ├── flutter/     # Dart/Flutter bindings
│   ├── ios/         # Swift/iOS bindings
│   ├── android/     # Kotlin/Java bindings
│   ├── wasm/        # WebAssembly bindings
│   └── dotnet/      # C#/.NET bindings
├── examples/        # Example applications
├── cloud/          # Cloud API and Docker setup
└── docs/           # Documentation
```

## 🔧 Building from Source

### Prerequisites
- Rust 1.75+
- Flutter 3.0+ (for Flutter bindings)
- Xcode (for iOS bindings)
- Android SDK (for Android bindings)
- .NET 8+ (for .NET bindings)

### Build Core Engine
```bash
cd core
cargo build --release
```

### Build All Platforms
```bash
# Build core and all bindings
make build-all

# Or build specific platforms
make build-flutter
make build-ios
make build-android
make build-wasm
make build-dotnet
```

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Run platform-specific tests
cd bindings/flutter && flutter test
cd examples/ios_app && xcodebuild test
```

## 📖 Documentation

- [Architecture Guide](docs/architecture.md)
- [Plugin Development](docs/plugins.md)
- [Platform Integration](docs/platform-integration.md)
- [API Reference](docs/api-reference.md)
- [Contributing](CONTRIBUTING.md)

## 🌟 Plugin System

FileFire's plugin architecture allows for easy extension:

```rust
use filefire_core::Plugin;

#[derive(Debug)]
pub struct OCRPlugin;

impl Plugin for OCRPlugin {
    fn name(&self) -> &str { "ocr" }
    fn version(&self) -> &str { "1.0.0" }
    
    async fn process(&self, input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // OCR implementation
        Ok(vec![])
    }
}
```

## 🚀 Roadmap

- [x] Core Rust engine with basic PDF support
- [x] Plugin system architecture
- [x] Flutter bindings and example app
- [ ] iOS/Android native bindings
- [ ] WebAssembly support
- [ ] OCR plugin implementation
- [ ] Office document support (DOCX, XLSX, PPTX)
- [ ] Cloud API with Docker deployment
- [ ] Real-time collaboration features
- [ ] AI-powered document analysis

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

This project is dual-licensed under the MIT OR Apache-2.0 license.

## 🏢 Enterprise Support

For enterprise support, custom features, or consulting services, please contact us at enterprise@filefire.dev.

---

**Built with ❤️ by the FileFire community**