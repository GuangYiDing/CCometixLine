# Project Context

## Purpose
CCometixLine (ccline) is a high-performance statusline tool designed for Claude Code that provides real-time information about your development environment. It integrates with Git to display branch status, tracks model usage, and offers an interactive TUI configuration interface. The tool enhances the Claude Code experience by showing essential context information including directory, Git status, model information, context window usage, and cost tracking.

## Tech Stack
- **Core Language**: Rust 2021
- **CLI Framework**: clap 4.0 (with derive feature)
- **Configuration**: TOML format with toml crate
- **Serialization**: serde with derive feature, serde_json
- **TUI/UI**: ratatui 0.29, crossterm 0.28
- **Ansi Handling**: ansi_term, ansi-to-tui
- **HTTP Client**: ureq 2.10 (with json feature, optional)
- **Version Management**: semver (optional)
- **Date/Time**: chrono 0.4 with serde support (optional)
- **Directory Handling**: dirs 5.0 (optional)
- **Pattern Matching**: regex 1.0
- **Package Manager**: npm for distribution (@cometix/ccline)

## Project Conventions

### Code Style
- **Formatting**: Rust standard formatting (run `cargo fmt`)
- **Linting**: Strict clippy checks (CI fails on warnings: `cargo clippy -- -D warnings`)
- **Naming Conventions**:
  - Snake case for variables and functions
  - Pascal case for types and enums
  - Upper case with underscores for constants
- **Documentation**: Use `///` for public API documentation
- **Module Organization**: Clear separation of concerns with dedicated modules for each feature area

### Architecture Patterns
- **Modular Architecture**: Core components separated into distinct modules:
  - `cli`: Command-line interface handling
  - `config`: Configuration management and loading
  - `core`: Business logic and statusline generation
  - `ui`: TUI interface and user interaction
  - `utils`: Utility functions and helper modules
- **Feature-Gated Design**: Optional features controlled by Cargo features:
  - `tui`: Enables interactive TUI interface
  - `self-update`: Enables automatic update checking
  - `dirs`: Enables directory configuration support
- **Segment-Based Design**: Statusline composed of modular segments (Git, Model, Usage, etc.)
- **Theme System**: Extensible theme architecture with preset themes
- **Plugin Architecture**: Claude Code patcher for enhancement features

### Testing Strategy
- **Unit Tests**: Inline tests using `#[cfg(test)]` annotations
- **Integration Testing**: Run via `cargo test --verbose`
- **Cross-Platform Testing**: CI runs tests on Ubuntu, Windows, and macOS
- **Quality Gates**:
  - All tests must pass
  - Code formatting check (`cargo fmt -- --check`)
  - Clippy linting with zero warnings
- **No Separate Test Directory**: Tests are co-located with source code

### Git Workflow
- **Versioning**: Semantic Versioning (SemVer) 2.0.0
- **Changelog**: Keep a Changelog 1.0.0 format
- **Branch Strategy**: Main branch (master/main)
- **CI/CD**:
  - Automated testing on push/PR to master
  - Cross-platform build checks (Linux, Windows, macOS)
  - Release automation via GitHub Actions
- **Release Process**: Automated via `.github/workflows/release.yml`

## Domain Context
- **Claude Code Integration**: Tool specifically designed to work with Anthropic's Claude Code CLI
- **Statusline Display**: Shows information in terminal status lines (not full-screen UI)
- **Git Integration**: Requires Git 1.5+ (Git 2.22+ recommended)
- **Nerd Font Requirement**: Terminal must support Nerd Fonts for icon display
- **Context Window Management**: Tracks token usage based on transcript analysis
- **Cost Tracking**: Monitors API usage costs with intelligent zero-cost handling
- **Session Management**: Tracks session duration and changes

## Important Constraints
- **Rust Edition**: Must use Rust 2021 or later
- **Feature Dependencies**:
  - TUI features require ratatui and crossterm
  - Self-update requires network access and semver support
- **Terminal Compatibility**: Requires terminal with Nerd Font support
- **Git Requirement**: Git must be installed and available in PATH
- **Cross-Platform Support**: Must work on Linux, macOS, and Windows
- **No Runtime Dependencies**: Static binaries preferred (except glibc version requirements)
- **Configuration File**: Uses TOML format stored in user config directory
- **Backwards Compatibility**: TUI changes must not break existing configurations

## External Dependencies
- **Git**: For branch detection and repository status
- **Terminal**: Must support ANSI escape codes and Nerd Fonts
- **Network**: Optional, for self-update checking and version verification
- **npm Registry**: For npm-based installation and distribution
- **GitHub Releases**: For binary distribution and updates
- **Claude Code**: Primary integration target for statusline display
