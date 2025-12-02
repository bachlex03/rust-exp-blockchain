# Project Structure

## Organization Pattern

The codebase is organized into thematic modules, each containing standalone executable examples:

```
src/
├── main.rs                    # Entry point (currently minimal)
├── syntax_and_semantics/      # Core language features
├── data_structures/           # Primitive and compound types
├── ownership_system/          # Ownership, borrowing, memory management
└── constructs/                # Language constructs (structs, enums, traits)
```

## Module Conventions

### File Organization
- Each `.rs` file contains a `main()` function with runnable examples
- Files are self-contained demonstrations of specific concepts
- Related concepts are grouped in subdirectories

### Code Style
- **Demonstration functions**: Named `demonstrate_*()` for clarity
- **Verbose examples**: Code includes both verbose and idiomatic approaches
- **Inline comments**: Extensive comments explain concepts and gotchas
- **Print statements**: Heavy use of `println!` for educational output
- **Section headers**: Printed section titles with `===` or numbered lists

### Naming Patterns
- Functions: `snake_case` (e.g., `demonstrate_basic_structs`)
- Structs/Enums: `PascalCase` (e.g., `Rectangle`, `User`)
- Constants: `SCREAMING_SNAKE_CASE`
- Variables: `snake_case`

### Documentation Style
- Each module includes explanatory comments before code blocks
- Comments explain "why" not just "what"
- References to Rust book chapters included where relevant
- Common pitfalls and errors noted with comments

## Key Directories

### `syntax_and_semantics/`
Core language features: variables, mutability, data types, control flow, pattern matching, shadowing, constants. Includes a README.md with learning notes.

### `data_structures/`
Demonstrations of primitive types (integers, floats, booleans, characters) and compound types (arrays, tuples, strings).

### `ownership_system/`
Rust's ownership model: ownership rules, borrowing, references, slices, Box, stack vs heap allocation.

### `constructs/`
Language constructs: struct definitions and methods, enums, traits, impl blocks.

## Adding New Examples

When adding new demonstration code:
1. Create a new `.rs` file in the appropriate subdirectory
2. Include a `main()` function that calls demonstration functions
3. Use `demonstrate_*()` naming for example functions
4. Add explanatory comments and print statements
5. Show both common mistakes and correct patterns where applicable
