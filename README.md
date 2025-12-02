# temporal_features

A high-performance library for temporal feature engineering, implemented in Rust with bindings for **Python** and **JavaScript/Node.js**.

> **Note**: This project currently contains a placeholder implementation (`add` function). Future versions will include comprehensive temporal feature engineering capabilities.

## 🌟 Key Features

- **Single Source of Truth**: Core logic written once in Rust
- **Dual Language Support**: Available for both Python and JavaScript/Node.js
- **High Performance**: Leverages Rust's speed and safety
- **Cross-Platform**: Works on macOS, Linux, and Windows

## 📦 Installation

### Python

```bash
pip install temporal_features
```

### JavaScript/Node.js

```bash
npm install temporal_features
```

## 🚀 Usage

### Python

```python
from temporal_features import TemporalFeatures

# Create an instance
tf = TemporalFeatures()

# Use the add function (placeholder implementation)
result = tf.add(5, 3)
print(f"5 + 3 = {result}")  # Output: 5 + 3 = 8
```

### JavaScript/Node.js

```javascript
const { TemporalFeatures } = require('temporal_features');

// Create an instance
const tf = new TemporalFeatures();

// Use the add function (placeholder implementation)
const result = tf.add(5, 3);
console.log(`5 + 3 = ${result}`);  // Output: 5 + 3 = 8
```

## 🛠️ Development

### Prerequisites

- **Rust** (latest stable version)
- **Python** 3.8+ (for Python bindings)
- **Node.js** 10+ (for JavaScript bindings)
- **maturin** (for Python builds)
- **@napi-rs/cli** (for Node.js builds)

### Building Locally

#### Python

```bash
# Install maturin
pip install maturin

# Build and install in development mode
maturin develop --features python
```

#### JavaScript/Node.js

```bash
# Install dependencies
npm install

# Build the native module
npm run build
```

### Running Tests

```bash
# Rust tests
cargo test

# Python tests (after building)
python -m pytest

# JavaScript tests
npm test
```

## 📚 Architecture

This project uses a **single source of truth** pattern:

```
┌─────────────────────────────────────┐
│   Core Logic (Rust)                 │
│   src/lib.rs                        │
│   - TemporalFeaturesCore            │
│   - Pure business logic             │
└──────────────┬──────────────────────┘
               │
       ┌───────┴───────┐
       │               │
┌──────▼──────┐ ┌─────▼──────┐
│   Python    │ │ JavaScript │
│   Bindings  │ │  Bindings  │
│   (PyO3)    │ │  (napi-rs) │
└─────────────┘ └────────────┘
```

- **Core Logic**: Written once in Rust (`TemporalFeaturesCore`)
- **Python Bindings**: Exposed via PyO3 with `#[cfg(feature = "python")]`
- **JavaScript Bindings**: Exposed via napi-rs with `#[cfg(feature = "javascript")]`

## 🚢 Deployment Guide

### Prerequisites

1. **PyPI Account**: For Python package publishing
2. **npm Account**: For JavaScript package publishing
3. **GitHub Repository**: With Actions enabled

### Setup Secrets

Add the following secrets to your GitHub repository (Settings → Secrets and variables → Actions):

- `PYPI_API_TOKEN`: Your PyPI API token
- `NPM_TOKEN`: Your npm access token

### Release Process

1. **Update Version Numbers**:
   ```bash
   # Update version in all three files:
   # - Cargo.toml
   # - pyproject.toml
   # - package.json
   ```

2. **Commit and Tag**:
   ```bash
   git add .
   git commit -m "Release v0.1.1"
   git tag v0.1.1
   git push origin main
   git push origin v0.1.1
   ```

3. **Monitor Workflows**:
   - Go to the "Actions" tab in your GitHub repository
   - Two workflows will run:
     - **Release** (Python/PyPI)
     - **NPM Release** (JavaScript/npm)
   - Once completed, packages will be available on both PyPI and npm

### CI/CD Pipeline Details

#### Python (PyPI)
- **Workflow**: `.github/workflows/release.yml`
- **Triggers**: Tags starting with `v*`
- **Platforms**: Linux, Windows, macOS
- **Architectures**: x86_64, x86, aarch64, armv7, s390x, ppc64le
- **Tool**: maturin

#### JavaScript (npm)
- **Workflow**: `.github/workflows/npm-release.yml`
- **Triggers**: Tags starting with `v*`
- **Platforms**: Linux, macOS
- **Architectures**: x86_64, aarch64, musl variants
- **Tool**: napi-rs

## 📄 License

MIT

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 🔮 Roadmap

- [ ] Implement temporal feature extraction
- [ ] Add time series transformations
- [ ] Support for multiple time zones
- [ ] Benchmarking suite
- [ ] Comprehensive documentation