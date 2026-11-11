# Ferrixel artwork

## Instrument

`ferrixel-instrument.py` constructs an image railway. One broad steel bed
follows a gentle S-curve and carries two rails; flanged-wheel wagons bring a
source picture under a raised guillotine, a half-size copy under a spindle
press, and the same motif as square pixels beneath an adjustable loupe. The
guillotine and press stand on both sides of the rails and the loupe reaches
over them, so a wagon can pass every station. The press sits on the middle
straight, where its oxide crosshead lies almost level in the view. The loupe
head tilts toward the orthographic camera so the glass lens magnifies the
pixels instead of reflecting the track. The geometry is an illustration of the
image pipeline, not a claim that the engine uses physical hardware or that
these stages are its complete or current feature set. It uses Ferramenta's
Blender studio and solid-modeling helpers from commit `5284a1264ed78f71c72387621e63fb997b7a8842`; the script checks the
commit and both helper hashes before importing them. The Ferramenta source is
MIT-licensed and is read without modification. Blender and the family source
are authoring tools only, not dependencies of the website build or tests.

From the Ferrixel root, with Blender 5.3 and the pinned Ferramenta checkout:

```sh
blender --background --python homepage/artwork/ferrixel-instrument.py -- \
  --family-source /path/to/ferramenta --output /tmp/ferrixel-draft.png --draft
blender --background --python homepage/artwork/ferrixel-instrument.py -- \
  --family-source /path/to/ferramenta --output /tmp/ferrixel-instrument-master.png
cwebp -q 90 /tmp/ferrixel-instrument-master.png -o homepage/public/ferrixel-instrument.webp
```

The draft uses 1200×600 and 32 samples; the final uses 2400×1200 and 192
samples. Both use Cycles, a transparent background, and AgX. Add
`--blend /path/to/scene.blend` to save the constructed scene for editing. The
final render writes a JSON sidecar beside the master. The committed provenance records the
source, helper and master hashes, final WebP hash, renderer, and geometry
checks. The Python recipe is the editable scene source; the PNG master is not
needed by the site and can be reproduced locally.

The committed export used `cwebp` 1.6.0 on macOS ARM64. Hashes identify the
captured source and output artifacts; a rerender with another Blender build,
Cycles execution environment or WebP encoder is not guaranteed to reproduce
identical bytes.

The mesh checks validate closed, nonempty individual solids and Boolean cuts;
they do not establish that every intended assembly contact is collision-free.

## Mark

`ferrixel-mark.py` builds the compact header mark with the family's mark
helpers (`studio.py` and `family_marks.py`, pinned to the same commit and
checked by hash). A polished steel picture plate has a stepped pixel mountain
cut through into a dark graphite bed; a crowned copper sun, as on the
instrument's picture samples, sits in its sky; a graphite registration corner
holds the plate. It stays site-local until the family registry includes
Ferrixel.

```sh
blender --background --python homepage/artwork/ferrixel-mark.py -- \
  --family-source /path/to/ferramenta --output /tmp/ferrixel-mark-master.png
M=/tmp/ferrixel-mark-master.png
B=$(magick "$M" -alpha extract -threshold 10% -trim -format "%wx%h%X%Y" info:)
for s in 64 256; do
  magick "$M" -crop "$B" +repage -background none -gravity center \
    -extent "%[fx:ceil(max(w,h)/0.875)]x%[fx:ceil(max(w,h)/0.875)]" \
    -filter Lanczos -resize "${s}x${s}" "/tmp/ferrixel-mark-$s.png"
done
magick /tmp/ferrixel-mark-256.png -define webp:lossless=true homepage/app/assets/ferrixel-mark.webp
cp /tmp/ferrixel-mark-64.png homepage/app/assets/ferrixel-favicon.png
magick /tmp/ferrixel-mark-256.png -resize 156x156 -background '#F4F0E6' \
  -gravity center -extent 180x180 -alpha remove homepage/app/assets/ferrixel-apple-touch-icon.png
```

The export mirrors Ferramenta's `design/ferra/build-assets.mjs`: the alpha
threshold only measures bounds, the square extent leaves the family's 12.5%
margin, and resizing uses Lanczos. The master is 1600×1600 with 160 samples.
`ferrixel-mark.provenance.json` records the helper, script, master and export
hashes; the committed exports used ImageMagick 7.1.2 on macOS ARM64.
