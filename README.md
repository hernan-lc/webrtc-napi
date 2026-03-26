# WebRTC NAPI

Node.js WebRTC bindings for Rust using NAPI-RS.

## Features

- Native Rust bindings for WebRTC
- Full Node.js/TypeScript support
- High-performance peer connections
- Cross-platform support (Linux, macOS, Windows)

## Installation

```bash
npm install webrtc-napi
```

## Usage

```javascript
const { PeerConnection, initWebrtc, getVersion } = require('webrtc-napi');

// Initialize the addon
initWebrtc();

// Get version
console.log('Version:', getVersion());

// Create a peer connection
const config = {
  iceServers: ['stun:stun.l.google.com:19302']
};

const pc = new PeerConnection(config);

console.log('Connection ID:', pc.getId());
console.log('Stats:', pc.getStats());

// Close the connection
pc.close();
```

## Development

### Prerequisites

- Rust 1.70+
- Node.js 16+
- npm or yarn

### Build

```bash
# Build release version
npm run build

# Build debug version
npm run build:debug
```

### Testing

```bash
npm test
```

## Project Structure

```
├── src/
│   └── lib.rs          # Main Rust implementation
├── Cargo.toml          # Rust dependencies
├── package.json        # Node.js configuration
└── README.md           # This file
```

## License

MIT