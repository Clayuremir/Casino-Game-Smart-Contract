# Contributing to Casino Jackpot Smart Contract

Thank you for your interest in contributing to this project! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/Casino-Smart-Contract.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes thoroughly
6. Submit a pull request

## Development Setup

1. Install dependencies:
   ```bash
   yarn install
   ```

2. Build the program:
   ```bash
   anchor build
   ```

3. Run tests:
   ```bash
   anchor test
   ```

## Code Style

### Rust

- Follow Rust naming conventions (snake_case for functions and variables)
- Use `rustfmt` for formatting
- Add documentation comments for public functions and structs
- Keep functions focused and single-purpose

### TypeScript

- Follow TypeScript/JavaScript naming conventions (camelCase for functions and variables)
- Use Prettier for formatting
- Add JSDoc comments for exported functions
- Use meaningful variable names

## Commit Messages

- Use clear, descriptive commit messages
- Start with a verb in imperative mood (e.g., "Add", "Fix", "Update")
- Reference issue numbers if applicable

## Pull Request Process

1. Ensure your code follows the project's style guidelines
2. Update documentation if needed
3. Add tests for new features
4. Ensure all tests pass
5. Request review from maintainers

## Questions?

If you have questions, please open an issue or contact the maintainers.

