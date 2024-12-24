# Space rework

Project to overhaul the `spacecodeur` course platform. The redesign of the project is an opportunity to explore cool technologies; TypeScript/React for the frontend and Rust (Axum) for the backend, all wrapped in a very young framework called `tuono`.

A CI/CD pipeline is set up via Jenkins, and the application is hosted through cloud providers (AWS, ...).

# Get started

- Prerequisites:
  - Install Docker

- Local setup:
  - Build the Docker image: `./commands.sh docker-build`
  - Start the development server: `./commands.sh server-start`
