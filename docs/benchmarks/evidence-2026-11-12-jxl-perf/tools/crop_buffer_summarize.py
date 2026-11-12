import re, statistics, sys
rows = {}
loads = []
for line in open(sys.argv[1]):
    parts = line.split()
    if len(parts) < 3 or parts[0] == 'warm':
        continue
    tag, case, which = parts[:3]
    kv = dict(re.findall(r'(\w+)=([0-9.]+)', line))
    loads.append(float(kv['load']))
    rows.setdefault((case, which), []).append(kv)
print(f"load {min(loads):.1f}-{max(loads):.1f}")
print("| case | build | decode ms median (min-max) | total ms median | managed MB | max RSS MB | full frames |")
print("| --- | --- | ---: | ---: | ---: | ---: | ---: |")
for (case, which), samples in rows.items():
    dec = [int(s['decode_us']) / 1000 for s in samples]
    tot = [int(s['total_us']) / 1000 for s in samples]
    man = {int(s['peak_managed_bytes']) for s in samples}
    rss = [int(s['rss']) / 1e6 for s in samples]
    ff = {s['decoder_full_frames'] for s in samples}
    print(f"| {case} | {which} | {statistics.median(dec):,.0f} ({min(dec):,.0f}-{max(dec):,.0f}) | {statistics.median(tot):,.0f} | "
          f"{'/'.join(f'{m/1e6:.1f}' for m in sorted(man))} | {statistics.median(rss):.1f} ({min(rss):.1f}-{max(rss):.1f}) | {'/'.join(sorted(ff))} |")
