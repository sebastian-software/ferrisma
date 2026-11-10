# Color evidence and conversion planning: first contract

**Scope: experimental inspection contract.** This document records the
[ADR-0015](../adr/0015-typed-output-color-signals.md) design boundary and its
early execution assumptions. Later work added supported color conversion and
codec paths; see [current status](../status.md) for their present scope and
[ADR-0016](../adr/0016-working-color-space-and-transform-boundaries.md) and
[ADR-0018](../adr/0018-optional-codec-and-cms-dependencies.md) for the related
decisions. The historical statements below describe this inspection experiment,
not a complete inventory of today's executor capabilities.

## Input contract

The decoder reports the profile and channel layout of the **samples delivered to Ferrixel**. A JPEG XL decoder that has already performed a CMS conversion must report its selected *output* profile, not reuse an original-file ICC tag. This same rule will apply to AVIF and future codecs.

The codec-independent graph stores an `ImageSemantics` description. Cropping preserves it without pixel conversion. The inspection stage analyzes a borrowed view of raw ICC bytes and/or CICP code points; it does not allocate a pixel buffer, modify decoded samples or create a GPU resource.

## Classification

| Emitted signal | Result for linear-sRGB reference | Why |
| --- | --- | --- |
| CICP `[1,13,0,1]` | Ready | Exact full-range RGB/sRGB transfer |
| CICP `[12,13,0,1]` | Requires gamut conversion | P3 primaries are not Rec.709 |
| CICP `[9,16,0,1]` | Requires HDR management | BT.2100 PQ needs luminance-domain policy |
| CICP `[9,18,9,0]` | Requires HDR management | BT.2100 HLG also needs reference/scene policy |
| ICC alone | Requires ICC color management | Profile header does not supply a usable transform |
| ICC + CICP | Requires profile precedence | No universal rule that two profiles are equivalent |
| Missing / unknown CICP | Missing / unsupported | No implicit sRGB assumption |
| Malformed ICC or CICP | Malformed metadata | Fail closed |

The CICP enum retains the actual code points. Unsupported combinations are never overwritten. An ICC *header* inspection validates only minimum container boundaries (`acsp`, declared profile size and tag-table extent); it does not verify tag content or authorize a CMM to execute arbitrary data.

## Future execution contract (not implemented)

```text
decoder output (pixel layout + rendered profile)
        ↓
color evidence classification
        ↓
transform requirement / output intent
        ↓
decide working space and linear-light/alpha precision
        ↓
future codec-native / CPU / GPU color kernel
        ↓
downstream region resampler and output sink
```

Future graph stages will need source/destination profile IDs, rendering intent, adaptation and black-point policy, transfer/linear-light domain, HDR reference-white handling, alpha association, clipping/gamut mapping and an explicit backend implementation. Those stages should accept borrowed regions and compose with resize without materializing whole intermediate frames.

## Deliberate limitations

There is **no** ICC parsing beyond a header and boundary inspection, no CMM, P3/BT.2020 transform, PQ/HLG conversion, tone mapping, or auto-assigned defaults. The only accepted linear-light kernel remains the strict RGB8 / straight-RGBA8 sRGB reference. No broader promises about the total memory of external codecs or profiles are made.
