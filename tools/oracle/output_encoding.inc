// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observe original output-encoding setters, including their partial state.
#include "lib/jxl/dec_xyb.h"
#include "lib/jxl/opsin_params.h"
JxlColorEncoding ReadColorEncoding(std::istringstream& in);
namespace jxl {bool CanOutputToColorEncoding(const ColorEncoding&);}
void OEIOpsin(const jxl::OpsinParams& p){for(auto [v,n]:{std::pair<const float*,size_t>{p.inverse_opsin_matrix,36},{p.opsin_biases,4},{p.opsin_biases_cbrt,4},{p.quant_biases,4}})for(size_t i=0;i<n;++i)std::cout<<' '<<SCBits<uint32_t>(v[i]);}
void OEIObservation(const jxl::OutputEncodingInfo& s){
  ColorBundleObservation(s.orig_color_encoding);std::cout<<' '<<SCBits<uint32_t>(s.orig_intensity_target);for(const auto& r:s.orig_inverse_matrix)for(float v:r)std::cout<<' '<<SCBits<uint32_t>(v);
  std::cout<<' '<<s.default_transform<<' '<<s.xyb_encoded;ColorBundleObservation(s.color_encoding);ColorBundleObservation(s.linear_color_encoding);
  std::cout<<' '<<s.color_encoding_is_original;OEIOpsin(s.opsin_params);std::cout<<' '<<s.all_default_opsin<<' '<<SCBits<uint32_t>(s.inverse_gamma);for(float v:s.luminances)std::cout<<' '<<SCBits<uint32_t>(v);std::cout<<' '<<SCBits<uint32_t>(s.desired_intensity_target)<<' '<<s.cms_set;
}
void OEICustom(jxl::OpsinInverseMatrix& m,unsigned custom){
  if(custom==0)return;m.all_default=false;for(size_t y=0;y<3;++y)for(size_t x=0;x<3;++x)m.inverse_matrix[y][x]=custom==2?0.0f:float(int(y*3+x)-4)/8.0f;
  m.opsin_biases[0]=-0.125f;m.opsin_biases[1]=-0.0f;m.opsin_biases[2]=custom==3?SCBits<float>(uint32_t{0x7fa12345}):1.0f;
  for(size_t i=0;i<4;++i)m.quant_biases[i]=float(i)/8.0f;
}
bool OutputEncodingCommand(const std::string& command,std::istringstream& in){
  if(command=="oei-sequence"){
    size_t n;in>>n;jxl::OutputEncodingInfo info{};for(size_t i=0;i<n;++i){bool xyb,raw;uint32_t target;unsigned custom;in>>xyb>>target>>custom>>raw;
      jxl::CodecMetadata metadata;auto converted=metadata.m.color_encoding.FromExternal(ReadColorEncoding(in));std::cout<<" E "<<int(converted.code());if(raw)metadata.m.color_encoding.SetICCRaw({9,8,7});metadata.m.xyb_encoded=xyb;metadata.m.tone_mapping.intensity_target=SCBits<float>(target);OEICustom(metadata.transform_data.opsin_inverse_matrix,custom);
      auto status=info.SetFromMetadata(metadata);std::cout<<" M "<<int(status.code());OEIObservation(info);
    }return true;
  }
  if(command=="oei-opsin") {uint32_t target;in>>target;jxl::OpsinParams p{};p.Init(SCBits<float>(target));OEIOpsin(p);return true;}
  if(command!="oei-meta")return false;
  bool xyb,raw;uint32_t target;unsigned custom;size_t repeats,n;in>>xyb>>target>>custom>>raw>>repeats>>n;
  jxl::CodecMetadata metadata;auto external=ReadColorEncoding(in);auto converted=metadata.m.color_encoding.FromExternal(external);std::cout<<" E "<<int(converted.code());if(!converted)return true;
  if(raw)metadata.m.color_encoding.SetICCRaw({9,8,7});metadata.m.xyb_encoded=xyb;metadata.m.tone_mapping.intensity_target=SCBits<float>(target);OEICustom(metadata.transform_data.opsin_inverse_matrix,custom);
  // Value-initialize aggregate scalars explicitly; constructor-indeterminate
  // source members receive the same fixture seeds as safe native storage.
  jxl::OutputEncodingInfo info{};
  for(size_t i=0;i<repeats;++i){auto s=info.SetFromMetadata(metadata);std::cout<<" M "<<int(s.code());OEIObservation(info);}
  for(size_t i=0;i<n;++i){bool raw;in>>raw;jxl::ColorEncoding desired;auto e=ReadColorEncoding(in);auto converted=desired.FromExternal(e);std::cout<<" E "<<int(converted.code());if(raw)desired.SetICCRaw({9,8,7});std::cout<<" C "<<jxl::CanOutputToColorEncoding(desired);auto status=info.MaybeSetColorEncoding(desired);std::cout<<" S "<<int(status.code());OEIObservation(info);}
  return true;
}
