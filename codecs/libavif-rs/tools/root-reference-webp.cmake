# Keep upstream's scalar DSP selection while retaining the x86-64 floating-point
# calling convention. WebP's SIMD=OFF appends -mno-sse2 even for scalar functions.
function(libavif_rs_sharpyuv_scalar_abi)
    if(CMAKE_SYSTEM_NAME STREQUAL "Linux" AND CMAKE_SYSTEM_PROCESSOR MATCHES "^(x86_64|AMD64)$"
       AND CMAKE_C_COMPILER_ID MATCHES "^(Clang|GNU)$")
        if(WEBP_ENABLE_SIMD)
            message(FATAL_ERROR "The root reference requires scalar SharpYUV DSP")
        endif()
        target_compile_options(sharpyuv PRIVATE -msse2)
    endif()
endfunction()
cmake_language(DEFER CALL libavif_rs_sharpyuv_scalar_abi)
