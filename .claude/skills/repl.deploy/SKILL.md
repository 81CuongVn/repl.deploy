```markdown
# repl.deploy Development Patterns

> Auto-generated skill from repository analysis

## Overview
This skill teaches you the core development patterns and conventions used in the `repl.deploy` TypeScript codebase. You'll learn about file naming, import/export styles, commit message habits, and how to write and run tests. While no specific frameworks or automated workflows were detected, this guide outlines the project's established practices to help you contribute effectively.

## Coding Conventions

### File Naming
- Use **camelCase** for file names.
  - Example: `deployService.ts`, `userConfig.ts`

### Import Style
- Use **relative imports** for modules within the project.
  - Example:
    ```typescript
    import { deploy } from './deployService';
    ```

### Export Style
- Use **named exports** rather than default exports.
  - Example:
    ```typescript
    // In deployService.ts
    export function deploy() { /* ... */ }
    export const DEPLOY_STATUS = 'active';
    ```

### Commit Messages
- Commit messages are **freeform** and do not follow a strict prefix or type.
- Average commit message length is about 31 characters.
  - Example: `fix bug in deployService logic`

## Workflows

### Code Contribution
**Trigger:** When adding or updating features or bug fixes  
**Command:** `/contribute`

1. Create or update files using camelCase naming.
2. Use relative imports for referencing other modules.
3. Export functions and constants using named exports.
4. Write clear, concise commit messages (no strict format required).
5. Add or update corresponding test files as needed.

### Testing
**Trigger:** Before submitting code or merging changes  
**Command:** `/test`

1. Locate or create test files named with the pattern `*.test.*` (e.g., `deployService.test.ts`).
2. Write tests for new or modified functionality.
3. Run tests using the project's preferred method (framework unknown; check project documentation or scripts).
4. Ensure all tests pass before committing.

## Testing Patterns

- **Test File Naming:** Use the pattern `*.test.*` for test files.
  - Example: `deployService.test.ts`
- **Testing Framework:** Not explicitly detected; refer to project documentation or package.json for details.
- **Test Placement:** Place test files alongside or near the modules they test.

## Commands

| Command      | Purpose                                      |
|--------------|----------------------------------------------|
| /contribute  | Steps for contributing code changes          |
| /test        | Steps for writing and running tests          |
```
