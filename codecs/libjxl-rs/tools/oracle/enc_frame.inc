// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Include the unmodified source to observe its actual private header helpers.
// This .cc is not separately compiled into the oracle.
#include "lib/jxl/enc_frame.cc"
#include <cstddef>
#include <jxl/cms.h>

jxl::Status EFParamsPostInit(jxl::CompressParams* p) {
  return jxl::ParamsPostInit(p);
}
jxl::Status EFPrepareLosslessThunder(const jxl::CompressParams& p,
                                    jxl::ProgressiveSplitter* splitter,
                                    jxl::FrameHeader* header) {
  jxl::SetProgressiveMode(p, splitter);
  return jxl::MakeFrameHeader(header->nonserialized_metadata->xsize(),
                             header->nonserialized_metadata->ysize(), p,
                             *splitter, jxl::FrameInfo(), nullptr, false, header);
}
jxl::Status EFEncodeThunderGroups(const jxl::FrameHeader& header,jxl::PassesEncoderState* state,
                                  jxl::ModularFrameEncoder* modular,
                                  std::vector<std::unique_ptr<jxl::BitWriter>>* codes,jxl::AuxOut* aux) {
  return jxl::EncodeGroups(header,state,modular,nullptr,codes,aux);
}
jxl::Status EFCopyThunderColorChannels(jxl::JxlEncoderChunkedFrameAdapter& input,
    const jxl::ImageMetadata& metadata,const jxl::Rect& rect,jxl::Image3F* color,bool* alpha) {
  return jxl::CopyColorChannels(input.GetInputSource(),rect,jxl::FrameInfo(),metadata,nullptr,color,nullptr,alpha);
}
jxl::Status EFComputeThunderFrame(const jxl::CompressParams& p,const jxl::CodecMetadata& metadata,
    jxl::JxlEncoderChunkedFrameAdapter& input,jxl::FrameHeader& header,jxl::ModularFrameEncoder& modular,
    jxl::PassesEncoderState& state,std::vector<std::unique_ptr<jxl::BitWriter>>& codes,jxl::AuxOut* aux) {
  JxlCmsInterface cms={};
  return jxl::ComputeEncodingData(p,jxl::FrameInfo(),&metadata,input,nullptr,0,0,input.xsize,input.ysize,cms,nullptr,header,modular,state,&codes,aux);
}
jxl::Status EFThunderOneShot(JxlMemoryManager* mm,const jxl::CompressParams& p,const jxl::CodecMetadata& metadata,
    jxl::JxlEncoderChunkedFrameAdapter& input,std::vector<uint8_t>& bytes,jxl::AuxOut* aux) {
  JxlEncoderOutputProcessorWrapper output(mm);JxlCmsInterface cms={};
  JXL_RETURN_IF_ERROR(jxl::EncodeFrameOneShot(mm,p,jxl::FrameInfo(),&metadata,input,cms,nullptr,&output,aux));
  JXL_RETURN_IF_ERROR(output.SetFinalizedPosition());return output.CopyOutput(bytes);
}

jxl::Status EFVarDctOneShot(JxlMemoryManager* mm,const jxl::CompressParams& p,
    const jxl::CodecMetadata& metadata,jxl::JxlEncoderChunkedFrameAdapter& input,
    std::vector<uint8_t>& bytes) {
  JxlEncoderOutputProcessorWrapper output(mm);
  JXL_RETURN_IF_ERROR(jxl::EncodeFrameOneShot(mm,p,jxl::FrameInfo(),&metadata,input,
      *JxlGetDefaultCms(),nullptr,&output,nullptr));
  JXL_RETURN_IF_ERROR(output.SetFinalizedPosition());return output.CopyOutput(bytes);
}

