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

Cratographer uses rust-analyzer's IDE APIs for semantic code analysis:

- **rust-analyzer integration**: Uses `ra_ap_ide` crate for full semantic understanding of Rust code
  - Loads Cargo workspaces with complete project metadata
  - Maintains a Virtual File System (VFS) for efficient file access
  - Leverages rust-analyzer's symbol search and code structure APIs
- **MCP server**: Built on the `rmcp` SDK for Model Context Protocol support
  - Async/await with Tokio runtime
  - JSON Schema validation for tool parameters
  - Stdio transport for AI agent integration

The current implementation loads the project once at startup. Future versions will support incremental updates to handle code changes efficiently.

## Use Cases

1. **AI-Assisted Development**: Enable AI agents to navigate and modify code with surgical precision
2. **Code Navigation**: Quick lookups for developers and tools
3. **Refactoring Support**: Understand impact of changes across a codebase
4. **Documentation Generation**: Extract and organize code documentation

## Requirements

- Rust 1.70 or later
- Cargo (comes with Rust)
- A Rust project with a `Cargo.toml` file to analyze

## Current Implementation

Cratographer is implemented as an MCP server using the official Rust SDK (`rmcp`) with rust-analyzer integration for semantic code analysis. The server provides two fully functional tools:

### Tools

#### find_symbol
Find all occurrences of a Rust symbol (struct, enum, trait, function, method) by name.

**Features:**
- **Search modes**: Exact, fuzzy (default), or prefix matching
- **Library inclusion**: Optionally search in dependencies and standard library
- **Type filtering**: Filter results to only type symbols (structs, enums, traits, type aliases)
- **Rich metadata**: Returns symbol name, kind, file path, line numbers, and documentation

**Example usage:**
```json
{
  "name": "HashMap",
  "mode": "exact",
  "include_library": true,
  "types_only": true
}
```

#### enumerate_file
List all symbols defined in a specific file.

**Features:**
- Returns all functions, methods, structs, enums, traits, constants, and more
- Provides symbol name, kind, and line number ranges
- Filters out irrelevant symbol kinds automatically

**Example usage:**
```json
{
  "file_path": "/path/to/file.rs"
}
```

### Implementation Details

- **Semantic analysis**: Uses rust-analyzer's IDE APIs (`ra_ap_ide`) for accurate type information
- **Project loading**: Automatically loads Cargo workspaces with all targets
- **VFS integration**: Maintains a virtual file system for efficient file access
- **Symbol kinds**: Supports Const, Enum, Function, Impl, Method, Module, Static, Struct, Trait, and TypeAlias
- **Error handling**: Comprehensive error types with clear messages

### Running the Server

```bash
# Build in release mode
cargo build --release

# Run the server (communicates via stdio)
cargo run --release
```

The server communicates via stdio and follows the MCP protocol specification. It can be integrated with AI agents like Claude Code or Kiro through their MCP configuration.

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_find_symbol
```

### Integration with AI Agents

To use Cratographer with an MCP-compatible AI agent, add it to your MCP configuration. For example, with Claude Code:

```json
{
  "mcpServers": {
    "cratographer": {
      "command": "/path/to/cratographer/target/release/cratographer",
      "args": []
    }
  }
}
```

Once configured, the AI agent will have access to the `find_symbol` and `enumerate_file` tools for navigating Rust codebases.

## Status

**Phase 1 - MCP Server Foundation**: ✅ Complete
- MCP server structure implemented using rmcp SDK
- Two core tools defined and documented
- Server info and capabilities properly configured

**Phase 2 - Tool Implementation**: ✅ Complete
- Full rust-analyzer integration for semantic analysis
- Implemented find_symbol with exact/fuzzy/prefix search modes
- Implemented enumerate_file for listing file symbols
- Comprehensive test suite with all tests passing
- Support for library symbol search and type filtering

**Phase 3 - Advanced Features**: 📋 Planned
- Incremental index updates on file changes
- Enhanced cross-crate analysis and relationship mapping
- Performance optimization for large workspaces
- Additional symbol metadata (trait implementations, references)

## License

TBD
