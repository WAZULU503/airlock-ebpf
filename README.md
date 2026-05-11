# Airlock

Kernel-level execution enforcement using Rust, eBPF, and Linux Security Modules (LSM).

Intercept and govern process execution before userspace execution occurs.

> **WARNING:** Airlock requires root privileges, BPF LSM enabled in the kernel, and can deny execution system-wide. Test inside a VM or isolated environment first.

# What Airlock Does

Airlock intercepts execution requests at the Linux kernel boundary using BPF LSM hooks.

The current prototype focuses on:

- BPF LSM execution interception
- CO-RE runtime kernel adaptation
- Canonical inode-backed identity extraction
- Verifier-safe kernel object traversal
- Kernel-level EPERM execution denial

This repository explores deterministic execution enforcement through kernel-level enforcement rather than userspace trust boundaries.

This prevents compromised or injected userspace components from bypassing execution policies, since enforcement occurs before userspace execution begins.

# Current Status

Verified on:

- Ubuntu ARM64 VM
- Linux kernel `6.7+`
- Apple Silicon virtualization environments (UTM / VMware Fusion)
- Aya eBPF runtime

Verified capabilities:

- BPF LSM hook attachment
- Runtime BTF / CO-RE adaptation
- `linux_binprm -> file -> f_path -> dentry -> d_inode` traversal
- Canonical `(i_ino + s_dev)` extraction
- Kernel-level execution denial via `EPERM`
- Stable verifier-safe kernel object traversal

The current prototype validates userspace-controlled execution governance through a verifier-safe POLICY_MAP integrated into the BPF LSM execution path.

Runtime policy insertion, identity lookup, telemetry emission, and ACTION_DENY -> EPERM enforcement have been verified against a live kernel runtime.

# Verified Execution Path

```text
linux_binprm
    -> file
    -> f_path
    -> dentry
    -> d_inode
    -> (i_ino + s_dev)
    -> enforcement verdict
```

This moved Airlock away from brittle pathname-only matching toward canonical kernel object identity.

# Example Enforcement

Example execution denial:

```bash
$ /usr/lib/cargo/bin/coreutils/ls
bash: /usr/lib/cargo/bin/coreutils/ls: Operation not permitted
```

The denial occurs inside the Linux kernel through a BPF LSM hook.

# Why This Exists

Most AI and automation runtimes rely entirely on userspace trust boundaries.

