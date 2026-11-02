// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Protocol construction/observation only. Both scheduler classes and the
// AC decoder template are exact pinned source fragments, without replacement.
#include <algorithm>
#include <array>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <memory>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include "lib/jxl/ac_context.h"
#include "lib/jxl/ac_strategy.h"
#include "lib/jxl/base/bits.h"
#include "lib/jxl/base/printf_macros.h"
#include "lib/jxl/coeff_order.h"
#include "lib/jxl/dec_ans.h"
#include "lib/jxl/dec_bit_reader.h"
#include "lib/jxl/dec_cache.h"
#include "lib/jxl/dct_util.h"
#include "lib/jxl/enc_ans.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/enc_bit_writer.h"
#include "lib/jxl/entropy_coder.h"
#include "lib/jxl/frame_dimensions.h"
#include "lib/jxl/frame_header.h"
#include "lib/jxl/image_metadata.h"

int Code(jxl::Status);
std::string Hex(jxl::Span<const uint8_t>);
std::vector<uint8_t> Unhex(const std::string&);
jxl::Status Prefix(jxl::BitWriter&, size_t);
jxl::Status Pad(jxl::BitWriter&);
namespace jxl { namespace {
#include "original_get_block.inc"
#include "original_decode_ac_var_block.inc"
#include "original_get_block_from_bitstream.inc"
} }
namespace {
void GSCFixture(std::istringstream& in, JxlMemoryManager* mm) {
  bool prefix; size_t lz, offset, histograms, selected, count;
  in >> prefix >> lz >> offset >> histograms >> selected >> count;
  std::vector<std::vector<jxl::Token>> tokens(1);
  for (size_t i=0; i<count; ++i) { uint32_t value; in>>value; tokens[0].emplace_back(0,value); }
  jxl::HistogramParams params;
  params.force_huffman=prefix; params.initialize_global_state=false;
  params.lz77_method=static_cast<jxl::HistogramParams::LZ77Method>(lz);
  params.clustering=jxl::HistogramParams::ClusteringType::kFastest;
  params.uint_method=jxl::HistogramParams::HybridUintMethod::kNone;
  jxl::EntropyEncodingData codes; codes.use_prefix_code=prefix;
  jxl::BitWriter writer(mm);
  if (!Prefix(writer,offset)) throw std::runtime_error("group fixture prefix");
  const size_t contexts=histograms*jxl::BlockCtxMap().NumACContexts()+16;
  auto cost=jxl::BuildAndEncodeHistograms(mm,params,contexts,tokens,&codes,&writer,jxl::LayerType::Ac,nullptr);
  if (!cost.ok()) throw std::runtime_error("group fixture histogram");
  size_t selector_bits=jxl::CeilLog2Nonzero(histograms);
  if(!writer.WithMaxBits(selector_bits,jxl::LayerType::Ac,nullptr,[&]()->jxl::Status { if(selector_bits)writer.Write(selector_bits,selected); return true; }))throw std::runtime_error("group fixture selector");
  if (!jxl::WriteTokens(tokens[0],codes,0,&writer,jxl::LayerType::Ac,nullptr)) throw std::runtime_error("group fixture tokens");
  size_t bits=writer.BitsWritten(); if(!Pad(writer))throw std::runtime_error("group fixture pad");
  std::cout<<bits<<' '<<Hex(writer.GetSpan())<<' '<<codes.lz77.enabled<<' '<<codes.use_prefix_code;
}
void GSCCache(std::istringstream& in, JxlMemoryManager* mm) {
  size_t count; in>>count; jxl::GroupDecCache cache;
  for(size_t i=0;i<count;++i){
    size_t passes,used;in>>passes>>used; auto status=cache.InitOnce(mm,passes,used);
    std::cout<<Code(status)<<' '<<cache.max_block_area_<<' '
      <<bool(cache.dec_group_block)<<' '<<bool(cache.dec_group_qblock)<<' '
      <<bool(cache.dec_group_qblock16)<<' '<<bool(cache.scratch_space);
    if(cache.dec_group_block)std::cout<<' '<<(cache.scratch_space-cache.dec_group_block);
    else std::cout<<" 0";
    for(const auto& im:cache.num_nzeroes)std::cout<<' '<<im.xsize()<<' '<<im.ysize()<<' '<<im.PixelsPerRow();
    std::cout<<" ; ";
  }
}
template<class T> void GSCCoeffs(const std::vector<T>& block,int32_t seed) {
  size_t changed=0;for(size_t i=0;i<block.size();++i)if(block[i]!=static_cast<T>(seed+int32_t(i%7)))++changed;
  std::cout<<' '<<block.size()<<' '<<changed;
  for(size_t i=0;i<block.size();++i)if(block[i]!=static_cast<T>(seed+int32_t(i%7)))std::cout<<' '<<i<<' '<<block[i];
}
void GSCDecode(std::istringstream& in,JxlMemoryManager* mm) {
  size_t width,raw,first,passes,sampling,order_mode,x0,y0,seed,calls,histograms;
  in>>width>>raw>>first>>passes>>sampling>>order_mode>>x0>>y0>>seed>>calls>>histograms;
  jxl::FrameHeader header(nullptr);
  const uint8_t hs[3]={uint8_t(sampling?2:1),1,1},vs[3]={uint8_t(sampling==2?2:1),1,1};
  if(!header.chroma_subsampling.Set(hs,vs))throw std::runtime_error("group sampling");
  header.passes.num_passes=first+passes;
  for(size_t p=0;p<first+passes;++p)in>>header.passes.shift[p];
  jxl::PassesDecoderState state(mm);auto& shared=state.shared_storage;
  shared.num_histograms=histograms;
  shared.coeff_order_size=jxl::kCoeffOrderMaxSize;
  shared.coeff_orders.resize((first+passes)*shared.coeff_order_size);
  auto acs=jxl::AcStrategy::FromRawStrategy(raw);
  size_t size=jxl::kDCTBlockSize<<acs.log2_covered_blocks();
  const size_t covered=size/jxl::kDCTBlockSize;
  for(size_t p=0;p<first+passes;++p)for(size_t c=0;c<3;++c){
    auto* order=shared.coeff_orders.data()+p*shared.coeff_order_size+jxl::kCoeffOrderOffset[3*jxl::kStrategyOrder[raw]+c]*jxl::kDCTBlockSize;
    acs.ComputeNaturalCoeffOrder(order);
    if(order_mode && p%2)std::reverse(order+covered,order+size);
  }
  {auto image=jxl::ImageI::Create(mm,x0+32,y0+32);if(!image.ok())throw std::runtime_error("group qf allocation");shared.raw_quant_field=std::move(image).value_();}
  {auto image=jxl::ImageB::Create(mm,x0+32,y0+32);if(!image.ok())throw std::runtime_error("group quant dc allocation");shared.quant_dc=std::move(image).value_();}
  for(size_t y=0;y<y0+32;++y)for(size_t x=0;x<x0+32;++x){shared.raw_quant_field.Row(y)[x]=1;shared.quant_dc.Row(y)[x]=0;}
  state.code.resize(first+passes);state.context_map.resize(first+passes);
  std::vector<std::vector<uint8_t>> backing(passes);
  std::vector<std::unique_ptr<jxl::BitReader>> readers(passes);
  std::vector<jxl::BitReader*> ptrs(passes);
  bool hist_ok=true;
  for(size_t p=0;p<passes;++p){
    std::string hex;size_t offset;in>>hex>>offset;auto bytes=Unhex(hex);
    backing[p].resize(bytes.size()+16);std::copy(bytes.begin(),bytes.end(),backing[p].begin()+8);
    readers[p]=jxl::make_unique<jxl::BitReader>(jxl::Bytes(backing[p].data()+8,bytes.size()));ptrs[p]=readers[p].get();
    readers[p]->SkipBits(offset);
    auto status=jxl::DecodeHistograms(mm,ptrs[p],histograms*shared.block_ctx_map.NumACContexts()+16,&state.code[p+first],&state.context_map[p+first],false);
    std::cout<<Code(status)<<' '<<readers[p]->TotalBitsConsumed()<<" ; ";hist_ok=hist_ok&&bool(status);
  }
  jxl::GroupDecCache cache;if(!cache.InitOnce(mm,passes,size_t{1}<<raw))throw std::runtime_error("group cache");
  for(size_t p=0;p<passes;++p)for(size_t c=0;c<3;++c)for(size_t y=0;y<32;++y)for(size_t x=0;x<32;++x)cache.num_nzeroes[p].PlaneRow(c,y)[x]=(p*7+c*13+y*5+x+seed)%64;
  std::array<std::vector<int16_t>,3>b16;std::array<std::vector<int32_t>,3>b32;
  for(size_t c=0;c<3;++c){b16[c].resize(size);b32[c].resize(size);for(size_t i=0;i<size;++i){b16[c][i]=int32_t(seed)+int32_t(i%7);b32[c][i]=int32_t(seed)+int32_t(i%7);}}
  jxl::GetBlockFromBitstream get;
  auto status=hist_ok?get.Init(header,ptrs.data(),passes,0,jxl::CeilLog2Nonzero(histograms),jxl::Rect(x0,y0,32,32),&cache,&state,first):JXL_FAILURE("fixture histograms");
  std::cout<<Code(status)<<" I";
  for(size_t i=0;i<calls;++i){size_t start,bx,by;in>>start>>bx>>by;if(status){get.StartRow(start);jxl::ACPtr block[3];for(size_t c=0;c<3;++c)if(width==16)block[c].ptr16=b16[c].data();else block[c].ptr32=b32[c].data();status=get.LoadBlock(bx,by,acs,size,acs.log2_covered_blocks(),block,width==16?jxl::ACType::k16:jxl::ACType::k32);}}
  std::cout<<' '<<Code(status);
  if(status){bool final=true;for(size_t p=0;p<passes;++p)final=final&&get.decoders[p].CheckANSFinalState();std::cout<<' '<<final;}else std::cout<<" 0";
  for(size_t c=0;c<3;++c)if(width==16)GSCCoeffs(b16[c],seed);else GSCCoeffs(b32[c],seed);
  std::cout<<" N";
  for(size_t p=0;p<passes;++p)for(size_t c=0;c<3;++c){size_t count=0;for(size_t y=0;y<32;++y)for(size_t x=0;x<32;++x)if(cache.num_nzeroes[p].PlaneRow(c,y)[x]!=int32_t((p*7+c*13+y*5+x+seed)%64))++count;std::cout<<' '<<count;for(size_t y=0;y<32;++y)for(size_t x=0;x<32;++x){auto v=cache.num_nzeroes[p].PlaneRow(c,y)[x];if(v!=int32_t((p*7+c*13+y*5+x+seed)%64))std::cout<<' '<<y*32+x<<' '<<v;}}
  std::cout<<" R";for(size_t p=0;p<passes;++p)std::cout<<' '<<readers[p]->TotalBitsConsumed()<<' '<<Code(readers[p]->Close());
}
}
bool GroupScheduleCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm){
  if(command=="gsc-fixture")GSCFixture(in,mm);
  else if(command=="gsc-cache")GSCCache(in,mm);
  else if(command=="gsc-decode")GSCDecode(in,mm);
  else return false;
  return true;
}
