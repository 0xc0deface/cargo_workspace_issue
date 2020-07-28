# cargo_workspace_issue
This project is to demonstrate an issue with workspaces.

When you build the project with `cargo build` or `cargo test` it will produce the following error:

```
error: multiple packages link to native library `sqlite3`, but a native library can be linked only once

package `libsqlite3-sys v0.9.1`
    ... which is depended on by `diesel v1.4.5`
    ... which is depended on by `lib1 v0.1.0 (/home/user/dev/cargo_workspace_issue/lib1)`
links to native library `sqlite3`

package `sqlite3-src v0.2.12`
    ... which is depended on by `sqlite3-sys v0.12.0`
    ... which is depended on by `sqlite v0.25.3`
    ... which is depended on by `lib2 v0.1.0 (/home/user/dev/cargo_workspace_issue/lib2)`
    ... which is depended on by `lib1 v0.1.0 (/home/user/dev/cargo_workspace_issue/lib1)`
also links to native library `sqlite3`
The terminal process "/bin/bash '-c', 'cargo build'" failed to launch (exit code: 101).
```

If however you build each of the libs seperately then everything works:

```
cargo build --package lib1
cargo build --package lib2
cargo test --package lib1
cargo test --package lib2
```

Whatever scan cargo does to determine if something links twice may be ignoring dev-dependencies. lib2 is linking sqlite for its tests 
and this seems to be causing lib1 to not build when building via the workspace.
