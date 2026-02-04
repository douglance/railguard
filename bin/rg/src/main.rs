//! Railguard CLI - Claude Code LLM Protection Hook

mod cli;
mod config_loader;
mod hook;
mod install;
mod lint;

use std::process::ExitCode;

use clap::Parser;
use cli::{Cli, Commands};
use rg_policy::RuntimePolicy;
use rg_types::HookInput;

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hook => run_hook(&cli.config),
        Commands::Install => run_install(),
        Commands::Uninstall => run_uninstall(),
        Commands::Lint => run_lint(&cli.config),
        Commands::Test {
            tool_name,
            tool_input,
        } => run_test(&cli.config, &tool_name, &tool_input),
    }
}

fn run_hook(config_path: &str) -> ExitCode {
    // Load config
    let config = match config_loader::load_config(config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(r#"{{"error": "Failed to load config: {e}"}}"#);
            return ExitCode::from(2);
        }
    };

    // Build policy (using full config to include tool-level permissions)
    let policy = RuntimePolicy::new(&config);

    // Run hook
    hook::run_hook(&policy)
}

fn run_install() -> ExitCode {
    match install::run_install() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run_uninstall() -> ExitCode {
    match install::run_uninstall() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run_lint(config_path: &str) -> ExitCode {
    let path = std::path::Path::new(config_path);
    let result = lint::lint_config(path);

    print!("{}", lint::format_human(&result));

    if result.has_errors() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run_test(config_path: &str, tool_name: &str, tool_input_json: &str) -> ExitCode {
    // Load config
    let config = match config_loader::load_config(config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading config: {e}");
            return ExitCode::FAILURE;
        }
    };

    // Build policy (using full config to include tool-level permissions)
    let policy = RuntimePolicy::new(&config);

    // Parse tool input
    let tool_input: serde_json::Value = match serde_json::from_str(tool_input_json) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing tool input JSON: {e}");
            return ExitCode::FAILURE;
        }
    };

    let input = HookInput {
        tool_name: tool_name.to_string(),
        tool_input,
    };

    // Inspect
    let (verdict, latency_us) = rg_policy::inspect(&input, &policy);

    // Output result
    println!("Tool: {tool_name}");
    println!("Latency: {latency_us}us");
    println!();

    match &verdict {
        rg_types::Verdict::Allow => {
            println!("Result: ALLOWED");
            ExitCode::SUCCESS
        }
        rg_types::Verdict::Deny { reason, context } => {
            println!("Result: DENIED");
            println!("Reason: {reason}");
            if let Some(ctx) = context {
                println!("Context: {ctx}");
            }
            ExitCode::from(2)
        }
        rg_types::Verdict::Ask { reason } => {
            println!("Result: ASK");
            println!("Reason: {reason}");
            ExitCode::SUCCESS // Ask is not an error
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_test_allowed() {
        // Test that run_test works with allowed input
        // This test relies on default config which should allow 'ls'
        let config = rg_types::Config::default();
        let policy = RuntimePolicy::from_config(&config.policy);

        let input = HookInput {
            tool_name: "Bash".to_string(),
            tool_input: serde_json::json!({ "command": "ls -la" }),
        };

        let (verdict, _) = rg_policy::inspect(&input, &policy);
        assert!(verdict.is_allow());
    }

    #[test]
    fn test_run_test_denied() {
        // Test that run_test works with denied input
        let config = rg_types::Config::default();
        let policy = RuntimePolicy::from_config(&config.policy);

        let input = HookInput {
            tool_name: "Bash".to_string(),
            tool_input: serde_json::json!({ "command": "rm -rf /" }),
        };

        let (verdict, _) = rg_policy::inspect(&input, &policy);
        assert!(verdict.is_deny());
    }
}
