use std::path::PathBuf;

use argh::FromArgs;
use anyhow::{Context, anyhow};

use rustpython_compiler as compile;

/// Compiles RustPython bytecode
#[derive(FromArgs)]
struct CompileArgs {
    /// the sourcefile to compile
    #[argh(option)]
    file_source: PathBuf,
    /// the mode of execution
    #[argh(option, from_str_fn(parse_mode), default="compile::Mode::Exec")]
    mode: compile::Mode,
    /// the optimization level
    #[argh(option, default="0")]
    optimize: u8,
}

pub fn main() -> anyhow::Result<()> {
    let args: CompileArgs = argh::from_env();
    let source: String = std::fs::read_to_string(&args.file_source)
        .with_context(|| format!("Unable to read {}", args.file_source.display()))?;
    let resolved_source: PathBuf = std::fs::canonicalize(&args.file_source)
        .with_context(|| format!("Unable to resolve path: {}", args.file_source.display()))?;
    let resolved_file_name = resolved_source.file_name()
        .ok_or_else(|| anyhow!("Expected source path to have filename: {}", resolved_source.display()))?
        .to_string_lossy().into_owned();
    let opts = compile::CompileOpts { optimize: args.optimize };
    eprintln!("Compiling...");
    let code = compile::compile(&source, args.mode, resolved_file_name.clone(), opts)
        .with_context(|| "Failed to compile bytecode".to_string())?;
    println!("dis(\"{}\"):", resolved_file_name.escape_default());
    println!("{}", code.display_expand_codeobjects());
    Ok(())
}

fn parse_mode(mode: &str) -> Result<compile::Mode, String> {
    match mode {
        "exec" => Ok(compile::Mode::Exec),
        "eval" => Ok(compile::Mode::Eval),
        "single" => Ok(compile::Mode::Single),
        "block-expr" => Ok(compile::Mode::BlockExpr),
        _ => Err(format!("Invalid mode name `{}`", mode.escape_default()))
    }
}
