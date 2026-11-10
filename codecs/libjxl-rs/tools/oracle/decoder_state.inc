// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Original shared/decoder initialization; observe defined state only.
#include "lib/jxl/dec_cache.h"
#include "lib/jxl/passes_state.h"
void DCSOrders(const std::vector<jxl::coeff_order_t>& v){
  std::cout<<" V "<<v.size();for(size_t i=0;i<v.size();){size_t end=i+1;while(end<v.size()&&v[end]==v[i])++end;std::cout<<' '<<v[i]<<','<<end-i;i=end;}std::cout<<" ;";
}
void DCSShared(const jxl::PassesSharedState& s,bool quant_pixels){
  std::cout<<" S "<<(s.metadata!=nullptr)<<' '<<(s.image_features.patches.reference_frames_==&s.reference_frames)<<' '<<s.coeff_order_size<<' '<<s.num_histograms;
  DimensionsObservation(s.frame_dim);SSPlane(s.ac_strategy.layers_,false);SSPlane(s.raw_quant_field,false);SSPlane(s.epf_sharpness,false);
  SSPlane(s.cmap.ytox_map,false);SSPlane(s.cmap.ytob_map,false);if(s.cmap.ytox_map.xsize())SSCorr(s.cmap.base());
  SSPlane(s.quant_dc,quant_pixels);DCSOrders(s.coeff_orders);
  int dc=-2;if(s.dc==&s.dc_storage)dc=-1;for(size_t i=0;i<4;++i)if(s.dc==&s.dc_frames[i])dc=int(i);std::cout<<" DC "<<dc;
  for(size_t c=0;c<3;++c)SSPlane(s.dc_storage.Plane(c),false);for(const auto& f:s.dc_frames)for(size_t c=0;c<3;++c)SSPlane(f.Plane(c),false);
}
struct DCSCallbackState {size_t calls=0,threads=0,pixels=0;};
extern "C" void* DCSInitCallback(void* p,size_t threads,size_t pixels){if(!p)return nullptr;auto& s=*static_cast<DCSCallbackState*>(p);++s.calls;s.threads=threads;s.pixels=pixels;return p;}
extern "C" void DCSRunCallback(void*,size_t,size_t,size_t,size_t,const void*){}
extern "C" void DCSDestroyCallback(void*){}
void DCSOutput(const jxl::ImageOutput& o){std::cout<<" O "<<o.format.num_channels<<' '<<unsigned(o.format.data_type)<<' '<<unsigned(o.format.endianness)<<' '<<o.format.align<<' '<<o.bits_per_sample<<' '<<bool(o.callback.init)<<' '<<bool(o.callback.run)<<' '<<bool(o.callback.destroy)<<' '<<bool(o.callback.init_opaque)<<' '<<bool(o.buffer)<<' '<<o.buffer_size<<' '<<o.stride;}
void DCSState(const jxl::PassesDecoderState& s){
  std::cout<<" D "<<(s.memory_manager()!=nullptr)<<' '<<(s.shared==&s.shared_storage)<<' '<<bool(s.upsampler8x)<<' '<<s.code.size()<<' '<<s.context_map.size();for(const auto& map:s.context_map){std::cout<<' '<<map.size();for(auto v:map)std::cout<<' '<<unsigned(v);}
  std::cout<<' '<<SCBits<uint32_t>(s.x_dm_multiplier)<<' '<<SCBits<uint32_t>(s.b_dm_multiplier);SSPlane(s.sigma,false);std::cout<<' '<<s.width<<' '<<s.height;DCSOutput(s.main_output);std::cout<<' '<<s.extra_output.size();for(const auto& o:s.extra_output)DCSOutput(o);
  std::cout<<' '<<s.fast_xyb_srgb8_conversion<<' '<<s.unpremul_alpha<<' '<<unsigned(s.undo_orientation)<<' '<<s.visible_frame_index<<' '<<s.nonvisible_frame_index<<' '<<s.used_acs.load()<<' '<<bool(s.coefficients);
  if(s.coefficients)std::cout<<' '<<int(s.coefficients->Type())<<' '<<s.coefficients->IsEmpty()<<' '<<s.coefficients->PixelsPerRow();std::cout<<' '<<bool(s.render_pipeline);
  SSBundle(s.frame_storage_for_referencing);OEIObservation(s.output_encoding_info);std::cout<<" C "<<s.shared_storage.coeff_order_size;DCSOrders(s.shared_storage.coeff_orders);
  if(s.upsampler8x){const auto& stage=*s.upsampler8x;const auto& settings=stage.settings_;std::cout<<" U "<<settings.shift_x<<' '<<settings.shift_y<<' '<<settings.border_x<<' '<<settings.border_y<<' '<<stage.GetName();for(size_t c=0;c<4;++c)std::cout<<' '<<int(stage.GetChannelMode(c));}
}
bool DecoderStateCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm){
  if(command=="dcs-callback"){
    unsigned mask;bool opaque;size_t threads,pixels;in>>mask>>opaque>>threads>>pixels;DCSCallbackState s;auto* p=opaque?&s:nullptr;
    jxl::PixelCallback cb(mask&1?DCSInitCallback:nullptr,mask&2?DCSRunCallback:nullptr,mask&4?DCSDestroyCallback:nullptr,p);void* result=nullptr;
    if(mask&1)result=cb.Init(threads,pixels);std::cout<<cb.IsPresent()<<' '<<bool(cb.init)<<' '<<bool(cb.run)<<' '<<bool(cb.destroy)<<' '<<bool(cb.init_opaque)<<' '<<(result==p)<<' '<<s.calls<<' '<<s.threads<<' '<<s.pixels;return true;
  }
  if(command=="dcs-shared"){
    size_t n,seed;unsigned dc_mask;in>>n>>dc_mask>>seed;std::vector<jxl::CodecMetadata> metadata(n);for(auto& m:metadata)m.m.xyb_encoded=false;jxl::PassesSharedState s(mm);s.metadata=nullptr;s.frame_dim.Set(1,1,0,0,0,true,1);s.image_features.patches.SetShared(nullptr);
    s.coeff_orders.resize(seed);for(size_t i=0;i<seed;++i)s.coeff_orders[i]=i*17+3;
    for(size_t i=0;i<4;++i)if(dc_mask&(1u<<i)){auto made=jxl::Image3F::Create(mm,i+1,2);if(!made.ok())throw std::runtime_error("decoder DC frame");s.dc_frames[i]=std::move(made).value_();}
    for(size_t i=0;i<n;++i){size_t w,h;bool modular,encoder,present;uint64_t flags;uint32_t level,passes;in>>w>>h>>modular>>encoder>>flags>>level>>passes>>present;if(!metadata[i].size.Set(w,h))throw std::runtime_error("decoder shared metadata");jxl::FrameHeader header(&metadata[i]);header.encoding=modular?jxl::FrameEncoding::kModular:jxl::FrameEncoding::kVarDCT;header.flags=flags;header.dc_level=level;header.passes.num_passes=passes;if(!present)header.nonserialized_metadata=nullptr;
      auto status=jxl::InitializePassesSharedState(header,&s,encoder);std::cout<<" R "<<int(status.code());DCSShared(s,present&&!encoder&&(flags&jxl::FrameHeader::kUseDcFrame)&&status);
    }return true;
  }
  if(command!="dcs-state")return false;
  size_t w,h,repeats,n;bool external;uint32_t xqm,bqm,epf;in>>w>>h>>external>>xqm>>bqm>>epf>>repeats>>n;
  jxl::CodecMetadata metadata,ext_metadata;if(!metadata.size.Set(w,h)||!ext_metadata.size.Set(65,33))throw std::runtime_error("decoder state metadata");jxl::FrameHeader header(&metadata),ext_header(&ext_metadata);header.x_qm_scale=xqm;header.b_qm_scale=bqm;header.loop_filter.epf_iters=epf;
  jxl::PassesSharedState ext(mm);ext.metadata=&ext_metadata;ext.frame_dim=ext_header.ToFrameDimensions();
  jxl::PassesDecoderState s(mm);s.shared_storage.metadata=&metadata;s.shared_storage.frame_dim=header.ToFrameDimensions();if(external)s.shared=&ext;
  // Explicit caller seeds for source-indeterminate fields and stale-output
  // retention. No uninitialized source rows or scalar values are observed.
  DCSCallbackState callback;s.x_dm_multiplier=0;s.b_dm_multiplier=0;s.width=21;s.height=13;s.main_output={};s.main_output.format={4,JXL_TYPE_UINT16,JXL_BIG_ENDIAN,16};s.main_output.bits_per_sample=12;s.main_output.callback=jxl::PixelCallback(DCSInitCallback,DCSRunCallback,DCSDestroyCallback,&callback);s.main_output.buffer=&callback;s.main_output.buffer_size=999;s.main_output.stride=77;s.extra_output={s.main_output,s.main_output};s.fast_xyb_srgb8_conversion=true;s.unpremul_alpha=true;s.undo_orientation=jxl::Orientation::kRotate90;
  s.visible_frame_index=9;s.nonvisible_frame_index=13;s.used_acs=0x7ffffff;s.shared_storage.coeff_order_size=123;s.shared_storage.coeff_orders={3,5,7};s.code.resize(2);s.context_map={{1,2},{3}};s.output_encoding_info={};
  auto sigma=jxl::ImageF::Create(mm,3,2);if(!sigma.ok())throw std::runtime_error("decoder sigma seed");s.sigma=std::move(sigma).value_();
  DCSState(s);for(size_t i=0;i<repeats;++i){auto status=s.Init(header);std::cout<<" R "<<int(status.code());DCSState(s);}
  for(size_t i=0;i<n;++i){uint32_t mask;size_t passes;in>>mask>>passes;s.used_acs=mask;auto status=s.InitForAC(passes,nullptr);std::cout<<" A "<<int(status.code());DCSState(s);}return true;
}