template<class... T> void EFValues(T... values) { ((std::cout<<' '<<values),...); }
float EFReadFloat(std::istringstream& in) { uint32_t bits;in>>bits;float value;memcpy(&value,&bits,4);return value; }
std::vector<float> EFReadFloats(std::istringstream& in) { size_t n;in>>n;std::vector<float> values(n);for(auto& v:values)v=EFReadFloat(in);return values; }
void EFFloats(const std::vector<float>& values) { EFValues(values.size());for(auto v:values)EFValues(FloatBits(v)); }
jxl::CompressParams EFReadParams(std::istringstream& in) {
  jxl::CompressParams p;p.cms={};p.debug_image_opaque=nullptr; // Do not observe indeterminate caller storage.
  unsigned transform,pred;int tier,progressive,quantized,gab,noise;
  p.butteraugli_distance=EFReadFloat(in);
  in>>p.modular_mode>>transform>>tier>>p.decoding_speed_tier>>p.modular_group_size_shift>>p.resampling>>p.ec_resampling>>p.already_downsampled;
  p.original_butteraugli_distance=EFReadFloat(in);
  in>>progressive>>quantized>>p.responsive>>p.epf>>gab>>noise>>p.disable_perceptual_optimizations;
  p.photon_noise_iso=EFReadFloat(in);in>>p.progressive_dc>>p.lossy_palette>>p.force_lfs_jpeg_recompression>>pred;
  p.color_transform=static_cast<jxl::ColorTransform>(transform);p.speed_tier=static_cast<jxl::SpeedTier>(tier);
  p.progressive_mode=static_cast<jxl::Override>(progressive);p.qprogressive_mode=static_cast<jxl::Override>(quantized);p.gaborish=static_cast<jxl::Override>(gab);p.noise=static_cast<jxl::Override>(noise);
  p.options.predictor=static_cast<jxl::Predictor>(pred);
  p.manual_noise=EFReadFloats(in);p.manual_xyb_factors=EFReadFloats(in);p.ec_distance=EFReadFloats(in);
  return p;
}
void EFHistogram(const jxl::HistogramParams& p) {
  EFValues(unsigned(p.clustering),unsigned(p.uint_method),unsigned(p.lz77_method),unsigned(p.ans_histogram_strategy),p.max_histograms,p.force_huffman,p.initialize_global_state,p.streaming_mode,p.add_missing_symbols,p.add_fixed_histograms,p.image_widths.size());
  for(auto v:p.image_widths) EFValues(v);
}
void EFParams(const jxl::CompressParams& p) {
  EFValues(FloatBits(p.butteraugli_distance));EFFloats(p.ec_distance);
  EFValues(p.max_error_mode);for(auto v:p.max_error)EFValues(FloatBits(v));
  EFValues(p.disable_perceptual_optimizations,int(p.speed_tier),p.brotli_effort,p.decoding_speed_tier,unsigned(p.color_transform),p.modular_mode,p.modular_group_size_shift,
    int(p.preview),int(p.noise),int(p.dots),int(p.patches),int(p.gaborish),p.epf,int(p.progressive_mode),int(p.qprogressive_mode),p.centerfirst,p.center_x,p.center_y,p.progressive_dc,int(p.keep_invisible),p.cms_set,
    p.force_cfl_jpeg_recompression,p.force_lfs_jpeg_recompression,p.jpeg_compress_boxes,p.jpeg_keep_exif,p.jpeg_keep_xmp,p.jpeg_keep_jumbf,FloatBits(p.photon_noise_iso));
  const auto& o=p.options;
  EFValues(o.max_chan_size,o.group_dim,FloatBits(o.nb_repeats),o.max_properties,o.splitting_heuristics_properties.size());for(auto v:o.splitting_heuristics_properties)EFValues(v);
  EFValues(FloatBits(o.splitting_heuristics_node_threshold),o.max_property_values,unsigned(o.predictor),o.wp_mode,FloatBits(o.fast_decode_multiplier),int(o.wp_tree_mode),o.skip_encoder_fast_path,unsigned(o.tree_kind),o.zero_tokens);EFHistogram(o.histogram_params);
  EFValues(p.responsive,p.colorspace,p.move_to_front_from_channel,FloatBits(p.channel_colors_pre_transform_percent),FloatBits(p.channel_colors_percent),p.palette_colors,p.lossy_palette,
    p.resampling,p.ec_resampling,p.already_downsampled,FloatBits(p.original_butteraugli_distance),FloatBits(p.quant_ac_rescale),p.level,p.buffering,p.output_mode,p.use_full_image_heuristics);
  EFFloats(p.manual_noise);EFFloats(p.manual_xyb_factors);
  EFValues(p.custom_fixed_tree.size(),p.custom_splines.HasAny(),p.custom_splines.splines.size(),p.custom_splines.starting_points.size(),p.custom_progressive_mode!=nullptr,p.debug_image!=nullptr);
}
jxl::jpeg::JPEGData EFReadJpeg(std::istringstream& in) {
  size_t n;in>>n;jxl::jpeg::JPEGData jpg;jpg.components.resize(n);for(auto& c:jpg.components)in>>c.id>>c.h_samp_factor>>c.v_samp_factor;
  std::string hex;in>>hex;jpg.marker_order=Unhex(hex);in>>n;jpg.app_data.resize(n);for(auto& d:jpg.app_data){in>>hex;d=Unhex(hex);}return jpg;
}
jxl::ProgressiveMode EFReadMode(std::istringstream& in) {
  jxl::ProgressiveMode mode;in>>mode.num_passes;for(auto& p:mode.passes)in>>p.num_coefficients>>p.shift>>p.suitable_for_downsampling_of_at_least;return mode;
}
void EFSeedPasses(jxl::Passes& p,uint32_t seed) {
  p.num_passes=3;p.num_downsample=2;for(size_t i=0;i<jxl::kMaxNumPasses;++i){p.downsample[i]=seed*1000+100+i;p.last_pass[i]=seed*1000+200+i;p.shift[i]=seed*1000+300+i;}
}
jxl::FrameInfo EFReadInfo(std::istringstream& in) {
  jxl::FrameInfo f;unsigned type,blend;std::string name;
  in>>f.save_before_color_transform>>f.ib_needs_color_transform>>type>>f.dc_level>>f.is_last>>f.is_preview>>f.save_as_reference>>f.source>>f.clamp>>f.alpha_channel>>f.origin.x0>>f.origin.y0>>f.blend>>blend>>f.duration>>f.timecode>>name;
  f.frame_type=static_cast<jxl::FrameType>(type);f.blendmode=static_cast<jxl::BlendMode>(blend);auto bytes=Unhex(name);f.name.assign(bytes.begin(),bytes.end());
  size_t n;in>>n;f.extra_channel_blending_info.resize(n);for(auto& b:f.extra_channel_blending_info){unsigned m;in>>m>>b.alpha_channel>>b.clamp>>b.source;b.mode=static_cast<jxl::BlendMode>(m);}
  return f;
}

