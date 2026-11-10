// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Original frame/rectangle/filter functions only. Initialize inactive pass
// array storage in the test input before observing it, avoiding source UB.
#include "lib/jxl/frame_header.h"
#include "lib/jxl/loop_filter.h"
#include "lib/jxl/enc_fields.h"
#include "lib/jxl/pack_signed.h"

void RectObservation(const jxl::Rect& r) { std::cout<<' '<<r.x0()<<','<<r.y0()<<','<<r.xsize()<<','<<r.ysize()<<','<<r.x1()<<','<<r.y1(); }
void DimensionsObservation(const jxl::FrameDimensions& d) {
  for (size_t v : {d.xsize,d.ysize,d.xsize_upsampled,d.ysize_upsampled,d.xsize_upsampled_padded,d.ysize_upsampled_padded,d.xsize_padded,d.ysize_padded,d.xsize_blocks,d.ysize_blocks,d.xsize_groups,d.ysize_groups,d.xsize_dc_groups,d.ysize_dc_groups,d.num_groups,d.num_dc_groups,d.group_dim,d.dc_group_dim}) std::cout<<' '<<v;
}
void FilterObservation(const jxl::LoopFilter& f) {
  std::cout<<' '<<f.all_default<<','<<f.gab<<','<<f.gab_custom<<','<<f.epf_iters<<','<<f.epf_sharp_custom<<','<<f.epf_weight_custom<<','<<f.epf_sigma_custom<<','<<f.extensions<<','<<f.nonserialized_is_modular;
  for (float v : {f.gab_x_weight1,f.gab_x_weight2,f.gab_y_weight1,f.gab_y_weight2,f.gab_b_weight1,f.gab_b_weight2}) std::cout<<' '<<FloatBits(v);
  for (float v : f.epf_sharp_lut) std::cout<<' '<<FloatBits(v);
  for (float v : f.epf_channel_scale) std::cout<<' '<<FloatBits(v);
  for (float v : {f.epf_pass1_zeroflush,f.epf_pass2_zeroflush,f.epf_quant_mul,f.epf_pass0_sigma_scale,f.epf_pass2_sigma_scale,f.epf_border_sad_mul,f.epf_sigma_for_modular}) std::cout<<' '<<FloatBits(v);
}
void ConfigureFilter(jxl::LoopFilter& f,unsigned variant) {
  if (variant==1) { f.gab=false;f.epf_iters=0; }
  if (variant==2) { f.gab_custom=true;f.gab_x_weight1=0.125f;f.gab_y_weight2=0.03125f;f.epf_sharp_custom=true;f.epf_sharp_lut[3]=0.375f;f.epf_weight_custom=true;f.epf_channel_scale[1]=4.f;f.epf_sigma_custom=true;f.epf_quant_mul=0.5f;f.epf_pass0_sigma_scale=0.75f;f.epf_sigma_for_modular=0.5f; }
  if (variant==3) f.epf_sigma_for_modular=1e-8f;
}
void SubsamplingObservation(const jxl::YCbCrChromaSubsampling& c) {
  std::cout<<' '<<unsigned(c.MaxHShift())<<','<<unsigned(c.MaxVShift())<<','<<c.Is444()<<','<<c.Is420()<<','<<c.Is422()<<','<<c.Is440();
  for (size_t i=0;i<3;++i) std::cout<<' '<<unsigned(c.RawHShift(i))<<','<<unsigned(c.RawVShift(i))<<','<<c.HShift(i)<<','<<c.VShift(i);
}
void PassesObservation(const jxl::Passes& p) {
  std::cout<<' '<<p.num_passes<<','<<p.num_downsample;
  for (auto* a : {p.downsample,p.last_pass,p.shift}) for (size_t i=0;i<jxl::kMaxNumPasses;++i) std::cout<<' '<<a[i];
}
void SeedPasses(jxl::Passes& p) { std::fill(std::begin(p.downsample),std::end(p.downsample),0);std::fill(std::begin(p.last_pass),std::end(p.last_pass),0);std::fill(std::begin(p.shift),std::end(p.shift),0); }
void PassTargetsObservation(const jxl::Passes& p) {
  for(size_t i=0;i<p.num_passes;++i) { int min,max;p.GetDownsamplingBracket(i,min,max);std::cout<<' '<<min<<','<<max; }
  for(uint32_t i=0;i<=p.num_passes+1;++i) std::cout<<' '<<p.GetDownsamplingTargetForCompletedPasses(i);
}
void BlendingObservation(const jxl::BlendingInfo& b) { std::cout<<' '<<unsigned(b.mode)<<','<<b.alpha_channel<<','<<b.clamp<<','<<b.source<<','<<b.nonserialized_num_extra_channels<<','<<b.nonserialized_is_partial_frame; }
void FrameObservation(const jxl::FrameHeader& f,bool dimensions) {
  std::cout<<' '<<f.all_default<<','<<unsigned(f.encoding)<<','<<unsigned(f.frame_type)<<','<<f.flags<<','<<unsigned(f.color_transform)<<','<<f.group_size_shift<<','<<f.x_qm_scale<<','<<f.b_qm_scale<<','<<f.custom_size_or_origin<<','<<f.frame_size.xsize<<','<<f.frame_size.ysize<<','<<f.upsampling<<','<<f.frame_origin.x0<<','<<f.frame_origin.y0<<','<<f.is_last<<','<<f.save_as_reference<<','<<f.save_before_color_transform<<','<<f.dc_level<<','<<f.nonserialized_is_preview<<','<<f.extensions;
  std::cout<<' '<<Hex(jxl::Span<const uint8_t>(reinterpret_cast<const uint8_t*>(f.name.data()),f.name.size()));SubsamplingObservation(f.chroma_subsampling);PassesObservation(f.passes);
  std::cout<<' '<<f.extra_channel_upsampling.size();for (auto u : f.extra_channel_upsampling) std::cout<<' '<<u;
  BlendingObservation(f.blending_info);std::cout<<' '<<f.extra_channel_blending_info.size();for (const auto& b:f.extra_channel_blending_info) BlendingObservation(b);
  std::cout<<' '<<f.animation_frame.duration<<','<<f.animation_frame.timecode<<','<<(f.animation_frame.nonserialized_metadata!=nullptr)<<','<<f.CanBeReferenced()<<','<<f.needs_color_transform()<<','<<f.default_xsize()<<','<<f.default_ysize();FilterObservation(f.loop_filter);
  if (dimensions) DimensionsObservation(f.ToFrameDimensions());
}
void ConfigureFrameMetadata(jxl::CodecMetadata& metadata,bool xyb,bool animation,size_t extras) {
  if (!metadata.size.Set(513,257) || !metadata.m.preview_size.Set(32,16)) throw std::runtime_error("frame metadata size");metadata.m.xyb_encoded=xyb;metadata.m.have_animation=animation;metadata.m.animation.have_timecodes=animation;
  metadata.m.extra_channel_info.resize(extras);for (size_t i=0;i<extras;++i) metadata.m.extra_channel_info[i].dim_shift=i%2;
}
bool FrameCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if (command=="frame-dimensions") {
    size_t x,y,g,h,v,u,index;bool modular;in>>x>>y>>g>>h>>v>>modular>>u>>index;jxl::FrameDimensions d;d.Set(x,y,g,h,v,modular,u);std::cout<<"D";DimensionsObservation(d);
    if (d.num_groups) { RectObservation(d.GroupRect(index%d.num_groups));RectObservation(d.BlockGroupRect(index%d.num_groups)); }
    if (d.num_dc_groups) RectObservation(d.DCGroupRect(index%d.num_dc_groups));
  } else if (command=="frame-rect") {
    size_t v[8],border;int64_t dx,dy;in>>dx>>dy>>border;for (auto& x:v) in>>x;
    jxl::Rect a(v[0],v[1],v[2],v[3]),b(v[4],v[5],v[6],v[7]);std::cout<<a.IsInside(b)<<' '<<a.IsSame(b);RectObservation(a.Intersection(b));RectObservation(a.Translate(dx,dy));RectObservation(a.Extend(border,b));
  } else if (command=="frame-signed") {
    int32_t value;size_t packed;in>>value>>packed;std::cout<<jxl::PackSigned(value)<<' '<<jxl::UnpackSigned(packed);
  } else if (command=="frame-subsampling") {
    uint8_t h[3],v[3];for (auto* a : {h,v}) for (size_t i=0;i<3;++i) { unsigned x;in>>x;a[i]=x; }
    jxl::YCbCrChromaSubsampling c;Status status=c.Set(h,v);std::cout<<Code(status);SubsamplingObservation(c);
  } else if (command=="frame-passes") {
    jxl::Passes p;SeedPasses(p);size_t prefix;in>>p.num_passes>>p.num_downsample>>prefix;
    for (auto* a : {p.downsample,p.last_pass,p.shift}) for (size_t i=0;i<jxl::kMaxNumPasses;++i) in>>a[i];
    size_t ext=0,bits=0;Status can=jxl::Bundle::CanEncode(p,&ext,&bits);std::cout<<Code(can)<<' '<<ext<<' '<<bits;
    if(can) { BitWriter writer(mm);if(!Prefix(writer,prefix)) throw std::runtime_error("passes prefix");Status write=jxl::Bundle::Write(p,&writer,LayerType::Header,nullptr);std::cout<<' '<<Code(write)<<' '<<writer.BitsWritten();if(!Pad(writer)) throw std::runtime_error("passes pad");std::cout<<' '<<Hex(writer.GetSpan());
      jxl::Passes d;SeedPasses(d);BitReader br(writer.GetSpan());br.SkipBits(prefix);Status read=jxl::Bundle::Read(&br,&d);std::cout<<' '<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());PassesObservation(d);if(read) PassTargetsObservation(d);
    }
    PassesObservation(p);
  } else if(command=="frame-passes-read") {
    size_t prefix;std::string hex;in>>prefix>>hex;jxl::Passes p;SeedPasses(p);Input input(hex);BitReader br(input.bytes());br.SkipBits(prefix);Status read=jxl::Bundle::Read(&br,&p);std::cout<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());PassesObservation(p);if(read) PassTargetsObservation(p);
  } else if (command=="frame-filter") {
    bool modular;unsigned variant;size_t prefix;in>>modular>>variant>>prefix;jxl::LoopFilter f;f.nonserialized_is_modular=modular;ConfigureFilter(f,variant);
    BitWriter writer(mm);if (!Prefix(writer,prefix)) throw std::runtime_error("filter prefix");size_t ext=0,bits=0;Status can=jxl::Bundle::CanEncode(f,&ext,&bits);std::cout<<Code(can)<<' '<<ext<<' '<<bits;
    if (can) { Status status=jxl::Bundle::Write(f,&writer,LayerType::Header,nullptr);std::cout<<' '<<Code(status)<<' '<<writer.BitsWritten();if (!Pad(writer)) throw std::runtime_error("filter pad");std::cout<<' '<<Hex(writer.GetSpan());
      jxl::LoopFilter decoded;decoded.nonserialized_is_modular=modular;BitReader br(writer.GetSpan());br.SkipBits(prefix);Status read=jxl::Bundle::Read(&br,&decoded);std::cout<<' '<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());FilterObservation(decoded); }
    FilterObservation(f);
  } else if (command=="frame-write") {
    bool xyb,animation,preview,modular,last,custom;unsigned type,transform,group,up,blend,filter;size_t extras,prefix;uint64_t flags;
    in>>xyb>>animation>>extras>>preview>>modular>>type>>flags>>transform>>group>>up>>blend>>filter>>last>>custom>>prefix;
    jxl::CodecMetadata metadata;ConfigureFrameMetadata(metadata,xyb,animation,extras);jxl::FrameHeader f(&metadata);SeedPasses(f.passes);f.nonserialized_is_preview=preview;
    f.encoding=modular?jxl::FrameEncoding::kModular:jxl::FrameEncoding::kVarDCT;f.frame_type=static_cast<jxl::FrameType>(type);f.dc_level=type==1?1:0;f.flags=flags;f.color_transform=static_cast<jxl::ColorTransform>(transform);f.group_size_shift=group;f.upsampling=up;
    for (size_t i=0;i<extras;++i) f.extra_channel_upsampling[i]=std::max<unsigned>(up,1u<<(i%2));
    f.blending_info.mode=static_cast<jxl::BlendMode>(blend);f.blending_info.source=2;f.blending_info.alpha_channel=0;f.blending_info.clamp=true;
    ConfigureFilter(f.loop_filter,filter);f.is_last=last;f.save_as_reference=1;f.custom_size_or_origin=custom;f.name="frame";f.animation_frame.duration=animation?17:0;f.animation_frame.timecode=0x12345678;
    if (custom) { f.frame_size={400,200};if (type==0 || type==3) f.frame_origin={-2,3};if(type==2) f.save_before_color_transform=true; }
    if (transform==2) { const uint8_t h[3]={2,1,1},v[3]={2,1,1};if (!f.chroma_subsampling.Set(h,v)) throw std::runtime_error("frame subsampling"); }
    BitWriter writer(mm);if (!Prefix(writer,prefix)) throw std::runtime_error("frame prefix");jxl::AuxOut aux;size_t ext=0,bits=0;Status can=jxl::Bundle::CanEncode(f,&ext,&bits);std::cout<<Code(can)<<' '<<ext<<' '<<bits;
    if (can) { Status written=jxl::WriteFrameHeader(f,&writer,&aux);std::cout<<' '<<Code(written)<<' '<<writer.BitsWritten();if (!Pad(writer)) throw std::runtime_error("frame pad");std::cout<<' '<<Hex(writer.GetSpan());
      jxl::FrameHeader decoded(&metadata);SeedPasses(decoded.passes);decoded.nonserialized_is_preview=preview;BitReader br(writer.GetSpan());br.SkipBits(prefix);Status read=jxl::ReadFrameHeader(&br,&decoded);std::cout<<' '<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());FrameObservation(decoded,read);
      auto a=aux.layer(LayerType::Header);std::cout<<' '<<a.total_bits<<','<<a.histogram_bits; }
    FrameObservation(f,can);
  } else if (command=="frame-read") {
    bool xyb,animation,preview;size_t extras,prefix;std::string hex;in>>xyb>>animation>>extras>>preview>>prefix>>hex;
    jxl::CodecMetadata metadata;ConfigureFrameMetadata(metadata,xyb,animation,extras);jxl::FrameHeader f(&metadata);SeedPasses(f.passes);f.nonserialized_is_preview=preview;
    Input input(hex);BitReader br(input.bytes());br.SkipBits(prefix);Status read=jxl::ReadFrameHeader(&br,&f);std::cout<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());FrameObservation(f,read);
  } else if (command=="frame-filter-read") {
    bool modular;size_t prefix;std::string hex;in>>modular>>prefix>>hex;jxl::LoopFilter f;f.nonserialized_is_modular=modular;Input input(hex);BitReader br(input.bytes());br.SkipBits(prefix);Status read=jxl::Bundle::Read(&br,&f);std::cout<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());FilterObservation(f);
  } else return false;
  return true;
}
