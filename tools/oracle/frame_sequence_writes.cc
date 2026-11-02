// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Compile pristine source unchanged. Its real output factory binds directly;
// DC and bundle factories receive forwarding initialized-span observers.
#define GetWriteToImage3FStage FrameSequenceOriginalWriteToImage3FStage
#define GetWriteToImageBundleStage FrameSequenceOriginalWriteToImageBundleStage
#include "lib/jxl/render_pipeline/stage_write.cc"
#undef GetWriteToImage3FStage
#undef GetWriteToImageBundleStage
#include "lib/jxl/passes_state.h"
#include "frame_sequence_writes.h"
namespace {
thread_local FrameSequenceWriteObservation* observation = nullptr;
class ObservedDcWriter final : public jxl::RenderPipelineStage {
 public:
  ObservedDcWriter(std::unique_ptr<jxl::RenderPipelineStage> original,
                   FrameSequenceDcCoverage* coverage)
      : RenderPipelineStage(original->settings_), original_(std::move(original)),
        coverage_(coverage) {}
  jxl::Status SetInputSizes(const std::vector<std::pair<size_t,size_t>>& sizes) override {
    JXL_RETURN_IF_ERROR(original_->SetInputSizes(sizes));
    coverage_->width=sizes[0].first;coverage_->height=sizes[0].second;
    coverage_->initialized.assign(coverage_->width*coverage_->height,false);
    return true;
  }
  jxl::Status ProcessRow(const RowInfo& input,const RowInfo& output,size_t left,
      size_t right,size_t width,size_t x,size_t y,size_t thread) const override {
    JXL_RETURN_IF_ERROR(original_->ProcessRow(input,output,left,right,width,x,y,thread));
    for(size_t i=0;i<width;++i)coverage_->initialized[y*coverage_->width+x+i]=true;
    return true;
  }
  jxl::RenderPipelineChannelMode GetChannelMode(size_t c) const override {return original_->GetChannelMode(c);}
  jxl::Status IsInitialized() const override {return original_->IsInitialized();}
  jxl::Status PrepareForThreads(size_t num) override {return original_->PrepareForThreads(num);}
  const char* GetName() const override {return original_->GetName();}
 private:
  std::unique_ptr<jxl::RenderPipelineStage> original_;
  FrameSequenceDcCoverage* coverage_;
};
class ObservedBundleWriter final : public jxl::RenderPipelineStage {
 public:
  ObservedBundleWriter(std::unique_ptr<jxl::RenderPipelineStage> original,
      jxl::ImageBundle* image,FrameSequenceBundleCoverage* coverage)
      : RenderPipelineStage(original->settings_),original_(std::move(original)),image_(image),coverage_(coverage) {}
  jxl::Status SetInputSizes(const std::vector<std::pair<size_t,size_t>>& sizes) override {
    JXL_RETURN_IF_ERROR(original_->SetInputSizes(sizes));
    coverage_->moved_color_slot=-1;coverage_->planes.resize(sizes.size());
    for(size_t c=0;c<sizes.size();++c)coverage_->planes[c].Allocate(sizes[c].first,sizes[c].second);
    return true;
  }
  jxl::Status ProcessRow(const RowInfo& input,const RowInfo& output,size_t left,
      size_t right,size_t width,size_t x,size_t y,size_t thread) const override {
    auto status=original_->ProcessRow(input,output,left,right,width,x,y,thread);
    // The source padding guard precedes three color copies, then each EC
    // width guard precedes its copy. Record successful prefixes on row errors.
    if(left==0&&right==0)for(size_t c=0;c<coverage_->planes.size();++c){
      if(c>=3&&image_->extra_channels()[c-3].xsize()<x+width)break;
      coverage_->planes[c].Row(x,y,width);
    }
    return status;
  }
  jxl::RenderPipelineChannelMode GetChannelMode(size_t c) const override {return original_->GetChannelMode(c);}
  jxl::Status IsInitialized() const override {return original_->IsInitialized();}
  jxl::Status PrepareForThreads(size_t n) override {return original_->PrepareForThreads(n);}
  const char* GetName() const override {return original_->GetName();}
 private:
  std::unique_ptr<jxl::RenderPipelineStage> original_;
  jxl::ImageBundle* image_;
  FrameSequenceBundleCoverage* coverage_;
};

}
FrameSequenceWriteObservation::FrameSequenceWriteObservation(jxl::PassesSharedState& state,jxl::ImageBundle& output,jxl::ImageBundle& storage) {
  bundles={&output,&storage};
  if(observation)throw std::runtime_error("nested frame write observation");
  for(size_t i=0;i<4;++i)slots[i]=&state.dc_frames[i];
  observation=this;
}
FrameSequenceWriteObservation::~FrameSequenceWriteObservation(){observation=nullptr;}
namespace jxl {
std::unique_ptr<RenderPipelineStage> GetWriteToImage3FStage(JxlMemoryManager* mm,Image3F* image) {
  auto original=FrameSequenceOriginalWriteToImage3FStage(mm,image);
  if(observation)for(size_t i=0;i<4;++i)if(observation->slots[i]==image)
    return std::make_unique<ObservedDcWriter>(std::move(original),&observation->coverage[i]);
  return original;
}
std::unique_ptr<RenderPipelineStage> GetWriteToImageBundleStage(ImageBundle* image,const ColorEncoding* color) {
  auto original=FrameSequenceOriginalWriteToImageBundleStage(image,color);
  if(observation)for(size_t i=0;i<2;++i)if(observation->bundles[i]==image){
    observation->bundle_coverage[i]={};
    return std::make_unique<ObservedBundleWriter>(std::move(original),image,&observation->bundle_coverage[i]);
  }
  return original;
}

}
