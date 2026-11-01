# Injected through CMAKE_PROJECT_libavif_INCLUDE. The vendored CMake remains
# unchanged; replace its library implementation only after it defines targets.
function(libavif_rs_use_rust_library)
    if(NOT EXISTS "${LIBAVIF_RS_ARCHIVE}")
        message(FATAL_ERROR "Build the Rust archive first: ${LIBAVIF_RS_ARCHIVE}")
    endif()
    if(BUILD_SHARED_LIBS)
        message(FATAL_ERROR "This parity harness currently requires a static build")
    endif()
    # CMake needs an object to retain the original library target and its usage
    # requirements. This empty file defines no implementation or fallback.
    file(WRITE "${CMAKE_BINARY_DIR}/rust_archive_anchor.c" "/* Rust archive linkage anchor; no C implementation. */\n")
    set_property(TARGET avif_obj PROPERTY SOURCES "${CMAKE_BINARY_DIR}/rust_archive_anchor.c")
    target_link_libraries(avif PRIVATE "${LIBAVIF_RS_ARCHIVE}")
    message(STATUS "libavif-rs: all avif_obj C implementations replaced by ${LIBAVIF_RS_ARCHIVE}")
endfunction()
cmake_language(DEFER CALL libavif_rs_use_rust_library)
