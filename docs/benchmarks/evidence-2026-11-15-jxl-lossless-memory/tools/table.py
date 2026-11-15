"""Per-pixel peaks from results.txt: max of the repetitions, RSS and heap.
python3 -I table.py results.txt"""
import re, sys
pixels = {"c512x1024": 512 * 1024, "c700x700": 700 * 700, "c1024x1024": 1024 * 1024,
          "c2048x2048": 2048 * 2048, "c3000x2000": 3000 * 2000,
          "photo12": 4032 * 3024, "photo24": 6000 * 4000}
rows = {}
for line in open(sys.argv[1]):
    m = re.match(r"(\S+)-(port-fixed|port-api0|port-api-default|cjxl-buffering0|cjxl-default)-\d+ "
                 r"status=0 max_rss=(\d+) heap_peak=(\S+)", line)
    if not m:
        continue
    image, mode, rss, heap = m[1], m[2], int(m[3]), m[4]
    r = rows.setdefault((image, mode), [0, 0])
    r[0] = max(r[0], rss)
    r[1] = max(r[1], int(heap) if heap != "-" else 0)
print("| Input | Pixels | Path | Peak RSS (MB) | RSS B/px | Heap B/px |")
print("| --- | ---: | --- | ---: | ---: | ---: |")
for (image, mode), (rss, heap) in rows.items():
    px = pixels[image.rstrip("a")]
    h = f"{heap / px:.1f}" if heap else "-"
    print(f"| {image} | {px:,} | {mode} | {rss / 1e6:,.1f} | {rss / px:.1f} | {h} |")
