# Rollback Strategy

This project currently uses a **versioning-based rollback strategy**:

1. Each release is tagged (e.g., `v1.0.0`).
2. In case of failure, the previous stable version is redeployed.\n\nFuture improvements:
- **Blue-Green Deployment** to eliminate downtime.
- **Feature Flags** for more granular control.
