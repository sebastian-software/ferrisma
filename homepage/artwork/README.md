# Ferrixel artwork

The raster loupe is Ferrixel's image-inspection motif. The homepage instrument
places an inspection ring above a coarse pixel grid, beside a finer source
raster on a bright steel plate with recessed hex screws. The compact product
mark keeps the ring, seven bold raster cells, and one oxide handle collar.
Its simpler silhouette is used in the header and browser icons.

Both are symbolic illustrations, not optical simulations or claims about
pipeline capabilities. Their native Blender recipes use Ferramenta's studio
and solid-modeling helpers from
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
blender --background --python homepage/artwork/ferrixel-mark.py -- \
  --family-source /path/to/ferramenta --output /tmp/ferrixel-mark-master.png
node homepage/artwork/export-mark.ts /tmp/ferrixel-mark-master.png /path/to/ferramenta
```

The instrument draft uses 1200×600 and 32 samples; the final uses 2400×1200
and 192 samples. The mark uses 768×768 and 48 samples with `--draft`, or
1600×1600 and 160 samples for the final. Both use Cycles, a transparent
background, and AgX. Each final render writes a JSON sidecar beside its master.
The committed provenance records the source, helper and master hashes,
exported asset hashes, renderer, and geometry checks. The Python recipes are
the editable scene sources; the PNG masters are not needed by the website and
can be reproduced locally.

The TypeScript mark exporter uses ImageMagick, follows the family's 12.5%
square-padding convention, and records its version. It produces a lossless
256px WebP for the header, a 512px PNG for reuse, a 32/48/64px ICO, and a
180px Apple touch icon on the family's paper color. These are resampled from
the same native mark master. The hero export uses `cwebp` 1.6.0 on macOS ARM64.
Hashes identify the captured source and output artifacts; rerendering with
another Blender build, Cycles environment, or image encoder is not guaranteed
to reproduce identical bytes.

The mesh checks validate closed, nonempty individual solids and Boolean cuts;
they do not establish that every intended assembly contact is collision-free.
