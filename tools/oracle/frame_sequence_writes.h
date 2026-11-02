// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#ifndef FRAME_SEQUENCE_WRITES_H_
#define FRAME_SEQUENCE_WRITES_H_
#include <array>
#include <vector>
#include "lib/jxl/image.h"
#include "lib/jxl/image_bundle.h"
#include "lib/jxl/passes_state.h"
// Test-only initialized-span observations; no unwritten sample is read.
struct FrameSequenceDcCoverage {
  size_t width = 0, height = 0;
  std::vector<bool> initialized;
  void Allocate(size_t w,size_t h) {width=w;height=h;initialized.assign(w*h,false);}
  void Row(size_t x,size_t y,size_t count) {for(size_t i=0;i<count;++i)initialized[y*width+x+i]=true;}
};
struct FrameSequenceBundleCoverage {
  std::vector<FrameSequenceDcCoverage> planes;
  int moved_color_slot = -1;
};
struct FrameSequenceWriteObservation {
  std::array<jxl::Image3F*, 4> slots;
  std::array<FrameSequenceDcCoverage, 4> coverage;
  std::array<jxl::ImageBundle*,2> bundles;
  std::array<FrameSequenceBundleCoverage,2> bundle_coverage;
  FrameSequenceWriteObservation(jxl::PassesSharedState&,jxl::ImageBundle&,jxl::ImageBundle&);
  ~FrameSequenceWriteObservation();
  void AfterInitFrame() {if(!bundles[0]->HasColor())bundle_coverage[0]={};}
  void AfterSections() {if(!bundles[1]->HasColor())bundle_coverage[1]={};}
  void AfterReferenceMove(size_t slot) {bundle_coverage[1].moved_color_slot=int(slot);bundle_coverage[1].planes.resize(3);}
};
#endif
