# Probe executables for the differential oracle (ADR 0002).
#
# tools/build_oracle.py first builds the unmodified upstream/libjpeg-turbo
# through its own CMake project into build/oracle/<variant>/libjpeg-turbo, then
# configures this project with ORACLE_UPSTREAM_BUILD pointing at that tree and
# the same CMAKE_C_FLAGS, so that probes are compiled like the library.
#
# Probes: every tools/oracle/probes/<wp>/<name>.c is one executable, written to
# ORACLE_PROBE_DIR/<wp>/<name>. Sources in probes/<wp>/support/ form a helper
# library linked into that package's probes, and an optional
# probes/<wp>/oracle.cmake adds further targets. Every probe links
# tools/oracle/probe_support and the oracle's static libjpeg, and may include
# public and internal headers ("jpeglib.h", "jpegint.h", "jdhuff.h", ...)
# from upstream/libjpeg-turbo/src and the generated jconfig.h.
cmake_minimum_required(VERSION 3.15)
project(libjpeg_turbo_rs_oracle LANGUAGES C)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

get_filename_component(PORT_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../.." ABSOLUTE)
set(UPSTREAM "${PORT_ROOT}/upstream/libjpeg-turbo")
if(NOT EXISTS "${UPSTREAM}/src/jpeglib.h")
  message(FATAL_ERROR "Initialize upstream/libjpeg-turbo first: git submodule update --init")
endif()
if(NOT ORACLE_UPSTREAM_BUILD OR NOT EXISTS "${ORACLE_UPSTREAM_BUILD}/jconfig.h")
  message(FATAL_ERROR "Set ORACLE_UPSTREAM_BUILD to a built upstream tree; run tools/build_oracle.py")
endif()
if(NOT ORACLE_PROBE_DIR)
  set(ORACLE_PROBE_DIR "${CMAKE_BINARY_DIR}/probes")
endif()

string(FIND " ${CMAKE_C_FLAGS} " " -ffp-contract=off " contract_off)
if(contract_off EQUAL -1)
  message(FATAL_ERROR "The reference configuration requires CMAKE_C_FLAGS to contain -ffp-contract=off")
endif()

add_library(oracle_jpeg STATIC IMPORTED)
set_target_properties(oracle_jpeg PROPERTIES
  IMPORTED_LOCATION "${ORACLE_UPSTREAM_BUILD}/libjpeg.a"
  INTERFACE_INCLUDE_DIRECTORIES "${ORACLE_UPSTREAM_BUILD};${UPSTREAM}/src")

add_library(probe_support STATIC probe_support/probe_support.c)
target_include_directories(probe_support PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/probe_support")
target_link_libraries(probe_support PUBLIC oracle_jpeg)

file(GLOB PACKAGE_DIRS LIST_DIRECTORIES true CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/probes/*")
set(ALL_PROBES)
foreach(package_dir ${PACKAGE_DIRS})
  if(NOT IS_DIRECTORY "${package_dir}")
    continue()
  endif()
  get_filename_component(package "${package_dir}" NAME)
  file(GLOB package_support CONFIGURE_DEPENDS "${package_dir}/support/*.c")
  set(package_libraries probe_support)
  if(package_support)
    add_library(${package}_support STATIC ${package_support})
    target_link_libraries(${package}_support PUBLIC probe_support)
    target_include_directories(${package}_support PUBLIC "${package_dir}/support")
    set(package_libraries ${package}_support)
  endif()
  file(GLOB probes CONFIGURE_DEPENDS "${package_dir}/*.c")
  foreach(source ${probes})
    get_filename_component(name "${source}" NAME_WE)
    set(target ${package}_${name})
    add_executable(${target} "${source}")
    target_link_libraries(${target} PRIVATE ${package_libraries})
    set_target_properties(${target} PROPERTIES
      OUTPUT_NAME ${name}
      RUNTIME_OUTPUT_DIRECTORY "${ORACLE_PROBE_DIR}/${package}")
    list(APPEND ALL_PROBES ${target})
  endforeach()
  # A package may add further targets. The fragment sees package, package_dir,
  # package_libraries, UPSTREAM, PORT_ROOT and ORACLE_UPSTREAM_BUILD, and
  # appends the targets that oracle_probes should build to ALL_PROBES.
  if(EXISTS "${package_dir}/oracle.cmake")
    include("${package_dir}/oracle.cmake")
  endif()
endforeach()

add_custom_target(oracle_probes ALL)
if(ALL_PROBES)
  add_dependencies(oracle_probes ${ALL_PROBES})
endif()
