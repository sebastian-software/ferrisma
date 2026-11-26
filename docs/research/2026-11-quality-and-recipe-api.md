# Ferrisma recipe API: web research (2026-11-26)

Every claim has a URL. "UNCONFIRMED" marks what I could not verify.

## 1. Quality scales in other tools

**sharp** has a per-format `quality` option, 1-100 ([sharp output docs](https://sharp.pixelplumbing.com/api-output)):

| Format | Default quality | Effort default |
|---|---|---|
| jpeg | 80 | none |
| webp | 80 | 4 (range 0-6) |
| avif | 50 | 4 (range 0-9) |
| jxl | `distance` 1.0 (`quality` has no default; if given it overrides `distance`) | 7 (range 1-9) |

The defaults are deliberately not equivalent.

**imgproxy** has a global `IMGPROXY_QUALITY` (default 80) and per-format `IMGPROXY_FORMAT_QUALITY` (default `webp=79,avif=63,jxl=77`). ([options](https://docs.imgproxy.net/configuration/options))
- Autoquality is Pro only. Methods are `none` (default), `size`, `dssim` and `ml`. The saved image is re-encoded several times to reach the target.
- Defaults: target `0.02`, allowed error `0.002` (dssim/ml only), min `70`, max `80`, per-format min/max `avif=60`/`avif=65`.
- Per-format neural-net model paths exist for jpeg, webp, avif and jxl.
- Not documented on the pages I read: what `size` does and how `target` is interpreted per method. UNCONFIRMED.
- Per-request options are `quality`/`q`, `format_quality`/`fq` and `autoquality`/`aq`. ([processing](https://docs.imgproxy.net/usage/processing))

**Cloudinary q_auto** has levels `best`, `good` (the default `q_auto`), `eco` and `low`. A `:sensitive` suffix raises quality partway to the next level. `q_auto` may become `eco` when the browser sends Save-Data. ([docs](https://cloudinary.com/documentation/image_optimization))
- How it works: it analyses each image and uses "perceptual metrics and heuristics tuned to the image content and format". The page gives no more detail. The algorithm is proprietary and not specified.
- The docs do not describe any `dpr` and `q_auto` interplay. UNCONFIRMED.

**Squoosh**: libsquoosh keeps defaults in `defaultEncoderOptions` in `codecs.ts`. I could not retrieve that file. Wrapper docs show mozjpeg 75 and AVIF 50, and one usage guide says AVIF 50 and WebP 75 are not equivalent targets. Treat the exact defaults as UNCONFIRMED. ([libsquoosh README](https://cdn.jsdelivr.net/npm/@squoosh/lib@0.5.3/README.md), [npm wrapper](https://www.npmjs.com/package/@frostoven/libsquoosh))

**Thumbor**: the config docs list only `QUALITY` (JPEG, default 80). I found no per-format or auto-quality setting on that page. ([config](https://thumbor.readthedocs.io/en/latest/configuration.html))

**libvips**: `jpegsave` `Q` defaults to 75. I did not find the other savers' defaults. ([jpegsave](https://www.libvips.org/API/current/method.Image.jpegsave.html))

**Other CDNs**: Fastly and Bunny take one `quality` value applied to jpg/webp/avif. Neither claims the value is calibrated across formats, and Bunny suggests 70-85. Akamai has a "perceptual quality" mode. ([Fastly](https://www.fastly.com/documentation/reference/io/quality/), [Bunny](https://docs.bunny.net/optimizer/dynamic-images/quality), [Akamai](https://techdocs.akamai.com/ivm/docs/optimize-images))

**Does anyone offer one cross-format scale?**
- Among mainstream tools, no. Every tool above takes a raw per-encoder number.
- Cloudinary `q_auto` and imgproxy Pro autoquality hide the per-format number, but by per-image search against a metric, not by a fixed calibrated scale.
- A vendor blog describes mapping one 1-100 value to per-format settings via SSIMULACRA 2, with no independent verification. ([modpagespeed](https://modpagespeed.com/blog/learned-quality-prediction/))

**Calibration evidence from users** (small studies, not scales a tool ships):
- Wagtail 7.3 matched formats by SSIMULACRA 2 on CID22 (250 images of 512x512; libjpeg-turbo, libavif 4:2:0, speed 6). ([Wagtail](https://wagtail.org/blog/40-smaller-images-same-quality/))

| Use case | JPEG | WebP | AVIF |
|---|---|---|---|
| General web | 76 | 80 | 61 |
| Full-size photography | 85 | 87 | 73 |
| Large thumbnails | 65 | 70 | 54 |
| Small thumbnails | 55 | 57 | 49 |

  - Their old defaults, JPEG 85 / WebP 80 / AVIF 80, were inconsistent.
  - The mapping is not linear: the JPEG-to-AVIF offset is 15 at 76/61 and 11 at 85/73.
- Industrial Empathy matched JPEG, WebP and AVIF by DSSIM, using sharp and only 4 images. ([post](https://industrialempathy.com/posts/avif-webp-quality-settings))

| JPEG | WebP | AVIF |
|---|---|---|
| 50 | 55 | 48 |
| 60 | 64 | 51 |
| 70 | 72 | 56 |
| 80 | 82 | 64 |

  - The post's headline rule is JPEG 60 = AVIF 50 / WebP 65, slightly different from its own table.
  - Caveat: encoder, image size and content change the mapping.

## 2. Metrics

| | DSSIM | SSIMULACRA 2 | Butteraugli |
|---|---|---|---|
| Basis | Multi-scale SSIM variant, computed in L\*a\*b\*, linear-light scaling | MS-SSIM-like in XYB, plus blockiness/ringing and blur error maps | Psychovisual difference model |
| Output | 1/SSIM - 1, 0 = identical, unbounded, not comparable across versions | -inf to 100, higher is better | Scalar plus spatial map, lower is better |
| License | AGPL or commercial | BSD-3-Clause (plus a PATENTS file) | Apache-2.0, repo archived 2023-11-14 |

Sources: [dssim](https://github.com/kornelski/dssim), [ssimulacra2](https://github.com/cloudinary/ssimulacra2), [butteraugli](https://github.com/google/butteraugli).

**SSIMULACRA 2 score bands** ([repo](https://github.com/cloudinary/ssimulacra2)):

| Score | Meaning |
|---|---|
| 10 | very low quality |
| 30 | low, annoying artifacts |
| 50 | medium |
| 70 | high, artifacts perceptible but not annoying |
| 80 | very high, not noticeable side by side at 1:1 |
| 85 | excellent, not noticeable in-place |
| 90 | "visually lossless" in a flicker test at 1:1 |
| 100 | lossless |

**Butteraugli**:
- The page says it is "reliable in the domain of barely noticeable differences" and tuned for JPEG quality of about 90-95.
- It splits scores into great / acceptable / not acceptable with two reference values. The numeric thresholds are not stated on that page. UNCONFIRMED. ([repo](https://github.com/google/butteraugli))

**Correlation with human ratings** on CID22, full set ([CID22 page](https://cloudinary.com/labs/cid22), [search summary of the same table](https://cloudinary.com/labs/cid22)):

| Metric | SRCC | PCC | KRCC |
|---|---|---|---|
| SSIMULACRA 2 | 0.882 | 0.860 | 0.693 |
| Butteraugli 2-norm | -0.846 | -0.809 | -0.658 |
| DSSIM 3.2 | -0.840 | -0.781 | -0.643 |

- The DSSIM and Butteraugli figures come from a search-result summary of the Cloudinary table, not read directly by me. Treat as probable.
- SSIMULACRA 2.1 (GitHub): SRCC 0.8805, PCC 0.8583, KRCC 0.6903 on the full set; SRCC 0.8904 on the validation set. ([repo](https://github.com/cloudinary/ssimulacra2))
- Dataset: 250 references and 21,903 distortions, with a 201/49 train/validation split; codecs are (Moz)JPEG, WebP, AVIF, JPEG XL, JPEG 2000 and HEIC; license CC BY-SA 4.0. ([CID22](https://cloudinary.com/labs/cid22))
- Scope caveat: only medium to near-lossless quality (about mozjpeg q30-q95), compression distortions only. The authors say it is possibly less relevant than older datasets for modeling the human visual system. ([search summary](https://cloudinary.com/labs/cid22))
- Bias warning: CID22 and SSIMULACRA 2 both come from Cloudinary (Sneyers). Independent check: a 2025 HDR study (AIC-3 methodology) also ranks SSIMULACRA 2 above Butteraugli. ([arXiv 2506.12505](https://arxiv.org/pdf/2506.12505))
- Speed: none of the three repos' pages give speed figures. DSSIM claims multi-core use only. UNCONFIRMED for relative speed.

## 3. High-DPI "compressive images"

**Origin**:
- Daan Jobsis's article "Retina Revolution". Its date is not given on the page that cites it.
- Filament Group (Scott Jehl) wrote it up on 2012-07-11 (11/07/2012 on the page; that is day/month/year, so UNCONFIRMED against US order) and coined "compressive images". ([Filament](https://filamentgroup.com/lab/compressive-images))
- Their example: 400x300 at JPEG q90 is 95 kb; 1024x768 at q0 shown at 400x300 is 44 kb. They note drawbacks and that the technique cannot do art direction. The page does not discuss memory or decode cost and does not mention srcset.

**Later data**:
- web.dev (Boris Smus): a quality-30 2x JPEG (76 kB) was preferred over a quality-90 1x JPEG (80 kB). Another tile was 2 kB smaller at 2x. WebP looked cleaner at low quality, but quality under 30 on gradients bands at any scale. It lists decode cost and downscale cost as open questions and does not give a date. ([web.dev](https://web.dev/articles/easy-high-dpi-images.md.txt))
- Jake Archibald (2021): a 400px red panda is 14.9 kB as WebP at 1x. At 2x it is 45.2 kB at the same quality and 21.2 kB at WebP quality 44, judged with Squoosh zoomed out to display size. 1x needed about quality 80. Savings shrink beyond 2x. ([post](https://jakearchibald.com/2021/serving-sharp-images-to-high-density-screens/))
  - He uses `w` descriptors with `sizes` for the 2x images, a media query on `<source>` to split by DPR, and a single `src` for 1x. He does not compare `x` against `w` descriptors.
  - Stats he cites: 80% of GOV.UK users in April 2021 had DPR >= 1.5.
  - He states decoding cost but does not quantify it.
- imgix (hand-tuned example, not a built-in default): dpr1 q75 17.26 kB (q55 tuned: 12.21 kB), dpr2 q25 15.54 kB, dpr3 q15 22.88 kB. It says high-DPR images "can withstand more compression". ([imgix](https://imgix.com/blog/dpr-quality))

**Memory caveat**:
- Kornel Lesinski (2013, W3C public-respimg) feared that memory-constrained user agents would struggle to decompress 4x the pixels even if the file is smaller. ([post](https://lists.w3.org/Archives/Public/public-respimg/2013Sep/0016.html))
- An unsourced "6.5x memory" figure appeared in a search summary; I could not trace it. UNCONFIRMED.

**Modern codecs**:
- I found no direct AVIF/WebP/JXL experiment of 2x-low-quality against 1x-high-quality with a perceptual metric. UNCONFIRMED. Two blog posts claiming such savings had no methodology and I do not cite them.
- Archibald's article uses AVIF/WebP/JPEG only for the format ladder, and AVIF at the 1598px "lazy" size was 37.5 kB against 57.5 kB WebP and 99.2 kB JPEG.

**Cloudinary dpr and q_auto**: no documented interaction (see section 1).

**x or w descriptors**: none of the sources fetched recommends one over the other beyond Archibald's `w` + `sizes` usage. Any recommendation needs the HTML spec, which I did not fetch. UNCONFIRMED.

## 4. Focus regions and faces as image metadata

**IPTC Image Region** (Photo Metadata Standard, 2019.1 and later, checked against 2025.1) ([spec](http://iptc.org/std/photometadata/specification/IPTC-PhotoMetadata-2025.1.html)):
- Property `Iptc4xmpExt:ImageRegion`, a bag of structures. It came in with IPTC Extension schema 1.5 / standard 2019.1. ExifTool name `XMP-iptcExt:ImageRegion`.
- `regionBoundary` (mandatory) has `rbShape` = rectangle | circle | polygon, and `rbUnit` = `pixel` | `relative` (0-1).
- Rectangle is X/Y/width/height. Circle is X/Y/radius along x. Polygon is a vertex list, and a polygon may be one point or a line.
- The origin is the top-left of the stored pixels; orientation tags are not applied. Pixel units must be rescaled if the image is rescaled.
- `rCtype` (content type) and `rRole` (role) are repeatable and are recommended to come from the IPTC Region Content Type and Image Region Role NewsCodes. I could not read the role vocabulary, so whether "cropping" and "subject area" are literal identifiers is UNCONFIRMED.

**MWG Regions** (Metadata Working Group, "Guidelines for Handling Image Metadata" v2.0, 2010) ([exiv2](https://exiv2.org/tags-xmp-mwg-rs.html), [ExifTool](https://exiftool.sourceforge.net/TagNames/MWG.html)):
- Namespace `http://www.metadataworkinggroup.com/schemas/regions/`, prefix `mwg-rs`.
- Region Type is Face, Pet, Focus or BarCode. The Area uses normalized x/y/w/h (center-based, I recall; not verified). `AppliedToDimensions` records the image size the regions were made for.
- Lightroom writes mwg-rs face regions. A 2015 forum thread says it did not read regions written by other tools. ([Adobe thread](https://community.adobe.com/t5/lightroom-classic-discussions/face-image-region-data-written-according-to-xmp-mwg-rs-but-not-read-by-lr/m-p/7144734/highlight/true)) Current behaviour is UNCONFIRMED.
- Lightroom does not recompute face regions when the image is cropped or rotated on export (forum report, not primary). ([thread](https://community.adobe.com/t5/lightroom-classic-ideas/p-recalculate-face-region-s-when-source-image-has-been-cropped-or-rotated-mirrored-web-module/idi-p/15174878)) A tool that crops must rewrite or drop the regions.
- Apple HEIC files have been seen carrying mwg-rs face regions with Apple-specific extension fields (KDE bug report, third-party, not Apple documentation). ([KDE bug](https://bugs.kde.org/show_bug.cgi?id=497470))
- Google/Android: Google's documented XMP namespaces for motion photos (Camera, Container) do not cover face regions. ([Android](https://developer.android.com/media/platform/motion-photo-format)) I found nothing on Google or Apple native region formats. UNCONFIRMED.

**Which containers carry XMP**:
- JPEG: XMP is embedded per the XMP spec. ([Wikipedia XMP](https://en.wikipedia.org/wiki/Extensible_Metadata_Platform))
- WebP: a dedicated XMP chunk, at most one of each metadata type. ([fast.io](https://fast.io/resources/extract-metadata-from-webp-avif-modern-image-formats/))
- AVIF and HEIF: XMP is supported (ISOBMFF). ([fast.io](https://fast.io/resources/extract-metadata-from-webp-avif-modern-image-formats/))
- JPEG XL: its container can hold Exif, XMP and JUMBF, stored Brotli-compressed or plain. ([Wikipedia FUIF/JXL result](https://www.wikipedia.org/wiki/FUIF)) Adobe apps were reported writing JXL metadata to sidecars. ([Adobe](https://community.adobe.com/questions-675/jpeg-xl-metadata-still-written-into-an-xmp-not-file-directly-978295))
- The box names (`xml ` for ISOBMFF, `brob` for compressed JXL boxes) are from my own knowledge and were not confirmed in a fetched source.

**Do image tools use stored regions for smart crop?**
- Azure, Akamai, Adobe and imgix smart crop run their own detection, and their pages do not say they read regions stored in the file. ([Azure](https://learn.microsoft.com/azure/ai-services/computer-vision/concept-generating-thumbnails), [Akamai](https://techdocs.akamai.com/ivm/docs/smart-crop))
- For imgproxy, Thumbor, libvips and Cloudinary I found no evidence they read XMP, IPTC or MWG regions. UNCONFIRMED. I did not search each tool's docs individually.
