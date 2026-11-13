cmake_minimum_required(VERSION 3.18)
project(libjxl_rust_oracle LANGUAGES C CXX)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# The Rust palette sort follows the recorded libc++ specialization. Select
# that same standard library explicitly on Linux instead of silently using
# libstdc++, whose equivalent-element order differs. This is oracle-only.
if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
  if(NOT CMAKE_CXX_COMPILER_ID MATCHES "Clang")
    message(FATAL_ERROR "The Linux differential oracle requires Clang with libc++")
  endif()
  string(APPEND CMAKE_CXX_FLAGS " -stdlib=libc++")
endif()

get_filename_component(PORT_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../.." ABSOLUTE)
set(UPSTREAM "${PORT_ROOT}/upstream/libjxl")
if(NOT EXISTS "${UPSTREAM}/third_party/highway/hwy/base.h")
  message(FATAL_ERROR "Initialize upstream/libjxl and its third_party/highway submodule first")
endif()
if(NOT EXISTS "${UPSTREAM}/third_party/brotli/c/include/brotli/encode.h")
  message(FATAL_ERROR "Initialize upstream/libjxl/third_party/brotli for frame reference headers")
endif()
if(NOT EXISTS "${UPSTREAM}/third_party/skcms/skcms.h")
  message(FATAL_ERROR "Initialize upstream/libjxl/third_party/skcms for the full image reference")
endif()

option(ORACLE_SANITIZERS "Instrument the complete C++ reference with ASan and UBSan" OFF)
# Highway target of the original (port ADR 0004). SCALAR is the scalar
# reference and stays the default. NEON (AArch64) and AVX2 (x86-64) build the
# same pristine sources restricted to exactly that one SIMD target; they are
# compared only with the port's SIMD path at the same target, never with the
# scalar reference. The adapters name the selected target's namespace through
# ORACLE_HWY_NS; HWY_DYNAMIC_DISPATCH in the original selects the same one.
set(ORACLE_HWY_TARGET "SCALAR" CACHE STRING "Highway target of the original: SCALAR, NEON or AVX2")
set_property(CACHE ORACLE_HWY_TARGET PROPERTY STRINGS SCALAR NEON AVX2)
if(ORACLE_HWY_TARGET STREQUAL "SCALAR")
  set(ORACLE_HWY_DEFINITIONS HWY_COMPILE_ONLY_SCALAR=1 ORACLE_HWY_NS=N_SCALAR)
  add_compile_definitions(HWY_COMPILE_ONLY_SCALAR=1)
elseif(ORACLE_HWY_TARGET STREQUAL "NEON" OR ORACLE_HWY_TARGET STREQUAL "AVX2")
  # Highway's own target knobs come from the forced include hwy_target.h
  # (via CMAKE_CXX_FLAGS, so upstream's CMake sees HWY_DISABLED_TARGETS as
  # forced and adds no default list). NEON: the AArch64 baseline already is
  # HWY_NEON; excluding the higher Arm targets leaves HWY_TARGETS = HWY_NEON,
  # identical to upstream's dispatch build on such a CPU. AVX2: every TU is
  # compiled with exactly Highway's HWY_AVX2 attribute string
  # (hwy/ops/set_macros-inl.h) and HWY_COMPILE_ONLY_STATIC, so HWY_AVX2 is the
  # only target. Upstream's dispatch build instead keeps baseline code at the
  # default x86-64 flags; the adapters that call target namespaces directly
  # do not compile in a multi-target build yet (port ADR 0004). hwy_ops.cc
  # asserts the target set and refuses to run where dispatch would select
  # another target (Rosetta 2 needs ROSETTA_ADVERTISE_AVX=1).
  string(APPEND CMAKE_CXX_FLAGS
    " -DORACLE_HWY_TARGET_${ORACLE_HWY_TARGET}=1 -include ${CMAKE_CURRENT_SOURCE_DIR}/hwy_target.h")
  if(ORACLE_HWY_TARGET STREQUAL "AVX2")
    string(APPEND CMAKE_CXX_FLAGS
      " -msse2 -mssse3 -msse4.1 -msse4.2 -mpclmul -maes -mavx -mavx2 -mbmi -mbmi2 -mfma -mf16c")
  endif()
  if(ORACLE_HWY_TARGET STREQUAL "NEON")
    set(ORACLE_HWY_DEFINITIONS ORACLE_HWY_NS=N_NEON ORACLE_HWY_REQUIRED=HWY_NEON)
  else()
    set(ORACLE_HWY_DEFINITIONS ORACLE_HWY_NS=N_AVX2 ORACLE_HWY_REQUIRED=HWY_AVX2)
  endif()
else()
  message(FATAL_ERROR "ORACLE_HWY_TARGET must be SCALAR, NEON or AVX2")
endif()
if(NOT ORACLE_HWY_TARGET STREQUAL "SCALAR" AND ORACLE_SANITIZERS)
  message(FATAL_ERROR "The SIMD oracle variants are built without sanitizers")
endif()
if(ORACLE_SANITIZERS)
  add_compile_options(-fsanitize=address,undefined -fno-sanitize-recover=all -fno-omit-frame-pointer)
  add_link_options(-fsanitize=address,undefined)
endif()

set(HWY_ENABLE_TESTS OFF CACHE BOOL "" FORCE)
set(HWY_ENABLE_EXAMPLES OFF CACHE BOOL "" FORCE)
set(HWY_ENABLE_CONTRIB OFF CACHE BOOL "" FORCE)
set(BUILD_SHARED_LIBS OFF CACHE BOOL "" FORCE)
set(BUILD_TESTING OFF CACHE BOOL "" FORCE)
foreach(feature TOOLS DEVTOOLS DOXYGEN MANPAGES BENCHMARK EXAMPLES JNI SJPEG
                OPENEXR VIEWERS TCMALLOC PLUGINS COVERAGE FUZZERS LTO)
  set(JPEGXL_ENABLE_${feature} OFF CACHE BOOL "" FORCE)
endforeach()
set(JPEGXL_BUNDLE_LIBPNG OFF CACHE BOOL "" FORCE)
set(JPEGXL_ENABLE_SKCMS ON CACHE BOOL "" FORCE)
foreach(dependency BROTLI HWY LCMS2 GTEST)
  set(JPEGXL_FORCE_SYSTEM_${dependency} OFF CACHE BOOL "" FORCE)
endforeach()

# Build the complete original static library through its own CMake project.
# Adapter TUs include selected pristine .cc files to reach private functions;
# the archive supplies the remaining original objects, without replacement code.
add_subdirectory("${UPSTREAM}" upstream EXCLUDE_FROM_ALL)
if(CMAKE_CXX_COMPILER_ID MATCHES "Clang|GNU")
  # Native Linux x86_64 ordinary/ASan O3, no-unroll and O1 objects choose different
  # competing NaN operands. Pristine isolated O0 stages agree on every retained
  # request, and all finite inputs match both unchanged archive graphs.
  if(CMAKE_SYSTEM_NAME STREQUAL "Linux" AND CMAKE_SYSTEM_PROCESSOR MATCHES "^(x86_64|amd64|AMD64)$")
    set_source_files_properties(chroma_ycbcr_stage.cc chroma_upsampling_stage.cc
      PROPERTIES COMPILE_OPTIONS "-O0;-fno-rtti")
  endif()
  set_source_files_properties(spot_color.cc PROPERTIES COMPILE_OPTIONS "-fno-rtti;-fno-access-control")
  if(CMAKE_SYSTEM_NAME STREQUAL "Linux" AND CMAKE_SYSTEM_PROCESSOR MATCHES "^(x86_64|amd64|AMD64)$")
    set_property(SOURCE spot_color.cc APPEND PROPERTY COMPILE_OPTIONS -O0)
  endif()
  set_source_files_properties(icc_codec.cc PROPERTIES COMPILE_OPTIONS -fno-rtti)
  # Read-only checkpoints of the original encoder's private adapter storage.
  set_source_files_properties(encoder_public.cc PROPERTIES COMPILE_OPTIONS "-fno-rtti;-fno-access-control")
  # Read-only access to the modular encoder's stream images.
  set_source_files_properties(encoder_coefficients.cc PROPERTIES COMPILE_OPTIONS "-fno-rtti;-fno-access-control")
  set_source_files_properties(encoder_ar.cc PROPERTIES COMPILE_OPTIONS "-fno-rtti;-fno-access-control")
  set_source_files_properties(chroma_image.cc PROPERTIES COMPILE_OPTIONS -fno-rtti)
  set_source_files_properties(chroma_stages.cc PROPERTIES COMPILE_OPTIONS "-fno-rtti;-fno-access-control")
  # Original protected lifecycle calls only; pristine archive flags stay unchanged.
  set_source_files_properties(output_stage.cc PROPERTIES COMPILE_OPTIONS "-fno-rtti;-fno-access-control")
  set_source_files_properties(blending_stage.cc PROPERTIES COMPILE_OPTIONS "-fno-access-control;-fno-rtti")
  # Keep Quantizer's original constructor/recomputation calls across plain and
  # sanitized builds. Inlining folds *1 into a raw copy only in the plain build,
  # changing signaling-NaN observations. The source routines stay unmodified.
  set_property(SOURCE "${UPSTREAM}/lib/jxl/quantizer.cc"
    DIRECTORY "${UPSTREAM}/lib" APPEND PROPERTY COMPILE_OPTIONS -fno-inline-functions)
endif()
find_package(Python3 REQUIRED COMPONENTS Interpreter)
set_property(DIRECTORY APPEND PROPERTY CMAKE_CONFIGURE_DEPENDS
  "${CMAKE_CURRENT_SOURCE_DIR}/extract_source.py"
  "${CMAKE_CURRENT_SOURCE_DIR}/source_fragments.json"
  "${UPSTREAM}/lib/jxl/render_pipeline/render_pipeline_test.cc"
  "${UPSTREAM}/lib/jxl/test_utils.cc"
  "${UPSTREAM}/lib/jxl/test_image.cc"
  "${UPSTREAM}/lib/jxl/enc_gaborish_test.cc"
  "${UPSTREAM}/lib/jxl/decode_test.cc"
  "${UPSTREAM}/lib/jxl/dec_group.cc"
  "${UPSTREAM}/lib/jxl/decode.cc"
  "${UPSTREAM}/third_party/skcms/skcms.cc")
execute_process(COMMAND "${Python3_EXECUTABLE}"
  "${CMAKE_CURRENT_SOURCE_DIR}/extract_source.py" "${CMAKE_CURRENT_BINARY_DIR}/generated"
  RESULT_VARIABLE extraction_status)
if(NOT extraction_status STREQUAL "0")
  message(FATAL_ERROR "Pinned source helper extraction failed")
endif()
add_executable(jxl_oracle color_output.cc decoder_api.cc decoder_public.cc encoder_public.cc encoder_transforms.cc encoder_transforms_unit.cc encoder_heuristics.cc encoder_coefficients.cc encoder_ar.cc decode_test_inputs.cc output_stage.cc decoder_geometry.cc extra_channels.cc spot_color.cc icc_codec.cc chroma_image.cc chroma_stages.cc chroma_ycbcr_stage.cc chroma_upsampling_stage.cc frame_sequence_writes.cc frame_sequences.cc blending_stage.cc main.cc retained_frame_owners.cc splines.cc alpha.cc patches.cc epf.cc noise_render.cc noise_stage.cc vardct_dc.cc group_schedule.cc dequant.cc group_render.cc block_context.cc dct.cc sample_conversion.cc noise.cc gaborish.cc xyb_stage.cc quant_ac.cc quantizer.cc ac_group_symbols.cc to_linear.cc from_linear.cc jpeg_writer.cc icc_reader.cc jpeg_metadata.cc decode_to_jpeg.cc skcms_parser.cc decoder_input.cc jpeg_reconstruction.cc skcms_transform.cc skcms_fitting.cc skcms_matrix.cc skcms_transfer.cc
  "${UPSTREAM}/lib/jxl/test_memory_manager.cc"
  # Pristine ParseDescription for the decode_test.cc color-space inputs.
  "${UPSTREAM}/lib/extras/dec/color_description.cc")
# Expose original internal state to the frame fixture adapter without changing
# source files, class layouts, or runtime algorithms. Limit this to its TU.
if(CMAKE_CXX_COMPILER_ID MATCHES "Clang|GNU")
  set_source_files_properties(icc_reader.cc PROPERTIES COMPILE_OPTIONS "-fno-rtti;-fno-access-control")
  set_source_files_properties(splines.cc PROPERTIES COMPILE_OPTIONS "-fno-access-control;-fno-rtti")
  # Match the original GetBlock class hierarchy in this traversal observer.
  # Keep the pristine noise stage operator order stable across ordinary and
  # sanitized objects. Only this isolated stage TU uses scoped -O0.
  set_source_files_properties(frame_sequences.cc frame_sequence_writes.cc PROPERTIES COMPILE_OPTIONS "-fno-access-control;-fno-rtti")
  # The color-output adapter instantiates original pipeline/decoder owners;
  # match upstream CMake's class hierarchy without changing any algorithm.
  set_source_files_properties(color_output.cc PROPERTIES COMPILE_OPTIONS -fno-rtti)
  set_source_files_properties(retained_frame_owners.cc PROPERTIES COMPILE_OPTIONS "-fno-access-control;-fno-rtti")
  set_source_files_properties(to_linear.cc PROPERTIES COMPILE_OPTIONS "-fno-access-control;-fno-rtti")
  set_source_files_properties(from_linear.cc PROPERTIES COMPILE_OPTIONS "-fno-access-control;-fno-rtti")
  # Native original alpha expressions commute exceptional operands differently
  # across optimized/instrumented graphs. O0 preserves their scalar boundaries;
  # native x86 additionally requires strict FP to retain the source subtraction
  # and ADDSS destinations. ARM keeps its original contraction: strict FP changes
  # finite words there. The pristine TU supplies all actual public alpha symbols,
  # with every sanitizer active. tools/alpha_reference.py audits both defaults.
  set_source_files_properties(alpha.cc PROPERTIES COMPILE_OPTIONS "-O0;-fno-inline-functions")
  if(CMAKE_SYSTEM_NAME STREQUAL "Linux" AND CMAKE_SYSTEM_PROCESSOR MATCHES "^(x86_64|amd64|AMD64)$")
    set_property(SOURCE alpha.cc APPEND PROPERTY COMPILE_OPTIONS -ffp-model=strict)
  endif()
  set_source_files_properties(patches.cc PROPERTIES COMPILE_OPTIONS "-fno-access-control;-fno-rtti")
  set_source_files_properties(epf.cc PROPERTIES COMPILE_OPTIONS "-O0;-fno-rtti;-fno-access-control")
  set_source_files_properties(noise_stage.cc PROPERTIES COMPILE_OPTIONS "-O0;-fno-rtti")
  set_source_files_properties(noise_render.cc PROPERTIES COMPILE_OPTIONS "-fno-rtti;-fno-access-control")
  set_source_files_properties(group_render.cc PROPERTIES COMPILE_OPTIONS -fno-rtti)
  set_source_files_properties(group_schedule.cc PROPERTIES COMPILE_OPTIONS -fno-access-control)
  if(CMAKE_SYSTEM_NAME STREQUAL "Linux" AND CMAKE_SYSTEM_PROCESSOR MATCHES "^(x86_64|amd64|AMD64)$")
    # Native x86 ordinary/ASan objects choose different competing NaN payloads
    # in the B correlation sum. Retain pristine scalar operator boundaries in
    # this isolated observer; original decoder objects and sanitizers stay intact.
    set_source_files_properties(dequant.cc PROPERTIES COMPILE_OPTIONS -O0)
  endif()
  # Pristine scalar compressed_dc.cc supplies the actual public DC symbols.
  # Keep its operand order reproducible with every sanitizer still active;
  # neither the original archive nor any earlier adapter changes flags.
  set_source_files_properties(vardct_dc.cc PROPERTIES COMPILE_OPTIONS "-O0;-fno-access-control;-fno-rtti")
  # Unrolling makes the ordinary AArch64 identity object use FMSUB while
  # ASan keeps FMADD with a negative scale, changing a NaN sign. Retain its
  # original loop shape in this new TU; contraction, O3 and sanitizers stay on.
  set_source_files_properties(dct.cc PROPERTIES COMPILE_OPTIONS -fno-unroll-loops)
  # The ordinary and ASan O3 encoder transform objects choose different
  # competing NaN operands. This observation TU keeps the same loop shape in
  # both graphs; the archive's own objects serve every other request.
  set_source_files_properties(encoder_transforms_unit.cc PROPERTIES COMPILE_OPTIONS -fno-unroll-loops)
  # Observe quantization cache masks and offsets without changing declarations.
  set_source_files_properties(quant_ac.cc PROPERTIES COMPILE_OPTIONS "-fno-access-control")
  # Match upstream's no-RTTI build when deriving fixture pipeline stages.
  set_source_files_properties(sample_conversion.cc PROPERTIES COMPILE_OPTIONS "-fno-access-control;-fno-rtti")
  # Clang's unrolled max-number reduction changes the early-return result when
  # a signaling NaN follows a value above the threshold. Retain the source loop.
  set_source_files_properties(noise.cc PROPERTIES COMPILE_OPTIONS -fno-unroll-loops)
  # O3 commutes the original scalar Gaborish operands differently with ASan.
  # Retain the pristine operator bodies in their own TU, with all sanitizer
  # flags active. The old adapter and original library flags stay unchanged.
  set_source_files_properties(gaborish.cc PROPERTIES COMPILE_OPTIONS "-O0;-fno-rtti")
  # Keep original scalar XYB operand order reproducible with active sanitizers.
  # Only this new adapter and its included pristine stage/helper use -O0;
  # neither the original codec library nor another fixture changes flags.
  set_source_files_properties(xyb_stage.cc PROPERTIES COMPILE_OPTIONS "-O0;-fno-access-control;-fno-rtti")
else()
  message(FATAL_ERROR "Frame state fixtures require Clang or GCC access-control support")
endif()
# Original ordinary/ASan x86 O3 concat chooses different paired NaN operands.
# Keep the complete source body isolated and all instrumentation enabled.
if(CMAKE_SYSTEM_NAME STREQUAL "Linux" AND CMAKE_SYSTEM_PROCESSOR MATCHES "^(x86_64|amd64|AMD64)$")
  target_sources(jxl_oracle PRIVATE skcms_concat.cc)
  set_source_files_properties(skcms_concat.cc PROPERTIES COMPILE_OPTIONS -O0)
endif()
# Decision D8 with working rule O1 (pending owner confirmation): NaN payloads
# are not part of the gate, and for requests whose inputs contain NaN or
# infinity a word is compared only where two scalar reference builds of the
# original agree. ORACLE_REFERENCE_PINS=OFF builds the second one: the same
# sources and Highway scalar configuration at upstream's normal optimization
# level, without the per-TU numeric pins above (-O0, -fno-unroll-loops,
# -fno-inline-functions, -ffp-model=strict). Adapter-only flags (-fno-rtti,
# -fno-access-control) and the target-wide upstream flags stay. The default
# (ON) is the pinned reference, unchanged.
option(ORACLE_REFERENCE_PINS "Keep the per-TU numeric pins of the pinned reference build" ON)
if(NOT ORACLE_REFERENCE_PINS)
  set(oracle_numeric_pins -O0 -fno-unroll-loops -fno-inline-functions -ffp-model=strict)
  get_target_property(oracle_sources jxl_oracle SOURCES)
  foreach(source IN LISTS oracle_sources)
    get_source_file_property(options "${source}" COMPILE_OPTIONS)
    if(options)
      list(REMOVE_ITEM options ${oracle_numeric_pins})
      set_source_files_properties("${source}" PROPERTIES COMPILE_OPTIONS "${options}")
    endif()
  endforeach()
  get_property(options SOURCE "${UPSTREAM}/lib/jxl/quantizer.cc"
    DIRECTORY "${UPSTREAM}/lib" PROPERTY COMPILE_OPTIONS)
  list(REMOVE_ITEM options ${oracle_numeric_pins})
  set_property(SOURCE "${UPSTREAM}/lib/jxl/quantizer.cc"
    DIRECTORY "${UPSTREAM}/lib" PROPERTY COMPILE_OPTIONS "${options}")
endif()
target_compile_features(jxl_oracle PRIVATE cxx_std_17)
target_include_directories(jxl_oracle PRIVATE "${UPSTREAM}" "${UPSTREAM}/lib/include"
  "${UPSTREAM}/third_party/brotli/c/include" "${CMAKE_CURRENT_BINARY_DIR}/generated")
target_compile_definitions(jxl_oracle PRIVATE ${ORACLE_HWY_DEFINITIONS} JXL_STATIC_DEFINE JXL_CMS_STATIC_DEFINE
  JXL_THREADS_STATIC_DEFINE)
# Generic Highway operation observer (hwy-op/hwy-target requests); it
# compiles per target through foreach_target like the original's own TUs.
target_sources(jxl_oracle PRIVATE hwy_ops.cc)
target_include_directories(jxl_oracle PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}")
# The original thread runners: decode_test.cc histories also run with them.
target_link_libraries(jxl_oracle PRIVATE jxl-internal jxl_threads)
if(CMAKE_CXX_COMPILER_ID MATCHES "Clang" AND NOT WIN32)
  # Preserve upstream lib/CMakeLists.txt's numeric code-generation flags.
  # Auto-vectorizing scalar cost reductions changes their FMA rounding, and
  # sanitizer instrumentation can otherwise select a different reduction loop.
  target_compile_options(jxl_oracle PRIVATE -fno-slp-vectorize -fno-vectorize)
endif()
if(CMAKE_CXX_COMPILER_ID MATCHES "Clang|GNU")
  target_compile_options(jxl_oracle PRIVATE -ffunction-sections -fdata-sections)
  if(APPLE)
    target_link_options(jxl_oracle PRIVATE -Wl,-dead_strip)
  else()
    target_link_options(jxl_oracle PRIVATE -Wl,--gc-sections)
  endif()
endif()

# Public skcms headers; algorithms remain in the original CMS archive.
target_include_directories(jxl_oracle PRIVATE "${UPSTREAM}/third_party/skcms")
# Private source operations live in a separate executable. Its compile command
# is copied byte-for-byte from the actual untouched skcms.cc build except for
# the input TU and output object paths; public archive bindings stay independent.
add_custom_command(OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/skcms_parser_private"
  COMMAND "${Python3_EXECUTABLE}" "${PORT_ROOT}/tools/skcms_parser_reference.py"
          --build-private "${CMAKE_CURRENT_BINARY_DIR}"
  DEPENDS jxl_cms "$<TARGET_FILE:jxl_cms>" skcms_parser_private.cc skcms_parser.cc
          "${PORT_ROOT}/tools/skcms_parser_reference.py"
  VERBATIM)
add_custom_target(skcms_parser_private_observer
  DEPENDS "${CMAKE_CURRENT_BINARY_DIR}/skcms_parser_private")
# The private skcms observer and the C ABI drivers serve scalar requests only.
if(ORACLE_HWY_TARGET STREQUAL "SCALAR")
  add_dependencies(jxl_oracle skcms_parser_private_observer)
endif()

# The decoder C ABI drivers. One driver, written against the pinned public
# headers only, links either the original archive (jxl_abi_original) or the
# Rust static library that exports src/jxl/decode_abi.rs and
# src/threads/abi.rs (jxl_abi_rust). Both link the original default CMS for JxlGetDefaultCms. Cargo builds the Rust
# library with the profile and features of the checked-profile tests, which
# therefore reuse it; Cargo decides whether it is current. Ordinary Rust
# builds still compile and link no C or C++ code.
set(ORACLE_RUST_PROFILE "checked" CACHE STRING
  "Cargo profile of the Rust library that jxl_abi_rust links")
find_program(CARGO_EXECUTABLE cargo REQUIRED)
if(ORACLE_RUST_PROFILE STREQUAL "dev")
  set(rust_profile_directory debug)
else()
  set(rust_profile_directory "${ORACLE_RUST_PROFILE}")
endif()
# Empty builds for the host; a triple (for example x86_64-apple-darwin, next
# to -DCMAKE_OSX_ARCHITECTURES=x86_64) cross-builds the Rust library for it.
set(ORACLE_RUST_TARGET "" CACHE STRING "Rust target triple of the library that jxl_abi_rust links")
set(rust_target_arguments)
set(rust_target_directory "")
if(ORACLE_RUST_TARGET)
  set(rust_target_arguments --target "${ORACLE_RUST_TARGET}")
  set(rust_target_directory "${ORACLE_RUST_TARGET}/")
endif()
set(RUST_DECODER_ABI_LIBRARY
  "${PORT_ROOT}/target/${rust_target_directory}${rust_profile_directory}/${CMAKE_STATIC_LIBRARY_PREFIX}jxl${CMAKE_STATIC_LIBRARY_SUFFIX}")
add_custom_target(rust_decoder_abi
  COMMAND "${CARGO_EXECUTABLE}" build --locked --offline --lib
          --profile "${ORACLE_RUST_PROFILE}" --features cpp-oracle
          ${rust_target_arguments}
          --target-dir "${PORT_ROOT}/target"
  WORKING_DIRECTORY "${PORT_ROOT}"
  BYPRODUCTS "${RUST_DECODER_ABI_LIBRARY}"
  COMMENT "Building the Rust decoder C ABI library"
  VERBATIM)
foreach(variant original rust)
  add_executable(jxl_abi_${variant} decoder_abi.cc decoder_abi_layout.c)
  target_compile_features(jxl_abi_${variant} PRIVATE cxx_std_17 c_std_11)
  target_include_directories(jxl_abi_${variant} PRIVATE "${UPSTREAM}/lib/include"
    "${CMAKE_CURRENT_BINARY_DIR}/upstream/lib/include")
  target_compile_definitions(jxl_abi_${variant} PRIVATE JXL_STATIC_DEFINE JXL_CMS_STATIC_DEFINE
    JXL_THREADS_STATIC_DEFINE)
endforeach()
# The original exports the runners from its separate jxl_threads library; the
# Rust library exports them next to the decoder.
target_link_libraries(jxl_abi_original PRIVATE jxl-internal jxl_threads jxl_cms)
add_dependencies(jxl_abi_rust rust_decoder_abi)
target_link_libraries(jxl_abi_rust PRIVATE "${RUST_DECODER_ABI_LIBRARY}" jxl_cms)
if(NOT APPLE)
  # The Rust standard library's system dependencies on Linux.
  find_package(Threads REQUIRED)
  target_link_libraries(jxl_abi_rust PRIVATE Threads::Threads ${CMAKE_DL_LIBS} m)
endif()
if(ORACLE_HWY_TARGET STREQUAL "SCALAR")
  add_dependencies(jxl_oracle jxl_abi_original jxl_abi_rust)
endif()

# SIMD variants: build like upstream's release library. The per-TU pins above
# (-O0, -fno-unroll-loops, -fno-inline-functions, -ffp-model=strict) only keep
# the scalar reference's competing-NaN operand order stable between ordinary
# and sanitized builds (port ADR 0002); vector intrinsics do not depend on
# them, and the SIMD variants have no sanitized twin. Remove them here, keeping
# -fno-rtti and -fno-access-control, which only let adapters compile.
if(NOT ORACLE_HWY_TARGET STREQUAL "SCALAR")
  set(oracle_scalar_pins -O0 -fno-unroll-loops -fno-inline-functions -ffp-model=strict)
  get_target_property(oracle_sources jxl_oracle SOURCES)
  foreach(source IN LISTS oracle_sources)
    get_source_file_property(options "${source}" COMPILE_OPTIONS)
    if(options)
      list(REMOVE_ITEM options ${oracle_scalar_pins})
      set_source_files_properties("${source}" PROPERTIES COMPILE_OPTIONS "${options}")
    endif()
  endforeach()
  get_property(options SOURCE "${UPSTREAM}/lib/jxl/quantizer.cc" DIRECTORY "${UPSTREAM}/lib"
    PROPERTY COMPILE_OPTIONS)
  list(REMOVE_ITEM options ${oracle_scalar_pins})
  set_property(SOURCE "${UPSTREAM}/lib/jxl/quantizer.cc" DIRECTORY "${UPSTREAM}/lib"
    PROPERTY COMPILE_OPTIONS "${options}")
endif()

# render stages: row requests of the SIMD render stages (port ADR 0004 Plan
# item 1). The TU only calls the original's public stage factories.
target_sources(jxl_oracle PRIVATE render_stages_simd.cc)
# end render stages

# public gate: whole decodes through the original public decoder (port ADR
# 0004, "Decoder path" condition (b)). The TU only calls the public API.
target_sources(jxl_oracle PRIVATE decoder_public_simd.cc)
# end public gate
# modular: requests of the SIMD modular and output-conversion kernels (port
# ADR 0004 Plan item 2). The TU only calls the original's exported kernels.
target_sources(jxl_oracle PRIVATE modular_simd.cc)
# end modular
# stages2: requests of the SIMD upsampling, noise, blending and splines
# stages (port ADR 0004 Plan item 1). The TU only calls the original's
# public stage factories and pipelines; it calls the protected
# PrepareForThreads of a single stage, as upsampling.inc does.
target_sources(jxl_oracle PRIVATE render_stages2_simd.cc)
set_source_files_properties(render_stages2_simd.cc PROPERTIES COMPILE_OPTIONS "-fno-access-control")
# end stages2
# color: row requests of the SIMD color stages (port ADR 0004 Plan item 1).
# The TU only calls the original's public stage factories.
target_sources(jxl_oracle PRIVATE color_stages_simd.cc)
# cl-cms calls the stage's protected SetInputSizes/PrepareForThreads, as
# the render pipeline does.
set_source_files_properties(color_stages_simd.cc PROPERTIES COMPILE_OPTIONS -fno-access-control)
# end color
