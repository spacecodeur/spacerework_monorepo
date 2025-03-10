# 📝 Project Documentation

Welcome to the documentation folder of this **Tuono-based fullstack project**.  
This directory contains all essential guides and best practices related to **Git, setup, backend, frontend, testing, CI/CD, and development processes**.

## 📂 Documentation Structure

### 🔹 [Git Guidelines](git/)
- [`commit-conventions.md`](git/commit-conventions.md) → Rules for commit messages  
- [`branching-strategy.md`](git/branching-strategy.md) → Branch naming conventions  

### 🔹 [Project Setup](setup/)
- [`installation.md`](setup/installation.md) → How to install and run the project locally  
- [`environment.md`](setup/environment.md) → Environment variables & secrets management  
- [`docker-setup.md`](setup/docker-setup.md) → Docker configuration and usage  

### 🔹 [Backend (Rust + Tuono)](backend/)
- [`architecture.md`](backend/architecture.md) → Overview of the backend structure  
- [`api-routes.md`](backend/api-routes.md) → API routes documentation  
- [`database-schema.md`](backend/database-schema.md) → Database schema & migrations  
- [`performance.md`](backend/performance.md) → Backend performance optimizations  

### 🔹 [Frontend (React + Tuono)](frontend/)
- [`architecture.md`](frontend/architecture.md) → Overview of the frontend structure  
- [`component-guidelines.md`](frontend/component-guidelines.md) → Best practices for React components  
- [`state-management.md`](frontend/state-management.md) → Guide to state management (if using Zustand, Redux, etc.)  

### 🔹 [Testing Strategy](testing/)
- [`strategy.md`](testing/strategy.md) → Overview of testing types (unit, integration, e2e, mutation, load…)  
- [`tools.md`](testing/tools.md) → List of testing tools used (Jest, Cypress, Mutation Testing, etc.)  
- [`writing-tests.md`](testing/writing-tests.md) → Best practices for writing tests  

### 🔹 [CI/CD & Deployment](ci-cd/)
- [`jenkins-pipeline.md`](ci-cd/jenkins-pipeline.md) → Jenkins pipeline setup & automation  
- [`deployment-aws.md`](ci-cd/deployment-aws.md) → AWS deployment process  
- [`rollback-strategy.md`](ci-cd/rollback-strategy.md) → Rollback plan in case of failed deployment  

### 🔹 [Development Processes](processes/)
- [`release-process.md`](processes/release-process.md) → Versioning and release management  
- [`contribution-guidelines.md`](processes/contribution-guidelines.md) → Guidelines for contributing (PRs, issues, branches)  
- [`code-review.md`](processes/code-review.md) → Code review process and best practices  

## 📌 How to Use This Documentation

- Each section is **self-contained** and linked above for easy access.  
- If you are **new to the project**, start with:  
  - [`setup/installation.md`](setup/installation.md)  
  - [`setup/environment.md`](setup/environment.md)  
  - [`git/commit-conventions.md`](git/commit-conventions.md)  
- If you are working on **backend features**, check [`backend/`](backend/).  
- If you are working on **frontend features**, check [`frontend/`](frontend/).  
- For **deployment and DevOps tasks**, refer to [`ci-cd/`](ci-cd/).  

## 🛠 Contributing to the Documentation

- If you find **missing or outdated information**, feel free to **open an issue or a Pull Request**.  
- Follow the [`contribution-guidelines.md`](processes/contribution-guidelines.md) before submitting changes.  
- Maintain **clarity, accuracy, and conciseness** in documentation.  

🚀 **Happy coding!**  
