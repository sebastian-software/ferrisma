cmake_minimum_required(VERSION 3.18)
project(brotli_bitfield_observer LANGUAGES C)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
get_filename_component(PORT_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../../.." ABSOLUTE)
option(ORACLE_SANITIZERS "Instrument the actual source bitfield observer" OFF)
if(ORACLE_SANITIZERS)
  add_compile_options(-fsanitize=address,undefined -fno-sanitize-recover=all -fno-omit-frame-pointer)
  add_link_options(-fsanitize=address,undefined)
endif()
add_executable(brotli_bitfield_observer ../brotli_bitfield.c)
target_include_directories(brotli_bitfield_observer PRIVATE
  "${PORT_ROOT}/upstream/libjxl/third_party/brotli"
  "${PORT_ROOT}/upstream/libjxl/third_party/brotli/c/include")
