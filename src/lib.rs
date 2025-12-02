#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "javascript")]
use napi_derive::napi;

// ==========================================
// CORE LOGIC (Pure Rust)
// ==========================================
// This contains the actual business logic.
// It knows nothing about Python or JavaScript.

/// Core implementation of temporal features
/// This is the single source of truth for all functionality
#[derive(Debug, Clone, Default)]
struct TemporalFeaturesCore {
    // Future: Add temporal feature state here
}

impl TemporalFeaturesCore {
    fn new() -> Self {
        TemporalFeaturesCore::default()
    }

    /// Adds two numbers together (placeholder implementation)
    fn add(&self, left: u64, right: u64) -> u64 {
        left + right
    }
}

// ==========================================
// PUBLIC INTERFACE (Bindings)
// ==========================================
// This struct is exposed to both Python and JavaScript.
// It wraps the core logic and translates calls.

#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone)]
pub struct TemporalFeatures {
    core: TemporalFeaturesCore,
}

// ==========================================
// JavaScript / Node.js Bindings
// ==========================================
#[cfg(feature = "javascript")]
#[napi]
impl TemporalFeatures {
    #[napi(constructor)]
    pub fn new() -> Self {
        TemporalFeatures {
            core: TemporalFeaturesCore::new(),
        }
    }

    #[napi]
    pub fn add(&self, left: u32, right: u32) -> u32 {
        self.core.add(left as u64, right as u64) as u32
    }
}

// ==========================================
// Python Bindings
// ==========================================
#[cfg(feature = "python")]
#[pymethods]
impl TemporalFeatures {
    #[new]
    fn new_py() -> Self {
        TemporalFeatures {
            core: TemporalFeaturesCore::new(),
        }
    }

    #[pyo3(name = "add")]
    fn add_py(&self, left: u64, right: u64) -> u64 {
        self.core.add(left, right)
    }
}

// Python module definition
#[cfg(feature = "python")]
#[pymodule]
fn temporal_features(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<TemporalFeatures>()?;
    Ok(())
}

// ==========================================
// Tests
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_add() {
        let core = TemporalFeaturesCore::new();
        assert_eq!(core.add(2, 2), 4);
    }
}
