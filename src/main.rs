mod generator;
mod report;
mod scanner;

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Print colored banner.
fn print_banner(color: bool) {
    let cyan = if color { "\x1b[36m" } else { "" };
    let green = if color { "\x1b[32m" } else { "" };
    let reset = if color { "\x1b[0m" } else { "" };
    println!(
        "{}╔══════════════════════════════════════════════════════════════╗{}",
        cyan, reset
    );
    println!(
        "{}║  {}██████╗ ███████╗██████╗  ██████╗ ███╗   ███╗███████╗██████╗ {} ║{}",
        cyan, green, cyan, reset
    );
    println!(
        "{}║  {}██╔══██╗██╔════╝██╔══██╗██╔═══██╗████╗ ████║██╔════╝██╔══██╗{} ║{}",
        cyan, green, cyan, reset
    );
    println!(
        "{}║  {}██████╔╝█████╗  ██████╔╝██║   ██║██╔████╔██║█████╗  ██║  ██║{} ║{}",
        cyan, green, cyan, reset
    );
    println!(
        "{}║  {}██╔══██╗██╔══╝  ██╔═══╝ ██║   ██║██║╚██╔╝██║██╔══╝  ██║  ██║{} ║{}",
        cyan, green, cyan, reset
    );
    println!(
        "{}║  {}██║  ██║███████╗██║     ╚██████╔╝██║ ╚═╝ ██║███████╗██████╔╝{} ║{}",
        cyan, green, cyan, reset
    );
    println!(
        "{}║  {}╚═╝  ╚═╝╚══════╝╚═╝      ╚═════╝ ╚═╝     ╚═╝╚══════╝╚═════╝ {} ║{}",
        cyan, green, cyan, reset
    );
    println!(
        "{}╚══════════════════════════════════════════════════════════════╝{}",
        cyan, reset
    );
    println!();
}

fn print_examples(color: bool) {
    let cyan = if color { "\x1b[36m" } else { "" };
    let green = if color { "\x1b[32m" } else { "" };
    let yellow = if color { "\x1b[33m" } else { "" };
    let reset = if color { "\x1b[0m" } else { "" };

    print_banner(color);
    println!(
        r#"{}Usage Examples{}

{}━━━ Quick Start ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}
  repomedic                       {}Scan current directory{}
  repomedic C:\Projects\MyApp     {}Scan specific path{}
  repomedic doctor                {}Quick health check (one line){}
  repomedic scan --score          {}Show score with README badge{}
  repomedic scan --suggest        {}Show fix commands{}

{}━━━ Export Results ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}
  repomedic export report.json    {}Save as JSON{}
  repomedic export report.md      {}Save as Markdown{}
  repomedic export report.txt     {}Save as plain text{}
  repomedic export report.csv     {}Save as CSV{}

{}━━━ Generate Files ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}
  repomedic init                  {}Generate all recommended files{}
  repomedic init --author "Name"  {}Set LICENSE author{}
  repomedic generate --readme     {}Generate only README.md{}
  repomedic generate --dry-run    {}Preview without writing{}

{}━━━ CI/CD Integration ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}
  repomedic -q                    {}Quiet mode (exit code only){}
  repomedic --fail-on-warning     {}Strict mode (exit 2 on warn){}
  repomedic -f json               {}JSON for pipelines{}
  repomedic -f sarif              {}SARIF for GitHub Security{}

{}━━━ Exit Codes ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{}
  {}0{} = Clean      {}1{} = Error      {}2{} = Issues found

{}GitHub:{} https://github.com/Brutus1066/repomedic
"#,
        cyan,
        reset,
        yellow,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        yellow,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        yellow,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        yellow,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        yellow,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        cyan,
        reset
    );
}

