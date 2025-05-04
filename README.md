# Space rework

Project to overhaul the `spacecodeur` course platform. The redesign of the project is an opportunity to explore cool technologies; TypeScript/React for the frontend and Rust (Axum) for the backend, all wrapped in a very young framework called `tuono`.

A CI/CD pipeline is set up via Jenkins, and the application is hosted through cloud providers (AWS, ...).

# Get started

- Prerequisites:
  - clone this repo
  - install Docker

- Local setup:
  - build the Docker image: `./commands.sh docker-build`
  - start the development server: `./commands.sh fc-server-start` (fc stand for "from (docker) container")
  - et voilà !

# [Important] Enable Git hooks

To activate the Git hooks provided by this base (like commit message validation), configure Git to use the custom hooks path:

```bash
git config core.hooksPath .git-hooks
```

This command should be executed from the root of your project, after merging.