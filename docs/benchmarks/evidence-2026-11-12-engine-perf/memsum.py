"""Summarize malloc_history -allBySize output: top live allocation sites.

Usage: python3 -I memsum.py <file> [top]
Prints bytes, call count and the innermost few non-allocator frames.
"""
import re
import sys

top = int(sys.argv[2]) if len(sys.argv) > 2 else 20
SKIP = re.compile(r"malloc|calloc|realloc|aom_memalign|aom_malloc|aom_calloc|avifAlloc|__rdl_|__rust_alloc|alloc::alloc|"
                  r"RawVec|_platform_|posix_memalign|mmap|vm_allocate|operator new|aligned_alloc|aom_mem|memalign|"
                  r"alloc::raw_vec|alloc::vec|try_allocate|finish_grow|libsystem_malloc")
rows = []
total = 0
for line in open(sys.argv[1], errors="replace"):
    m = re.match(r"(\d+) calls? for (\d+) bytes: (.*)", line)
    if not m:
        continue
    calls, size, stack = int(m.group(1)), int(m.group(2)), m.group(3)
    total += size
    frames = [f.strip() for f in stack.split(" | ")]
    names = []
    for f in reversed(frames):
        n = re.sub(r"^0x[0-9a-f]+ \([^)]*\) ", "", f)
        n = re.sub(r"\s+\S+\.(rs|c):\d+$", "", n)
        if SKIP.search(n):
            continue
        names.append(n[:70])
        if len(names) == 3:
            break
    rows.append((size, calls, " < ".join(names)))
rows.sort(reverse=True)
print(f"live malloc bytes in report: {total/1e6:.1f} MB")
print("| MB | calls | innermost frames (callee < caller) |\n| ---: | ---: | --- |")
for size, calls, names in rows[:top]:
    print(f"| {size/1e6:.1f} | {calls} | `{names}` |")
