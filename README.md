# temporal_features

A high-performance Python library for temporal feature engineering, implemented in Rust.

## Installation

```bash
pip install temporal_features
```

## Usage

Here is a simple example of how to use the package in your Python application:

```python
import temporal_features

# Using the 'add' function (current placeholder implementation)
result = temporal_features.add(5, 3)
print(f"5 + 3 = {result}")
```

## Development

### Prerequisites

- Rust (latest stable)
- Python 3.8+
- `maturin` build tool

### Building Locally

```bash
# Install maturin
pip install maturin

# Build and install in current environment
maturin develop
```

## Deployment Guide

This project uses GitHub Actions for automated deployment to PyPI.

### Steps to Deploy

1.  **Configure PyPI Token**:
    *   Go to your GitHub repository settings -> Secrets and variables -> Actions.
    *   Add a new repository secret named `PYPI_API_TOKEN` with your PyPI API token.

2.  **Create a Release**:
    *   Update the `version` in `Cargo.toml`.
    *   Commit and push your changes.
    *   Create a new git tag with the version number (e.g., `v0.1.0`).
    *   Push the tag to GitHub:
        ```bash
        git tag v0.1.0
        git push origin v0.1.0
        ```

3.  **Monitor the Workflow**:
    *   Go to the "Actions" tab in your GitHub repository.
    *   You should see a "Release" workflow running.
    *   Once completed, the package will be available on PyPI.

### CI/CD Pipeline Details

The `.github/workflows/release.yml` file defines the CI/CD pipeline:
*   **Triggers**: The workflow runs automatically whenever a tag starting with `v` is pushed.
*   **Build Jobs**: It builds wheels for Linux, Windows, and macOS across multiple architectures.
*   **Publish Job**: It collects all built wheels and the source distribution, then uploads them to PyPI using the `maturin-action`.