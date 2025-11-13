# Cratographer Project Steering Rules

## Task Completion Workflow

**CRITICAL**: After completing each task, you MUST follow this workflow:

1. **Run Tests**: Execute `cargo test` to ensure all tests pass
2. **Commit Changes**: Create a git commit with a clear, descriptive message explaining what changed
   - Use conventional commit format when applicable (e.g., "feat:", "fix:", "docs:", "refactor:")
   - Focus on the "why" rather than just the "what"
   - Include relevant context about the changes

This workflow ensures that:
- The codebase remains in a working state after each change
- Changes are tracked with clear commit history
- Problems are caught early before moving to the next task

## Documentation Maintenance

**IMPORTANT**: Whenever you make significant changes to the codebase, architecture, or functionality, you MUST update the README.md to reflect these changes. This includes:

- Adding new features or components
- Changing the architecture or design approach
- Implementing new indexing capabilities
- Modifying the MCP integration approach
- Adding or changing dependencies
- Updating project goals or use cases

The README.md should always accurately represent the current state of the project so that users and contributors can understand what cratographer does and how it works.

## Code Quality Standards

- Follow Rust best practices and idioms
- Use rust-analyzer's recommendations
- Write clear documentation comments for public APIs
- Include examples in documentation where helpful
- Keep error messages informative and actionable

## MCP Integration

- Ensure all MCP protocol implementations follow the official specification
- Keep the MCP interface clean and well-documented
- Test MCP integration with real AI agents when possible

## Performance Considerations

- Index updates should be incremental and efficient
- Consider memory usage for large codebases
- Profile and optimize hot paths
- Benchmark critical operations
