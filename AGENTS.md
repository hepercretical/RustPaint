# Project instructions

These instructions apply throughout this repository. See README.md for feature
documentation, translation history, detailed limitations and regeneration notes.

## Product direction

- Build a standalone Rust Windows application as close as practical to current
  Microsoft Paint in appearance and behavior, without Copilot or generative AI
  features. Use current Microsoft documentation when assessing version parity.
- Preserve working drawing, editing, image I/O, undo/redo and unsaved-change
  prompts when changing the interface. State remaining differences honestly.
- Complete requested implementation and appropriate verification. Do not stop
  at a proposal when the requested work is authorized and actionable.

## Reference integrity and translation

- `mspaint.c` is an ignored local reference, not a clean-checkout dependency.
  Never edit, reformat or regenerate a local copy. Its SHA-256 must remain
  `01ca7b8866b6918f7e24c65fe02893aea444f2cbde9867981e78160b7a49719d`.
- Treat the original `mspaint.exe` as a read-only recovery reference. Ghidra may
  be used for recovery when needed; retain reproducible evidence and provenance.
- Preserve Windows integer widths, signedness, wrapping arithmetic, call order,
  memory access widths and observable effects before failures. Use checked
  memory and explicit errors rather than unchecked process pointers.
- Do not invent missing globals, vtables, ABI arguments or runtime behavior.
  Keep unresolved dependencies explicit; do not disguise them as successful
  zero returns, empty stubs or whole-function forwarding callbacks.
- Generated Rust adapters and declarations are committed product source. Their
  old generators are ignored local tools. When available and useful, regenerate
  only affected outputs, preserve marked hand-written sections, and run relevant
  provenance checks. Routine Rust development must work without the local archive.
- Preserve exact bytes of reference files, fixtures and recovery evidence.
  `.gitattributes` disables automatic line-ending conversion for this reason.
- Audit coverage before reporting percentages. Distinguish function coverage,
  code lines excluding blank/comment-only lines, and historical physical-file
  coverage. Do not change denominators or count comments as new translations.
- Current audited baselines are 4,410/4,410 functions, 158,221/158,221 code lines,
  and 168,645/187,218 historical lines (90.08%, the usable source ceiling).
  Recheck these values when changing translations. Coverage does not establish
  runtime integration, feature completeness or Microsoft Paint parity.

## Architecture

- Keep `src/lib.rs` free of unsafe code (`#![forbid(unsafe_code)]`). Keep native
  Windows FFI in executable modules and document pointer ownership/lifetimes.
- `src/canvas.rs` owns raster data and bounded undo/redo; `src/runtime.rs` owns
  checked translated startup integration; `src/native_host.rs` supplies native
  runtime calls. `src/desktop.rs` and `src/desktop/` implement the Windows UI;
  `src/image_codec.rs` handles native image codecs.
- Preserve checked image dimensions, bounded history and atomic save behavior:
  failed encoding must not truncate an existing document.
- Keep builds and packaged applications reproducible. The current application
  has no third-party Rust crate dependencies; justify additions when needed.

## Build and verification

Run commands from the repository root. If the local toolchain is needed, use
process-local PowerShell environment variables without changing system PATH:

```powershell
$env:CARGO_HOME = Join-Path $PWD '.tools\cargo'
$env:RUSTUP_HOME = Join-Path $PWD '.tools\rustup'
$env:PATH = "$env:CARGO_HOME\bin;$env:PATH"
```

- For Rust changes, run relevant tests, `cargo fmt --check` and
  `cargo clippy --all-targets -- -D warnings`. Run `cargo test --release` for
  substantial behavior changes and release verification. Add meaningful tests
  for changed behavior rather than tests that merely mirror implementation.
- Run `node scripts/check-repo-hygiene.mjs` after changing ignore rules or test
  input dependencies. Build and test a clean checkout without ignored references.