fn print_help_detailed(color: bool) {
    let cyan = if color { "\x1b[36m" } else { "" };
    let green = if color { "\x1b[32m" } else { "" };
    let yellow = if color { "\x1b[33m" } else { "" };
    let reset = if color { "\x1b[0m" } else { "" };

    print_banner(color);
    println!(
        r#"{}RepoMedic v{}{} - Repository Health Scanner

{}WHAT IT DOES:{}
  Scans Git repositories for missing files, security issues, and
  best practices. Generates health scores and fix suggestions.

{}COMMANDS:{}
  {}scan{}        Scan repository and show results (default)
              --score    Show health score (0-100) with badge
              --suggest  Show fix commands for each issue

  {}doctor{}      Quick one-line health summary
              Perfect for shell prompts or quick checks

  {}export{}      Save results to file
              Formats: .json, .md, .txt, .csv
              Example: repomedic export report.json

  {}init{}        Generate all recommended files at once
              Creates: README, LICENSE, .gitignore, etc.

  {}generate{}    Generate specific files
              --readme, --license, --gitignore, etc.
              --all      Generate all missing
              --dry-run  Preview without writing

  {}report{}      Save full report to REPO_REPORT.md

  {}examples{}    Show usage examples with colors

{}GLOBAL FLAGS:{}
  {}-f, --format{}    Output: console, json, markdown, sarif
  {}-q, --quiet{}     Exit code only (for scripts)
  {}-v, --verbose{}   Show scan timing and stats
  {}--no-color{}      Disable colored output
  {}--fail-on-warning{}  Exit 2 on warnings (strict)

{}HEALTH SCORE:{}
  A (90-100)  Excellent - Ready for production
  B (80-89)   Good - Minor improvements needed
  C (70-79)   Fair - Some issues to address
  D (60-69)   Poor - Needs attention
  F (<60)     Failing - Critical issues

{}MORE INFO:{}
  Website:  https://kindware.dev
  GitHub:   https://github.com/Brutus1066/repomedic
  Author:   LazyFrog <contact@kindware.dev>
"#,
        cyan,
        VERSION,
        reset,
        yellow,
        reset,
        yellow,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        yellow,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        green,
        reset,
        yellow,
        reset,
        yellow,
        reset
    );
}

#[derive(Clone, Copy, ValueEnum)]
enum OutputFormat {
    Console,
    Json,
    Markdown,
    Sarif,
}

