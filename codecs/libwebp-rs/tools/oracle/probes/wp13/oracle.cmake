# WP13 consumer build: upstream's cwebp, unmodified, linked against the Rust
# static library instead of libwebp (ADR 0004), and the
# WP13 probes that use only the public API linked against it as well. It reuses
# WP12's libwebp_rs target and wp12_consumer() (probes/wp12/oracle.cmake),
# which tools/oracle/CMakeLists.txt includes first; without
# LIBWEBP_RS_STATICLIB nothing is added.
#
# cwebp lands in probes/wp12/ next to WP12's dwebp and webpinfo, and links
# consumer/link_shim.c for SharpYuvGetVersion(), which only -version calls.
# The probes land in probes/wp13/ as <name>_rs.
if(LIBWEBP_RS_STATICLIB)
  if(NOT COMMAND wp12_consumer)
    message(FATAL_ERROR "probes/wp13/oracle.cmake needs probes/wp12/oracle.cmake first")
  endif()

  add_library(wp13_link_shim STATIC "${package_dir}/consumer/link_shim.c")
  target_include_directories(wp13_link_shim PRIVATE "${UPSTREAM}")

  wp12_consumer(cwebp executable)
  target_link_libraries(rs_cwebp PRIVATE wp13_link_shim)
  list(APPEND ALL_PROBES rs_cwebp)

  # The probes that use only encode.h, linked against the Rust library: the
  # same records through the C ABI (src/abi/encode.rs) as through the C library.
  foreach(name encode api picture)
    if(EXISTS "${package_dir}/${name}.c")
      add_executable(wp13_${name}_rs "${package_dir}/${name}.c"
                     "${CMAKE_CURRENT_SOURCE_DIR}/probe_support/probe_support.c")
      target_include_directories(wp13_${name}_rs PRIVATE
        "${CMAKE_CURRENT_SOURCE_DIR}/probe_support" "${UPSTREAM}"
        "${CMAKE_BINARY_DIR}/libwebp/src")
      target_compile_definitions(wp13_${name}_rs PRIVATE ${UPSTREAM_DEFINITIONS})
      target_link_libraries(wp13_${name}_rs PRIVATE libwebp_rs)
      set_target_properties(wp13_${name}_rs PROPERTIES
        OUTPUT_NAME ${name}_rs
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/probes/wp13")
      list(APPEND ALL_PROBES wp13_${name}_rs)
    endif()
  endforeach()
endif()
