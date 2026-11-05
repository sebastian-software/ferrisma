# Gate pull requests on Linux CI and local macOS checks

Hosted macOS runners are shared and scarce across the organization. On
2026-11-05 macOS jobs waited about 40 minutes each before starting while
Linux jobs started at once, and every defect that hosted gates found that day
was Linux-specific. Development happens on macOS ARM64, where the root and
standalone checks already run locally before a change is pushed.

During development, pull requests and pushes to `main` therefore run hosted
checks on GNU Linux x86-64 only. The dav1d and SharpYUV workflows drop their
macOS parity job for those events; the root workflow already restricts its
macOS profiles to the nightly and manual tiers. A change that affects the
macOS build or its translations lands after the applicable local macOS checks
pass, and the pull request records the commands and results. Nightly runs and
manual full runs keep both targets unchanged.

A local run remains local evidence, recorded as such (see PLATFORMS.md). A
release, a new production target admission, or any claim of hosted macOS
validation needs the corresponding hosted macOS runs on the exact revision.
Revisit this when hosted macOS capacity stops delaying pull requests or before
the first release.
