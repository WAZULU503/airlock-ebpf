
# Airlock

**Kernel-level execution control. Zero trust by default.**

Airlock attaches to the Linux kernel's LSM hook (`bprm_check_security`) and intercepts every `execve()` call before a new program image is executed.

```text
Return  0      → process runs
Return -EPERM  → execution denied before program start
```

No userspace wrappers. No shell hooks. The kernel decides.

---

## What this version does

**v1.0.0 is a denial baseline.**

Every new process execution is blocked.

```sh
$ ls      # Permission denied
$ whoami  # Permission denied
```

Already-running processes are not affected.
This is not a killswitch. It is a proof of enforcement.

---

## Why this matters

Most execution control lives in userspace — watchdogs, wrappers, shell hooks.
All of them can be bypassed by code that gains execution first.

Airlock runs through the Linux Security Module framework at the kernel boundary, where execution decisions are enforced before the target program runs.

---

## Run it

```sh
sudo cargo xtask run
```

Stop it:

```sh
Ctrl+C
# or
sudo kill <pid>
```

Not persistent across reboot unless explicitly pinned.

---

## Requirements

| Requirement               | Detail                      |
| ------------------------- | --------------------------- |
| Linux kernel              | BPF + BPF LSM + BTF enabled |
| `/sys/kernel/btf/vmlinux` | Must exist                  |
| Privileges                | Root required               |
| Toolchain                 | Rust (nightly), eBPF target |

---

## Architecture

```text
execve() syscall
    ↓
bprm_check_security  ← LSM hook
    ↓
Airlock eBPF program
    ↓
0 = allow  |  -EPERM = deny
```

One hook. One decision point. Deterministic.

---

## What's not in this version

| Missing                  | Planned |
| ------------------------ | ------- |
| Per-binary filtering     | v1.1.0  |
| Rule-based policy engine | v1.2.0  |
| Audit / logging pipeline | v1.3.0  |
| CO-RE struct traversal   | v1.3.0  |

This version enforces. It does not discriminate.

---

## Roadmap

```text
v1.0.0  baseline denial        ← this release
v1.1.0  targeted binary control
v1.2.0  rule-based policy engine
v1.3.0  audit pipeline + BTF-aware field access
```

---

## Warning

This tool blocks process execution at the kernel level.
Test in a VM or isolated environment.
`Ctrl+C` is your off switch.

---

## License

GPL-2.0 — see [LICENSE](./LICENSE) for details.

Airlock is intended to remain open and freely inspectable.
Commercial closed-source redistribution is not permitted under the license terms.
