# Jenkins Pipeline

This project uses a **multi-step Jenkins pipeline** with the following stages:

1. **Build**: Compile Rust backend & prepare frontend assets.
2. **Test**: Run unit & integration tests.
3. **Deploy**: Push to AWS.

Triggers (e.g., push to `develop` or `main`) are not yet fully defined.
