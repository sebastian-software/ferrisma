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

# Compile the original source, including its writer and memory implementation.
# Unused image/frame entry points in enc_fields.cc are discarded by the linker;
# they are not replaced with stubs or altered copies of upstream functions.
add_executable(jxl_oracle
  main.cc
  "${UPSTREAM}/lib/jxl/ans_common.cc"
  "${UPSTREAM}/lib/jxl/dec_bit_reader.cc"
  "${UPSTREAM}/lib/jxl/dec_huffman.cc"
  "${UPSTREAM}/lib/jxl/enc_bit_writer.cc"
  "${UPSTREAM}/lib/jxl/fields.cc"
  "${UPSTREAM}/lib/jxl/enc_fields.cc"
  "${UPSTREAM}/lib/jxl/enc_huffman.cc"
  "${UPSTREAM}/lib/jxl/enc_huffman_tree.cc"
  "${UPSTREAM}/lib/jxl/headers.cc"
  "${UPSTREAM}/lib/jxl/huffman_table.cc"
  "${UPSTREAM}/lib/jxl/memory_manager_internal.cc"
  "${UPSTREAM}/lib/jxl/simd_util.cc")
target_compile_features(jxl_oracle PRIVATE cxx_std_17)
target_include_directories(jxl_oracle PRIVATE "${UPSTREAM}" "${UPSTREAM}/lib/include")
target_compile_definitions(jxl_oracle PRIVATE HWY_COMPILE_ONLY_SCALAR=1)
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
