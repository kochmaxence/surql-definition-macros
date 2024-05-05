Here's an updated `CONTRIBUTING.md` that includes a recommendation for using conventional commit messages:

---

# Contributing to surql-definition-macros

Thank you for considering contributing to `surql-definition-macros`! This project is an open-source Rust procedural macro for generating SurrealDB table and field definitions. Your contributions are valuable and help improve this project for everyone.

## How to Contribute

### Reporting Bugs

If you find a bug in `surql-definition-macros`, please file an issue on the [GitHub issue tracker](https://github.com/your-repo-name/surql-definition-macros/issues). Provide as much detail as possible, including:

1. The version of `surql-definition-macros` you're using.
2. A description of the bug.
3. Steps to reproduce the issue.
4. Any error messages or logs.

### Suggesting Features

To suggest a new feature or enhancement, please file an issue on the [GitHub issue tracker](https://github.com/your-repo-name/surql-definition-macros/issues). Clearly describe the feature and the reasons why it would be beneficial.

### Contributing Code

1. **Fork the repository**: Start by forking the repository to your GitHub account.

2. **Clone your fork**: Clone your forked repository to your local machine.

   ```
   git clone https://github.com/your-username/surql-definition-macros.git
   ```

3. **Create a new branch**: Create a new branch for your changes.

   ```
   git checkout -b my-feature-branch
   ```

4. **Make your changes**: Make the necessary code changes in your branch.

5. **Run tests**: Ensure that all tests pass.

   ```
   cargo test
   ```

6. **Commit your changes**: Commit your changes using conventional commit messages. This helps maintain a clean and consistent commit history.

   ```
   git commit -m "feat: add new feature XYZ"
   ```

   Conventional commit messages follow the format: `<type>: <description>`. Common types include:
   - `feat`: A new feature.
   - `fix`: A bug fix.
   - `docs`: Documentation only changes.
   - `style`: Changes that do not affect the meaning of the code (white-space, formatting, etc.).
   - `refactor`: A code change that neither fixes a bug nor adds a feature.
   - `test`: Adding missing tests or correcting existing tests.

7. **Push to your fork**: Push your changes to your forked repository.

   ```
   git push origin my-feature-branch
   ```

8. **Create a pull request**: Go to the original repository and create a pull request. Clearly describe your changes and the reasons for them.

### Code Style

Follow Rust's standard style guide. To format your code, run:

```
cargo fmt
```

### Testing

This project uses Rust's built-in testing framework. To run the tests, use:

```
cargo test
```

### Licensing

All contributions to this project are licensed under the MIT License. By submitting a pull request, you agree to license your contribution under the MIT License.

## Code of Conduct

This project adheres to the Contributor Covenant [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report any unacceptable behavior to the project maintainers.