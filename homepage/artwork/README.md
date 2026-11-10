# Ferrixel instrument

`ferrixel-instrument.py` constructs a source raster, an adjustable crop
aperture, a filter carriage and a smaller output raster. The geometry is an
illustration of the image pipeline, not a claim that the engine uses physical
hardware. It uses Ferramenta's Blender studio and solid-modeling helpers from
commit `5284a1264ed78f71c72387621e63fb997b7a8842`; the script checks the
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
samples. Both use Cycles, a transparent background, and AgX. The final render
writes a JSON sidecar beside the master. The committed provenance records the
source, helper and master hashes, final WebP hash, renderer, and geometry
checks. The Python recipe is the editable scene source; the PNG master is not
needed by the site and can be reproduced locally.

The committed export used `cwebp` 1.6.0 on macOS ARM64. Hashes identify the
captured source and output artifacts; a rerender with another Blender build,
Cycles execution environment or WebP encoder is not guaranteed to reproduce
identical bytes.

The mesh checks validate closed, nonempty individual solids and Boolean cuts;
they do not establish that every intended assembly contact is collision-free.
