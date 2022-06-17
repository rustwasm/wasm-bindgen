# Unstable APIs

It's common for browsers to implement parts of a web API while the specification
for that API is still being written. The API may require frequent changes as the
specification continues to be developed, so the WebIDL is relatively unstable.

This causes some challenges for `web-sys` because it means `web-sys` would have
to make breaking API changes whenever the WebIDL changes. It also means that
previously published `web-sys` versions would be invalid, because the browser
API may have been changed to match the updated WebIDL.

To avoid frequent breaking changes for unstable APIs, `web-sys` hides all
unstable APIs behind the `unstable` feature.

By hiding unstable APIs through a feature gate, it's necessary for crates to
explicitly opt-in to these reduced stability guarantees in order to use these
APIs. Specifically, these APIs do not follow [semver](https://semver.org/) and
may break whenever the WebIDL changes.

Crates can opt-in to unstable APIs at compile-time by setting the `unstable`
feature flag for `web-sys` in their `Cargo.toml`. For example, since the `Gpu`
struct is unstable, a user would write
```
[dependencies]
web-sys = { version = "0.3", features = ["unstable", "Gpu"] }
```
