# Project Structure

This document explains the structure of the temporal_features project.

## Overview

temporal_features is a dual-language package that provides the same functionality to both Python and JavaScript/Node.js users, with a single Rust codebase as the source of truth.

## Directory Structure

```
temporal_features/
├── src/
│   └── lib.rs                 # Main Rust source with core logic and bindings
├── examples/
│   ├── python/
│   │   └── example.py         # Python usage example
│   └── nodejs/
│       └── example.js         # JavaScript usage example
├── .github/
│   └── workflows/
│       ├── release.yml        # CI for Python/PyPI
│       └── npm-release.yml    # CI for JavaScript/npm
├── Cargo.toml                 # Rust package configuration
├── pyproject.toml             # Python package configuration
├── package.json               # Node.js package configuration
├── build.rs                   # napi-rs build script
├── README.md                  # Main documentation
└── LICENSE                    # MIT License

```

## Key Files

### Core Implementation

- **`src/lib.rs`**: Contains three main sections:
  1. **Core Logic** (`TemporalFeaturesCore`): Pure Rust implementation
  2. **Python Bindings**: PyO3-based wrappers (enabled with `--features python`)
  3. **JavaScript Bindings**: napi-rs-based wrappers (enabled with `--features javascript`)

### Configuration Files

- **`Cargo.toml`**: 
  - Defines Rust package metadata
  - Declares optional dependencies (pyo3, napi)
  - Defines feature flags (`python`, `javascript`)

- **`pyproject.toml`**:
  - Python package metadata
  - Maturin build configuration
  - Specifies `features = ["python"]` for builds

- **`package.json`**:
  - npm package metadata
  - napi-rs build scripts
  - Build commands include `--features javascript`

### CI/CD Workflows

- **`.github/workflows/release.yml`**:
  - Builds Python wheels for multiple platforms
  - Publishes to PyPI on tag push
  - Uses maturin-action

- **`.github/workflows/npm-release.yml`**:
  - Builds native Node.js modules for multiple platforms
  - Publishes to npm on tag push
  - Uses napi-rs

## Build Process

### Python

```bash
# Development
maturin develop --features python

# Production wheels
maturin build --release --features python
```

The Python feature flag enables:
- PyO3 dependency
- `#[pyclass]` and `#[pymethods]` attributes
- Python module definition

### JavaScript/Node.js

```bash
# Development
npm run build:debug

# Production
npm run build
```

The JavaScript feature flag enables:
- napi and napi-derive dependencies
- `#[napi]` attributes
- Node.js native module generation

## Single Source of Truth Pattern

The core business logic lives in `TemporalFeaturesCore` struct:

```rust
struct TemporalFeaturesCore {
    // Core state and logic
}

impl TemporalFeaturesCore {
    fn add(&self, left: u64, right: u64) -> u64 {
        // Implementation here - SINGLE SOURCE OF TRUTH
        left + right
    }
}
```

Both Python and JavaScript bindings simply wrap this core:

```rust
// Python wrapper
#[pymethods]
impl TemporalFeatures {
    fn add_py(&self, left: u64, right: u64) -> u64 {
        self.core.add(left, right)  // Delegates to core
    }
}

// JavaScript wrapper
#[napi]
impl TemporalFeatures {
    pub fn add(&self, left: u32, right: u32) -> u32 {
        self.core.add(left as u64, right as u64) as u32  // Delegates to core
    }
}
```

## Release Process

1. Update version in all three files:
   - `Cargo.toml`
   - `pyproject.toml`
   - `package.json`

2. Create and push a git tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

3. GitHub Actions automatically:
   - Builds Python wheels for all platforms
   - Builds Node.js native modules for all platforms
   - Publishes to both PyPI and npm

## Adding New Features

When adding new functionality:

1. **Implement in Core** (`TemporalFeaturesCore`):
   ```rust
   impl TemporalFeaturesCore {
       fn new_feature(&self, param: Type) -> ReturnType {
           // Implementation
       }
   }
   ```

2. **Add Python Binding**:
   ```rust
   #[cfg(feature = "python")]
   #[pymethods]
   impl TemporalFeatures {
       fn new_feature_py(&self, param: Type) -> ReturnType {
           self.core.new_feature(param)
       }
   }
   ```

3. **Add JavaScript Binding**:
   ```rust
   #[cfg(feature = "javascript")]
   #[napi]
   impl TemporalFeatures {
       pub fn new_feature(&self, param: Type) -> ReturnType {
           self.core.new_feature(param)
       }
   }
   ```

This ensures the logic is written once and exposed to both languages consistently.
