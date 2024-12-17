use pyo3::prelude::*;

use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::{
    cairo_run::{cairo_run as cairo_run_inner, CairoRunConfig},
    types::layout_name::LayoutName,
};

fn cairo_run(path: &str) -> PyResult<String> {
    let code = std::fs::read_to_string(path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

    let config = CairoRunConfig::default();
    let result = cairo_run_inner(&code, &config, &mut BuiltinHintProcessor::new_empty());
    Ok(result.to_string())
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn cairo_vm_rs_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
