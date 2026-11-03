// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#[path = "../src/brotli/running.rs"]
pub mod running;
#[path = "../src/brotli/status.rs"]
pub mod status;
use running::*;
use status::*;

fn declarations() -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    writeln!(
        out,
        "version {} {} {} {} {} {} {}",
        brotli_decoder_version(),
        BROTLI_VERSION_MAJOR,
        BROTLI_VERSION_MINOR,
        BROTLI_VERSION_PATCH,
        BROTLI_ABI_CURRENT,
        BROTLI_ABI_REVISION,
        BROTLI_ABI_AGE
    )
    .unwrap();
    writeln!(
        out,
        "layout BrotliDecoderResult {} {} 0",
        std::mem::size_of::<BrotliDecoderResult>(),
        std::mem::align_of::<BrotliDecoderResult>()
    )
    .unwrap();
    writeln!(
        out,
        "layout BrotliDecoderErrorCode {} {} 1",
        std::mem::size_of::<BrotliDecoderErrorCode>(),
        std::mem::align_of::<BrotliDecoderErrorCode>()
    )
    .unwrap();
    writeln!(
        out,
        "result BROTLI_DECODER_RESULT_ERROR {}",
        BROTLI_DECODER_RESULT_ERROR
    )
    .unwrap();
    writeln!(
        out,
        "result BROTLI_DECODER_RESULT_SUCCESS {}",
        BROTLI_DECODER_RESULT_SUCCESS
    )
    .unwrap();
    writeln!(
        out,
        "result BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT {}",
        BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT
    )
    .unwrap();
    writeln!(
        out,
        "result BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT {}",
        BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_NO_ERROR {} {}",
        BROTLI_DECODER_NO_ERROR,
        brotli_decoder_error_string(BROTLI_DECODER_NO_ERROR)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_SUCCESS {} {}",
        BROTLI_DECODER_SUCCESS,
        brotli_decoder_error_string(BROTLI_DECODER_SUCCESS)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_NEEDS_MORE_INPUT {} {}",
        BROTLI_DECODER_NEEDS_MORE_INPUT,
        brotli_decoder_error_string(BROTLI_DECODER_NEEDS_MORE_INPUT)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_NEEDS_MORE_OUTPUT {} {}",
        BROTLI_DECODER_NEEDS_MORE_OUTPUT,
        brotli_decoder_error_string(BROTLI_DECODER_NEEDS_MORE_OUTPUT)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_NIBBLE {} {}",
        BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_NIBBLE,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_NIBBLE)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_FORMAT_RESERVED {} {}",
        BROTLI_DECODER_ERROR_FORMAT_RESERVED,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_FORMAT_RESERVED)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_META_NIBBLE {} {}",
        BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_META_NIBBLE,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_META_NIBBLE)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_ALPHABET {} {}",
        BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_ALPHABET,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_ALPHABET)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_SAME {} {}",
        BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_SAME,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_SAME)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_FORMAT_CL_SPACE {} {}",
        BROTLI_DECODER_ERROR_FORMAT_CL_SPACE,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_FORMAT_CL_SPACE)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_FORMAT_HUFFMAN_SPACE {} {}",
        BROTLI_DECODER_ERROR_FORMAT_HUFFMAN_SPACE,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_FORMAT_HUFFMAN_SPACE)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_FORMAT_CONTEXT_MAP_REPEAT {} {}",
        BROTLI_DECODER_ERROR_FORMAT_CONTEXT_MAP_REPEAT,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_FORMAT_CONTEXT_MAP_REPEAT)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_1 {} {}",
        BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_1,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_1)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_2 {} {}",
        BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_2,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_2)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_FORMAT_TRANSFORM {} {}",
        BROTLI_DECODER_ERROR_FORMAT_TRANSFORM,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_FORMAT_TRANSFORM)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_FORMAT_DICTIONARY {} {}",
        BROTLI_DECODER_ERROR_FORMAT_DICTIONARY,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_FORMAT_DICTIONARY)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS {} {}",
        BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_FORMAT_PADDING_1 {} {}",
        BROTLI_DECODER_ERROR_FORMAT_PADDING_1,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_FORMAT_PADDING_1)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_FORMAT_PADDING_2 {} {}",
        BROTLI_DECODER_ERROR_FORMAT_PADDING_2,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_FORMAT_PADDING_2)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_FORMAT_DISTANCE {} {}",
        BROTLI_DECODER_ERROR_FORMAT_DISTANCE,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_FORMAT_DISTANCE)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_FORMAT_BLOCK_SWITCH {} {}",
        BROTLI_DECODER_ERROR_FORMAT_BLOCK_SWITCH,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_FORMAT_BLOCK_SWITCH)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_COMPOUND_DICTIONARY {} {}",
        BROTLI_DECODER_ERROR_COMPOUND_DICTIONARY,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_COMPOUND_DICTIONARY)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_DICTIONARY_NOT_SET {} {}",
        BROTLI_DECODER_ERROR_DICTIONARY_NOT_SET,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_DICTIONARY_NOT_SET)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_INVALID_ARGUMENTS {} {}",
        BROTLI_DECODER_ERROR_INVALID_ARGUMENTS,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_INVALID_ARGUMENTS)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MODES {} {}",
        BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MODES,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MODES)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_ALLOC_TREE_GROUPS {} {}",
        BROTLI_DECODER_ERROR_ALLOC_TREE_GROUPS,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_ALLOC_TREE_GROUPS)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MAP {} {}",
        BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MAP,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MAP)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_1 {} {}",
        BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_1,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_1)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_2 {} {}",
        BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_2,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_2)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_ALLOC_BLOCK_TYPE_TREES {} {}",
        BROTLI_DECODER_ERROR_ALLOC_BLOCK_TYPE_TREES,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_ALLOC_BLOCK_TYPE_TREES)
    )
    .unwrap();
    writeln!(
        out,
        "code BROTLI_DECODER_ERROR_UNREACHABLE {} {}",
        BROTLI_DECODER_ERROR_UNREACHABLE,
        brotli_decoder_error_string(BROTLI_DECODER_ERROR_UNREACHABLE)
    )
    .unwrap();
    writeln!(
        out,
        "alias BROTLI_LAST_ERROR_CODE {}",
        BROTLI_LAST_ERROR_CODE
    )
    .unwrap();
    writeln!(
        out,
        "layout BrotliRunningState {} {} 0",
        std::mem::size_of::<BrotliRunningState>(),
        std::mem::align_of::<BrotliRunningState>()
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_UNINITED {}",
        BROTLI_STATE_UNINITED
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_LARGE_WINDOW_BITS {}",
        BROTLI_STATE_LARGE_WINDOW_BITS
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_INITIALIZE {}",
        BROTLI_STATE_INITIALIZE
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_METABLOCK_BEGIN {}",
        BROTLI_STATE_METABLOCK_BEGIN
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_METABLOCK_HEADER {}",
        BROTLI_STATE_METABLOCK_HEADER
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_METABLOCK_HEADER_2 {}",
        BROTLI_STATE_METABLOCK_HEADER_2
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_CONTEXT_MODES {}",
        BROTLI_STATE_CONTEXT_MODES
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_COMMAND_BEGIN {}",
        BROTLI_STATE_COMMAND_BEGIN
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_COMMAND_INNER {}",
        BROTLI_STATE_COMMAND_INNER
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_COMMAND_POST_DECODE_LITERALS {}",
        BROTLI_STATE_COMMAND_POST_DECODE_LITERALS
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_COMMAND_POST_WRAP_COPY {}",
        BROTLI_STATE_COMMAND_POST_WRAP_COPY
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_UNCOMPRESSED {}",
        BROTLI_STATE_UNCOMPRESSED
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_METADATA {}",
        BROTLI_STATE_METADATA
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_COMMAND_INNER_WRITE {}",
        BROTLI_STATE_COMMAND_INNER_WRITE
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_METABLOCK_DONE {}",
        BROTLI_STATE_METABLOCK_DONE
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_COMMAND_POST_WRITE_1 {}",
        BROTLI_STATE_COMMAND_POST_WRITE_1
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_COMMAND_POST_WRITE_2 {}",
        BROTLI_STATE_COMMAND_POST_WRITE_2
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_HEADER {}",
        BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_HEADER
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_HUFFMAN_CODE_0 {}",
        BROTLI_STATE_HUFFMAN_CODE_0
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_HUFFMAN_CODE_1 {}",
        BROTLI_STATE_HUFFMAN_CODE_1
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_HUFFMAN_CODE_2 {}",
        BROTLI_STATE_HUFFMAN_CODE_2
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_HUFFMAN_CODE_3 {}",
        BROTLI_STATE_HUFFMAN_CODE_3
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_CONTEXT_MAP_1 {}",
        BROTLI_STATE_CONTEXT_MAP_1
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_CONTEXT_MAP_2 {}",
        BROTLI_STATE_CONTEXT_MAP_2
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_TREE_GROUP {}",
        BROTLI_STATE_TREE_GROUP
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_BODY {}",
        BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_BODY
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningState BROTLI_STATE_DONE {}",
        BROTLI_STATE_DONE
    )
    .unwrap();
    writeln!(
        out,
        "layout BrotliRunningMetablockHeaderState {} {} 0",
        std::mem::size_of::<BrotliRunningMetablockHeaderState>(),
        std::mem::align_of::<BrotliRunningMetablockHeaderState>()
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningMetablockHeaderState BROTLI_STATE_METABLOCK_HEADER_NONE {}",
        BROTLI_STATE_METABLOCK_HEADER_NONE
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningMetablockHeaderState BROTLI_STATE_METABLOCK_HEADER_EMPTY {}",
        BROTLI_STATE_METABLOCK_HEADER_EMPTY
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningMetablockHeaderState BROTLI_STATE_METABLOCK_HEADER_NIBBLES {}",
        BROTLI_STATE_METABLOCK_HEADER_NIBBLES
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningMetablockHeaderState BROTLI_STATE_METABLOCK_HEADER_SIZE {}",
        BROTLI_STATE_METABLOCK_HEADER_SIZE
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningMetablockHeaderState BROTLI_STATE_METABLOCK_HEADER_UNCOMPRESSED {}",
        BROTLI_STATE_METABLOCK_HEADER_UNCOMPRESSED
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningMetablockHeaderState BROTLI_STATE_METABLOCK_HEADER_RESERVED {}",
        BROTLI_STATE_METABLOCK_HEADER_RESERVED
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningMetablockHeaderState BROTLI_STATE_METABLOCK_HEADER_BYTES {}",
        BROTLI_STATE_METABLOCK_HEADER_BYTES
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningMetablockHeaderState BROTLI_STATE_METABLOCK_HEADER_METADATA {}",
        BROTLI_STATE_METABLOCK_HEADER_METADATA
    )
    .unwrap();
    writeln!(
        out,
        "layout BrotliRunningUncompressedState {} {} 0",
        std::mem::size_of::<BrotliRunningUncompressedState>(),
        std::mem::align_of::<BrotliRunningUncompressedState>()
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningUncompressedState BROTLI_STATE_UNCOMPRESSED_NONE {}",
        BROTLI_STATE_UNCOMPRESSED_NONE
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningUncompressedState BROTLI_STATE_UNCOMPRESSED_WRITE {}",
        BROTLI_STATE_UNCOMPRESSED_WRITE
    )
    .unwrap();
    writeln!(
        out,
        "layout BrotliRunningTreeGroupState {} {} 0",
        std::mem::size_of::<BrotliRunningTreeGroupState>(),
        std::mem::align_of::<BrotliRunningTreeGroupState>()
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningTreeGroupState BROTLI_STATE_TREE_GROUP_NONE {}",
        BROTLI_STATE_TREE_GROUP_NONE
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningTreeGroupState BROTLI_STATE_TREE_GROUP_LOOP {}",
        BROTLI_STATE_TREE_GROUP_LOOP
    )
    .unwrap();
    writeln!(
        out,
        "layout BrotliRunningContextMapState {} {} 0",
        std::mem::size_of::<BrotliRunningContextMapState>(),
        std::mem::align_of::<BrotliRunningContextMapState>()
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningContextMapState BROTLI_STATE_CONTEXT_MAP_NONE {}",
        BROTLI_STATE_CONTEXT_MAP_NONE
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningContextMapState BROTLI_STATE_CONTEXT_MAP_READ_PREFIX {}",
        BROTLI_STATE_CONTEXT_MAP_READ_PREFIX
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningContextMapState BROTLI_STATE_CONTEXT_MAP_HUFFMAN {}",
        BROTLI_STATE_CONTEXT_MAP_HUFFMAN
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningContextMapState BROTLI_STATE_CONTEXT_MAP_DECODE {}",
        BROTLI_STATE_CONTEXT_MAP_DECODE
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningContextMapState BROTLI_STATE_CONTEXT_MAP_TRANSFORM {}",
        BROTLI_STATE_CONTEXT_MAP_TRANSFORM
    )
    .unwrap();
    writeln!(
        out,
        "layout BrotliRunningHuffmanState {} {} 0",
        std::mem::size_of::<BrotliRunningHuffmanState>(),
        std::mem::align_of::<BrotliRunningHuffmanState>()
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningHuffmanState BROTLI_STATE_HUFFMAN_NONE {}",
        BROTLI_STATE_HUFFMAN_NONE
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningHuffmanState BROTLI_STATE_HUFFMAN_SIMPLE_SIZE {}",
        BROTLI_STATE_HUFFMAN_SIMPLE_SIZE
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningHuffmanState BROTLI_STATE_HUFFMAN_SIMPLE_READ {}",
        BROTLI_STATE_HUFFMAN_SIMPLE_READ
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningHuffmanState BROTLI_STATE_HUFFMAN_SIMPLE_BUILD {}",
        BROTLI_STATE_HUFFMAN_SIMPLE_BUILD
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningHuffmanState BROTLI_STATE_HUFFMAN_COMPLEX {}",
        BROTLI_STATE_HUFFMAN_COMPLEX
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningHuffmanState BROTLI_STATE_HUFFMAN_LENGTH_SYMBOLS {}",
        BROTLI_STATE_HUFFMAN_LENGTH_SYMBOLS
    )
    .unwrap();
    writeln!(
        out,
        "layout BrotliRunningDecodeUint8State {} {} 0",
        std::mem::size_of::<BrotliRunningDecodeUint8State>(),
        std::mem::align_of::<BrotliRunningDecodeUint8State>()
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningDecodeUint8State BROTLI_STATE_DECODE_UINT8_NONE {}",
        BROTLI_STATE_DECODE_UINT8_NONE
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningDecodeUint8State BROTLI_STATE_DECODE_UINT8_SHORT {}",
        BROTLI_STATE_DECODE_UINT8_SHORT
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningDecodeUint8State BROTLI_STATE_DECODE_UINT8_LONG {}",
        BROTLI_STATE_DECODE_UINT8_LONG
    )
    .unwrap();
    writeln!(
        out,
        "layout BrotliRunningReadBlockLengthState {} {} 0",
        std::mem::size_of::<BrotliRunningReadBlockLengthState>(),
        std::mem::align_of::<BrotliRunningReadBlockLengthState>()
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningReadBlockLengthState BROTLI_STATE_READ_BLOCK_LENGTH_NONE {}",
        BROTLI_STATE_READ_BLOCK_LENGTH_NONE
    )
    .unwrap();
    writeln!(
        out,
        "running BrotliRunningReadBlockLengthState BROTLI_STATE_READ_BLOCK_LENGTH_SUFFIX {}",
        BROTLI_STATE_READ_BLOCK_LENGTH_SUFFIX
    )
    .unwrap();
    out
}

