// Handwritten consumer seam included after the generated read translation.
// The generated read.rs bytes and their provenance hashes remain unchanged.

/// Return each parsed primary color tile's coded AV1 sequence header.
/// This only prepares compressed samples; it does not decode image pixels.
///
/// # Safety
/// `decoder` must be a live decoder after a successful `avifDecoderParse`.
/// Its input reader must remain valid for the whole call. This helper mutates
/// retained sample storage and must not race another decoder operation.
pub unsafe fn avif_decoder_color_sequence_headers(
    decoder: *mut avifDecoder,
) -> Result<Vec<avifSequenceHeader>, avifResult> {
    if decoder.is_null() {
        return Err(AVIF_RESULT_INVALID_ARGUMENT);
    }
    // SAFETY: Successful Parse allocates decoder data and its tile array.
    // Keep raw pointers here because PrepareSample mutates decoder-owned data.
    let data = unsafe { (*decoder).data };
    if data.is_null() {
        return Err(AVIF_RESULT_NO_CONTENT);
    }
    let tile_count = unsafe { (*data).tiles.count };
    let tiles = unsafe { (*data).tiles.tile };
    if tile_count == 0 || tiles.is_null() {
        return Err(AVIF_RESULT_NO_CONTENT);
    }
    let mut headers = Vec::new();
    headers
        .try_reserve(tile_count as usize)
        .map_err(|_| AVIF_RESULT_OUT_OF_MEMORY)?;
    for index in 0..tile_count as usize {
        // SAFETY: index is within the parser-owned tile allocation.
        let tile = unsafe { tiles.add(index) };
        // SAFETY: Parse initializes each tile's input.
        let input = unsafe { (*tile).input };
        if input.is_null() {
            return Err(AVIF_RESULT_NO_CONTENT);
        }
        if unsafe { (*input).itemCategory } != AVIF_ITEM_COLOR {
            continue;
        }
        if unsafe { (*tile).codecType } != AVIF_CODEC_TYPE_AV1
            || unsafe { (*input).samples.count } == 0
            || unsafe { (*input).samples.sample.is_null() }
        {
            return Err(AVIF_RESULT_NOT_IMPLEMENTED);
        }
        // SAFETY: The color tile has at least one parser-owned sample.
        let sample = unsafe { (*input).samples.sample };
        // SAFETY: Parsed decoder and sample are live and exclusively borrowed.
        let result = unsafe { avifDecoderPrepareSample(decoder, sample, 0) };
        if result != AVIF_RESULT_OK {
            return Err(result);
        }
        let mut header: avifSequenceHeader = unsafe { std::mem::zeroed() };
        // SAFETY: PrepareSample made sample.data readable for its size.
        if unsafe { avifSequenceHeaderParse(&raw mut header, &raw const (*sample).data, (*tile).codecType) } == 0 {
            return Err(AVIF_RESULT_BMFF_PARSE_FAILED);
        }
        headers.push(header);
    }
    if headers.is_empty() {
        return Err(AVIF_RESULT_NO_CONTENT);
    }
    Ok(headers)
}
