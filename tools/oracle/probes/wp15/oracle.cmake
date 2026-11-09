# WP15 consumer builds (ADR 0004, docs/status/wp15.md).
#
# webpmux: upstream's examples/webpmux.c, unmodified. The reference
# configuration turns WEBP_BUILD_WEBPMUX off, so the oracle has no webpmux;
# this fragment builds it from the sources and libraries that
# examples/Makefile.am lists, against the C library (probes/wp15/webpmux) and,
# with the Rust library, against it (probes/wp15/webpmux_rs).
#
# anim_diff: upstream's examples/anim_diff.c with anim_util.c and gifdec.c,
# unmodified, built twice as WP14 builds anim_dump: without HAVE_CONFIG_H, so
# that both builds read WebP only and need no giflib (probes/wp15/anim_diff and
# anim_diff_rs).
#
# img2webp: the oracle builds it against the C library (WP14); the copy of
# upstream's target (sources, include directories, definitions) links the Rust
# library instead of libwebp and libwebpmux (probes/wp15/img2webp_rs), and
# WP13's link stand-in for SharpYuvGetVersion(), which only -version calls.
#
# With LIBWEBP_RS_STATICLIB (tools/build_oracle.py --rust-lib), the probes that
# use only the public mux API are linked against the Rust library as well:
# probes/wp15/mux_rs and anim_enc_rs. The pinned headers come first in every
# include path (PORT_STATUS.md, "Consumer builds"). The Rust builds reuse WP12's
# libwebp_rs target and rs_ copies of upstream's imageio and example libraries
# (probes/wp12/oracle.cmake) and WP13's wp13_link_shim (probes/wp13/oracle.cmake).

function(wp15_tool target output)
  cmake_parse_arguments(TOOL "NO_CONFIG" "" "SOURCES;LIBRARIES" ${ARGN})
  add_executable(${target} ${TOOL_SOURCES})
  target_include_directories(${target} PRIVATE "${UPSTREAM}/src" "${UPSTREAM}"
                             "${CMAKE_BINARY_DIR}/libwebp/src")
  if(NOT TOOL_NO_CONFIG)
    target_compile_definitions(${target} PRIVATE ${UPSTREAM_DEFINITIONS})
  endif()
  target_link_libraries(${target} PRIVATE ${TOOL_LIBRARIES})
  set_target_properties(${target} PROPERTIES
    OUTPUT_NAME ${output}
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/probes/wp15")
endfunction()

set(wp15_webpmux_sources "${UPSTREAM}/examples/webpmux.c")
set(wp15_anim_diff_sources
    "${UPSTREAM}/examples/anim_diff.c"
    "${UPSTREAM}/examples/anim_util.c"
    "${UPSTREAM}/examples/gifdec.c")

# Upstream's webpmux links exampleutil, imageioutil, libwebpmux and webp;
# anim_diff links exampleutil, imagedec, imageenc, imageioutil, webp and
# webpdemux.
wp15_tool(wp15_webpmux webpmux SOURCES ${wp15_webpmux_sources}
          LIBRARIES exampleutil imageioutil libwebpmux webp)
wp15_tool(wp15_anim_diff anim_diff NO_CONFIG SOURCES ${wp15_anim_diff_sources}
          LIBRARIES exampleutil imagedec imageenc imageioutil webpdemux webp)
list(APPEND ALL_PROBES wp15_webpmux wp15_anim_diff)

if(LIBWEBP_RS_STATICLIB)
  if(NOT TARGET libwebp_rs OR NOT TARGET rs_imagedec OR NOT TARGET wp13_link_shim)
    message(FATAL_ERROR "probes/wp15/oracle.cmake needs probes/wp12 and probes/wp13 first")
  endif()

  wp15_tool(wp15_webpmux_rs webpmux_rs SOURCES ${wp15_webpmux_sources}
            LIBRARIES rs_exampleutil rs_imageioutil libwebp_rs)
  wp15_tool(wp15_anim_diff_rs anim_diff_rs NO_CONFIG SOURCES ${wp15_anim_diff_sources}
            LIBRARIES rs_exampleutil rs_imagedec rs_imageenc rs_imageioutil libwebp_rs)
  list(APPEND ALL_PROBES wp15_webpmux_rs wp15_anim_diff_rs)

  # img2webp: a copy of upstream's target with libwebp and libwebpmux replaced.
  get_target_property(wp15_img2webp_sources img2webp SOURCES)
  get_target_property(wp15_img2webp_dir img2webp SOURCE_DIR)
  set(wp15_img2webp_absolute)
  foreach(source ${wp15_img2webp_sources})
    if(NOT IS_ABSOLUTE "${source}")
      set(source "${wp15_img2webp_dir}/${source}")
    endif()
    list(APPEND wp15_img2webp_absolute "${source}")
  endforeach()
  get_target_property(wp15_img2webp_includes img2webp INCLUDE_DIRECTORIES)
  get_directory_property(wp15_directory_includes DIRECTORY "${UPSTREAM}"
                         INCLUDE_DIRECTORIES)
  add_executable(wp15_img2webp_rs ${wp15_img2webp_absolute})
  target_include_directories(wp15_img2webp_rs PRIVATE "${UPSTREAM}/src"
                             ${wp15_img2webp_includes} ${wp15_directory_includes})
  target_compile_definitions(wp15_img2webp_rs PRIVATE ${UPSTREAM_DEFINITIONS})
  get_target_property(wp15_img2webp_libraries img2webp LINK_LIBRARIES)
  set(wp15_img2webp_replaced)
  foreach(library ${wp15_img2webp_libraries})
    if(library STREQUAL "webp" OR library STREQUAL "webpdemux"
       OR library STREQUAL "libwebpmux")
      list(APPEND wp15_img2webp_replaced libwebp_rs)
    elseif(library MATCHES "^(exampleutil|imageioutil|imagedec|imageenc)$")
      list(APPEND wp15_img2webp_replaced rs_${library})
    elseif(library MATCHES "^(webpdecoder|sharpyuv)$")
      message(FATAL_ERROR "img2webp links ${library}, which the Rust library does not replace")
    else()
      list(APPEND wp15_img2webp_replaced ${library})
    endif()
  endforeach()
  target_link_libraries(wp15_img2webp_rs PRIVATE ${wp15_img2webp_replaced}
                        wp13_link_shim libwebp_rs)
  set_target_properties(wp15_img2webp_rs PROPERTIES
    OUTPUT_NAME img2webp_rs
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/probes/wp15")
  list(APPEND ALL_PROBES wp15_img2webp_rs)

  # The probes that use only the public API, linked against the Rust library:
  # the same commands through the C ABI (src/abi/mux.rs) as through the C
  # library.
  foreach(name mux anim_enc)
    add_executable(wp15_${name}_rs "${package_dir}/${name}.c"
                   "${CMAKE_CURRENT_SOURCE_DIR}/probe_support/probe_support.c")
    target_include_directories(wp15_${name}_rs PRIVATE
      "${CMAKE_CURRENT_SOURCE_DIR}/probe_support" "${UPSTREAM}"
      "${CMAKE_BINARY_DIR}/libwebp/src")
    target_compile_definitions(wp15_${name}_rs PRIVATE ${UPSTREAM_DEFINITIONS})
    target_link_libraries(wp15_${name}_rs PRIVATE libwebp_rs)
    set_target_properties(wp15_${name}_rs PROPERTIES
      OUTPUT_NAME ${name}_rs
      RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/probes/wp15")
    list(APPEND ALL_PROBES wp15_${name}_rs)
  endforeach()
endif()
