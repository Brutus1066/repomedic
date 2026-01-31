use crate::report::clean_path;
use crate::scanner::{Language, ScanResult};
use std::fs;
use std::io;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// Get current year from system time.
fn current_year() -> u32 {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Approximate: seconds since 1970 / seconds per year
    1970 + (secs / 31_557_600) as u32
}

#[derive(Default)]
pub struct GenerateOptions {
    pub dry_run: bool,
}

fn write_file(path: &Path, content: &str, opts: &GenerateOptions) -> io::Result<()> {
    if opts.dry_run {
        println!("[dry-run] Would write: {}", clean_path(path));
        println!("--- content preview ---");
        let lines: Vec<_> = content.lines().collect();
        println!(
            "{}",
            lines
                .iter()
                .take(10)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        );
        if lines.len() > 10 {
            println!("... ({} more lines)", lines.len() - 10);
        }
        println!("-----------------------");
        return Ok(());
    }
    fs::write(path, content)?;
    println!("Generated: {}", clean_path(path));
    Ok(())
}

pub fn generate_readme(path: &Path, result: &ScanResult, opts: &GenerateOptions) -> io::Result<()> {
    let output = path.join("README.md");
    if output.exists() && !opts.dry_run {
        println!("README.md already exists, skipping.");
        return Ok(());
    }

    let project_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Project");

    let mut content = String::new();
    content.push_str(&format!("# {}\n\n", project_name));
    content.push_str("A brief description of the project.\n\n");

    if !result.languages.is_empty() {
        content.push_str("## Requirements\n\n");
        for lang in &result.languages {
            match lang {
                Language::Rust => content.push_str("- Rust (stable)\n"),
                Language::Python => content.push_str("- Python 3.8+\n"),
                Language::JavaScript | Language::TypeScript => content.push_str("- Node.js 18+\n"),
                Language::Go => content.push_str("- Go 1.21+\n"),
                Language::Java => content.push_str("- Java 17+\n"),
                Language::CSharp => content.push_str("- .NET 8.0+\n"),
                _ => {}
            }
        }
        content.push('\n');
    }

    content.push_str("## Installation\n\n");
    content.push_str("```sh\n# Add installation instructions\n```\n\n");

    content.push_str("## Usage\n\n");
    content.push_str("```sh\n# Add usage examples\n```\n\n");

    content.push_str("## License\n\n");
    content.push_str("MIT License\n");

    write_file(&output, &content, opts)
}

pub fn generate_license(
    path: &Path,
    author: Option<&str>,
    opts: &GenerateOptions,
) -> io::Result<()> {
    let output = path.join("LICENSE");
    if output.exists() && !opts.dry_run {
        println!("LICENSE already exists, skipping.");
        return Ok(());
    }

    let year = current_year();
    let holder = author.unwrap_or("Author");

    let content = format!(
        r#"MIT License

Copyright (c) {} {}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"#,
        year, holder
    );

    write_file(&output, &content, opts)
}

pub fn generate_gitignore(
    path: &Path,
    result: &ScanResult,
    opts: &GenerateOptions,
) -> io::Result<()> {
    let output = path.join(".gitignore");
    if output.exists() && !opts.dry_run {
        println!(".gitignore already exists, skipping.");
        return Ok(());
    }

    let mut content = String::new();

    content.push_str("# OS\n");
    content.push_str(".DS_Store\n");
    content.push_str("Thumbs.db\n");
    content.push_str("*.swp\n");
    content.push_str("*~\n\n");

    content.push_str("# IDE\n");
    content.push_str(".idea/\n");
    content.push_str(".vscode/\n");
    content.push_str("*.iml\n\n");

    for lang in &result.languages {
        match lang {
            Language::Rust => {
                content.push_str("# Rust\n");
                content.push_str("target/\n\n");
            }
            Language::Python => {
                content.push_str("# Python\n");
                content.push_str("__pycache__/\n");
                content.push_str("*.py[cod]\n");
                content.push_str("*.egg-info/\n");
                content.push_str("dist/\n");
                content.push_str("build/\n");
                content.push_str("venv/\n");
                content.push_str(".venv/\n");
                content.push_str(".env\n\n");
            }
            Language::JavaScript | Language::TypeScript => {
                content.push_str("# Node.js\n");
                content.push_str("node_modules/\n");
                content.push_str("dist/\n");
                content.push_str("build/\n");
                content.push_str(".env\n");
                content.push_str("*.log\n\n");
            }
            Language::Go => {
                content.push_str("# Go\n");
                content.push_str("bin/\n");
                content.push_str("*.exe\n\n");
            }
            Language::Java | Language::Kotlin | Language::Scala => {
                content.push_str("# Java/JVM\n");
                content.push_str("target/\n");
                content.push_str("build/\n");
                content.push_str("*.class\n");
                content.push_str("*.jar\n\n");
            }
            Language::CSharp => {
                content.push_str("# .NET\n");
                content.push_str("bin/\n");
                content.push_str("obj/\n");
                content.push_str("*.user\n");
                content.push_str("*.suo\n\n");
            }
            Language::Cpp | Language::C => {
                content.push_str("# C/C++\n");
                content.push_str("build/\n");
                content.push_str("*.o\n");
                content.push_str("*.a\n");
                content.push_str("*.so\n");
                content.push_str("*.dylib\n\n");
            }
            _ => {}
        }
    }

    if result.languages.is_empty() {
        content.push_str("# Build\n");
        content.push_str("build/\n");
        content.push_str("dist/\n");
        content.push_str("out/\n\n");
    }

    write_file(&output, &content, opts)
}

