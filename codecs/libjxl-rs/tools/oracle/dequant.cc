// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Protocol framing around the pristine scalar quantizer header and exactly
// extracted DequantLane/DequantBlock templates; no numeric algorithm lives here.
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include "lib/jxl/dec_transforms-inl.h"
#include "lib/jxl/dct_util.h"
#include "lib/jxl/quantizer-inl.h"
#include "lib/jxl/quantizer.h"
#include "aligned_vector.h"

namespace jxl { namespace ORACLE_HWY_NS { namespace OracleDequant {
using D = HWY_FULL(float);
using DI = HWY_FULL(int32_t);
using DI16 = hwy::ORACLE_HWY_NS::Rebind<int16_t, DI>;
constexpr D d;
constexpr DI di;
constexpr DI16 di16;
#include "original_dequant_block.inc"
}}}
namespace {
float DQFloat(std::istringstream& in) {
  uint32_t bits; in >> bits; float v; std::memcpy(&v, &bits, 4); return v;
}
template <class V>
void DQWords(const V& values) {
  std::cout << " B";
  for (float v : values) { uint32_t bits; std::memcpy(&bits, &v, 4); std::cout << ' ' << bits; }
}
}
bool DequantCommand(const std::string& command, std::istringstream& in, JxlMemoryManager* mm) {
  if (command == "dq-bias") {
    size_t c, n; in >> c >> n; float biases[4]; for(float& b : biases) b=DQFloat(in);
    std::cout << "B";
    for(size_t i=0;i<n;++i) { int32_t v; in >> v; auto result=jxl::ORACLE_HWY_NS::AdjustQuantBias(jxl::ORACLE_HWY_NS::OracleDequant::di,c,hwy::ORACLE_HWY_NS::Set(jxl::ORACLE_HWY_NS::OracleDequant::di,v),biases); uint32_t bits; float f=hwy::ORACLE_HWY_NS::GetLane(result); std::memcpy(&bits,&f,4); std::cout << ' ' << bits; }
    return true;
  }
  if(command!="dq-lane" && command!="dq-block")return false;
  unsigned raw, type; size_t size; in >> raw >> type >> size;
  float inv=DQFloat(in); int quant; in >> quant;
  float xdm=DQFloat(in),bdm=DQFloat(in),xcc=DQFloat(in),bcc=DQFloat(in);
  float biases[4];for(float& b:biases)b=DQFloat(in);
  auto acs=jxl::AcStrategy::FromRawStrategy(static_cast<uint8_t>(raw));
  OracleAlignedVector<float> matrix(size*3);if(command=="dq-lane")for(float& v:matrix)v=DQFloat(in);
  // vardct kernels: qblock, block and scratch aligned like libjxl's
  // coefficient storage and group cache (aligned Load/Store on AVX2).
  OracleAlignedVector<int32_t> q32(size*3);OracleAlignedVector<int16_t> q16(size*3);
  jxl::ACPtr ptr[3];for(size_t c=0;c<3;++c){for(size_t k=0;k<size;++k){int32_t v;in>>v;q32[c*size+k]=v;q16[c*size+k]=static_cast<int16_t>(v);}if(type==0)ptr[c].ptr16=q16.data()+c*size;else ptr[c].ptr32=q32.data()+c*size;}
  OracleAlignedVector<float> block(size*3+4,-123.25f),scratch(size*4+260,-123.25f);
  if(command=="dq-lane") {
    float scale=inv/quant;auto sx=hwy::ORACLE_HWY_NS::Set(jxl::ORACLE_HWY_NS::OracleDequant::d,scale*xdm),sy=hwy::ORACLE_HWY_NS::Set(jxl::ORACLE_HWY_NS::OracleDequant::d,scale),sb=hwy::ORACLE_HWY_NS::Set(jxl::ORACLE_HWY_NS::OracleDequant::d,scale*bdm);
    for(size_t k=0;k<size;++k){if(type==0)jxl::ORACLE_HWY_NS::OracleDequant::DequantLane<jxl::ACType::k16>(sx,sy,sb,matrix.data(),size,k,hwy::ORACLE_HWY_NS::Set(jxl::ORACLE_HWY_NS::OracleDequant::d,xcc),hwy::ORACLE_HWY_NS::Set(jxl::ORACLE_HWY_NS::OracleDequant::d,bcc),biases,ptr,block.data());else jxl::ORACLE_HWY_NS::OracleDequant::DequantLane<jxl::ACType::k32>(sx,sy,sb,matrix.data(),size,k,hwy::ORACLE_HWY_NS::Set(jxl::ORACLE_HWY_NS::OracleDequant::d,xcc),hwy::ORACLE_HWY_NS::Set(jxl::ORACLE_HWY_NS::OracleDequant::d,bcc),biases,ptr,block.data());}
  } else {
    size_t dc_stride;in>>dc_stride;size_t sbx[3];for(auto& x:sbx)in>>x;
    std::vector<float> dc[3];const float* rows[3];for(size_t c=0;c<3;++c){dc[c].resize(acs.covered_blocks_y()*dc_stride+4);for(auto& v:dc[c])v=DQFloat(in);rows[c]=dc[c].data();}
    jxl::DequantMatrices matrices;if(!matrices.EnsureComputed(mm,1u<<raw))throw std::runtime_error("dequant matrices");jxl::Quantizer q(matrices);
    if(type==0)jxl::ORACLE_HWY_NS::OracleDequant::DequantBlock<jxl::ACType::k16>(inv,quant,xdm,bdm,hwy::ORACLE_HWY_NS::Set(jxl::ORACLE_HWY_NS::OracleDequant::d,xcc),hwy::ORACLE_HWY_NS::Set(jxl::ORACLE_HWY_NS::OracleDequant::d,bcc),acs.Strategy(),size,q,acs.covered_blocks_x()*acs.covered_blocks_y(),sbx,rows,dc_stride,biases,ptr,block.data(),scratch.data());
    else jxl::ORACLE_HWY_NS::OracleDequant::DequantBlock<jxl::ACType::k32>(inv,quant,xdm,bdm,hwy::ORACLE_HWY_NS::Set(jxl::ORACLE_HWY_NS::OracleDequant::d,xcc),hwy::ORACLE_HWY_NS::Set(jxl::ORACLE_HWY_NS::OracleDequant::d,bcc),acs.Strategy(),size,q,acs.covered_blocks_x()*acs.covered_blocks_y(),sbx,rows,dc_stride,biases,ptr,block.data(),scratch.data());
  }
  std::cout << "O";DQWords(block);DQWords(scratch);return true;
}
