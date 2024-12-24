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