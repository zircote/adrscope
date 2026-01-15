//! Command handlers that execute use cases.

use std::io::{self, Write};

use crate::application::{
    GenerateOptions, GenerateUseCase, StatsOptions, StatsUseCase, ValidateOptions, ValidateUseCase,
    WikiOptions, WikiUseCase,
};
use crate::cli::args::{Cli, Commands, GenerateArgs, StatsArgs, ValidateArgs, WikiArgs};
use crate::domain::Severity;
use crate::error::Result;
use crate::infrastructure::RealFileSystem;

/// Runs the CLI with the parsed arguments.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cli: Cli) -> Result<i32> {
    match cli.command {
        Commands::Generate(args) => handle_generate(args, cli.verbose),
        Commands::Wiki(args) => handle_wiki(args, cli.verbose),
        Commands::Validate(args) => handle_validate(args, cli.verbose),
        Commands::Stats(args) => handle_stats(args, cli.verbose),
    }
}

fn handle_generate(args: GenerateArgs, verbose: bool) -> Result<i32> {
    let fs = RealFileSystem::new();
    let use_case = GenerateUseCase::new(fs);

    let options = GenerateOptions::new(&args.input)
        .with_output(&args.output)
        .with_title(&args.title)
        .with_theme(args.theme.into())
        .with_pattern(&args.pattern);

    if verbose {
        eprintln!("Scanning for ADRs in: {}", args.input);
    }

    let result = use_case.execute(&options)?;

    // Report parse errors
    if result.has_errors() {
        eprintln!("\nWarnings:");
        for (path, error) in &result.parse_errors {
            eprintln!("  {} - {}", path.display(), error);
        }
    }

    println!(
        "Generated {} with {} ADRs",
        result.output_path, result.adr_count
    );

    Ok(0)
}

fn handle_wiki(args: WikiArgs, verbose: bool) -> Result<i32> {
    let fs = RealFileSystem::new();
    let use_case = WikiUseCase::new(fs);

    let mut options = WikiOptions::new(&args.input)
        .with_output_dir(&args.output)
        .with_pattern(&args.pattern);

    if let Some(url) = &args.pages_url {
        options = options.with_pages_url(url);
    }

    if verbose {
        eprintln!("Scanning for ADRs in: {}", args.input);
    }

    let result = use_case.execute(&options)?;

    // Report parse errors
    if result.has_errors() {
        eprintln!("\nWarnings:");
        for (path, error) in &result.parse_errors {
            eprintln!("  {} - {}", path.display(), error);
        }
    }

    println!(
        "Generated {} wiki files in {} from {} ADRs",
        result.generated_files.len(),
        result.output_dir,
        result.adr_count
    );

    if verbose {
        eprintln!("\nGenerated files:");
        for file in &result.generated_files {
            eprintln!("  {file}");
        }
    }

    Ok(0)
}

fn handle_validate(args: ValidateArgs, verbose: bool) -> Result<i32> {
    let fs = RealFileSystem::new();
    let use_case = ValidateUseCase::new(fs);

    let options = ValidateOptions::new(&args.input)
        .with_pattern(&args.pattern)
        .with_strict(args.strict);

    if verbose {
        eprintln!("Validating ADRs in: {}", args.input);
    }

    let result = use_case.execute(&options)?;

    // Report parse errors
    for (path, error) in &result.parse_errors {
        eprintln!("ERROR: {} - {}", path.display(), error);
    }

    // Report validation issues
    let mut stdout = io::stdout();
    for (path, issue) in result.all_issues() {
        let prefix = match issue.severity {
            Severity::Error => "ERROR",
            Severity::Warning => "WARNING",
        };
        let _ = writeln!(
            stdout,
            "{}: {} - {} [{}]",
            prefix,
            path.display(),
            issue.message,
            issue.rule
        );
    }

    // Summary
    println!(
        "\nValidation complete: {} errors, {} warnings",
        result.total_errors, result.total_warnings
    );

    if result.passed {
        println!("All checks passed.");
        Ok(0)
    } else {
        println!("Validation failed.");
        Ok(1)
    }
}

fn handle_stats(args: StatsArgs, verbose: bool) -> Result<i32> {
    let fs = RealFileSystem::new();
    let use_case = StatsUseCase::new(fs);

    let options = StatsOptions::new(&args.input)
        .with_pattern(&args.pattern)
        .with_format(args.format.into());

    if verbose {
        eprintln!("Computing statistics for ADRs in: {}", args.input);
    }

    let result = use_case.execute(&options)?;

    // Report parse errors
    if result.has_errors() {
        eprintln!("\nWarnings:");
        for (path, error) in &result.parse_errors {
            eprintln!("  {} - {}", path.display(), error);
        }
        eprintln!();
    }

    println!("{}", result.output);

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Integration tests would require file system setup,
    // so we primarily test through the use cases directly.
    // These handler functions are thin wrappers.

    #[test]
    fn test_handler_functions_exist() {
        // Verify that all handler functions are properly defined
        // by checking they can be referenced
        let _: fn(GenerateArgs, bool) -> Result<i32> = handle_generate;
        let _: fn(WikiArgs, bool) -> Result<i32> = handle_wiki;
        let _: fn(ValidateArgs, bool) -> Result<i32> = handle_validate;
        let _: fn(StatsArgs, bool) -> Result<i32> = handle_stats;
    }
}
