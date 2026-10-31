cmake_minimum_required(VERSION 3.16)
project(libjxl_rust_oracle LANGUAGES C CXX)

get_filename_component(PORT_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../.." ABSOLUTE)
set(UPSTREAM "${PORT_ROOT}/upstream/libjxl")
if(NOT EXISTS "${UPSTREAM}/third_party/highway/hwy/base.h")
  message(FATAL_ERROR "Initialize upstream/libjxl and its third_party/highway submodule first")
endif()

set(HWY_ENABLE_TESTS OFF CACHE BOOL "" FORCE)
set(HWY_ENABLE_EXAMPLES OFF CACHE BOOL "" FORCE)
set(HWY_ENABLE_CONTRIB OFF CACHE BOOL "" FORCE)
add_subdirectory("${UPSTREAM}/third_party/highway" highway EXCLUDE_FROM_ALL)

# Follow upstream lib/jxl.cmake's export/version header generation. The
# upstream empty object source only supplies CMake target metadata; it is not
# linked and does not replace any codec implementation.
include(GenerateExportHeader)
add_library(jxl_oracle_export OBJECT EXCLUDE_FROM_ALL "${UPSTREAM}/lib/nothing.cc")
set_target_properties(jxl_oracle_export PROPERTIES CXX_VISIBILITY_PRESET hidden
  VISIBILITY_INLINES_HIDDEN 1 DEFINE_SYMBOL JXL_INTERNAL_LIBRARY_BUILD)
generate_export_header(jxl_oracle_export BASE_NAME JXL
  EXPORT_FILE_NAME "${CMAKE_CURRENT_BINARY_DIR}/include/jxl/jxl_export.h")
file(STRINGS "${UPSTREAM}/lib/CMakeLists.txt" JXL_VERSION_SETTINGS
  REGEX "^set\\(JPEGXL_(MAJOR|MINOR|PATCH)_VERSION [0-9]+\\)$")
foreach(setting IN LISTS JXL_VERSION_SETTINGS)
  string(REGEX MATCH "set\\((JPEGXL_[A-Z]+_VERSION) ([0-9]+)\\)" parsed "${setting}")
  set(${CMAKE_MATCH_1} ${CMAKE_MATCH_2})
endforeach()
configure_file("${UPSTREAM}/lib/jxl/version.h.in" "include/jxl/version.h")

# Compile the original source, including its writer and memory implementation.
# Unused image/frame entry points in enc_fields.cc are discarded by the linker;
# they are not replaced with stubs or altered copies of upstream functions.
add_executable(jxl_oracle
  main.cc
  "${UPSTREAM}/lib/jxl/ans_common.cc"
  "${UPSTREAM}/lib/jxl/dec_bit_reader.cc"
  "${UPSTREAM}/lib/jxl/dec_ans.cc"
  "${UPSTREAM}/lib/jxl/dec_context_map.cc"
  "${UPSTREAM}/lib/jxl/dec_huffman.cc"
  "${UPSTREAM}/lib/jxl/enc_bit_writer.cc"
  "${UPSTREAM}/lib/jxl/enc_ans.cc"
  "${UPSTREAM}/lib/jxl/fields.cc"
  "${UPSTREAM}/lib/jxl/enc_fields.cc"
  "${UPSTREAM}/lib/jxl/enc_huffman.cc"
  "${UPSTREAM}/lib/jxl/enc_huffman_tree.cc"
  "${UPSTREAM}/lib/jxl/headers.cc"
  "${UPSTREAM}/lib/jxl/huffman_table.cc"
  "${UPSTREAM}/lib/jxl/memory_manager_internal.cc"
  "${UPSTREAM}/lib/jxl/simd_util.cc")
target_compile_features(jxl_oracle PRIVATE cxx_std_17)
target_include_directories(jxl_oracle PRIVATE "${UPSTREAM}" "${UPSTREAM}/lib/include"
  "${CMAKE_CURRENT_BINARY_DIR}/include")
target_compile_definitions(jxl_oracle PRIVATE HWY_COMPILE_ONLY_SCALAR=1 JXL_STATIC_DEFINE)
target_link_libraries(jxl_oracle PRIVATE hwy)
if(CMAKE_CXX_COMPILER_ID MATCHES "Clang|GNU")
  target_compile_options(jxl_oracle PRIVATE -ffunction-sections -fdata-sections)
  if(APPLE)
    target_link_options(jxl_oracle PRIVATE -Wl,-dead_strip)
  else()
    target_link_options(jxl_oracle PRIVATE -Wl,--gc-sections)
  endif()
endif()

option(ORACLE_SANITIZERS "Instrument the C++ oracle with ASan and UBSan" OFF)
if(ORACLE_SANITIZERS)
  target_compile_options(jxl_oracle PRIVATE -fsanitize=address,undefined -fno-sanitize-recover=all -fno-omit-frame-pointer)
  target_link_options(jxl_oracle PRIVATE -fsanitize=address,undefined)
endif()
