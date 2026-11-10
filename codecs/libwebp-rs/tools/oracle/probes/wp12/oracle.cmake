# WP12 consumer build: upstream's dwebp, webpinfo and imageio, unmodified,
# linked against the Rust static library instead of libwebp (ADR 0004).
# tools/build_oracle.py --rust-lib builds the library and passes
# LIBWEBP_RS_STATICLIB and LIBWEBP_RS_NATIVE_LIBS; without them nothing is
# added.
#
# The executables and helper libraries take the sources, include directories
# and compile definitions of upstream's own targets, so the build compiles
# what the oracle's dwebp and webpinfo compile. Upstream's src directory,
# which they get from the linked webp target, is added explicitly and searched
# first, so the pinned headers are used. Only the libraries change: libwebp
# and libwebpdemux become the Rust library, which exports decode.h (WP12) and
# demux.h (WP14, which removed the link stand-ins that WP12 used for the demux
# functions that imageio/webpdec.c references). The tools land in probes/wp12/
# next to the probes, as dwebp and webpinfo.
set(LIBWEBP_RS_STATICLIB "" CACHE FILEPATH
    "Rust static library (libwebp.a) for the WP12 consumer build")
set(LIBWEBP_RS_NATIVE_LIBS "" CACHE STRING
    "Native libraries that the Rust static library needs")

if(LIBWEBP_RS_STATICLIB)
  add_library(libwebp_rs STATIC IMPORTED)
  separate_arguments(wp12_native_libs NATIVE_COMMAND "${LIBWEBP_RS_NATIVE_LIBS}")
  set_target_properties(libwebp_rs PROPERTIES
    IMPORTED_LOCATION "${LIBWEBP_RS_STATICLIB}"
    INTERFACE_LINK_LIBRARIES "${wp12_native_libs}")

  get_directory_property(wp12_directory_includes DIRECTORY "${UPSTREAM}"
                         INCLUDE_DIRECTORIES)

  # Copies an upstream target as `rs_<name>`, with the upstream libraries
  # replaced as described above.
  function(wp12_consumer name kind)
    get_target_property(sources ${name} SOURCES)
    get_target_property(source_dir ${name} SOURCE_DIR)
    set(absolute)
    foreach(source ${sources})
      if(NOT IS_ABSOLUTE "${source}")
        set(source "${source_dir}/${source}")
      endif()
      list(APPEND absolute "${source}")
    endforeach()
    if(kind STREQUAL "library")
      add_library(rs_${name} STATIC ${absolute})
    else()
      add_executable(rs_${name} ${absolute})
      set_target_properties(rs_${name} PROPERTIES
        OUTPUT_NAME ${name}
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/probes/wp12")
    endif()
    # Upstream's targets reach ${UPSTREAM}/src (the public webp/*.h headers)
    # only through the PUBLIC include directories of the webp and exampleutil
    # targets they link, which INCLUDE_DIRECTORIES does not list. It comes
    # first, so the pinned headers also win over a system copy of libwebp in
    # the dependency include directories (for example /opt/homebrew/include).
    get_target_property(includes ${name} INCLUDE_DIRECTORIES)
    target_include_directories(rs_${name} PRIVATE "${UPSTREAM}/src" ${includes}
                               ${wp12_directory_includes})
    target_compile_definitions(rs_${name} PRIVATE ${UPSTREAM_DEFINITIONS})
    get_target_property(definitions ${name} COMPILE_DEFINITIONS)
    if(definitions)
      target_compile_definitions(rs_${name} PRIVATE ${definitions})
    endif()
    get_target_property(libraries ${name} LINK_LIBRARIES)
    set(replaced)
    foreach(library ${libraries})
      if(library STREQUAL "webp" OR library STREQUAL "webpdemux")
        list(APPEND replaced libwebp_rs)
      elseif(library MATCHES "^(exampleutil|imageioutil|imagedec|imageenc)$")
        list(APPEND replaced rs_${library})
      elseif(library MATCHES "^(webpdecoder|libwebpmux|sharpyuv)$")
        message(FATAL_ERROR "${name} links ${library}, which the Rust library does not replace")
      else()
        list(APPEND replaced ${library})
      endif()
    endforeach()
    target_link_libraries(rs_${name} PRIVATE ${replaced} libwebp_rs)
  endfunction()

  wp12_consumer(imageioutil library)
  wp12_consumer(exampleutil library)
  wp12_consumer(imagedec library)
  wp12_consumer(imageenc library)
  wp12_consumer(dwebp executable)
  wp12_consumer(webpinfo executable)
  list(APPEND ALL_PROBES rs_dwebp rs_webpinfo)

  # The decode probe linked against the Rust library: the same commands
  # through the C ABI (src/abi/decode.rs) as through the C library.
  add_executable(rs_decode "${package_dir}/decode.c"
                 "${CMAKE_CURRENT_SOURCE_DIR}/probe_support/probe_support.c")
  target_include_directories(rs_decode PRIVATE
    "${CMAKE_CURRENT_SOURCE_DIR}/probe_support" "${UPSTREAM}"
    "${CMAKE_BINARY_DIR}/libwebp/src")
  target_compile_definitions(rs_decode PRIVATE ${UPSTREAM_DEFINITIONS})
  target_link_libraries(rs_decode PRIVATE libwebp_rs)
  set_target_properties(rs_decode PROPERTIES
    OUTPUT_NAME decode_rs
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/probes/wp12")
  list(APPEND ALL_PROBES rs_decode)
endif()
