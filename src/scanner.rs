use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize)]
pub struct Issue {
    pub message: String,
    pub severity: Severity,
}

impl Issue {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            severity: Severity::Error,
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            severity: Severity::Warning,
        }
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            severity: Severity::Info,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CISystem {
    GitHubActions,
    GitLab,
    Jenkins,
    CircleCI,
    Travis,
    Azure,
}

impl CISystem {
    pub fn name(&self) -> &str {
        match self {
            CISystem::GitHubActions => "GitHub Actions",
            CISystem::GitLab => "GitLab CI",
            CISystem::Jenkins => "Jenkins",
            CISystem::CircleCI => "CircleCI",
            CISystem::Travis => "Travis CI",
            CISystem::Azure => "Azure Pipelines",
        }
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct ScanStats {
    pub files_scanned: usize,
    pub dirs_traversed: usize,
    pub scan_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PotentialSecret {
    pub file: String,
    pub pattern: String,
    pub line: usize,
}

#[derive(Debug, Default, Serialize)]
pub struct ScanResult {
    pub has_readme: bool,
    pub has_license: bool,
    pub has_gitignore: bool,
    pub has_git: bool,
    pub has_changelog: bool,
    pub has_contributing: bool,
    pub has_code_of_conduct: bool,
    pub has_security: bool,
    pub has_codeowners: bool,
    pub has_funding: bool,
    pub has_issue_template: bool,
    pub has_pr_template: bool,
    pub has_editorconfig: bool,
    pub has_gitattributes: bool,
    pub has_tests: bool,
    pub has_docs: bool,
    pub is_monorepo: bool,
    pub workspace_type: Option<String>,
    pub ci_systems: Vec<CISystem>,
    pub languages: Vec<Language>,
    pub build_systems: Vec<BuildSystem>,
    pub dependency_files: Vec<String>,
    pub linter_configs: Vec<String>,
    pub large_files: Vec<String>,
    pub potential_secrets: Vec<PotentialSecret>,
    pub scan_stats: ScanStats,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    Java,
    CSharp,
    Cpp,
    C,
    Ruby,
    Php,
    Swift,
    Kotlin,
    Scala,
    Haskell,
    Elixir,
    Zig,
    Nim,
    Lua,
    R,
    Perl,
    Dart,
    Crystal,
}

impl Language {
    pub fn name(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::Python => "Python",
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Go => "Go",
            Language::Java => "Java",
            Language::CSharp => "C#",
            Language::Cpp => "C++",
            Language::C => "C",
            Language::Ruby => "Ruby",
            Language::Php => "PHP",
            Language::Swift => "Swift",
            Language::Kotlin => "Kotlin",
            Language::Scala => "Scala",
            Language::Haskell => "Haskell",
            Language::Elixir => "Elixir",
            Language::Zig => "Zig",
            Language::Nim => "Nim",
            Language::Lua => "Lua",
            Language::R => "R",
            Language::Perl => "Perl",
            Language::Dart => "Dart",
            Language::Crystal => "Crystal",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum BuildSystem {
    Cargo,
    Pip,
    Poetry,
    Npm,
    Yarn,
    Pnpm,
    Go,
    Maven,
    Gradle,
    Msbuild,
    Cmake,
    Make,
    Bundler,
    Composer,
    Mix,
    Cabal,
    Stack,
    Zig,
    Nimble,
}

impl BuildSystem {
    pub fn name(&self) -> &str {
        match self {
            BuildSystem::Cargo => "Cargo (Rust)",
            BuildSystem::Pip => "pip (Python)",
            BuildSystem::Poetry => "Poetry (Python)",
            BuildSystem::Npm => "npm (Node.js)",
            BuildSystem::Yarn => "Yarn (Node.js)",
            BuildSystem::Pnpm => "pnpm (Node.js)",
            BuildSystem::Go => "Go modules",
            BuildSystem::Maven => "Maven (Java)",
            BuildSystem::Gradle => "Gradle (Java/Kotlin)",
            BuildSystem::Msbuild => "MSBuild (.NET)",
            BuildSystem::Cmake => "CMake (C/C++)",
            BuildSystem::Make => "Make",
            BuildSystem::Bundler => "Bundler (Ruby)",
            BuildSystem::Composer => "Composer (PHP)",
            BuildSystem::Mix => "Mix (Elixir)",
            BuildSystem::Cabal => "Cabal (Haskell)",
            BuildSystem::Stack => "Stack (Haskell)",
            BuildSystem::Zig => "Zig",
            BuildSystem::Nimble => "Nimble (Nim)",
        }
    }
}

pub fn scan(path: &Path) -> std::io::Result<ScanResult> {
    let start = Instant::now();
    let mut result = ScanResult::default();
    let mut languages: HashSet<Language> = HashSet::new();
    let mut stats = ScanStats::default();

    result.has_git = path.join(".git").is_dir();
    result.has_readme = has_readme(path);
    result.has_license = has_license(path);
    result.has_gitignore = path.join(".gitignore").is_file();
    result.has_editorconfig = path.join(".editorconfig").is_file();
    result.has_gitattributes = path.join(".gitattributes").is_file();

    detect_community_health(path, &mut result);
    detect_ci_systems(path, &mut result);
    detect_templates(path, &mut result);
    detect_tests_and_docs(path, &mut result);
    detect_linter_configs(path, &mut result);
    detect_monorepo(path, &mut result);
    detect_secrets(path, &mut result);

    scan_directory(path, path, &mut result, &mut languages, &mut stats, 0)?;

    result.languages = languages.into_iter().collect();
    result.languages.sort_by(|a, b| a.name().cmp(b.name()));

    stats.scan_duration_ms = start.elapsed().as_millis() as u64;
    result.scan_stats = stats;

    Ok(result)
}

fn has_readme(path: &Path) -> bool {
    [
        "README.md",
        "README",
        "README.txt",
        "readme.md",
        "Readme.md",
    ]
    .iter()
    .any(|n| path.join(n).is_file())
}

fn has_license(path: &Path) -> bool {
    [
        "LICENSE",
        "LICENSE.md",
        "LICENSE.txt",
        "LICENCE",
        "license",
        "License",
    ]
    .iter()
    .any(|n| path.join(n).is_file())
}

fn detect_community_health(path: &Path, result: &mut ScanResult) {
    result.has_changelog = [
        "CHANGELOG.md",
        "CHANGELOG",
        "CHANGELOG.txt",
        "HISTORY.md",
        "CHANGES.md",
    ]
    .iter()
    .any(|n| path.join(n).is_file());
    result.has_contributing = ["CONTRIBUTING.md", "CONTRIBUTING", "CONTRIBUTING.txt"]
        .iter()
        .any(|n| path.join(n).is_file())
        || path.join(".github/CONTRIBUTING.md").is_file();
    result.has_code_of_conduct = ["CODE_OF_CONDUCT.md", "CODE_OF_CONDUCT"]
        .iter()
        .any(|n| path.join(n).is_file())
        || path.join(".github/CODE_OF_CONDUCT.md").is_file();
    result.has_security = ["SECURITY.md", "SECURITY"]
        .iter()
        .any(|n| path.join(n).is_file())
        || path.join(".github/SECURITY.md").is_file();
    result.has_codeowners = ["CODEOWNERS", ".github/CODEOWNERS", "docs/CODEOWNERS"]
        .iter()
        .any(|n| path.join(n).is_file());
    result.has_funding = path.join(".github/FUNDING.yml").is_file();
}

fn detect_ci_systems(path: &Path, result: &mut ScanResult) {
    let workflows = path.join(".github/workflows");
    if workflows.is_dir() {
        if let Ok(entries) = fs::read_dir(&workflows) {
            if entries.flatten().any(|e| {
                let n = e.file_name();
                n.to_string_lossy().ends_with(".yml") || n.to_string_lossy().ends_with(".yaml")
            }) {
                result.ci_systems.push(CISystem::GitHubActions);
            }
        }
    }
    if path.join(".gitlab-ci.yml").is_file() {
        result.ci_systems.push(CISystem::GitLab);
    }
    if path.join("Jenkinsfile").is_file() {
        result.ci_systems.push(CISystem::Jenkins);
    }
    if path.join(".circleci").is_dir() {
        result.ci_systems.push(CISystem::CircleCI);
    }
    if path.join(".travis.yml").is_file() {
        result.ci_systems.push(CISystem::Travis);
    }
    if path.join("azure-pipelines.yml").is_file() {
        result.ci_systems.push(CISystem::Azure);
    }
}

fn detect_templates(path: &Path, result: &mut ScanResult) {
    result.has_issue_template = path.join(".github/ISSUE_TEMPLATE").is_dir()
        || path.join(".github/ISSUE_TEMPLATE.md").is_file()
        || path.join("ISSUE_TEMPLATE.md").is_file();
    result.has_pr_template = path.join(".github/PULL_REQUEST_TEMPLATE.md").is_file()
        || path.join(".github/PULL_REQUEST_TEMPLATE").is_dir()
        || path.join("PULL_REQUEST_TEMPLATE.md").is_file();
}

fn detect_tests_and_docs(path: &Path, result: &mut ScanResult) {
    result.has_tests = ["tests", "test", "spec", "__tests__", "Tests", "Test"]
        .iter()
        .any(|d| path.join(d).is_dir());
    result.has_docs = ["docs", "doc", "documentation", "Docs", "Doc"]
        .iter()
        .any(|d| path.join(d).is_dir());
}

fn detect_linter_configs(path: &Path, result: &mut ScanResult) {
    const CONFIGS: &[&str] = &[
        ".eslintrc",
        ".eslintrc.js",
        ".eslintrc.json",
        ".eslintrc.yml",
        ".prettierrc",
        ".prettierrc.js",
        ".prettierrc.json",
        ".prettierrc.yml",
        "prettier.config.js",
        ".stylelintrc",
        ".stylelintrc.json",
        "rustfmt.toml",
        ".rustfmt.toml",
        "clippy.toml",
        ".clippy.toml",
        ".pylintrc",
        "pylintrc",
        ".flake8",
        "setup.cfg",
        "pyproject.toml",
        ".rubocop.yml",
        ".golangci.yml",
        ".golangci.yaml",
        "tslint.json",
        "biome.json",
    ];
    for c in CONFIGS {
        if path.join(c).is_file() {
            result.linter_configs.push(c.to_string());
        }
    }
}

fn detect_monorepo(path: &Path, result: &mut ScanResult) {
    if let Ok(c) = fs::read_to_string(path.join("Cargo.toml")) {
        if c.contains("[workspace]") {
            result.is_monorepo = true;
            result.workspace_type = Some("Cargo workspace".to_string());
            return;
        }
    }
    if let Ok(c) = fs::read_to_string(path.join("package.json")) {
        if c.contains("\"workspaces\"") {
            result.is_monorepo = true;
            result.workspace_type = Some("npm/yarn workspaces".to_string());
            return;
        }
    }
    if path.join("pnpm-workspace.yaml").is_file() {
        result.is_monorepo = true;
        result.workspace_type = Some("pnpm workspace".to_string());
    } else if path.join("lerna.json").is_file() {
        result.is_monorepo = true;
        result.workspace_type = Some("Lerna".to_string());
    }
}

fn detect_secrets(path: &Path, result: &mut ScanResult) {
    const FILES: &[&str] = &[
        ".env",
        ".env.local",
        ".env.development",
        ".env.production",
        "config.json",
        "config.yaml",
        "config.yml",
        "settings.json",
    ];
    for f in FILES {
        let fp = path.join(f);
        if fp.is_file() {
            scan_file_for_secrets(&fp, f, result);
        }
    }
}

fn scan_file_for_secrets(file_path: &Path, name: &str, result: &mut ScanResult) {
    let Ok(content) = fs::read_to_string(file_path) else {
        return;
    };
    for (ln, line) in content.lines().enumerate() {
        let t = line.trim();
        if t.starts_with("AKIA") && t.len() >= 20 {
            result.potential_secrets.push(PotentialSecret {
                file: name.into(),
                pattern: "AWS Access Key".into(),
                line: ln + 1,
            });
        }
        if ["ghp_", "gho_", "ghs_", "ghr_", "github_pat_"]
            .iter()
            .any(|p| t.contains(p))
        {
            result.potential_secrets.push(PotentialSecret {
                file: name.into(),
                pattern: "GitHub token".into(),
                line: ln + 1,
            });
        }
        if ["sk_live_", "sk_test_", "rk_live_", "rk_test_"]
            .iter()
            .any(|p| t.contains(p))
        {
            result.potential_secrets.push(PotentialSecret {
                file: name.into(),
                pattern: "Stripe key".into(),
                line: ln + 1,
            });
        }
        if t.contains("-----BEGIN") && t.contains("PRIVATE KEY") {
            result.potential_secrets.push(PotentialSecret {
                file: name.into(),
                pattern: "Private key".into(),
                line: ln + 1,
            });
        }
    }
}

const LARGE_FILE_THRESHOLD: u64 = 5 * 1024 * 1024;

fn scan_directory(
    root: &Path,
    dir: &Path,
    result: &mut ScanResult,
    languages: &mut HashSet<Language>,
    stats: &mut ScanStats,
    depth: usize,
) -> std::io::Result<()> {
    if depth > 10 {
        return Ok(());
    }
    stats.dirs_traversed += 1;
    let Ok(entries) = fs::read_dir(dir) else {
        return Ok(());
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name_str = entry.file_name().to_string_lossy().to_string();
        if (name_str.starts_with('.') && name_str != ".gitignore") || is_ignored_dir(&name_str) {
            continue;
        }
        if path.is_dir() {
            scan_directory(root, &path, result, languages, stats, depth + 1)?;
        } else if path.is_file() {
            stats.files_scanned += 1;
            detect_language(&name_str, languages);
            if let Ok(meta) = path.metadata() {
                if meta.len() > LARGE_FILE_THRESHOLD {
                    if let Ok(rel) = path.strip_prefix(root) {
                        result.large_files.push(rel.display().to_string());
                    }
                }
            }
            if dir == root {
                detect_build_system(&name_str, result);
                detect_dependency_file(&name_str, result);
            }
        }
    }
    Ok(())
}

fn is_ignored_dir(name: &str) -> bool {
    matches!(
        name,
        "node_modules"
            | "target"
            | "dist"
            | "build"
            | "out"
            | ".git"
            | "__pycache__"
            | "venv"
            | ".venv"
            | "vendor"
            | "deps"
            | "_build"
            | ".build"
            | "bin"
            | "obj"
            | "packages"
            | ".cache"
            | ".next"
            | ".nuxt"
    )
}

fn detect_language(name: &str, languages: &mut HashSet<Language>) {
    let ext = name.rsplit('.').next().unwrap_or("");
    let lang = match ext {
        "rs" => Some(Language::Rust),
        "py" | "pyw" | "pyi" => Some(Language::Python),
        "js" | "mjs" | "cjs" => Some(Language::JavaScript),
        "ts" | "mts" | "cts" | "tsx" | "jsx" => Some(Language::TypeScript),
        "go" => Some(Language::Go),
        "java" => Some(Language::Java),
        "cs" => Some(Language::CSharp),
        "cpp" | "cc" | "cxx" | "hpp" | "hxx" => Some(Language::Cpp),
        "c" | "h" => Some(Language::C),
        "rb" => Some(Language::Ruby),
        "php" => Some(Language::Php),
        "swift" => Some(Language::Swift),
        "kt" | "kts" => Some(Language::Kotlin),
        "scala" | "sc" => Some(Language::Scala),
        "hs" | "lhs" => Some(Language::Haskell),
        "ex" | "exs" => Some(Language::Elixir),
        "zig" => Some(Language::Zig),
        "nim" => Some(Language::Nim),
        "lua" => Some(Language::Lua),
        "r" | "R" => Some(Language::R),
        "pl" | "pm" => Some(Language::Perl),
        "dart" => Some(Language::Dart),
        "cr" => Some(Language::Crystal),
        _ => None,
    };
    if let Some(l) = lang {
        languages.insert(l);
    }
}

fn detect_build_system(name: &str, result: &mut ScanResult) {
    let system = match name {
        "Cargo.toml" => Some(BuildSystem::Cargo),
        "setup.py" => Some(BuildSystem::Pip),
        "pyproject.toml" => Some(BuildSystem::Poetry),
        "requirements.txt" => Some(BuildSystem::Pip),
        "package.json" => Some(BuildSystem::Npm),
        "yarn.lock" => Some(BuildSystem::Yarn),
        "pnpm-lock.yaml" => Some(BuildSystem::Pnpm),
        "go.mod" => Some(BuildSystem::Go),
        "pom.xml" => Some(BuildSystem::Maven),
        "build.gradle" | "build.gradle.kts" => Some(BuildSystem::Gradle),
        n if n.ends_with(".csproj") || n.ends_with(".sln") => Some(BuildSystem::Msbuild),
        "CMakeLists.txt" => Some(BuildSystem::Cmake),
        "Makefile" | "makefile" | "GNUmakefile" => Some(BuildSystem::Make),
        "Gemfile" => Some(BuildSystem::Bundler),
        "composer.json" => Some(BuildSystem::Composer),
        "mix.exs" => Some(BuildSystem::Mix),
        n if n.ends_with(".cabal") => Some(BuildSystem::Cabal),
        "stack.yaml" => Some(BuildSystem::Stack),
        "build.zig" => Some(BuildSystem::Zig),
        n if n.ends_with(".nimble") => Some(BuildSystem::Nimble),
        _ => None,
    };
    if let Some(s) = system {
        if !result.build_systems.contains(&s) {
            result.build_systems.push(s);
        }
    }
}

fn detect_dependency_file(name: &str, result: &mut ScanResult) {
    let is_dep = matches!(
        name,
        "Cargo.toml"
            | "Cargo.lock"
            | "package.json"
            | "package-lock.json"
            | "yarn.lock"
            | "pnpm-lock.yaml"
            | "requirements.txt"
            | "Pipfile"
            | "Pipfile.lock"
            | "pyproject.toml"
            | "poetry.lock"
            | "go.mod"
            | "go.sum"
            | "pom.xml"
            | "build.gradle"
            | "build.gradle.kts"
            | "Gemfile"
            | "Gemfile.lock"
            | "composer.json"
            | "composer.lock"
            | "mix.exs"
            | "mix.lock"
    ) || name.ends_with(".csproj")
        || name.ends_with(".cabal");
    if is_dep && !result.dependency_files.contains(&name.to_string()) {
        result.dependency_files.push(name.to_string());
    }
}
