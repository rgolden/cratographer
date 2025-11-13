# cratographer

An MCP (Model Context Protocol) tool for indexing and analyzing Rust code to work seamlessly with AI agents like Q cli and Kiro.

## Overview

Cratographer solves a critical problem when AI agents work with complex Rust codebases: **quickly locating code elements without expensive searches**. When a user says "let's modify DurableFlags", the AI agent can instantly find the exact location of the struct definition instead of running multiple grep commands or bash scripts to hunt it down.

## Goals

### Primary Goals

- **Fast Code Location**: Provide instant lookups for Rust code elements (structs, enums, functions, traits, modules, etc.)
- **Rich Metadata**: Index not just locations, but also:
  - Type information
  - Documentation comments
  - Trait implementations
  - Module hierarchy
  - Dependencies between code elements
- **MCP Integration**: Expose the index through the Model Context Protocol for seamless AI agent integration
- **Incremental Updates**: Efficiently update the index when files change, without full rebuilds

### Secondary Goals

- **Cross-crate Analysis**: Handle workspaces and understand relationships across multiple crates
- **IDE-Quality Analysis**: Leverage rust-analyzer for accurate semantic understanding
- **Extensible Architecture**: Support additional indexing tools and data sources beyond rust-analyzer
- **Low Overhead**: Minimal performance impact, suitable for large codebases

## Architecture

Cratographer will use a combination of tools:

- **rust-analyzer**: Primary tool for semantic analysis and type information
- **syn**: For fast parsing of Rust syntax when full semantic analysis isn't needed
- **Additional tools**: As needed for specialized indexing tasks

The index will be maintained incrementally, with efficient update strategies to handle code changes.

## Use Cases

1. **AI-Assisted Development**: Enable AI agents to navigate and modify code with surgical precision
2. **Code Navigation**: Quick lookups for developers and tools
3. **Refactoring Support**: Understand impact of changes across a codebase
4. **Documentation Generation**: Extract and organize code documentation

## Status

This project is in early development.

## License

TBD
