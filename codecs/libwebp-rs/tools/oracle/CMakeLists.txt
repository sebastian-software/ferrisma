# The differential oracle: the pinned, unmodified libwebp built through its own
# CMake project in the reference configuration (docs/PORTING.md, ADR 0002),
# plus probe executables that call it. tools/build_oracle.py configures this
# project once per variant into build/oracle/<variant>/.
#
# Probes: every tools/oracle/probes/<wp>/<name>.c is one executable, written to
# build/oracle/<variant>/probes/<wp>/<name>. Sources in probes/<wp>/support/
# form a helper library linked into that package's probes, and an optional
# probes/<wp>/oracle.cmake adds further targets. Every probe links
# tools/oracle/probe_support and the upstream static libraries, and may include
# internal headers as "src/..." or "sharpyuv/..." from upstream/libwebp.
cmake_minimum_required(VERSION 3.18)
project(libwebp_rs_oracle LANGUAGES C)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

get_filename_component(PORT_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../.." ABSOLUTE)
set(UPSTREAM "${PORT_ROOT}/upstream/libwebp")
if(NOT EXISTS "${UPSTREAM}/src/webp/decode.h")
  message(FATAL_ERROR "Initialize upstream/libwebp first: git submodule update --init")
endif()

# The reference configuration. tools/build_oracle.py passes the same values on
# the command line; forcing them here keeps a manual configure from drifting.
# ORACLE_SIMD builds upstream with its SIMD code: the variant release-simd of
# ADR 0008, the reference of the port's SIMD build, never the scalar
# reference configuration.
option(ORACLE_SIMD "Build upstream with its SIMD code (release-simd, ADR 0008)" OFF)
if(ORACLE_SIMD)
  set(WEBP_ENABLE_SIMD ON CACHE BOOL "" FORCE)
else()
  set(WEBP_ENABLE_SIMD OFF CACHE BOOL "" FORCE)
endif()
set(WEBP_USE_THREAD ON CACHE BOOL "" FORCE)
set(WEBP_NEAR_LOSSLESS ON CACHE BOOL "" FORCE)
set(WEBP_ENABLE_SWAP_16BIT_CSP OFF CACHE BOOL "" FORCE)
set(WEBP_BUILD_DWEBP ON CACHE BOOL "" FORCE)
set(WEBP_BUILD_CWEBP ON CACHE BOOL "" FORCE)
set(WEBP_BUILD_WEBPINFO ON CACHE BOOL "" FORCE)
set(WEBP_BUILD_LIBWEBPMUX ON CACHE BOOL "" FORCE)
# img2webp (WP14) assembles the animated inputs of the animation decoder tests
# from frame sequences; it needs only libwebpmux and imageio, and changes no
# library object.
set(WEBP_BUILD_IMG2WEBP ON CACHE BOOL "" FORCE)
foreach(tool ANIM_UTILS GIF2WEBP VWEBP WEBPMUX EXTRAS WEBP_JS FUZZTEST)
  set(WEBP_BUILD_${tool} OFF CACHE BOOL "" FORCE)
endforeach()
set(BUILD_SHARED_LIBS OFF CACHE BOOL "" FORCE)

string(FIND " ${CMAKE_C_FLAGS} " " -ffp-contract=off " contract_off)
if(contract_off EQUAL -1)
  message(FATAL_ERROR "The reference configuration requires CMAKE_C_FLAGS to contain -ffp-contract=off")
endif()

# ORACLE_SANITIZERS instruments the library, the tools and the probes alike.
option(ORACLE_SANITIZERS "Instrument the oracle with ASan and UBSan" OFF)
if(ORACLE_SANITIZERS)
  add_compile_options(-fsanitize=address,undefined -fno-sanitize-recover=all -fno-omit-frame-pointer)
  add_link_options(-fsanitize=address,undefined)
endif()

# On x86-64, upstream's WEBP_ENABLE_SIMD=OFF appends -mno-avx2 -mno-sse4.1
# -mno-sse2 to CMAKE_C_FLAGS. Without SSE2 the x86-64 psABI cannot return a
# double, and Clang rejects such functions (src/dsp/ssim.c has several). A
# trailing -msse2 restores only the ABI's SSE2 baseline, which the Rust port
# uses too; -mno-sse4.1 and -mno-avx2 stay, and config.h still defines no
# WEBP_HAVE_SSE2, so no intrinsics file is compiled (ADR 0002).
if(NOT ORACLE_SIMD AND (CMAKE_SYSTEM_PROCESSOR MATCHES "^(x86_64|AMD64|amd64)$" OR CMAKE_OSX_ARCHITECTURES STREQUAL "x86_64"))
  add_compile_options(-msse2)
endif()

add_subdirectory("${UPSTREAM}" libwebp)

# Compile probes exactly like the library's own objects: upstream appends its
# SIMD-disabling flags to CMAKE_C_FLAGS and adds HAVE_CONFIG_H in its directory
# scope, so read that scope's final values instead of repeating its logic.
get_directory_property(UPSTREAM_C_FLAGS DIRECTORY "${UPSTREAM}" DEFINITION CMAKE_C_FLAGS)
get_directory_property(UPSTREAM_DEFINITIONS DIRECTORY "${UPSTREAM}" COMPILE_DEFINITIONS)
set(CMAKE_C_FLAGS "${UPSTREAM_C_FLAGS}")
file(WRITE "${CMAKE_BINARY_DIR}/upstream-flags.txt"
     "c_flags=${UPSTREAM_C_FLAGS}\ndefinitions=${UPSTREAM_DEFINITIONS}\n")

set(ORACLE_LIBRARIES webpdemux libwebpmux webp sharpyuv)

add_library(probe_support STATIC probe_support/probe_support.c)
target_include_directories(probe_support PUBLIC
  "${CMAKE_CURRENT_SOURCE_DIR}/probe_support"
  "${UPSTREAM}"
  "${CMAKE_BINARY_DIR}/libwebp"
  "${CMAKE_BINARY_DIR}/libwebp/src")
target_compile_definitions(probe_support PUBLIC ${UPSTREAM_DEFINITIONS})
target_link_libraries(probe_support PUBLIC ${ORACLE_LIBRARIES})

file(GLOB PACKAGE_DIRS LIST_DIRECTORIES true CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/probes/*")
# Sorted, so that probes/wp12/oracle.cmake defines libwebp_rs and wp12_consumer()
# before probes/wp13/oracle.cmake uses them.
list(SORT PACKAGE_DIRS)
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
      RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/probes/${package}")
    list(APPEND ALL_PROBES ${target})
  endforeach()
  # A package may add further targets, for example a consumer build of an
  # upstream tool against the Rust library. The fragment sees package,
  # package_dir, package_libraries, UPSTREAM, PORT_ROOT and ORACLE_LIBRARIES,
  # and appends the targets that oracle_probes should build to ALL_PROBES.
  if(EXISTS "${package_dir}/oracle.cmake")
    include("${package_dir}/oracle.cmake")
  endif()
endforeach()

# Every executable, upstream's tools included, links simd_level.c, which caps
# the CPU query at the level LIBWEBP_SIMD_LEVEL names (ADR 0008). Upstream's
# tools use the plain target_link_libraries signature, the probes the keyword
# one.
add_library(oracle_simd_level OBJECT probe_support/simd_level.c)
target_include_directories(oracle_simd_level PRIVATE
  "${UPSTREAM}" "${CMAKE_BINARY_DIR}/libwebp" "${CMAKE_BINARY_DIR}/libwebp/src")
target_compile_definitions(oracle_simd_level PRIVATE ${UPSTREAM_DEFINITIONS})
foreach(tool dwebp cwebp webpinfo img2webp)
  target_link_libraries(${tool} oracle_simd_level)
endforeach()
foreach(target ${ALL_PROBES})
  get_target_property(type ${target} TYPE)
  if(type STREQUAL "EXECUTABLE")
    target_link_libraries(${target} PRIVATE oracle_simd_level)
  endif()
endforeach()

add_custom_target(oracle_probes)
if(ALL_PROBES)
  add_dependencies(oracle_probes ${ALL_PROBES})
endif()
add_custom_target(oracle_tools)
add_dependencies(oracle_tools dwebp cwebp webpinfo img2webp)
