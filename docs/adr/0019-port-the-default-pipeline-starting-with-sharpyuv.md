# Port the default pipeline starting with SharpYUV

Start with the standalone portable SharpYUV library from libwebp 1.6.0, at
commit `4fa21912338357f89e4fd51cf2368325b59e9bd9` (SharpYUV API 0.4.2).
Use its small independent library boundary to establish source translation,
direct comparison, original-consumer integration, provenance, and two-platform
evidence together. The public release has no dedicated SharpYUV unit suite,
so explicitly add differential coverage alongside available original tests.

The first program phase proceeds through SharpYUV, dav1d, and libaom, then
verifies their integration into libavif's default path as Rust dependencies.
Keep each library a separate crate in the shared workspace. The portable
reference contract and the evidence requirements of ADRs 0010 to 0018 apply
before claiming a milestone.

Plan full libyuv, libgav1, and SVT-AV1 as later milestones. AVM, JPEG-dependent
features, and ML dependency closures require separate detailed scope before
implementation. This prioritizes a verified default pipeline without abandoning
the long-term complete-library goal or treating disabled features as completed.
