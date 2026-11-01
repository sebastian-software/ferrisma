# Optional dependencies built inside this checkout; never modify vendored source.
get_filename_component(LIBAVIF_RS_ROOT "${CMAKE_CURRENT_LIST_DIR}/.." ABSOLUTE)
if(EXISTS "${LIBAVIF_RS_ROOT}/build/deps/libgav1-build/libgav1.a" AND NOT TARGET libgav1::libgav1)
    add_library(libgav1::libgav1 STATIC IMPORTED GLOBAL)
    set_target_properties(libgav1::libgav1 PROPERTIES
        IMPORTED_LOCATION "${LIBAVIF_RS_ROOT}/build/deps/libgav1-build/libgav1.a"
        INTERFACE_INCLUDE_DIRECTORIES "${LIBAVIF_RS_ROOT}/build/deps/libgav1/src")
endif()
if(EXISTS "${LIBAVIF_RS_ROOT}/build/deps/libyuv-build/libyuv.a" AND NOT TARGET yuv::yuv)
    add_library(yuv::yuv STATIC IMPORTED GLOBAL)
    set_target_properties(yuv::yuv PROPERTIES
        IMPORTED_LOCATION "${LIBAVIF_RS_ROOT}/build/deps/libyuv-build/libyuv.a"
        INTERFACE_INCLUDE_DIRECTORIES "${LIBAVIF_RS_ROOT}/build/deps/libyuv/include")
    file(READ "${LIBAVIF_RS_ROOT}/build/deps/libyuv/include/libyuv/version.h" LIBAVIF_RS_YUV_VERSION)
    string(REGEX MATCH "#define LIBYUV_VERSION ([0-9]+)" _ "${LIBAVIF_RS_YUV_VERSION}")
    set(LIBYUV_VERSION "${CMAKE_MATCH_1}")
endif()
if(EXISTS "${LIBAVIF_RS_ROOT}/build/deps/avm-build/libavm.dylib" AND NOT TARGET avm)
    add_library(avm SHARED IMPORTED GLOBAL)
    set_target_properties(avm PROPERTIES
        IMPORTED_LOCATION "${LIBAVIF_RS_ROOT}/build/deps/avm-build/libavm.dylib"
        INTERFACE_INCLUDE_DIRECTORIES "${LIBAVIF_RS_ROOT}/build/deps/avm;${LIBAVIF_RS_ROOT}/build/deps/avm-build")
endif()
if(LIBAVIF_RS_ARCHIVE)
    include("${CMAKE_CURRENT_LIST_DIR}/rust-library.cmake")
endif()
