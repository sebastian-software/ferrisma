# Injected after the unmodified WebP project creates its original targets.
function(libavif_rs_replace_sharpyuv)
    if(NOT EXISTS "${SHARPYUV_RS_ARCHIVE}" OR BUILD_SHARED_LIBS)
        message(FATAL_ERROR "The SharpYUV test replacement requires a Rust static archive")
    endif()
    file(WRITE "${CMAKE_BINARY_DIR}/sharpyuv_rust_anchor.c"
         "/* Linkage anchor only; all SharpYUV implementation is in Rust. */\n")
    set_property(TARGET sharpyuv PROPERTY SOURCES "${CMAKE_BINARY_DIR}/sharpyuv_rust_anchor.c")
    # Match the original target's plain signature (its Linux math dependency
    # already establishes that signature before this deferred replacement).
    target_link_libraries(sharpyuv "${SHARPYUV_RS_ARCHIVE}" ${CMAKE_DL_LIBS})
    if(APPLE)
        target_link_libraries(sharpyuv resolv iconv)
    else()
        target_link_libraries(sharpyuv m pthread)
    endif()
endfunction()
cmake_language(DEFER CALL libavif_rs_replace_sharpyuv)
