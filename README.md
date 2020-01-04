A benchmark of mutex vs spinlock throughput for an extremely short critical section under varying levels of contention on "average" desktop.

Inspiration and code for `AmdSpinlock` are from https://probablydance.com/2019/12/30/measuring-mutexes-spinlocks-and-how-bad-the-linux-scheduler-really-is/.

Summary of results:

* Spinlocks are almost always significantly worse than a good mutex, and never significantly better,
* Contention makes spinlocks relatively slower.

Biggest known caveat (apart from this being a single benchmark run on a single machine):

The best mutex implementation seems to be relatively more optimized than the best spinlock implementation.

## Results

**extreme contention:**
```
12:31:05|~/projects/lock-bench|master⚡*
λ cargo run --release 32 2 10000 100
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/lock-bench 32 2 10000 100`
Options {
    n_threads: 32,
    n_locks: 2,
    n_ops: 10000,
    n_rounds: 100,
}

std::sync::Mutex     avg 97.770106ms  min 38.799445ms  max 103.42306ms
parking_lot::Mutex   avg 68.350542ms  min 32.139233ms  max 72.404877ms
spin::Mutex          avg 142.257494ms min 69.860396ms  max 217.587871ms
AmdSpinlock          avg 127.612286ms min 50.407761ms  max 219.429909ms

std::sync::Mutex     avg 98.838394ms  min 68.180052ms  max 125.635571ms
parking_lot::Mutex   avg 68.51149ms   min 58.805279ms  max 71.512899ms
spin::Mutex          avg 139.751964ms min 54.499263ms  max 193.70374ms
AmdSpinlock          avg 127.757924ms min 50.249234ms  max 210.452623ms
```

**heavy contention:**
```
12:34:39|~/projects/lock-bench|master⚡*
λ cargo run --release 32 64 10000 100
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/lock-bench 32 64 10000 100`
Options {
    n_threads: 32,
    n_locks: 64,
    n_ops: 10000,
    n_rounds: 100,
}

std::sync::Mutex     avg 21.538657ms  min 11.704293ms  max 23.688275ms
parking_lot::Mutex   avg 10.016941ms  min 6.787887ms   max 11.7508ms
spin::Mutex          avg 55.555043ms  min 7.639845ms   max 161.030869ms
AmdSpinlock          avg 40.82985ms   min 6.174719ms   max 123.545934ms

std::sync::Mutex     avg 21.489658ms  min 20.344423ms  max 24.05294ms
parking_lot::Mutex   avg 9.640073ms   min 6.782365ms   max 12.600402ms
spin::Mutex          avg 48.74331ms   min 7.601425ms   max 138.172171ms
AmdSpinlock          avg 40.993328ms  min 8.365127ms   max 110.106416ms
```

**light contention:**
```
12:29:01|~/projects/lock-bench|master⚡*
λ cargo run --release 32 1000 10000 100
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/lock-bench 32 1000 10000 100`
Options {
    n_threads: 32,
    n_locks: 1000,
    n_ops: 10000,
    n_rounds: 100,
}

std::sync::Mutex     avg 13.897816ms  min 8.176447ms   max 15.680984ms
parking_lot::Mutex   avg 6.553058ms   min 3.284944ms   max 8.230407ms
spin::Mutex          avg 37.946399ms  min 4.668167ms   max 115.748116ms
AmdSpinlock          avg 39.530919ms  min 2.049988ms   max 127.139724ms

std::sync::Mutex     avg 13.922504ms  min 12.885598ms  max 15.28518ms
parking_lot::Mutex   avg 6.80546ms    min 5.621588ms   max 8.932723ms
spin::Mutex          avg 39.411306ms  min 4.752935ms   max 102.888667ms
AmdSpinlock          avg 37.423773ms  min 5.087086ms   max 103.319751ms
```

**no contention:**
```
12:26:25|~/projects/lock-bench|master⚡*
λ cargo run --release 32 1000000 10000 100
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/lock-bench 32 1000000 10000 100`
Options {
    n_threads: 32,
    n_locks: 1000000,
    n_ops: 10000,
    n_rounds: 100,
}

std::sync::Mutex     avg 15.975801ms  min 8.487617ms   max 27.106072ms
parking_lot::Mutex   avg 7.142194ms   min 4.594504ms   max 9.178952ms
spin::Mutex          avg 5.947778ms   min 4.600631ms   max 8.262928ms
AmdSpinlock          avg 6.512221ms   min 5.059464ms   max 10.408178ms

std::sync::Mutex     avg 15.793032ms  min 8.260458ms   max 27.850666ms
parking_lot::Mutex   avg 6.910369ms   min 4.32757ms    max 9.08715ms
spin::Mutex          avg 5.890677ms   min 4.424554ms   max 7.622798ms
AmdSpinlock          avg 6.416132ms   min 5.852334ms   max 7.349909ms
```

## Machine Spec

```
12:45:27|~/projects/lock-bench|master⚡*?
λ cat /etc/os-release
NAME=NixOS
ID=nixos
VERSION="19.09.1693.eab4ee0c27c (Loris)"
VERSION_CODENAME=loris
VERSION_ID="19.09.1693.eab4ee0c27c"
PRETTY_NAME="NixOS 19.09.1693.eab4ee0c27c (Loris)"
LOGO="nix-snowflake"
HOME_URL="https://nixos.org/"
DOCUMENTATION_URL="https://nixos.org/nixos/manual/index.html"
SUPPORT_URL="https://nixos.org/nixos/support.html"
BUG_REPORT_URL="https://github.com/NixOS/nixpkgs/issues"

12:45:32|~/projects/lock-bench|master⚡*?
λ uname -r
4.19.91

12:45:34|~/projects/lock-bench|master⚡*?
λ lscpu
Architecture:        x86_64
CPU op-mode(s):      32-bit, 64-bit
Byte Order:          Little Endian
Address sizes:       39 bits physical, 48 bits virtual
CPU(s):              8
On-line CPU(s) list: 0-7
Thread(s) per core:  2
Core(s) per socket:  4
Socket(s):           1
NUMA node(s):        1
Vendor ID:           GenuineIntel
CPU family:          6
Model:               158
Model name:          Intel(R) Core(TM) i7-7820HQ CPU @ 2.90GHz
Stepping:            9
CPU MHz:             1073.198
CPU max MHz:         3900.0000
CPU min MHz:         800.0000
BogoMIPS:            5808.00
Virtualization:      VT-x
L1d cache:           32K
L1i cache:           32K
L2 cache:            256K
L3 cache:            8192K
NUMA node0 CPU(s):   0-7
Flags:               fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq dtes64 monitor ds_cpl vmx smx est tm2 ssse3 sdbg fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb invpcid_single pti ssbd ibrs ibpb stibp tpr_shadow vnmi flexpriority ept vpid ept_ad fsgsbase tsc_adjust bmi1 hle avx2 smep bmi2 erms invpcid rtm mpx rdseed adx smap clflushopt intel_pt xsaveopt xsavec xgetbv1 xsaves dtherm ida arat pln pts hwp hwp_notify hwp_act_window hwp_epp flush_l1d
```
