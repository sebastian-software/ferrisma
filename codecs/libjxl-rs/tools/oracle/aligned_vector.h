// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// vardct kernels: owners shaped like libjxl's own buffers for the SIMD
// variants. The original's transforms and dequantization use aligned
// Load/Store (vmovaps on AVX2) on scratch, coefficient blocks and image
// rows that its callers allocate with hwy::AllocateAligned (HWY_ALIGNMENT,
// 128 bytes); a std::vector is only 16-byte aligned. Values, sizes and the
// observed words are unchanged.
#ifndef ORACLE_ALIGNED_VECTOR_H_
#define ORACLE_ALIGNED_VECTOR_H_
#include <cstddef>
#include <new>
#include <vector>
#include <hwy/aligned_allocator.h>
template <class T>
struct OracleAlignedAllocator {
  using value_type = T;
  OracleAlignedAllocator() = default;
  template <class U>
  OracleAlignedAllocator(const OracleAlignedAllocator<U>&) {}
  T* allocate(size_t n) {
    void* p = hwy::AllocateAlignedBytes(n * sizeof(T) + 1);
    if (!p) throw std::bad_alloc();
    return static_cast<T*>(p);
  }
  void deallocate(T* p, size_t) { hwy::FreeAlignedBytes(p, nullptr, nullptr); }
  template <class U>
  bool operator==(const OracleAlignedAllocator<U>&) const { return true; }
  template <class U>
  bool operator!=(const OracleAlignedAllocator<U>&) const { return false; }
};
template <class T>
using OracleAlignedVector = std::vector<T, OracleAlignedAllocator<T>>;
#endif  // ORACLE_ALIGNED_VECTOR_H_
