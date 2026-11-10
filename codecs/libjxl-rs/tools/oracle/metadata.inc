// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Call original bundle and metadata entry points; retain partial read state.
#include "lib/jxl/color_encoding_internal.h"
#include "lib/jxl/image_metadata.h"
#include "lib/jxl/enc_fields.h"

void ColorStorageObservation(const jxl::cms::ColorEncoding& s) {
  std::cout<<' '<<unsigned(s.color_space)<<','<<unsigned(s.white_point)<<','<<unsigned(s.primaries)<<','<<unsigned(s.rendering_intent)
    <<','<<s.have_fields<<','<<s.cmyk<<','<<s.tf.have_gamma<<','<<s.tf.gamma<<','<<unsigned(s.tf.transfer_function);
  for (const auto* xy : {&s.white,&s.red,&s.green,&s.blue}) std::cout<<','<<xy->x<<','<<xy->y;
  std::cout<<' '<<Hex(jxl::Span<const uint8_t>(s.icc));
}
void ColorBundleObservation(const jxl::ColorEncoding& c) {
  std::cout<<' '<<c.all_default<<','<<c.WantICC()<<','<<c.IsSRGB()<<','<<c.IsLinearSRGB();ColorStorageObservation(c.View());
}
void DepthObservation(const jxl::BitDepth& b) { std::cout<<' '<<b.floating_point_sample<<','<<b.bits_per_sample<<','<<b.exponent_bits_per_sample; }
void ExtraObservation(const jxl::ExtraChannelInfo& e) {
  std::cout<<' '<<e.all_default<<','<<unsigned(e.type)<<','<<e.dim_shift<<','<<e.alpha_associated<<','<<e.cfa_channel;
  DepthObservation(e.bit_depth);std::cout<<' '<<Hex(jxl::Span<const uint8_t>(reinterpret_cast<const uint8_t*>(e.name.data()),e.name.size()));
  for (float v : e.spot_color) std::cout<<' '<<FloatBits(v);
}
void ToneObservation(const jxl::ToneMapping& t) { std::cout<<' '<<t.all_default<<','<<FloatBits(t.intensity_target)<<','<<FloatBits(t.min_nits)<<','<<t.relative_to_max_display<<','<<FloatBits(t.linear_below); }
void TransformObservation(const jxl::CustomTransformData& t) {
  std::cout<<' '<<t.all_default<<','<<t.nonserialized_xyb_encoded<<','<<t.custom_weights_mask<<','<<t.opsin_inverse_matrix.all_default;
  for (const auto& row:t.opsin_inverse_matrix.inverse_matrix) for (float v:row) std::cout<<' '<<FloatBits(v);
  for (float v:t.opsin_inverse_matrix.opsin_biases) std::cout<<' '<<FloatBits(v);
  for (float v:t.opsin_inverse_matrix.quant_biases) std::cout<<' '<<FloatBits(v);
  for (float v:t.upsampling2_weights) std::cout<<' '<<FloatBits(v);
  for (float v:t.upsampling4_weights) std::cout<<' '<<FloatBits(v);
  for (float v:t.upsampling8_weights) std::cout<<' '<<FloatBits(v);
}
void MetadataObservation(const jxl::ImageMetadata& m) {
  std::cout<<' '<<m.all_default<<','<<m.modular_16_bit_buffer_sufficient<<','<<m.xyb_encoded<<','<<m.orientation<<','<<m.have_preview<<','<<m.have_animation<<','<<m.have_intrinsic_size<<','<<m.num_extra_channels<<','<<m.extensions<<','<<m.nonserialized_only_parse_basic_info;
  DepthObservation(m.bit_depth);ToneObservation(m.tone_mapping);ColorBundleObservation(m.color_encoding);
  std::cout<<' '<<m.intrinsic_size.xsize()<<','<<m.intrinsic_size.ysize()<<','<<m.preview_size.xsize()<<','<<m.preview_size.ysize()
    <<','<<m.animation.tps_numerator<<','<<m.animation.tps_denominator<<','<<m.animation.num_loops<<','<<m.animation.have_timecodes;
  std::cout<<' '<<m.extra_channel_info.size();for (const auto& e:m.extra_channel_info) ExtraObservation(e);
}
void ConfigureMetadata(std::istringstream& in,jxl::CodecMetadata& metadata) {
  unsigned kind,bits,exponent,orientation,flags,extras,xyb;in>>kind>>bits>>exponent>>orientation>>flags>>extras>>xyb;
  auto& m=metadata.m;m.bit_depth.floating_point_sample=kind;m.bit_depth.bits_per_sample=bits;m.bit_depth.exponent_bits_per_sample=exponent;
  m.orientation=orientation;m.have_preview=flags&1;m.have_animation=flags&2;m.have_intrinsic_size=flags&4;m.xyb_encoded=xyb;
  auto c=ReadColorEncoding(in);if (!m.color_encoding.FromExternal(c)) throw std::runtime_error("metadata color config");
  m.tone_mapping.intensity_target=ReadFloatBits(in);m.tone_mapping.min_nits=ReadFloatBits(in);in>>m.tone_mapping.relative_to_max_display;m.tone_mapping.linear_below=ReadFloatBits(in);
  if (!metadata.size.Set(257,129) || !m.preview_size.Set(32,16) || !m.intrinsic_size.Set(300,150)) throw std::runtime_error("metadata sizes");
  m.animation.tps_numerator=1001;m.animation.tps_denominator=30;m.animation.num_loops=2;m.animation.have_timecodes=true;
  for (size_t i=0;i<extras;++i) {
    unsigned type,b,d,associated;std::string name;in>>type>>b>>d>>associated>>name;
    jxl::ExtraChannelInfo info;info.type=static_cast<jxl::ExtraChannel>(type);info.bit_depth.bits_per_sample=b;info.dim_shift=d;info.alpha_associated=associated;
    auto bytes=Unhex(name);info.name.assign(bytes.begin(),bytes.end());info.spot_color[0]=0.125f;info.spot_color[1]=0.5f;info.spot_color[2]=1.0f;info.spot_color[3]=0.75f;
    m.extra_channel_info.push_back(info);
  }
}
bool MetadataCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if (command=="meta-store") {
    bool have;in>>have;auto external=ReadColorEncoding(in);jxl::cms::ColorEncoding c;c.have_fields=have;c.icc={9,8,7};
    Status status=c.FromExternal(external);std::cout<<Code(status);ColorStorageObservation(c);
  } else if (command=="meta-color-state") {
    auto external=ReadColorEncoding(in);jxl::ColorEncoding c;Status status=c.FromExternal(external);std::cout<<Code(status);ColorBundleObservation(c);
  } else if (command=="meta-color") {
    bool raw;size_t prefix;in>>raw>>prefix;auto external=ReadColorEncoding(in);jxl::ColorEncoding c;
    Status converted=c.FromExternal(external);std::cout<<Code(converted);ColorBundleObservation(c);
    if (raw) c.SetICCRaw({255,0,17});
    BitWriter writer(mm);if (!Prefix(writer,prefix)) throw std::runtime_error("color prefix");
    size_t ext=0,bits=0;Status can=jxl::Bundle::CanEncode(c,&ext,&bits);
    std::cout<<' '<<Code(can)<<' '<<ext<<' '<<bits;
    if (can) {
      Status write=jxl::Bundle::Write(c,&writer,LayerType::Header,nullptr);std::cout<<' '<<Code(write)<<' '<<writer.BitsWritten();
      if (!Pad(writer)) throw std::runtime_error("color padding");std::cout<<' '<<Hex(writer.GetSpan());
      jxl::ColorEncoding decoded;BitReader br(writer.GetSpan());br.SkipBits(prefix);
      Status read=jxl::Bundle::Read(&br,&decoded);std::cout<<' '<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());ColorBundleObservation(decoded);
    }
    ColorBundleObservation(c);
  } else if (command=="meta-transform") {
    bool xyb,alter;unsigned mask;size_t prefix;in>>xyb>>mask>>alter>>prefix;
    jxl::CustomTransformData t;t.nonserialized_xyb_encoded=xyb;t.custom_weights_mask=mask;
    if (alter) { t.opsin_inverse_matrix.inverse_matrix[0][0]=2.25f;t.opsin_inverse_matrix.opsin_biases[0]=0.03125f;t.opsin_inverse_matrix.quant_biases[3]=0.25f;t.upsampling2_weights[0]+=1.f;t.upsampling8_weights[209]-=0.125f; }
    BitWriter writer(mm);if (!Prefix(writer,prefix)) throw std::runtime_error("transform prefix");
    size_t ext=0,bits=0;Status can=jxl::Bundle::CanEncode(t,&ext,&bits);std::cout<<Code(can)<<' '<<ext<<' '<<bits;
    if (can) {
      Status write=jxl::Bundle::Write(t,&writer,LayerType::Header,nullptr);std::cout<<' '<<Code(write)<<' '<<writer.BitsWritten();if (!Pad(writer)) throw std::runtime_error("transform pad");std::cout<<' '<<Hex(writer.GetSpan());
      jxl::CustomTransformData decoded;decoded.nonserialized_xyb_encoded=xyb;BitReader br(writer.GetSpan());br.SkipBits(prefix);Status read=jxl::Bundle::Read(&br,&decoded);
      std::cout<<' '<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());TransformObservation(decoded);
    }TransformObservation(t);
  } else if (command=="meta-depth") {
    jxl::BitDepth depth;bool fp;in>>fp>>depth.bits_per_sample>>depth.exponent_bits_per_sample;depth.floating_point_sample=fp;
    size_t prefix;in>>prefix;BitWriter writer(mm);if (!Prefix(writer,prefix)) throw std::runtime_error("depth prefix");
    size_t ext=0,bits=0;Status can=jxl::Bundle::CanEncode(depth,&ext,&bits);std::cout<<Code(can)<<' '<<ext<<' '<<bits;DepthObservation(depth);
    if (can) {
      Status write=jxl::Bundle::Write(depth,&writer,LayerType::Header,nullptr);std::cout<<' '<<Code(write)<<' '<<writer.BitsWritten();if (!Pad(writer)) throw std::runtime_error("depth pad");std::cout<<' '<<Hex(writer.GetSpan());
      jxl::BitDepth decoded;BitReader br(writer.GetSpan());br.SkipBits(prefix);Status read=jxl::Bundle::Read(&br,&decoded);
      std::cout<<' '<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());DepthObservation(decoded);
    }
  } else if (command=="meta-write") {
    bool full;size_t prefix;in>>full>>prefix;jxl::CodecMetadata metadata;ConfigureMetadata(in,metadata);
    BitWriter writer(mm);if (!Prefix(writer,prefix)) throw std::runtime_error("metadata prefix");jxl::AuxOut aux;
    size_t ext=0,bits=0;Status can=jxl::Bundle::CanEncode(metadata.m,&ext,&bits);std::cout<<Code(can)<<' '<<ext<<' '<<bits;
    if (can) {
      Status write=full ? jxl::WriteCodestreamHeaders(&metadata,&writer,&aux) : jxl::WriteImageMetadata(metadata.m,&writer,LayerType::Header,&aux);
      std::cout<<' '<<Code(write)<<' '<<writer.BitsWritten();if (!Pad(writer)) throw std::runtime_error("metadata pad");std::cout<<' '<<Hex(writer.GetSpan());
      const auto& a=aux.layer(LayerType::Header);std::cout<<' '<<a.total_bits<<','<<a.histogram_bits;
    }
    MetadataObservation(metadata.m);if (full) TransformObservation(metadata.transform_data);
  } else if (command=="meta-read") {
    std::string kind,hex;size_t prefix;bool basic;in>>kind>>prefix>>basic>>hex;Input input(hex);BitReader br(input.bytes());br.SkipBits(prefix);
    if (kind=="color") {
      jxl::ColorEncoding c;Status status=jxl::Bundle::Read(&br,&c);std::cout<<Code(status)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());ColorBundleObservation(c);
    } else if (kind=="transform") {
      jxl::CustomTransformData t;t.nonserialized_xyb_encoded=basic;Status status=jxl::Bundle::Read(&br,&t);std::cout<<Code(status)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());TransformObservation(t);
    } else if (kind=="depth") {
      jxl::BitDepth d;Status status=jxl::Bundle::Read(&br,&d);std::cout<<Code(status)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());DepthObservation(d);
    } else if (kind=="image" || kind=="reused") {
      jxl::ImageMetadata m;m.nonserialized_only_parse_basic_info=basic;
      if (kind=="reused") { jxl::ExtraChannelInfo info;info.type=jxl::ExtraChannel::kDepth;info.name="old";m.extra_channel_info.push_back(info);m.extra_channel_info.push_back(info);m.orientation=8;m.have_animation=true; }
      Status status=jxl::ReadImageMetadata(&br,&m);std::cout<<Code(status)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());MetadataObservation(m);
    } else throw std::runtime_error("metadata read kind");
  } else return false;
  return true;
}
