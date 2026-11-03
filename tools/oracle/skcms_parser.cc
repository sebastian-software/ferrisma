// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Data-only observation of actual public functions from pristine jxl_cms.
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include "third_party/skcms/skcms.h"
#include "third_party/skcms/src/skcms_internals.h"

std::vector<uint8_t> Unhex(const std::string&);
namespace {
uint32_t Word(float value) { uint32_t word; std::memcpy(&word,&value,4); return word; }
void Pointer(const uint8_t* p, const uint8_t* base) {
  std::cout << ' ' << (p ? uint64_t(uintptr_t(p)-uintptr_t(base)) : UINT64_MAX);
}
void Function(const skcms_TransferFunction& f) {
  std::cout << ' ' << Word(f.g) << ' ' << Word(f.a) << ' ' << Word(f.b)
            << ' ' << Word(f.c) << ' ' << Word(f.d) << ' ' << Word(f.e) << ' ' << Word(f.f);
}
void Curve(const skcms_Curve& c, const uint8_t* base) {
  std::cout << ' ' << c.table_entries;
  if (c.table_entries == 0) { Function(c.parametric); }
  else {
    Pointer(c.table_8,base); Pointer(c.table_16,base);
    // The observers seed every representation byte before calling source. Keep
    // all retained non-pointer words, including inactive payload/padding words;
    // pointer words are normalized separately to caller-buffer offsets.
    for(size_t offset=4;offset<sizeof(c);offset+=4) {
      if((offset>=offsetof(skcms_Curve,table_8)&&offset<offsetof(skcms_Curve,table_8)+sizeof(c.table_8)) ||
         (offset>=offsetof(skcms_Curve,table_16)&&offset<offsetof(skcms_Curve,table_16)+sizeof(c.table_16)))continue;
      uint32_t word;std::memcpy(&word,reinterpret_cast<const uint8_t*>(&c)+offset,4);std::cout<<' '<<word;
    }
  }
}
template<size_t N> void Curves(const skcms_Curve (&curves)[N],const uint8_t* base) {
  for (const auto& c: curves) Curve(c,base);
}
template<class Matrix> void MatrixWords(const Matrix& matrix) {
  for (const auto& row:matrix.vals) for(float x:row) std::cout << ' ' << Word(x);
}
void Profile(const skcms_ICCProfile& p,const uint8_t* base) {
  std::cout << int(p.has_trc) << ' ' << int(p.has_toXYZD50) << ' ' << int(p.has_A2B)
            << ' ' << int(p.has_B2A) << ' ' << int(p.has_CICP);
  Pointer(p.buffer,base);
  std::cout << ' ' << p.size << ' ' << p.data_color_space << ' ' << p.pcs << ' ' << p.tag_count;
  Curves(p.trc,base); MatrixWords(p.toXYZD50);
  const auto& a=p.A2B;
  Curves(a.input_curves,base); Pointer(a.grid_8,base); Pointer(a.grid_16,base);
  std::cout << ' ' << a.input_channels; for(uint8_t x:a.grid_points) std::cout << ' ' << unsigned(x);
  Curves(a.matrix_curves,base); MatrixWords(a.matrix);
  std::cout << ' ' << a.matrix_channels << ' ' << a.output_channels; Curves(a.output_curves,base);
  const auto& b=p.B2A;
  Curves(b.input_curves,base); std::cout << ' ' << b.input_channels << ' ' << b.matrix_channels;
  Curves(b.matrix_curves,base); MatrixWords(b.matrix); Curves(b.output_curves,base);
  Pointer(b.grid_8,base); Pointer(b.grid_16,base);
  for(uint8_t x:b.grid_points) std::cout << ' ' << unsigned(x);
  std::cout << ' ' << b.output_channels << ' ' << unsigned(p.CICP.color_primaries)
            << ' ' << unsigned(p.CICP.transfer_characteristics)
            << ' ' << unsigned(p.CICP.matrix_coefficients)
            << ' ' << unsigned(p.CICP.video_full_range_flag);
}
void Tag(const skcms_ICCTag& tag,const uint8_t* base) {
  std::cout << ' ' << tag.signature << ' ' << tag.type << ' ' << tag.size; Pointer(tag.buf,base);
}
}