bool EncFrameCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if(command=="ef-layout") {
    std::cout<<sizeof(JxlColorEncoding)<<' '<<alignof(JxlColorEncoding)
      <<' '<<offsetof(JxlColorEncoding,color_space)<<' '<<offsetof(JxlColorEncoding,white_point)<<' '<<offsetof(JxlColorEncoding,white_point_xy)
      <<' '<<offsetof(JxlColorEncoding,primaries)<<' '<<offsetof(JxlColorEncoding,primaries_red_xy)<<' '<<offsetof(JxlColorEncoding,primaries_green_xy)<<' '<<offsetof(JxlColorEncoding,primaries_blue_xy)
      <<' '<<offsetof(JxlColorEncoding,transfer_function)<<' '<<offsetof(JxlColorEncoding,gamma)<<' '<<offsetof(JxlColorEncoding,rendering_intent)
      <<' '<<sizeof(JxlColorProfile)<<' '<<alignof(JxlColorProfile)<<' '<<offsetof(JxlColorProfile,icc)<<' '<<offsetof(JxlColorProfile,color_encoding)<<' '<<offsetof(JxlColorProfile,num_channels)
      <<' '<<sizeof(JxlCmsInterface)<<' '<<alignof(JxlCmsInterface)<<' '<<offsetof(JxlCmsInterface,set_fields_data)<<' '<<offsetof(JxlCmsInterface,set_fields_from_icc)
      <<' '<<offsetof(JxlCmsInterface,init_data)<<' '<<offsetof(JxlCmsInterface,init)<<' '<<offsetof(JxlCmsInterface,get_src_buf)<<' '<<offsetof(JxlCmsInterface,get_dst_buf)<<' '<<offsetof(JxlCmsInterface,run)<<' '<<offsetof(JxlCmsInterface,destroy)
      <<' '<<sizeof(JxlBitDepth)<<' '<<alignof(JxlBitDepth)<<' '<<offsetof(JxlBitDepth,type)<<' '<<offsetof(JxlBitDepth,bits_per_sample)<<' '<<offsetof(JxlBitDepth,exponent_bits_per_sample);
  } else if(command=="ef-point") {
    jxl::Spline::Point a(EFReadFloat(in),0),b;a.y=EFReadFloat(in);b.x=EFReadFloat(in);b.y=EFReadFloat(in);std::cout<<(a==b);
  } else if(command=="ef-override") {
    int raw;bool d;in>>raw>>d;std::cout<<jxl::ApplyOverride(static_cast<jxl::Override>(raw),d);EFValues(int(jxl::OverrideFromBool(d)));
  } else if(command=="ef-default") {
    jxl::CompressParams p;std::cout<<"P";EFParams(p);
  } else if(command=="ef-params") {
    bool lossless,post;in>>lossless>>post;auto p=EFReadParams(in);std::cout<<p.IsLossless()<<' '<<p.ModularPartIsLossless();
    if(lossless)p.SetLossless();Status status=post?jxl::ParamsPostInit(&p):Status(true);EFValues(Code(status),p.IsLossless(),p.ModularPartIsLossless());EFParams(p);
  } else if(command=="ef-policy") {
    auto p=EFReadParams(in);bool streaming;size_t n;in>>streaming>>n;std::vector<uint8_t> extra(n);for(auto& v:extra){unsigned x;in>>x;v=x;}
    std::cout<<"H";EFHistogram(jxl::HistogramParams::ForModular(p,extra,streaming));
  } else if(command=="ef-groups") {
    auto p=EFReadParams(in);size_t x,y;bool count;in>>x>>y>>count;std::cout<<jxl::GetGroupSizeShift(x,y,p);if(count)EFValues(jxl::NumGroupsForFrame(x,y,p));
  } else if(command=="ef-progressive") {
    bool select,custom;int progressive,quantized;uint32_t seed;in>>select>>custom>>progressive>>quantized>>seed;auto mode=EFReadMode(in);
    jxl::ProgressiveSplitter splitter;splitter.SetProgressiveMode(mode);jxl::CompressParams p;
    p.progressive_mode=static_cast<jxl::Override>(progressive);p.qprogressive_mode=static_cast<jxl::Override>(quantized);if(custom)p.custom_progressive_mode=&mode;
    if(select)jxl::SetProgressiveMode(p,&splitter);jxl::Passes passes;EFSeedPasses(passes,seed);Status status=splitter.InitPasses(&passes);std::cout<<splitter.GetNumPasses()<<' '<<Code(status);PassesObservation(passes);
  } else if(command=="ef-stream") {
    size_t groups,dc,group,pass,qt,passes;int kind;in>>groups>>dc>>kind>>qt>>group>>pass>>passes;jxl::FrameDimensions dim;dim.num_groups=groups;dim.num_dc_groups=dc;
    jxl::ModularStreamId id{static_cast<jxl::ModularStreamId::Kind>(kind),qt,group,pass};std::cout<<id.ID(dim);
    for(auto s:{jxl::ModularStreamId::Global(),jxl::ModularStreamId::VarDCTDC(group),jxl::ModularStreamId::ModularDC(group),jxl::ModularStreamId::ACMetadata(group),jxl::ModularStreamId::ModularAC(group,pass)})EFValues(s.ID(dim));
    auto quant=jxl::ModularStreamId::QuantTable(qt);EFValues(Code(quant.status()));if(quant.ok())EFValues(std::move(quant).value_().ID(dim));EFValues(jxl::ModularStreamId::Num(dim,passes),unsigned(jxl::kNumQuantTables));
  } else if(command=="ef-jpeg") {
    auto jpg=EFReadJpeg(in);jxl::YCbCrChromaSubsampling cs;const uint8_t h[3]={2,1,1},v[3]={2,1,1};if(!cs.Set(h,v))throw std::runtime_error("JPEG fixture seed");jxl::ColorTransform t=jxl::ColorTransform::kXYB;
    Status chroma=jxl::jpeg::SetChromaSubsamplingFromJpegData(jpg,&cs),color=jxl::jpeg::SetColorTransformFromJpegData(jpg,&t);std::cout<<Code(chroma)<<' '<<Code(color)<<' '<<unsigned(t);SubsamplingObservation(cs);
  } else if(command=="ef-filter") {
    auto p=EFReadParams(in);unsigned encoding,variant;bool streaming;in>>encoding>>variant>>streaming;jxl::CodecMetadata metadata;ConfigureFrameMetadata(metadata,false,false,0);jxl::FrameHeader header(&metadata);header.encoding=static_cast<jxl::FrameEncoding>(encoding);ConfigureFilter(header.loop_filter,variant);
    Status status=jxl::LoopFilterFromParams(p,streaming,&header);std::cout<<Code(status)<<' '<<jxl::FrameFlagsFromParams(p);FilterObservation(header.loop_filter);
  } else if(command=="ef-header") {
    auto p=EFReadParams(in);bool xyb,animation,has_jpeg,streaming,encode;size_t extras,x,y,prefix;unsigned seed;in>>xyb>>animation>>extras>>x>>y>>has_jpeg>>streaming>>encode>>prefix>>seed;
    jxl::CodecMetadata metadata;ConfigureFrameMetadata(metadata,xyb,animation,extras);for(auto& e:metadata.m.extra_channel_info){unsigned type;in>>type;e.type=static_cast<jxl::ExtraChannel>(type);}
    auto info=EFReadInfo(in);jxl::jpeg::JPEGData jpeg;if(has_jpeg)jpeg=EFReadJpeg(in);
    jxl::FrameHeader header(&metadata);SeedPasses(header.passes);ConfigureFilter(header.loop_filter,seed%3);if(seed&1){header.custom_size_or_origin=true;header.frame_size={400,200};header.frame_origin={-2,3};header.flags=128;}
    if(seed&2){const uint8_t h[3]={2,1,1},v[3]={2,1,1};if(!header.chroma_subsampling.Set(h,v))throw std::runtime_error("header fixture seed");}
    jxl::ProgressiveSplitter splitter;jxl::SetProgressiveMode(p,&splitter);Status status=jxl::MakeFrameHeader(x,y,p,splitter,info,has_jpeg?&jpeg:nullptr,streaming,&header);std::cout<<Code(status);FrameObservation(header,false);
    if(status && encode){BitWriter writer(mm);if(!Prefix(writer,prefix))throw std::runtime_error("frame prep prefix");size_t ext=0,bits=0;Status can=jxl::Bundle::CanEncode(header,&ext,&bits);EFValues(Code(can),ext,bits);if(can){Status written=jxl::WriteFrameHeader(header,&writer,nullptr);EFValues(Code(written),writer.BitsWritten());if(!Pad(writer))throw std::runtime_error("frame prep pad");EFValues(Hex(writer.GetSpan()));}FrameObservation(header,false);}
  } else return false;
  return true;
}