- When changing translations and the local recovery archive is available, also
  run its relevant reference/regression tests and source audits:

  ```text
  node scripts/audit-progress.mjs --require-complete --require-90
  node scripts/audit-code-coverage.mjs --require-complete
  node scripts/audit-source-ceiling.mjs
  ```

  These audit scripts and `mspaint.c` are intentionally ignored. They are not
  requirements for ordinary builds/tests from GitHub. Do not require users to
  recover proprietary binaries or obtain the translation archive to contribute.

- Build a distributable with
  `powershell -NoProfile -ExecutionPolicy Bypass -File scripts/package-desktop.ps1 -PackageName RustPaint-renderfix`.
  This script performs the packaged native window smoke test. Audit imports with
  `node scripts/audit-desktop-binary.mjs dist/RustPaint-renderfix/RustPaint.exe`.
- Check actual process exit codes. The local rustup installation can emit a
  benign path-canonicalization warning; stderr alone does not prove failure.
- After requested desktop implementation/build work, launch the compiled GUI
  for the user's manual testing. Never stop an existing editor or overwrite its
  running executable merely to release a build lock: it may hold unsaved work.
  Use a different package name/output directory if necessary.
- Report exactly which checks ran. A native smoke test is not mouse/keyboard
  manual verification. If computer-use tooling is unavailable, say so without
  claiming screenshots or interactions were verified.
- Documentation-only and ignore-rule changes do not require recompiling the
  application; verify links, reference integrity and Git ignore behavior instead.

## Repository hygiene and collaboration

- Use the correctly named `.gitignore`; do not create `.gititnore`.
- Treat translation-process files as temporary/local once they are no longer
  actively needed to compile, test, package or maintain the Rust product. Add
  them to `.gitignore`, even if they remain historically important. This includes
  `mspaint.c`, original executables, decompiled/disassembled exports, Ghidra
  projects, old translation generators/harnesses, coverage inventories, recovery
  evidence and one-off verification scripts. Preserve local files unless the
  user separately requests deletion.
- Commit Rust source (including generated Rust that is compiled), `Cargo.toml`,
  `Cargo.lock`, `build.rs`, current documentation/instructions, active tests and
  their required inputs, and scripts used for current builds/releases/checks.
  A translation origin alone does not make an active test dependency disposable.
- Before ignoring a file/directory, check build scripts, module paths, embedded
  `include_str!`/`include_bytes!` inputs, tests, packaging and runtime resources.
  Keep narrow exceptions for required inputs; do not hide missing dependencies
  by removing or disabling tests. Verify a clean checkout builds all test targets.
- Exclude build/release outputs, downloaded toolchains/dependencies, local caches
  and environments, scratch files, logs, backups, crash/debug/profiling outputs,
  machine-specific editor/OS settings and secrets. Prefer scoped patterns; avoid
  global extension rules that would hide real source, assets or test fixtures.
- Update ignore rules as files become active or obsolete. Remove exceptions for
  inputs with no active consumers; add exceptions when a new required file would
  otherwise be ignored. Document why retained translation metadata is required.
- Translation metadata and reference-capture directories must default to ignored,
  with exact file exceptions for active consumers. Avoid broad exceptions such as
  `!batch-*.json` that could expose future unused exports. Run the hygiene checker
  when adding/removing an embedded input and update the exact exceptions together.
- For pending-file audits, report actual current counts and the inventory scope,
  distinguish real files from hypothetical ignore examples, and save an exact
  excluded-file list under ignored `.tools/`. Use Git status/index data when a
  repository exists; otherwise say that a displayed pending-commit count cannot
  be verified. Do not remove active tests or compiled Rust just to reduce a count.
- `.gitignore` does not remove already tracked files from Git. If a repository
  exists, inspect its index before claiming excluded material will not be pushed;
  never delete local reference files to achieve repository hygiene.
- Check ignore changes using Git, including examples that must remain trackable.
  If no root Git repository exists, use an ignored scratch repository for checks.
  Do not initialize or publish a root repository unless requested.
- Keep README.md focused on users, architecture and reproducible development
  documentation; keep agent workflow rules here. Update documented behavior and
  limitations when relevant changes are made.
- Preserve unrelated user work. Give concise progress updates during substantial
  work and summarize changes, validation and material remaining limitations.
