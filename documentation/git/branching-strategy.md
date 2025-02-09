# Git Branching Strategy

## 📌 Naming Conventions:

| Prefix         | Usage |
|---------------|--------------------------|
| **feature/**  | New feature development |
| **fix/**      | Bug fixes |
| **hotfix/**   | Critical production fixes |
| **docs/**     | Documentation updates |
| **test/**     | Testing-related work |
| **refactor/** | Code refactoring |
| **chore/**    | Maintenance and tooling |
| **poc/**      | Proof of Concept for feasibility testing |

### 🔹 Examples:
```
feature/user-authentication
fix/navbar-alignment
docs/project-guidelines
chore/update-dependencies
```

## 🔄 Workflow:

1. **Main branches**:
   - `main`: Production-ready code
   - `develop`: Integration branch for new features
2. **Feature workflow**:
   - Create a branch: `git checkout -b feature/your-feature`
   - Push and open a pull request to `develop`
   - Merge only after review
3. **Proof of Concept (PoC) workflow**:
   - Create a PoC branch: `git checkout -b poc/your-concept`
   - Test feasibility before integrating into `feature/` branches
   - If validated, merge into a feature branch or discard