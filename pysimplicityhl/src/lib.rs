use base64::display::Base64Display;
use base64::engine::general_purpose::STANDARD;
use clap::{Arg, ArgAction, Command};
use serde::Serialize;
use simplicityhl::{Arguments, CompiledProgram};
use std::{fs, path::Path};
use shell_words;

/// JSON output for Python
#[derive(Serialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum CompilerResult {
    Success {
        program: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        witness: Option<String>,
    },
    Error {
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        backtrace: Option<String>,
    },
}

/// Runs the compiler with a string of arguments and returns JSON.
/// Example for `args`: `"my_program.hl --debug"`
pub fn run_compiler(args: &str) -> String {
    match inner_run(args) {
        Ok(ok) => serde_json::to_string(&ok).unwrap_or_else(|e| {
            serde_json::json!({
                "status": "error",
                "message": format!("JSON serialization failed: {}", e)
            })
            .to_string()
        }),
        Err(err) => {
            let result = CompilerResult::Error {
                message: err.clone(),
                backtrace: Some(std::backtrace::Backtrace::capture().to_string()),
            };
            serde_json::to_string(&result).unwrap_or_else(|_| {
                format!(
                    "{{\"status\":\"error\",\"message\":\"{}\"}}",
                    err.replace('"', "\\\"")
                )
            })
        }
    }
}

/// Internal function core (returns structured result)
fn inner_run(args: &str) -> Result<CompilerResult, String> {
    // Safely split CLI string into arguments
    let mut argv = vec!["".to_string()];
    argv.extend(shell_words::split(args).map_err(|e| format!("Argument parsing failed: {e}"))?);

    // CLI definition (simplified)
    let cmd = Command::new("simplicityhl")
        .disable_help_flag(true)
        .arg(
            Arg::new("prog_file")
                .required(true)
                .value_name("PROGRAM_FILE")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("wit_file")
                .value_name("WITNESS_FILE")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .action(ArgAction::SetTrue),
        );

    let matches = cmd
        .try_get_matches_from(argv)
        .map_err(|e| format!("CLI argument error: {e}"))?;

    let prog_file = matches
        .get_one::<String>("prog_file")
        .ok_or("Missing program file argument")?;
    let prog_text = fs::read_to_string(Path::new(prog_file))
        .map_err(|e| format!("Cannot read program file: {e}"))?;
    let include_debug = matches.get_flag("debug");

    // Compilation
    let compiled = CompiledProgram::new(prog_text, Arguments::default(), include_debug)
        .map_err(|e| format!("Compilation failed: {e}"))?;

    // Optionally load witness
    let witness_opt = matches
        .get_one::<String>("wit_file")
        .map(|wit| -> Result<simplicityhl::WitnessValues, String> {
            let wit_text = fs::read_to_string(Path::new(wit))
                .map_err(|e| format!("Cannot read witness: {e}"))?;
            serde_json::from_str(&wit_text).map_err(|e| format!("Invalid witness JSON: {e}"))
        })
        .transpose()?;

    // Output
    if let Some(witness) = witness_opt {
        let satisfied = compiled
            .satisfy(witness)
            .map_err(|e| format!("Satisfy error: {e}"))?;
        let (prog_bytes, wit_bytes) = satisfied.redeem().to_vec_with_witness();
        Ok(CompilerResult::Success {
            program: Base64Display::new(&prog_bytes, &STANDARD).to_string(),
            witness: Some(Base64Display::new(&wit_bytes, &STANDARD).to_string()),
        })
    } else {
        let prog_bytes = compiled.commit().to_vec_without_witness();
        Ok(CompilerResult::Success {
            program: Base64Display::new(&prog_bytes, &STANDARD).to_string(),
            witness: None,
        })
    }
}

//
// Python bindings (via PyO3)
//
use pyo3::prelude::*;

#[pyfunction]
fn run_from_python(args: String) -> PyResult<String> {
    Ok(run_compiler(&args))
}

#[pymodule]
fn pysimplicityhl(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_from_python, m)?)?;
    Ok(())
}
