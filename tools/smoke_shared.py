#!/usr/bin/env python3
"""Exercise the Rust dylib's C ABI, initialization, decode, encode, and cleanup."""
import ctypes as C
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
library = C.CDLL(str(ROOT / "target/release/libavif.dylib"))


class RWData(C.Structure):
    _fields_ = [("data", C.c_void_p), ("size", C.c_size_t)]


def function(name, result, *arguments):
    value = getattr(library, name)
    value.restype, value.argtypes = result, list(arguments)
    return value


version = function("avifVersion", C.c_char_p)
create_decoder = function("avifDecoderCreate", C.c_void_p)
create_image = function("avifImageCreateEmpty", C.c_void_p)
create_encoder = function("avifEncoderCreate", C.c_void_p)
decode = function("avifDecoderReadMemory", C.c_uint, C.c_void_p, C.c_void_p, C.c_void_p, C.c_size_t)
encode = function("avifEncoderWrite", C.c_uint, C.c_void_p, C.c_void_p, C.POINTER(RWData))
width = function("avifImagePlaneWidth", C.c_uint32, C.c_void_p, C.c_int)
height = function("avifImagePlaneHeight", C.c_uint32, C.c_void_p, C.c_int)
destroy_decoder = function("avifDecoderDestroy", None, C.c_void_p)
destroy_image = function("avifImageDestroy", None, C.c_void_p)
destroy_encoder = function("avifEncoderDestroy", None, C.c_void_p)
free_data = function("avifRWDataFree", None, C.POINTER(RWData))
decoder, image, encoder = create_decoder(), create_image(), create_encoder()
output = RWData()
try:
    if not all([decoder, image, encoder]):
        raise SystemExit("Shared-library object allocation failed")
    data = (ROOT / "upstream/libavif/tests/data/draw_points_idat.avif").read_bytes()
    buffer = C.create_string_buffer(data)
    if decode(decoder, image, buffer, len(data)) != 0:
        raise SystemExit("Shared-library decoding failed")
    dimensions = (width(image, 0), height(image, 0))
    if not all(dimensions) or encode(encoder, image, C.byref(output)) != 0 or not output.size:
        raise SystemExit("Shared-library encoding failed")
    print(f"Shared C ABI: libavif {version().decode()}, decoded {dimensions}, encoded {output.size} bytes")
finally:
    if encoder:
        destroy_encoder(encoder)
    if decoder:
        destroy_decoder(decoder)
    if image:
        destroy_image(image)
    free_data(C.byref(output))
