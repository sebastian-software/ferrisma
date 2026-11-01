// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observe initialized state of original owners; never read indeterminate LUTs,
// frame dimensions, metadata pointers, patch bindings, or fourth quantizer lanes.
void ColorBundleObservation(const jxl::ColorEncoding&);
void SubsamplingObservation(const jxl::YCbCrChromaSubsampling&);
bool SSNoiseHasAny(const jxl::NoiseParams&);
void SSBundle(const jxl::ImageBundle& b) {
  std::cout<<" B "<<(b.memory_manager()!=nullptr)<<' '<<(b.jpeg_data!=nullptr)<<' '<<unsigned(b.color_transform)
    <<' '<<b.origin.x0<<' '<<b.origin.y0<<' '<<b.duration<<' '<<b.timecode<<' '<<b.use_for_next_frame<<' '<<b.blend<<' '<<unsigned(b.blendmode)
    <<' '<<Hex(jxl::Bytes(reinterpret_cast<const uint8_t*>(b.name.data()),b.name.size()))<<' '<<(b.metadata()!=nullptr)<<' '<<b.extra_channels().size()<<' '<<b.decoded_bytes()<<' '<<b.color_.xsize();
  for(size_t c=0;c<3;++c)SSPlane(b.color_.Plane(c),false);
  SubsamplingObservation(b.chroma_subsampling);ColorBundleObservation(b.c_current());
}
void SSOwners(std::istringstream& in,JxlMemoryManager* mm) {
  bool metadata;size_t moves,decoded;in>>metadata>>moves>>decoded;
  jxl::NoiseParams noise;for(auto& v:noise.lut)v=EFReadFloat(in);
  std::cout<<"N "<<SSNoiseHasAny(noise);noise.Clear();std::cout<<' '<<SSNoiseHasAny(noise);for(auto v:noise.lut)std::cout<<' '<<SCBits<uint32_t>(v);
  jxl::ImageMetadata m;jxl::ImageBundle bundle(mm,metadata?&m:nullptr);bundle.SetDecodedBytes(decoded);
  for(size_t i=0;i<moves;++i){auto moved=std::move(bundle);bundle=std::move(moved);}SSBundle(bundle);
  auto state=std::make_unique<jxl::PassesEncoderState>(mm);
  for(size_t i=0;i<moves;++i){auto moved=std::move(state);state=std::move(moved);}
  const auto& e=*state;
  std::cout<<" E "<<(e.memory_manager()!=nullptr)<<' '<<e.streaming_mode<<' '<<e.initialize_global_state<<' '<<e.dc_group_index<<' '<<e.coeffs.size()<<' '<<e.special_frames.size()<<' '<<e.progressive_splitter.GetNumPasses()<<' '<<e.passes.size()<<' '<<e.histogram_idx.size()<<' '<<e.used_acs<<' '<<e.used_orders.size()<<' '<<SCBits<uint32_t>(e.x_qm_multiplier)<<' '<<SCBits<uint32_t>(e.b_qm_multiplier)<<' '<<e.shared.num_histograms;
  EFParams(e.cparams);SSPlane(e.initial_quant_masking1x1,false);
  auto& s=state->shared;
  std::cout<<" S "<<s.coeff_order_size<<' '<<s.coeff_orders.size()<<' '<<(s.dc==&s.dc_storage)<<' '<<s.block_ctx_map.num_ctxs<<' '<<s.block_ctx_map.num_dc_ctxs<<' '<<Hex(jxl::Bytes(s.block_ctx_map.ctx_map));
  for(const auto& t:s.block_ctx_map.dc_thresholds)std::cout<<' '<<t.size();std::cout<<' '<<s.block_ctx_map.qf_thresholds.size();
  for(const auto& r:s.reference_frames){std::cout<<" R "<<(r.frame!=nullptr)<<' '<<r.ib_is_in_xyb;SSBundle(*r.frame);}
  for(const auto& dc:s.dc_frames)for(size_t c=0;c<3;++c)SSPlane(dc.Plane(c),false);
  auto& p=s.image_features.patches;
  std::cout<<" H "<<(p.memory_manager_!=nullptr)<<' '<<p.HasAny()<<' '<<p.positions_.size()<<' '<<p.ref_positions_.size()<<' '<<p.blendings_.size()<<' '<<p.patch_tree_.size()<<' '<<p.num_patches_.size()<<' '<<p.sorted_patches_y0_.size()<<' '<<p.sorted_patches_y1_.size();
  p.SetShared(&s.reference_frames);std::cout<<' '<<(p.reference_frames_==&s.reference_frames);
  auto& sp=s.image_features.splines;
  for(size_t i=0;i<=moves;++i){
    std::cout<<" L "<<(sp.memory_manager_!=nullptr)<<' '<<sp.HasAny()<<' '<<sp.GetQuantizationAdjustment()<<' '<<sp.QuantizedSplines().size()<<' '<<sp.StartingPoints().size()<<' '<<sp.splines_storage_.size()<<' '<<sp.starting_points_storage_.size()<<' '<<sp.segments_.size()<<' '<<bool(sp.segment_indices_)<<' '<<bool(sp.segment_y_start_);
    sp.Clear();
  }
}
template<class T> void SSAcImage(std::istringstream& in,JxlMemoryManager* mm) {
  bool empty;size_t w,h,mode;in>>empty>>w>>h>>mode;
  auto result=empty?jxl::StatusOr<std::unique_ptr<jxl::ACImageT<T>>>(std::make_unique<jxl::ACImageT<T>>()):jxl::ACImageT<T>::Make(mm,w,h);
  std::cout<<int(result.status().code());if(!result.ok())return;
  std::unique_ptr<jxl::ACImage> image=std::move(result).value_();
  if(!empty)for(size_t c=0;c<3;++c)for(size_t y=0;y<h;++y){
    T* row;if constexpr(sizeof(T)==2)row=image->PlaneRow(c,y,0).ptr16;else row=image->PlaneRow(c,y,0).ptr32;
    for(size_t x=0;x<image->PixelsPerRow();++x)row[x]=static_cast<T>(c*1931+y*997+x*31337);
  }
  if(mode==1)image->ZeroFillPlane(1);if(mode==2)image->ZeroFill();
  auto moved=std::move(image);image=std::move(moved);
  std::cout<<' '<<int(image->Type())<<' '<<image->PixelsPerRow()<<' '<<image->IsEmpty();
  const auto& view=*image;
  if(!empty)for(size_t c=0;c<3;++c)for(size_t y=0;y<h;++y)for(size_t x=0;x<image->PixelsPerRow();++x){
    if constexpr(sizeof(T)==2)std::cout<<' '<<view.PlaneRow(c,y,x).ptr16[0];else std::cout<<' '<<view.PlaneRow(c,y,x).ptr32[0];
  }
}