#[test]
fn result_aliases_and_reserved_error_gaps_retain_original_values() {
    assert_eq!(BROTLI_DECODER_RESULT_ERROR, 0);
    assert_eq!(BROTLI_DECODER_NO_ERROR, 0);
    assert_eq!(BROTLI_DECODER_SUCCESS as u32, BROTLI_DECODER_RESULT_SUCCESS);
    assert_eq!(
        BROTLI_DECODER_NEEDS_MORE_INPUT as u32,
        BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT
    );
    assert_eq!(
        BROTLI_DECODER_NEEDS_MORE_OUTPUT as u32,
        BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT
    );
    assert_eq!(BROTLI_LAST_ERROR_CODE, -31);
    for code in [i32::MIN, -32, -29, -28, -24, -23, 4, i32::MAX] {
        assert_eq!(brotli_decoder_error_string(code), "INVALID");
    }
    assert_eq!(brotli_decoder_error_string(0), "_NO_ERROR");
    assert_eq!(brotli_decoder_error_string(-31), "_ERROR_UNREACHABLE");
}

#[test]
fn version_and_running_storage_retain_the_selected_native_declarations() {
    assert_eq!(brotli_decoder_version(), 0x01002001);
    assert_eq!(BROTLI_STATE_DONE, 26);
    let unknown: BrotliRunningState = u32::MAX;
    assert_eq!(unknown, u32::MAX);
    assert_eq!(std::mem::size_of::<BrotliRunningState>(), 4);
    assert_eq!(
        declarations()
            .lines()
            .filter(|line| line.starts_with("running "))
            .count(),
        55
    );
}

