This stops the userspace loader and detaches the eBPF program.

Normal system behavior is restored immediately if the program is not pinned.

---

## 🔍 If Ctrl+C Is Not Available

From another already-open terminal:

```bash
ps aux | grep airlock
sudo kill <pid>
