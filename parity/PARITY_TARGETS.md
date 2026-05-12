# Minimal Parity Targets

## Allow Target

Executable:

/usr/bin/true

Expected Runtime Result:

- execution succeeds
- exit code = 0
- telemetry emits ALLOW
- Rust model predicts ALLOW

Reason:

- deterministic
- minimal userspace behavior
- stable across distributions

---

## Deny Target

Executable:

/usr/bin/false

Expected Runtime Result:

- execution denied before userspace execution
- shell receives EPERM
- telemetry emits DENY
- Rust model predicts DENY

Reason:

- deterministic
- minimal runtime noise
- easy behavioral distinction from allow case

---

# Canonical Identity Requirement

Both targets MUST resolve through:

linux_binprm
    -> file
    -> f_path
    -> dentry
    -> d_inode
    -> (i_ino + s_dev)

Pathnames are NOT authoritative.

Only canonical kernel object identity is authoritative.

---

# Minimal Telemetry Requirement

Telemetry MUST expose:

- executable identity
- allow/deny verdict
- policy decision
- pid_tgid
- stage identifier

No additional telemetry required for Phase 10.

