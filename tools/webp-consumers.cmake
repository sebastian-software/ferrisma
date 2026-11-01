# Build compatibility for the original, pinned test dependencies only.
# Upstream registers tests in its subdirectory but omits root enable_testing().
enable_testing()
function(libavif_rs_webp_test_platform)
    if(APPLE AND CMAKE_SYSTEM_PROCESSOR STREQUAL "arm64")
        # The original EncDec fuzzer wraps and calls VP8GetCPUInfo even when
        # SIMD=OFF makes it NULL on ARM. Compile only WebP's original CPU-query
        # unit with its ARM detector. DSP and SharpYUV units remain portable.
        set_property(SOURCE "${CMAKE_SOURCE_DIR}/src/dsp/cpu.c" APPEND
                     PROPERTY COMPILE_DEFINITIONS WEBP_HAVE_NEON=1)
        # CMake deduplicates repeated -Xarch_x86_64 prefixes in Abseil 20240116.0,
        # leaving -msse4.1 unguarded on ARM. Retain only this single target's
        # native ARM crypto option. Neither WebP nor its tests are modified.
        foreach(name random_internal_randen_hwaes_impl random_internal_randen_hwaes)
            get_target_property(target absl::${name} ALIASED_TARGET)
            get_target_property(options ${target} COMPILE_OPTIONS)
            list(REMOVE_ITEM options "-Xarch_x86_64" "-maes" "-msse4.1" "-Xarch_arm64")
            set_property(TARGET ${target} PROPERTY COMPILE_OPTIONS "${options}")
        endforeach()
    endif()
endfunction()
cmake_language(DEFER CALL libavif_rs_webp_test_platform)
if(SHARPYUV_RS_ARCHIVE)
    include("${CMAKE_CURRENT_LIST_DIR}/sharpyuv-library.cmake")
endif()