#[cfg(feature = "cpp-oracle")]
mod parity {
    use super::*;
    use std::io::Write as _;
    use std::process::{Command, Stdio};

    fn oracle(mode: &str) -> std::process::Child {
        Command::new(
            std::env::var_os("BROTLI_STATUS_ORACLE")
                .expect("build the pinned C status oracle and set BROTLI_STATUS_ORACLE"),
        )
        .arg(mode)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
    }

    #[test]
    fn all_declared_results_errors_versions_and_running_values_match_original() {
        let mut child = oracle("declarations");
        drop(child.stdin.take());
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success());
        assert_eq!(String::from_utf8(output.stdout).unwrap(), declarations());
        eprintln!(
            "Brotli status declaration observations: {}",
            declarations().lines().count()
        );
    }

    #[test]
    fn all_declared_reserved_and_dense_unknown_i32_boundaries_match_original_c_enum() {
        let mut values: Vec<i32> = (-131072..=131072).collect();
        values.extend([i32::MIN, i32::MIN + 1, i32::MAX - 1, i32::MAX]);
        for bit in 0..31 {
            let value = 1i32 << bit;
            values.extend([value - 1, value, value + 1, -value - 1, -value, -value + 1]);
        }
        let mut seed = 78191u32;
        for _ in 0..32768 {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            values.push(seed as i32);
        }
        values.sort_unstable();
        values.dedup();
        let mut request = String::new();
        {
            use std::fmt::Write as _;
            for value in &values {
                writeln!(request, "{value}").unwrap();
            }
        }
        let mut child = oracle("strings");
        let mut stdin = child.stdin.take().unwrap();
        let writer = std::thread::spawn(move || stdin.write_all(request.as_bytes()).unwrap());
        let output = child.wait_with_output().unwrap();
        writer.join().unwrap();
        assert!(output.status.success());
        let output = String::from_utf8(output.stdout).unwrap();
        let lines: Vec<_> = output.lines().collect();
        assert_eq!(lines.len(), values.len());
        for (value, actual) in values.iter().zip(lines) {
            assert_eq!(
                actual,
                format!("{value} {}", brotli_decoder_error_string(*value))
            );
        }
        eprintln!("Brotli error-string source requests: {}", values.len());
    }
}
