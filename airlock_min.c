#include "vmlinux.h"

#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "GPL";

SEC("lsm/bprm_check_security")
int BPF_PROG(airlock_min, struct linux_binprm *bprm)
{
    bpf_printk("AIRLOCK EXEC HIT");

    return 0;
}
