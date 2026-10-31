// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Original header implementations execute all math, formatting, and ICC work.
#include "lib/jxl/cms/jxl_cms_internal.h"

uint64_t DoubleBits(double value) { uint64_t bits;std::memcpy(&bits,&value,8);return bits; }
double ReadDoubleBits(std::istringstream& in) { uint64_t bits;in>>bits;double value;std::memcpy(&value,&bits,8);return value; }
float ReadFloatBits(std::istringstream& in) { uint32_t bits;in>>bits;float value;std::memcpy(&value,&bits,4);return value; }
JxlColorEncoding ReadColorEncoding(std::istringstream& in) {
  JxlColorEncoding c{};unsigned cs,wp,p,tf,ri;
  in>>cs>>wp>>p>>tf>>ri;
  c.color_space=static_cast<JxlColorSpace>(cs);c.white_point=static_cast<JxlWhitePoint>(wp);
  c.primaries=static_cast<JxlPrimaries>(p);c.transfer_function=static_cast<JxlTransferFunction>(tf);
  c.rendering_intent=static_cast<JxlRenderingIntent>(ri);
  c.gamma=ReadDoubleBits(in);
  for (double* xy : {c.white_point_xy,c.primaries_red_xy,c.primaries_green_xy,c.primaries_blue_xy})
    for (size_t i=0;i<2;++i) xy[i]=ReadDoubleBits(in);
  return c;
}
template<typename T> void MatrixCommand(std::istringstream& in) {
  std::array<std::array<T,3>,3> a,b,c;
  std::array<T,3> v,w;
  for (auto* m : {&a,&b}) for (auto& row : *m) for (auto& val : row) val=ReadDoubleBits(in);
  for (auto& val : v) val=ReadDoubleBits(in);
  jxl::Mul3x3Matrix(a,b,c);jxl::Mul3x3Vector(a,v,w);
  for (const auto& row : c) for (T val : row) std::cout<<DoubleBits(val)<<' ';
  for (T val : w) std::cout<<DoubleBits(val)<<' ';
  Status status=jxl::Inv3x3Matrix(a);std::cout<<Code(status);
  for (const auto& row : a) for (T val : row) std::cout<<' '<<DoubleBits(val);
}
bool CmsCommand(const std::string& command,std::istringstream& in) {
  if (command=="cms-format") {
    std::cout<<jxl::ToString(ReadDoubleBits(in));
  } else if (command=="cms-md5") {
    std::string hex;in>>hex;auto bytes=Unhex(hex);uint8_t out[16];
    jxl::detail::ICCComputeMD5(bytes,out);std::cout<<Hex(jxl::Span<const uint8_t>(out,16));
  } else if (command=="cms-matrix") {
    unsigned type;in>>type;if (type==32) MatrixCommand<float>(in);else MatrixCommand<double>(in);
  } else if (command=="cms-transfer") {
    unsigned which;in>>which;float target=ReadFloatBits(in);double x=ReadDoubleBits(in),out;
    if (which==0) out=jxl::TF_HLG_Base::DisplayFromEncoded(x);
    else if (which==1) out=jxl::TF_HLG_Base::EncodedFromDisplay(x);
    else if (which==2) out=jxl::TF_PQ_Base::DisplayFromEncoded(target,x);
    else out=jxl::TF_PQ_Base::EncodedFromDisplay(target,x);
    std::cout<<DoubleBits(out);
  } else if (command=="cms-tone") {
    unsigned which;in>>which;jxl::Range source,target;jxl::Vector3 luminances;jxl::Color rgb;
    for (auto* range : {&source,&target}) for (auto& val : *range) val=ReadFloatBits(in);
    for (auto* color : {&luminances,&rgb}) for (auto& val : *color) val=ReadFloatBits(in);
    float saturation=ReadFloatBits(in);
    if (which==0) jxl::Rec2408ToneMapperBase(source,target,luminances).ToneMap(rgb);
    else if (which==1) jxl::HlgOOTF_Base(source[1],target[1],luminances).Apply(rgb);
    else jxl::GamutMapScalar(rgb,luminances,saturation);
    for (size_t i=0;i<3;++i) { if (i) std::cout<<' ';std::cout<<FloatBits(rgb[i]); }
  } else if (command=="cms-profile") {
    auto c=ReadColorEncoding(in);std::vector<uint8_t> icc={21,37,42};
    auto status=jxl::MaybeCreateProfile(c,&icc);
    std::cout<<Code(status)<<' '<<jxl::ColorEncodingDescription(c,true)<<' '
             <<jxl::ColorEncodingDescription(c,false)<<' '<<Hex(jxl::Span<const uint8_t>(icc));
  } else if (command=="cms-color-matrix") {
    auto c=ReadColorEncoding(in);jxl::Matrix3x3 m{};unsigned which;in>>which;
    Status status=which==0 ? jxl::PrimariesToXYZ(c.primaries_red_xy[0],c.primaries_red_xy[1],c.primaries_green_xy[0],c.primaries_green_xy[1],c.primaries_blue_xy[0],c.primaries_blue_xy[1],c.white_point_xy[0],c.white_point_xy[1],m)
      : jxl::PrimariesToXYZD50(c.primaries_red_xy[0],c.primaries_red_xy[1],c.primaries_green_xy[0],c.primaries_green_xy[1],c.primaries_blue_xy[0],c.primaries_blue_xy[1],c.white_point_xy[0],c.white_point_xy[1],m);
    std::cout<<Code(status);for (auto& row : m) for (float value : row) std::cout<<' '<<FloatBits(value);
  } else if (command=="cms-curve") {
    bool pq,tone;in>>pq>>tone;
    auto table=pq ? jxl::CreateTableCurve<64,jxl::ExtraTF::kPQ>(tone) : jxl::CreateTableCurve<64,jxl::ExtraTF::kHLG>(tone);
    std::cout<<table.size();for (auto value : table) std::cout<<' '<<value;
  } else return false;
  return true;
}
