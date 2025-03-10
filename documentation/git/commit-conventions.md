# Commit Message Conventions

## 🎯 Why use a structured commit format?
A clear and consistent commit message format improves readability, history tracking, and automation (e.g., changelogs, release notes).

## 📌 Recommended commit format:
```
<type>(<scope>): <message>
```

### 🔹 Types of commits:
| Type       | Usage |
|------------|---------------------------|
| **feat**   | Introduces a new feature |
| **fix**    | Fixes a bug |
| **chore**  | Maintenance or tooling updates |
| **docs**   | Documentation changes |
| **test**   | Adding or updating tests |
| **refactor** | Code improvement without changing behavior |
| **style**  | Formatting changes (whitespace, linting) |
| **perf**   | Performance improvements |
| **ci**     | CI/CD related updates |

### 📌 Examples:
```
feat(auth): add JWT authentication middleware
fix(ui): resolve navbar rendering issue
chore(git): add logs/ to .gitignore
docs(readme): update installation guide
```
