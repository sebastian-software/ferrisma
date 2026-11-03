cmake_minimum_required(VERSION 3.18)
project(box_content_reference LANGUAGES C CXX)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
get_filename_component(PORT_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../../.." ABSOLUTE)
set(UPSTREAM "${PORT_ROOT}/upstream/libjxl")
set(BROTLI "${UPSTREAM}/third_party/brotli")
option(ORACLE_SANITIZERS "Instrument the entire original graph" OFF)
if(ORACLE_SANITIZERS)
  add_compile_options(-fsanitize=address,undefined -fno-sanitize-recover=all -fno-omit-frame-pointer)
  add_link_options(-fsanitize=address,undefined)
endif()
set(BUILD_SHARED_LIBS OFF CACHE BOOL "" FORCE)
set(BROTLI_BUILD_TOOLS OFF CACHE BOOL "" FORCE)
set(BROTLI_DISABLE_TESTS ON CACHE BOOL "" FORCE)
add_subdirectory("${BROTLI}" brotli EXCLUDE_FROM_ALL)
# Generate the source's public export-header pattern for its static API headers;
# no original source/declaration or algorithm is replaced.
include(GenerateExportHeader)
generate_export_header(brotlidec BASE_NAME JXL
  EXPORT_FILE_NAME "${CMAKE_CURRENT_BINARY_DIR}/generated/jxl/jxl_export.h")
# Keep the original version template and its three literal source definitions.
file(STRINGS "${UPSTREAM}/lib/CMakeLists.txt" version_lines
  REGEX "^set\\(JPEGXL_(MAJOR|MINOR|PATCH)_VERSION [0-9]+\\)$")
list(LENGTH version_lines version_count)
if(NOT version_count EQUAL 3)
  message(FATAL_ERROR "Pinned source version definitions changed")
endif()
foreach(line IN LISTS version_lines)
  cmake_language(EVAL CODE "${line}")
endforeach()
configure_file("${UPSTREAM}/lib/jxl/version.h.in"
  "${CMAKE_CURRENT_BINARY_DIR}/generated/jxl/version.h" @ONLY)
add_executable(box_content_decoder_oracle ../box_content_decoder.cc
  "${UPSTREAM}/lib/jxl/box_content_decoder.cc")
target_compile_features(box_content_decoder_oracle PRIVATE cxx_std_17)
set_target_properties(box_content_decoder_oracle PROPERTIES CXX_EXTENSIONS OFF)
if(CMAKE_CXX_COMPILER_ID MATCHES "Clang|GNU")
  # Preserve the actual original libjxl scalar object's RTTI/exception and
  # vectorization boundaries. Brotli archive units retain their own defaults.
  set_source_files_properties("${UPSTREAM}/lib/jxl/box_content_decoder.cc"
    PROPERTIES COMPILE_OPTIONS "-fno-rtti;-fno-exceptions")
  if(CMAKE_CXX_COMPILER_ID MATCHES "Clang")
    set_property(SOURCE "${UPSTREAM}/lib/jxl/box_content_decoder.cc"
      APPEND PROPERTY COMPILE_OPTIONS -fno-slp-vectorize -fno-vectorize)
  endif()
endif()
target_include_directories(box_content_decoder_oracle PRIVATE "${UPSTREAM}"
  "${UPSTREAM}/lib/include" "${BROTLI}/c/include" "${CMAKE_CURRENT_BINARY_DIR}/generated")
target_compile_definitions(box_content_decoder_oracle PRIVATE JXL_STATIC_DEFINE)
# Private fields are read only in the observer; the actual original object
# remains compiled normally, without access-control or numeric option changes.
set_source_files_properties(../box_content_decoder.cc PROPERTIES COMPILE_OPTIONS -fno-access-control)
target_link_libraries(box_content_decoder_oracle PRIVATE brotlidec brotlienc)
if(APPLE)
  target_link_options(box_content_decoder_oracle PRIVATE "-Wl,-map,${CMAKE_CURRENT_BINARY_DIR}/box_content_decoder.map")
else()
  target_link_options(box_content_decoder_oracle PRIVATE "-Wl,-Map,${CMAKE_CURRENT_BINARY_DIR}/box_content_decoder.map")
endif()
