# WP14 consumer builds (ADR 0004).
#
# anim_dump: upstream's examples/anim_dump.c with anim_util.c and gifdec.c,
# unmodified. Upstream's CMake builds it only with WEBP_BUILD_ANIM_UTILS, which
# the reference configuration turns off (and which needs giflib), so the oracle
# has no anim_dump. This fragment builds it twice from the sources that
# examples/Makefile.am lists, once against the C library (probes/wp14/anim_dump)
# and, with the Rust library, once against it (probes/wp14/anim_dump_rs), with
# the same flags. The three sources are compiled without HAVE_CONFIG_H: in them
# config.h only defines WEBP_HAVE_GIF (and the Windows-only HAVE_WINCODEC_H), so
# both builds read WebP input only and need no giflib on any host, as upstream's
# own build does where giflib is missing. The pinned headers come first in the
# include path: upstream's targets get upstream/libwebp/src only through the
# webp target they link, and a system libwebp (Homebrew's on macOS) would
# otherwise supply webp/*.h. Keep that order when adding a consumer.
#
# With LIBWEBP_RS_STATICLIB (tools/build_oracle.py --rust-lib), the probes that
# use only the public demux API are linked against the Rust library as well:
# probes/wp14/anim_rs from anim.c, and probes/wp14/demux_rs from WP8's
# probes/wp8/demux.c. They reuse WP12's libwebp_rs target and the rs_ copies of
# upstream's imageio and example libraries (probes/wp12/oracle.cmake).

set(wp14_anim_dump_sources
    "${UPSTREAM}/examples/anim_dump.c"
    "${UPSTREAM}/examples/anim_util.c"
    "${UPSTREAM}/examples/gifdec.c")

function(wp14_anim_dump target output)
  add_executable(${target} ${wp14_anim_dump_sources})
  target_include_directories(${target} PRIVATE "${UPSTREAM}/src" "${UPSTREAM}")
  target_link_libraries(${target} PRIVATE ${ARGN})
  set_target_properties(${target} PROPERTIES
    OUTPUT_NAME ${output}
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/probes/wp14")
endfunction()

# Upstream's anim_dump links exampleutil, imagedec, imageenc, imageioutil, webp
# and webpdemux.
wp14_anim_dump(wp14_anim_dump anim_dump
               exampleutil imagedec imageenc imageioutil webpdemux webp)
list(APPEND ALL_PROBES wp14_anim_dump)

if(LIBWEBP_RS_STATICLIB)
  if(NOT TARGET libwebp_rs OR NOT TARGET rs_imageenc)
    message(FATAL_ERROR "probes/wp14/oracle.cmake needs probes/wp12/oracle.cmake first")
  endif()

  wp14_anim_dump(wp14_anim_dump_rs anim_dump_rs
                 rs_exampleutil rs_imagedec rs_imageenc rs_imageioutil libwebp_rs)
  list(APPEND ALL_PROBES wp14_anim_dump_rs)

  # The probes that use only the public API, linked against the Rust library:
  # the same commands through the C ABI (src/abi/demux.rs) as through the C
  # library.
  function(wp14_rs_probe name source)
    add_executable(wp14_${name}_rs "${source}"
                   "${CMAKE_CURRENT_SOURCE_DIR}/probe_support/probe_support.c")
    target_include_directories(wp14_${name}_rs PRIVATE
      "${CMAKE_CURRENT_SOURCE_DIR}/probe_support" "${UPSTREAM}"
      "${CMAKE_BINARY_DIR}/libwebp/src")
    target_compile_definitions(wp14_${name}_rs PRIVATE ${UPSTREAM_DEFINITIONS})
    target_link_libraries(wp14_${name}_rs PRIVATE libwebp_rs)
    set_target_properties(wp14_${name}_rs PROPERTIES
      OUTPUT_NAME ${name}_rs
      RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/probes/wp14")
  endfunction()
  wp14_rs_probe(anim "${package_dir}/anim.c")
  wp14_rs_probe(demux "${PORT_ROOT}/tools/oracle/probes/wp8/demux.c")
  list(APPEND ALL_PROBES wp14_anim_rs wp14_demux_rs)
endif()