#[derive(Parser)]
#[command(name = "repomedic")]
#[command(author = "LazyFrog <contact@kindware.dev>")]
#[command(version = VERSION)]
#[command(about = "Local Git repository scanner and hygiene tool")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to repository (default: current directory)
    #[arg(global = true, default_value = ".")]
    path: PathBuf,

    /// Output format
    #[arg(
        long,
        short = 'f',
        value_enum,
        default_value = "console",
        global = true
    )]
    format: OutputFormat,

    /// Quiet mode (exit code only, no output)
    #[arg(long, short = 'q', global = true)]
    quiet: bool,

    /// Verbose mode (show scan stats, timing)
    #[arg(long, short = 'v', global = true)]
    verbose: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,

    /// Exit with code 2 on warnings (not just errors)
    #[arg(long, global = true)]
    fail_on_warning: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan repository and print summary
    Scan {
        /// Show health score and badge
        #[arg(long)]
        score: bool,

        /// Show fix suggestions with commands
        #[arg(long)]
        suggest: bool,
    },

    /// One-line health check (great for shell prompts)
    Doctor,

    /// Show usage examples
    Examples,

    /// Show detailed help and feature explanations
    Info,

    /// Export scan results to file (.json, .md, .txt, .csv)
    Export {
        /// Output file path (format detected from extension)
        file: PathBuf,
    },

    /// Initialize a healthy repository (generate all recommended files)
    Init {
        /// Author name for LICENSE
        #[arg(long)]
        author: Option<String>,

        /// Preview without writing files
        #[arg(long)]
        dry_run: bool,
    },

    /// Generate REPO_REPORT.md
    Report,

    /// Generate missing files (README, LICENSE, .gitignore, etc.)
    Generate {
        /// Generate README.md
        #[arg(long)]
        readme: bool,

        /// Generate LICENSE (MIT)
        #[arg(long)]
        license: bool,

        /// Generate .gitignore
        #[arg(long)]
        gitignore: bool,

        /// Generate CONTRIBUTING.md
        #[arg(long)]
        contributing: bool,

        /// Generate CHANGELOG.md
        #[arg(long)]
        changelog: bool,

        /// Generate .editorconfig
        #[arg(long)]
        editorconfig: bool,

        /// Generate CODE_OF_CONDUCT.md
        #[arg(long)]
        codeofconduct: bool,

        /// Generate SECURITY.md
        #[arg(long)]
        security: bool,

        /// Generate all missing files
        #[arg(long)]
        all: bool,

        /// Preview generation without writing files
        #[arg(long)]
        dry_run: bool,

        /// Author name for LICENSE
        #[arg(long)]
        author: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    let path = match cli.path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: cannot access path '{}': {}", cli.path.display(), e);
            process::exit(1);
        }
    };

    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory", report::clean_path(&path));
        process::exit(1);
    }

    let result = match scanner::scan(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error scanning repository: {}", e);
            process::exit(1);
        }
    };

    let use_color = report::use_color(cli.no_color);

    match cli.command {
        None
        | Some(Commands::Scan {
            score: false,
            suggest: false,
        }) => {
            match cli.format {
                OutputFormat::Console => {
                    report::print_summary(&result, cli.quiet, cli.verbose, use_color)
                }
                OutputFormat::Json => {
                    if !cli.quiet {
                        println!("{}", report::to_json(&result, &path));
                    }
                }
                OutputFormat::Markdown => {
                    if !cli.quiet {
                        if let Ok(md) = report::generate(&result, &path) {
                            print!("{}", md);
                        }
                    }
                }
                OutputFormat::Sarif => {
                    if !cli.quiet {
                        println!("{}", report::to_sarif(&result, &path));
                    }
                }
            }

            if report::has_errors(&result) {
                process::exit(2);
            }
            if cli.fail_on_warning && report::has_warnings(&result) {
                process::exit(2);
            }
        }
        Some(Commands::Scan { score, suggest }) => {
            if score {
                report::print_score(&result, use_color);
            }
            if suggest {
                report::print_suggestions(&result, use_color);
            }
            if report::has_errors(&result) {
                process::exit(2);
            }
            if cli.fail_on_warning && report::has_warnings(&result) {
                process::exit(2);
            }
        }
        Some(Commands::Doctor) => {
            report::print_doctor(&result, use_color);
            if report::has_errors(&result) {
                process::exit(2);
            }
        }
        Some(Commands::Examples) => {
            print_examples(use_color);
        }
        Some(Commands::Info) => {
            print_help_detailed(use_color);
        }
        Some(Commands::Export { file }) => {
            if let Err(e) = report::export_to_file(&result, &path, &file) {
                eprintln!("Error exporting: {}", e);
                process::exit(1);
            }
        }
        Some(Commands::Init { author, dry_run }) => {
            let opts = generator::GenerateOptions { dry_run };
            println!("Initializing healthy repository...\n");
            if let Err(e) = generator::generate_all(&path, &result, author.as_deref(), &opts) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
            println!("\nRun 'repomedic scan --score' to check your new health score!");
        }
        Some(Commands::Report) => {
            if let Err(e) = report::write(&result, &path) {
                eprintln!("Error writing report: {}", e);
                process::exit(1);
            }
        }
        Some(Commands::Generate {
            readme,
            license,
            gitignore,
            contributing,
            changelog,
            editorconfig,
            codeofconduct,
            security,
            all,
            dry_run,
            author,
        }) => {
            let opts = generator::GenerateOptions { dry_run };
            let author_ref = author.as_deref();
            let none_specified = !readme
                && !license
                && !gitignore
                && !contributing
                && !changelog
                && !editorconfig
                && !codeofconduct
                && !security;

            if all || none_specified {
                if let Err(e) = generator::generate_all(&path, &result, author_ref, &opts) {
                    eprintln!("Error generating files: {}", e);
                    process::exit(1);
                }
            } else {
                if readme {
                    if let Err(e) = generator::generate_readme(&path, &result, &opts) {
                        eprintln!("Error generating README: {}", e);
                        process::exit(1);
                    }
                }
                if license {
                    if let Err(e) = generator::generate_license(&path, author_ref, &opts) {
                        eprintln!("Error generating LICENSE: {}", e);
                        process::exit(1);
                    }
                }
                if gitignore {
                    if let Err(e) = generator::generate_gitignore(&path, &result, &opts) {
                        eprintln!("Error generating .gitignore: {}", e);
                        process::exit(1);
                    }
                }
                if contributing {
                    if let Err(e) = generator::generate_contributing(&path, &opts) {
                        eprintln!("Error generating CONTRIBUTING.md: {}", e);
                        process::exit(1);
                    }
                }
                if changelog {
                    if let Err(e) = generator::generate_changelog(&path, &opts) {
                        eprintln!("Error generating CHANGELOG.md: {}", e);
                        process::exit(1);
                    }
                }
                if editorconfig {
                    if let Err(e) = generator::generate_editorconfig(&path, &opts) {
                        eprintln!("Error generating .editorconfig: {}", e);
                        process::exit(1);
                    }
                }
                if codeofconduct {
                    if let Err(e) = generator::generate_code_of_conduct(&path, &opts) {
                        eprintln!("Error generating CODE_OF_CONDUCT.md: {}", e);
                        process::exit(1);
                    }
                }
                if security {
                    if let Err(e) = generator::generate_security(&path, &opts) {
                        eprintln!("Error generating SECURITY.md: {}", e);
                        process::exit(1);
                    }
                }
            }
        }
    }
}