pub fn generate_contributing(path: &Path, opts: &GenerateOptions) -> io::Result<()> {
    let output = path.join("CONTRIBUTING.md");
    if output.exists() && !opts.dry_run {
        println!("CONTRIBUTING.md already exists, skipping.");
        return Ok(());
    }

    let project_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("this project");

    let content = format!(
        r#"# Contributing to {}

Thank you for your interest in contributing.

## How to Contribute

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Make your changes
4. Run tests and linting
5. Commit your changes (`git commit -m 'Add feature'`)
6. Push to the branch (`git push origin feature/your-feature`)
7. Open a Pull Request

## Code Style

- Follow the existing code style
- Write clear commit messages
- Add tests for new functionality

## Reporting Issues

- Check existing issues before creating a new one
- Provide clear steps to reproduce bugs
- Include relevant environment details

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.
"#,
        project_name
    );

    write_file(&output, &content, opts)
}

pub fn generate_changelog(path: &Path, opts: &GenerateOptions) -> io::Result<()> {
    let output = path.join("CHANGELOG.md");
    if output.exists() && !opts.dry_run {
        println!("CHANGELOG.md already exists, skipping.");
        return Ok(());
    }

    let content = r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Fixed

### Removed
"#;

    write_file(&output, content, opts)
}

pub fn generate_editorconfig(path: &Path, opts: &GenerateOptions) -> io::Result<()> {
    let output = path.join(".editorconfig");
    if output.exists() && !opts.dry_run {
        println!(".editorconfig already exists, skipping.");
        return Ok(());
    }

    let content = r#"root = true

[*]
indent_style = space
indent_size = 4
end_of_line = lf
charset = utf-8
trim_trailing_whitespace = true
insert_final_newline = true

[*.md]
trim_trailing_whitespace = false

[*.{yml,yaml}]
indent_size = 2

[*.json]
indent_size = 2

[Makefile]
indent_style = tab
"#;

    write_file(&output, content, opts)
}

pub fn generate_code_of_conduct(path: &Path, opts: &GenerateOptions) -> io::Result<()> {
    let output = path.join("CODE_OF_CONDUCT.md");
    if output.exists() && !opts.dry_run {
        println!("CODE_OF_CONDUCT.md already exists, skipping.");
        return Ok(());
    }

    let content = r#"# Contributor Covenant Code of Conduct

## Our Pledge

We as members, contributors, and leaders pledge to make participation in our
community a harassment-free experience for everyone, regardless of age, body
size, visible or invisible disability, ethnicity, sex characteristics, gender
identity and expression, level of experience, education, socio-economic status,
nationality, personal appearance, race, religion, or sexual identity
and orientation.

## Our Standards

Examples of behavior that contributes to a positive environment:

* Using welcoming and inclusive language
* Being respectful of differing viewpoints and experiences
* Gracefully accepting constructive criticism
* Focusing on what is best for the community
* Showing empathy towards other community members

Examples of unacceptable behavior:

* The use of sexualized language or imagery
* Trolling, insulting or derogatory comments, and personal or political attacks
* Public or private harassment
* Publishing others' private information without explicit permission
* Other conduct which could reasonably be considered inappropriate

## Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be
reported to the project maintainers. All complaints will be reviewed and
investigated and will result in a response that is deemed necessary and
appropriate to the circumstances.

## Attribution

This Code of Conduct is adapted from the [Contributor Covenant](https://www.contributor-covenant.org),
version 2.0.
"#;

    write_file(&output, content, opts)
}

pub fn generate_security(path: &Path, opts: &GenerateOptions) -> io::Result<()> {
    let output = path.join("SECURITY.md");
    if output.exists() && !opts.dry_run {
        println!("SECURITY.md already exists, skipping.");
        return Ok(());
    }

    let content = r#"# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| latest  | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly:

1. **Do not** open a public issue
2. Email the maintainers directly with details
3. Include steps to reproduce if possible
4. Allow reasonable time for a fix before disclosure

We take security seriously and will respond promptly to valid reports.
"#;

    write_file(&output, content, opts)
}

pub fn generate_all(
    path: &Path,
    result: &ScanResult,
    author: Option<&str>,
    opts: &GenerateOptions,
) -> io::Result<()> {
    if !result.has_readme {
        generate_readme(path, result, opts)?;
    }
    if !result.has_license {
        generate_license(path, author, opts)?;
    }
    if !result.has_gitignore {
        generate_gitignore(path, result, opts)?;
    }
    if !result.has_contributing {
        generate_contributing(path, opts)?;
    }
    if !result.has_changelog {
        generate_changelog(path, opts)?;
    }
    if !result.has_editorconfig {
        generate_editorconfig(path, opts)?;
    }
    if !result.has_code_of_conduct {
        generate_code_of_conduct(path, opts)?;
    }
    if !result.has_security {
        generate_security(path, opts)?;
    }
    Ok(())
}
