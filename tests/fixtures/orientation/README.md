# Orientation fixtures

Independent references for the reorientation stage (ADR-0019).

The source is a 24×16 pattern of 6×4 blocks (4×4 pixels each) with distinct colors, so a wrong orientation produces large errors.

- `avif-N.avif` — written by ImageMagick 7.1.2-35 with libheif 1.23.7 from the pattern with `-orient <EXIF N name> -quality 100`. libheif stores the orientation as `irot`/`imir`; the resulting combinations are (N: irot, imir) 1: –, –; 2: –, 1; 3: 2, –; 4: –, 0; 5: 3, 1; 6: 3, –; 7: 3, 0; 8: 1, –.
- `jpeg-N.jpg` — the pattern encoded once by ImageMagick (`-quality 100 -sampling-factor 1x1`), with an APP1 EXIF segment inserted after SOI whose big-endian IFD0 holds `Orientation = N`.
- `jxl-N.jxl` — the pattern encoded losslessly by libjxl a16d32d (the revision pinned by libjxl-rs) through the public encoder API with `JxlBasicInfo::orientation = N`; `jxl-N.rgb` is the pattern transformed by the EXIF definition of N (exact, cross-checked against the libheif references within 2).
- `avif-N.rgb`, `jpeg-N.rgb` — the upright reference: `magick <file> -auto-orient -depth 8 rgb:<file>.rgb`, i.e. the image as libheif (AVIF) or libjpeg (JPEG) render it after applying the stored orientation. Each reference was checked against explicit flips/rotations of the source pattern (mean error 0.22, max 2).

The JPEG XL fixtures were written by `tools/jxl_orientation.c`, linked against a static libjxl built from that revision.
