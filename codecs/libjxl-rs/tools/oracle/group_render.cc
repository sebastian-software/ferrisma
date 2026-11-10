// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Execute the pristine full group traversal with fixture-supplied GetBlock.
// The fixture supplies coefficients, never dequantization or transform code.
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include "lib/jxl/dec_group.h"
#include "lib/jxl/ac_context.h"
#include "lib/jxl/ac_strategy.h"
#include "lib/jxl/base/bits.h"
#include "lib/jxl/base/common.h"
#include "lib/jxl/base/printf_macros.h"
#include "lib/jxl/base/rect.h"
#include "lib/jxl/chroma_from_luma.h"
#include "lib/jxl/coeff_order.h"
#include "lib/jxl/dec_ans.h"
#include "lib/jxl/dec_cache.h"
#include "lib/jxl/dec_transforms-inl.h"
#include "lib/jxl/dec_xyb.h"
#include "lib/jxl/entropy_coder.h"
#include "lib/jxl/image_ops.h"
#include "lib/jxl/jpeg/jpeg_data.h"
#include "lib/jxl/quantizer-inl.h"
#include "lib/jxl/quantizer.h"
#include "lib/jxl/render_pipeline/render_pipeline.h"
#include "lib/jxl/render_pipeline/stage_write.h"
// Keep every original implementation body unchanged while the surrounding
// test-only namespace prevents this observer from replacing archive symbols.
namespace jxl { namespace N_SCALAR { namespace OracleGroup {
#include "original_group_get_block.inc"
using hwy::N_SCALAR::AllFalse;
using hwy::N_SCALAR::Gt;
using hwy::N_SCALAR::Le;
using hwy::N_SCALAR::MaskFromVec;
using hwy::N_SCALAR::Or;
using hwy::N_SCALAR::Rebind;
using hwy::N_SCALAR::ShiftRight;
using D = HWY_FULL(float);
using DI = HWY_FULL(int32_t);
using DI16 = Rebind<int16_t, DI>;
using DI16_FULL = HWY_CAPPED(int16_t, kDCTBlockSize);
constexpr D d;
constexpr DI di;
constexpr DI16 di16;
constexpr DI16_FULL di16_full;
#include "original_group_transpose.inc"
#include "original_dequant_block.inc"
#include "original_decode_group_impl.inc"
}}}
namespace {
struct GRGetBlock : jxl::N_SCALAR::OracleGroup::GetBlock {
 size_t calls=0;std::vector<size_t> rows;int fail=-1;bool jpeg=false;int seed=0;
 void StartRow(size_t by) override {rows.push_back(by);}
 jxl::Status LoadBlock(size_t bx,size_t by,const jxl::AcStrategy& acs,size_t size,size_t log,jxl::ACPtr block[3],jxl::ACType type) override {
  for(size_t c=0;c<3;++c){size_t k=(calls*13+c*7+1)%size;int32_t v=int32_t((bx*11+by*19+c*23+calls*5)%193)-96;if(type==jxl::ACType::k16)block[c].ptr16[k]+=v;else block[c].ptr32[k]+=v;}
  if(jpeg)for(size_t c=0;c<3;++c)for(size_t k=0;k<size;++k)block[c].ptr32[k]=seed+int((k*17+c*29+bx*7+by*13)%257)-128;
  return int(calls++)!=fail;
 }
};

}
bool GroupRenderCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm){
 if(command!="dq-group"&&command!="dq-jpeg")return false;
 bool jpeg_output=command=="dq-jpeg";
 unsigned raw,type,mode;bool accumulate;int fail;in>>raw>>type>>mode>>accumulate>>fail;
 int transform=2,sampling=0,gray=0,cflx=0,cflb=0,qmode=0,seed=0,dcmode=0;
 if(jpeg_output)in>>transform>>sampling>>gray>>cflx>>cflb>>qmode>>seed>>dcmode;
 auto acs=jxl::AcStrategy::FromRawStrategy(static_cast<uint8_t>(raw));size_t cx=acs.covered_blocks_x(),cy=acs.covered_blocks_y();size_t gx=(mode&1)?32:0;size_t nx=(cx<=4)?2:1;size_t ny=(cy<=4)?2:1;size_t w=(gx+nx*cx)*8,h=ny*cy*8;size_t group_idx=gx?1:0;
 jxl::FrameHeader header(nullptr);
 if(jpeg_output){header.color_transform=static_cast<jxl::ColorTransform>(transform);uint8_t hs[3]={uint8_t(sampling&1?2:1),1,1},vs[3]={uint8_t(sampling&2?2:1),1,1};if(!header.chroma_subsampling.Set(hs,vs))throw std::runtime_error("JPEG sampling");}
 jxl::PassesDecoderState state(mm);auto& shared=state.shared_storage;shared.frame_dim.Set(w,h,1,0,0,false,1);size_t bw=shared.frame_dim.xsize_blocks,bh=shared.frame_dim.ysize_blocks;
 auto astrat=jxl::AcStrategyImage::Create(mm,bw,bh);if(!astrat.ok())throw std::runtime_error("group strategy");shared.ac_strategy=std::move(astrat).value_();shared.ac_strategy.FillDCT8();
 for(size_t y=0;y<ny*cy;y+=cy)for(size_t x=gx;x<gx+nx*cx;x+=cx)if(!shared.ac_strategy.Set(x,y,acs.Strategy()))throw std::runtime_error("group set");
 auto qf=jxl::ImageI::Create(mm,bw,bh);if(!qf.ok())throw std::runtime_error("group qf");shared.raw_quant_field=std::move(qf).value_();for(size_t y=0;y<bh;++y)for(size_t x=0;x<bw;++x)shared.raw_quant_field.Row(y)[x]=1+(x*13+y*7)%256;
 auto dc=jxl::Image3F::Create(mm,bw,bh);if(!dc.ok())throw std::runtime_error("group dc");shared.dc_storage=std::move(dc).value_();for(size_t c=0;c<3;++c)for(size_t y=0;y<bh;++y)for(size_t x=0;x<bw;++x)shared.dc_storage.PlaneRow(c,y)[x]=float(int((c*19+x*7+y*13)%97)-48)/64.0f;
 auto cmap=jxl::ColorCorrelationMap::Create(mm,w,h,true);if(!cmap.ok())throw std::runtime_error("group cmap");shared.cmap=std::move(cmap).value_();for(size_t y=0;y<shared.cmap.ytox_map.ysize();++y)for(size_t x=0;x<shared.cmap.ytox_map.xsize();++x){shared.cmap.ytox_map.Row(y)[x]=int8_t(int((x*13+y*7)%71)-35);shared.cmap.ytob_map.Row(y)[x]=int8_t(int((x*3+y*17)%73)-36);}
 jxl::jpeg::JPEGData jpeg;
 if(jpeg_output){
  auto replacement=jxl::ColorCorrelationMap::Create(mm,w,h,false);if(!replacement.ok())throw std::runtime_error("JPEG cmap");shared.cmap=std::move(replacement).value_();
  for(size_t y=0;y<shared.cmap.ytox_map.ysize();++y)for(size_t x=0;x<shared.cmap.ytox_map.xsize();++x){shared.cmap.ytox_map.Row(y)[x]=int8_t(cflx);shared.cmap.ytob_map.Row(y)[x]=int8_t(cflb);}
  std::vector<int> qt(192);for(size_t i=0;i<192;++i)qt[i]=1+int((i*31+7)%255);
  if(qmode==1)qt[65]=0;if(qmode==2)qt[65]=65536;if(qmode==3)qt[130]=-1;if(qmode==7)qt[0]=0;if(qmode==8)qt[0]=INT32_MIN;if(qmode==9)qt.resize(191);
  auto encodings=shared.matrices.encodings();encodings[0]=jxl::QuantEncoding::RAW(std::move(qt),qmode==4?1:0);shared.matrices.SetEncodings(encodings);
  if(qmode==5){auto incompatible=jxl::ColorCorrelationMap::Create(mm,w,h,true);if(!incompatible.ok())throw std::runtime_error("JPEG incompatible cmap");shared.cmap=std::move(incompatible).value_();}
  if(qmode==6){auto encodings=shared.matrices.encodings();encodings[0]=jxl::QuantEncoding::Library<0>();shared.matrices.SetEncodings(encodings);}
  jpeg.components.resize(gray?1:3);auto map=jxl::JpegOrder(header.color_transform,gray);for(size_t c=0;c<3;++c){if(gray&&c!=1)continue;auto& component=jpeg.components[map[c]];component.width_in_blocks=bw>>header.chroma_subsampling.HShift(c);component.height_in_blocks=bh>>header.chroma_subsampling.VShift(c);component.coeffs.assign(size_t(component.width_in_blocks)*component.height_in_blocks*64,1234);}
  for(size_t c=0;c<3;++c)for(size_t y=0;y<bh;++y)for(size_t x=0;x<bw;++x){float value=dcmode==0?float(int((c*19+x*7+y*13)%97)-48):dcmode==1?4096.75f:dcmode==2?-4096.75f:0.75f;if(dcmode>=4){uint32_t bits=dcmode==4?0x7fc12345u:dcmode==5?0x7fa12345u:dcmode==6?0x7f800000u:0xff800000u;std::memcpy(&value,&bits,4);}shared.dc_storage.PlaneRow(c,y)[x]=value;}
 }
 if(!jpeg_output&&!shared.matrices.EnsureComputed(mm,1u<<raw))throw std::runtime_error("group matrices");state.x_dm_multiplier=0.8f;state.b_dm_multiplier=1.25f;state.output_encoding_info.opsin_params.Init(255);
 size_t count=nx*ny*cx*cy*64;
 if(accumulate){if(type==0){auto made=jxl::ACImageT<int16_t>::Make(mm,count,shared.frame_dim.num_groups);if(!made.ok())throw std::runtime_error("group coefficient16");state.coefficients=std::move(made).value_();}else{auto made=jxl::ACImageT<int32_t>::Make(mm,count,shared.frame_dim.num_groups);if(!made.ok())throw std::runtime_error("group coefficient32");state.coefficients=std::move(made).value_();}for(size_t c=0;c<3;++c)for(size_t g=0;g<shared.frame_dim.num_groups;++g){auto ptr=state.coefficients->PlaneRow(c,g,0);for(size_t k=0;k<count;++k){int v=int((k*7+c*13+g*5)%29)-14;if(type==0)ptr.ptr16[k]=v;else ptr.ptr32[k]=v;}}}
 else{if(type==0)state.coefficients=std::make_unique<jxl::ACImageT<int16_t>>();else state.coefficients=std::make_unique<jxl::ACImageT<int32_t>>();}
 jxl::GroupDecCache cache;if(!cache.InitOnce(mm,1,1u<<raw))throw std::runtime_error("group cache");size_t area=cx*cy*64;std::fill(cache.dec_group_block,cache.dec_group_block+area*7,-123.25f);std::fill(cache.dec_group_qblock,cache.dec_group_qblock+area*3,1234);std::fill(cache.dec_group_qblock16,cache.dec_group_qblock16+area*3,1234);
 jxl::Image3F output;jxl::RenderPipeline::Builder builder(mm,3);builder.UseSimpleImplementation();if(!builder.AddStage(jxl::GetWriteToImage3FStage(mm,&output)))throw std::runtime_error("group write stage");auto made=std::move(builder).Finalize(shared.frame_dim);if(!made.ok())throw std::runtime_error("group pipeline");auto pipeline=std::move(made).value_();if(!pipeline->PrepareForThreads(1,false))throw std::runtime_error("group pipeline prepare");auto input=pipeline->GetInputBuffers(group_idx,0);for(size_t c=0;c<3;++c){auto buffer=input.GetBuffer(c);for(size_t y=0;y<buffer.first->ysize();++y)std::fill(buffer.first->Row(y),buffer.first->Row(y)+buffer.first->xsize(),-123.25f);}
 GRGetBlock get;get.fail=fail;get.jpeg=jpeg_output;get.seed=seed;auto draw=(mode&2)?jxl::N_SCALAR::OracleGroup::kDontDraw:jxl::N_SCALAR::OracleGroup::kDraw;auto status=jxl::N_SCALAR::OracleGroup::DecodeGroupImpl(header,&get,&cache,&state,0,group_idx,input,jpeg_output?&jpeg:nullptr,draw);std::cout<<int(status.code())<<' '<<get.calls<<" R";for(auto y:get.rows)std::cout<<' '<<y;
 std::cout<<" C";if(accumulate)for(size_t c=0;c<3;++c){auto ptr=state.coefficients->PlaneRow(c,group_idx,0);for(size_t k=0;k<count;++k)std::cout<<' '<<(type==0?ptr.ptr16[k]:ptr.ptr32[k]);}
 std::cout<<" P";for(size_t c=0;c<3;++c){auto buffer=input.GetBuffer(c);auto r=buffer.second;for(size_t y=0;y<r.ysize();++y)for(size_t x=0;x<r.xsize();++x){float value=r.ConstRow(*buffer.first,y)[x];uint32_t bits;std::memcpy(&bits,&value,4);std::cout<<' '<<bits;}}
 if(jpeg_output){std::cout<<" J";for(const auto& component:jpeg.components){std::cout<<' '<<component.coeffs.size();for(auto v:component.coeffs)std::cout<<' '<<v;}std::cout<<" B";for(size_t i=0;i<area*3;++i)std::cout<<' '<<cache.dec_group_qblock[i];}
 return true;
}
