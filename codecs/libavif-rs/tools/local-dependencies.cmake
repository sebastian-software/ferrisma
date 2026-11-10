# Optional dependencies built inside this checkout; never modify vendored source.
get_filename_component(LIBAVIF_RS_ROOT "${CMAKE_CURRENT_LIST_DIR}/.." ABSOLUTE)
if(NOT LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT)
    set(LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT "${LIBAVIF_RS_ROOT}")
endif()
if(LIBAVIF_RS_AOM_IN_RUST OR LIBAVIF_RS_AOM_REFERENCE_BUILD)
    if(TARGET aom OR (LIBAVIF_RS_AOM_IN_RUST AND (NOT LIBAVIF_RS_ARCHIVE OR LIBAVIF_RS_AOM_REFERENCE_BUILD)))
        message(FATAL_ERROR "Select exactly one explicit Rust or original C AOM implementation with no preexisting aom target")
    endif()
    # Both explicit implementations use the pinned headers.
    add_library(aom INTERFACE IMPORTED GLOBAL)
    target_include_directories(aom INTERFACE "${LIBAVIF_RS_ROOT}/upstream/aom")
    if(LIBAVIF_RS_AOM_REFERENCE_BUILD)
        if(LIBAVIF_RS_ARCHIVE OR NOT EXISTS "${LIBAVIF_RS_AOM_REFERENCE_BUILD}/libaom.a")
            message(FATAL_ERROR "The original C AOM reference requires its complete archive and no Rust root implementation")
        endif()
        target_link_libraries(aom INTERFACE "${LIBAVIF_RS_AOM_REFERENCE_BUILD}/libaom.a")
    endif()
endif()
if(EXISTS "${LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT}/build/deps/libgav1-build/libgav1.a" AND NOT TARGET libgav1::libgav1)
    add_library(libgav1::libgav1 STATIC IMPORTED GLOBAL)
    set_target_properties(libgav1::libgav1 PROPERTIES
        IMPORTED_LOCATION "${LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT}/build/deps/libgav1-build/libgav1.a"
        INTERFACE_INCLUDE_DIRECTORIES "${LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT}/build/deps/libgav1/src")
endif()
if(EXISTS "${LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT}/build/deps/libyuv-build/libyuv.a" AND NOT TARGET yuv::yuv)
    add_library(yuv::yuv STATIC IMPORTED GLOBAL)
    set_target_properties(yuv::yuv PROPERTIES
        IMPORTED_LOCATION "${LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT}/build/deps/libyuv-build/libyuv.a"
        INTERFACE_INCLUDE_DIRECTORIES "${LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT}/build/deps/libyuv/include")
    file(READ "${LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT}/build/deps/libyuv/include/libyuv/version.h" LIBAVIF_RS_YUV_VERSION)
    string(REGEX MATCH "#define LIBYUV_VERSION ([0-9]+)" _ "${LIBAVIF_RS_YUV_VERSION}")
    set(LIBYUV_VERSION "${CMAKE_MATCH_1}")
endif()
if(EXISTS "${LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT}/build/deps/avm-build/libavm.dylib" AND NOT TARGET avm)
    add_library(avm SHARED IMPORTED GLOBAL)
    set_target_properties(avm PROPERTIES
        IMPORTED_LOCATION "${LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT}/build/deps/avm-build/libavm.dylib"
        INTERFACE_INCLUDE_DIRECTORIES "${LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT}/build/deps/avm;${LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT}/build/deps/avm-build")
endif()
if(LIBAVIF_RS_ARCHIVE)
    include("${CMAKE_CURRENT_LIST_DIR}/rust-library.cmake")
endif()