bool SkcmsParserCommand(const std::string& command,std::istringstream& in) {
  if(command.rfind("skcms-profile-",0)!=0) return false;
  if(command=="skcms-profile-layout") {
    std::cout << sizeof(skcms_Curve) << ' ' << alignof(skcms_Curve)
      << ' ' << offsetof(skcms_Curve,parametric) << ' ' << offsetof(skcms_Curve,table_8)
      << ' ' << offsetof(skcms_Curve,table_16) << ' ' << sizeof(skcms_A2B) << ' ' << alignof(skcms_A2B)
      << ' ' << sizeof(skcms_B2A) << ' ' << alignof(skcms_B2A)
      << ' ' << sizeof(skcms_CICP) << ' ' << sizeof(skcms_ICCProfile) << ' ' << alignof(skcms_ICCProfile)
      << ' ' << offsetof(skcms_ICCProfile,trc) << ' ' << offsetof(skcms_ICCProfile,toXYZD50)
      << ' ' << offsetof(skcms_ICCProfile,A2B) << ' ' << offsetof(skcms_ICCProfile,B2A)
      << ' ' << offsetof(skcms_ICCProfile,CICP) << ' ' << offsetof(skcms_ICCProfile,has_trc)
      << ' ' << sizeof(skcms_ICCTag) << ' ' << alignof(skcms_ICCTag) << ' ' << offsetof(skcms_ICCTag,buf);
    return true;
  }
  if(command=="skcms-profile-static") {
    unsigned which; in>>which;
    auto p=which ? skcms_XYZD50_profile() : skcms_sRGB_profile(); Profile(*p,nullptr);
    std::cout << ' ' << (p==(which?skcms_XYZD50_profile():skcms_sRGB_profile()))
              << ' ' << (skcms_sRGB_TransferFunction()==&skcms_sRGB_profile()->trc[0].parametric);
    Function(*skcms_sRGB_TransferFunction()); Function(*skcms_sRGB_Inverse_TransferFunction());
    Function(*skcms_Identity_TransferFunction()); return true;
  }
  if(command=="skcms-profile-short-zero") {
    size_t len;in>>len;if(len>=132)std::abort();
    skcms_ICCProfile value;std::memset(&value,0xa5,sizeof(value));
    std::cout<<int(skcms_Parse(nullptr,len,&value));
    const auto* bytes=reinterpret_cast<const uint8_t*>(&value);
    for(size_t i=0;i<sizeof(value);++i)std::cout<<' '<<unsigned(bytes[i]);return true;
  }
  if(command=="skcms-profile-setters") {
    int source;unsigned self_matrix;in>>source>>self_matrix;
    skcms_ICCProfile value;std::memset(&value,0xa5,sizeof(value));skcms_Init(&value);
    for(unsigned i=0;i<3;++i)value.trc[i].parametric=skcms_TransferFunction{float(i+1),float(i+2),float(i+3),float(i+4),float(i+5),float(i+6),float(i+7)};
    skcms_TransferFunction external={2.4f,0.8f,0.1f,0.2f,0.3f,0.4f,0.5f};
    skcms_SetTransferFunction(&value,source<0?&external:&value.trc[source].parametric);
    skcms_Matrix3x3 m={{{0.125f,0.25f,0.375f},{0.5f,0.625f,0.75f},{0.875f,1.0f,1.125f}}};
    skcms_SetXYZD50(&value,self_matrix?&value.toXYZD50:&m);
    Profile(value,nullptr);return true;
  }
  if(command=="skcms-profile-null-getters") {
    unsigned flags;in>>flags;skcms_ICCProfile value;skcms_Init(&value);
    uint8_t backing[132]={0};if(flags&4)value.buffer=backing;
    auto* p=(flags&1)?nullptr:&value;skcms_ICCTag tag{};tag.signature=0x12345678;tag.type=0xabcdef01;tag.size=0x87654321;
    auto* t=(flags&2)?nullptr:&tag;
    skcms_GetTagByIndex(p,UINT32_MAX,t);Tag(tag,backing);
    std::cout<<' '<<skcms_GetTagBySignature(p,0x12345678,t);Tag(tag,backing);
    skcms_Matrix3x3 m{};float xyz[3]={0.25f,0.5f,0.75f};
    std::cout<<' '<<skcms_GetCHAD(p,&m);MatrixWords(m);
    std::cout<<' '<<skcms_GetWTPT(p,xyz);for(float x:xyz)std::cout<<' '<<Word(x);
    return true;
  }
  std::string hex; int mode, count; in>>mode>>count;
  std::vector<int> priorities;
  for(int i=0;i<count;++i) { int value;in>>value;priorities.push_back(value); }
  in>>hex; auto data=Unhex(hex);
  // Keep logical length exact; these extra physical bytes only support empty
  // vector address formation and are not passed as part of the ICC buffer.
  size_t len=data.size(); if(data.empty())data.resize(1);
  skcms_ICCProfile profile;
  std::memset(&profile,0xa5,sizeof(profile));
  bool ok=mode==0 ? skcms_Parse(data.data(),len,&profile)
         : skcms_ParseWithA2BPriority(data.data(),len,priorities.data(),count,mode==2?nullptr:&profile);
  std::cout << int(ok);
  if(mode==2) return true;
  std::cout << ' '; Profile(profile,data.data());
  if(command=="skcms-profile-parse") return true;
  if(command=="skcms-profile-getters") {
    // Only admit getter reads when the actual parser has validated the tag
    // table/extents; this command is used with complete, successful profiles.
    if(!ok) {std::cerr<<"getter fixture did not parse\n";std::abort();}
    uint32_t index,signature; in>>index>>signature;
    skcms_ICCTag tag{}; tag.signature=0x12345678;tag.type=0xabcdef01;tag.size=0x87654321;
    skcms_GetTagByIndex(&profile,index,&tag); Tag(tag,data.data());
    std::cout << ' ' << skcms_GetTagBySignature(&profile,signature,&tag); Tag(tag,data.data());
    skcms_Matrix3x3 matrix; for(auto& row:matrix.vals) for(float& x:row)x=0.25f;
    std::cout << ' ' << skcms_GetCHAD(&profile,&matrix); MatrixWords(matrix);
    float xyz[3]={0.25f,0.5f,0.75f};
    std::cout << ' ' << skcms_GetWTPT(&profile,xyz); for(float x:xyz)std::cout << ' ' << Word(x);
    std::cout << ' ' << skcms_GetInputChannelCount(&profile); return true;
  }
  std::cerr<<"unknown skcms profile command\n";std::abort();
}
