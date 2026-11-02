// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Boundary framing and observation only; all dictionary, entropy, alpha, and
// rendering operations call the unchanged pinned library implementation.
#include <algorithm>
#include <array>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include "lib/jxl/alpha.h"
#include "lib/jxl/blending.h"
#include "lib/jxl/dec_patch_dictionary.h"
#include "lib/jxl/enc_ans.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/enc_bit_writer.h"
#include "lib/jxl/image_metadata.h"
#include "lib/jxl/render_pipeline/stage_patches.h"
int Code(jxl::Status);
std::string Hex(jxl::Span<const uint8_t>);
std::vector<uint8_t> Unhex(const std::string&);
jxl::Status Prefix(jxl::BitWriter&, size_t);
jxl::Status Pad(jxl::BitWriter&);
namespace {
float ReadFloat(std::istringstream& in) { uint32_t bits; in >> bits; float f; memcpy(&f,&bits,4); return f; }
void Float(float f) { uint32_t bits; memcpy(&bits,&f,4); std::cout << ' ' << bits; }
std::vector<jxl::ExtraChannelInfo> Extras(size_t n, size_t assoc) {
 std::vector<jxl::ExtraChannelInfo> result(n);
 for(size_t i=0;i<n;++i) { result[i].type = (i==0||i==2) ? jxl::ExtraChannel::kAlpha : jxl::ExtraChannel::kDepth; result[i].alpha_associated = (assoc >> i)&1; }
 return result;
}
float Pixel(size_t ref,size_t c,size_t x,size_t y) { return static_cast<float>((ref*37+c*11+x*3+y*7)%47)/16.0f - 0.5f; }
}
bool PatchesCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
 if(command=="patch-needs") {
  unsigned type,custom,mode;size_t n;in>>type>>custom>>mode>>n;jxl::CodecMetadata metadata;jxl::FrameHeader header(&metadata);header.frame_type=static_cast<jxl::FrameType>(type);header.custom_size_or_origin=custom;header.blending_info.mode=static_cast<jxl::BlendMode>(mode);header.extra_channel_blending_info.resize(n);for(auto& ec:header.extra_channel_blending_info) {in>>mode;ec.mode=static_cast<jxl::BlendMode>(mode);}std::cout<<jxl::NeedsBlending(header);return true;
 }
 if(command=="patch-fixture") {
  size_t prefix,n; in>>prefix>>n; std::vector<std::vector<jxl::Token>> tokens(1);
  for(size_t i=0;i<n;++i) { uint32_t ctx,value; in>>ctx>>value; tokens[0].emplace_back(ctx,value); }
  jxl::BitWriter writer(mm); auto status=Prefix(writer,prefix); jxl::EntropyEncodingData codes;
  if(status) { auto cost=jxl::BuildAndEncodeHistograms(mm,jxl::HistogramParams(),10,tokens,&codes,&writer,jxl::LayerType::Dictionary,nullptr); if(!cost.ok()) status=cost.status(); }
  if(status) status=jxl::WriteTokens(tokens[0],codes,0,&writer,jxl::LayerType::Dictionary,nullptr);
  size_t bits=writer.BitsWritten(); if(status) status=Pad(writer);
  std::cout<<Code(status)<<' '<<bits<<' '<<Hex(writer.GetSpan()); return true;
 }
 if(command=="patch-blend") {
  size_t n,assoc,width,x0; unsigned mode,alpha,clamp; in>>n>>assoc>>width>>x0>>mode>>alpha>>clamp;
  auto extras=Extras(n,assoc); jxl::PatchBlending color{static_cast<jxl::PatchBlendMode>(mode),alpha,clamp!=0};
  std::vector<jxl::PatchBlending> ec(n); for(auto& e:ec) { in>>mode>>alpha>>clamp; e={static_cast<jxl::PatchBlendMode>(mode),alpha,clamp!=0}; }
  std::vector<std::vector<float>> bg(3+n),fg(3+n),out(3+n); std::vector<const float*> b(3+n),f(3+n); std::vector<float*> o(3+n);
  for(size_t c=0;c<3+n;++c) { for(size_t x=0;x<x0+width+2;++x) bg[c].push_back(ReadFloat(in)); }
  for(size_t c=0;c<3+n;++c) { for(size_t x=0;x<x0+width+2;++x) fg[c].push_back(ReadFloat(in)); out[c]=bg[c]; b[c]=bg[c].data();f[c]=fg[c].data();o[c]=out[c].data(); }
  auto status=jxl::PerformBlending(mm,b.data(),f.data(),o.data(),x0,width,color,ec.data(),extras);
  std::cout<<Code(status); for(auto& row:out) for(float v:row) Float(v); return true;
 }
 if(command=="patch-alpha") {
  size_t op,premul,clamp,n,bi,bai,fi,fai;in>>op>>premul>>clamp>>n>>bi>>bai>>fi>>fai;
  std::array<std::vector<float>,5> input; for(auto& row:input) {row.resize(std::max<size_t>(1,n));for(size_t x=0;x<n;++x) row[x]=ReadFloat(in);}
  std::array<std::vector<float>,4> out;for(auto& row:out) row.assign(std::max<size_t>(1,n),19.0f);
  if(op==0) jxl::PerformAlphaBlending(input[bi].data(),input[bai].data(),input[fi].data(),input[fai].data(),out[0].data(),n,premul,clamp);
  else if(op==1) jxl::PerformAlphaBlending({input[0].data(),input[1].data(),input[2].data(),input[bai].data()},{input[2].data(),input[3].data(),input[4].data(),input[fai].data()},{out[0].data(),out[1].data(),out[2].data(),out[3].data()},n,premul,clamp);
  else if(op==2) jxl::PerformAlphaWeightedAdd(input[bi].data(),input[fi].data(),input[fai].data(),out[0].data(),n,clamp);
  else if(op==3) jxl::PerformMulBlending(input[bi].data(),input[fi].data(),out[0].data(),n,clamp);
  else {for(size_t c=0;c<3;++c) out[c]=input[c]; if(op==4) jxl::PremultiplyAlpha(out[0].data(),out[1].data(),out[2].data(),input[fai].data(),n);else jxl::UnpremultiplyAlpha(out[0].data(),out[1].data(),out[2].data(),input[fai].data(),n);}
  std::cout<<"A";for(auto& row:out) for(size_t x=0;x<n;++x) Float(row[x]);return true;
 }
 if(command=="patch-read") {
  size_t n,assoc,w,h,valid,xyb,render,x0,width,left,right,count; in>>n>>assoc>>w>>h>>valid>>xyb>>render>>x0>>width>>left>>right>>count;
  auto extras=Extras(n,assoc); jxl::ImageMetadata metadata; metadata.extra_channel_info=extras;
  std::array<jxl::ReferenceFrame,4> refs;
  for(size_t r=0;r<4;++r) {
   refs[r].frame=jxl::make_unique<jxl::ImageBundle>(mm,&metadata); refs[r].ib_is_in_xyb=(xyb>>r)&1;
   if(!((valid>>r)&1)) continue;
   auto image=jxl::Image3F::Create(mm,12,11).value_();
   for(size_t c=0;c<3;++c) for(size_t y=0;y<11;++y) for(size_t x=0;x<12;++x) image.PlaneRow(c,y)[x]=Pixel(r,c,x,y);
   if(!refs[r].frame->SetFromImage(std::move(image),jxl::ColorEncoding::SRGB())) throw std::runtime_error("reference image");
   for(size_t c=0;c<n;++c) { auto plane=jxl::ImageF::Create(mm,12,11).value_(); for(size_t y=0;y<11;++y) for(size_t x=0;x<12;++x) plane.Row(y)[x]=Pixel(r,3+c,x,y); refs[r].frame->extra_channels().push_back(std::move(plane)); }
  }
  jxl::PatchDictionary pd(mm); pd.SetShared(&refs); bool uses=false;
  auto persistent_stage=jxl::GetPatchesStage(&pd,&extras);
  for(size_t cycle=0;cycle<count;++cycle) {
   if(render==3) {for(size_t r=0;r<4;++r) if((valid>>r)&1) refs[r].frame->color()->PlaneRow(0,1)[1]=Pixel(r,0,1,1)+static_cast<float>(cycle)*0.125f;}
   size_t prefix; std::string hex; in>>prefix>>hex; auto bytes=Unhex(hex); std::vector<uint8_t> backing(bytes.size()+16); std::copy(bytes.begin(),bytes.end(),backing.begin()+8);
   jxl::BitReader reader(jxl::Span<const uint8_t>(backing.data()+8,bytes.size())); reader.SkipBits(prefix);
   auto status=pd.Decode(mm,&reader,w,h,n,&uses); auto consumed=reader.TotalBitsConsumed(); auto close=reader.Close();
   std::cout<<(cycle ? " D " : "D ")<<Code(status)<<' '<<consumed<<' '<<Code(close)<<' '<<uses;
   std::cout<<" S "<<pd.positions_.size()<<' '<<pd.ref_positions_.size()<<' '<<pd.blendings_.size()<<' '<<pd.patch_tree_.size()<<' '<<pd.num_patches_.size()<<' '<<pd.sorted_patches_y0_.size()<<' '<<pd.sorted_patches_y1_.size()<<' '<<pd.blendings_stride_<<' '<<pd.GetReferences();
   for(auto p:pd.positions_) std::cout<<" P "<<p.x<<' '<<p.y<<' '<<p.ref_pos_idx;
   for(auto p:pd.ref_positions_) std::cout<<" R "<<p.ref<<' '<<p.x0<<' '<<p.y0<<' '<<p.xsize<<' '<<p.ysize;
   for(auto p:pd.blendings_) std::cout<<" B "<<static_cast<unsigned>(p.mode)<<' '<<p.alpha_channel<<' '<<p.clamp;
   for(auto node:pd.patch_tree_) std::cout<<" N "<<node.left_child<<' '<<node.right_child<<' '<<node.y_center<<' '<<node.start<<' '<<node.num;
   std::cout<<" C";for(auto count:pd.num_patches_) std::cout<<' '<<count;
   std::cout<<" L";for(auto patch:pd.sorted_patches_y0_) std::cout<<' '<<patch.first<<' '<<patch.second;
   std::cout<<" H";for(auto patch:pd.sorted_patches_y1_) std::cout<<' '<<patch.first<<' '<<patch.second;
   if(status) {
    for(size_t y=0;y<h+2;++y) { auto patches=pd.GetPatchesForRow(y); std::cout<<" T "<<patches.size(); for(auto p:patches) std::cout<<' '<<p; }
    if(render) {
     const size_t span=width+left+right;
     for(size_t y=0;y<h;++y) {
      std::vector<std::vector<float>> storage(3+n,std::vector<float>(span+64)); std::vector<float*> rows(3+n);
      for(size_t c=0;c<3+n;++c) { rows[c]=storage[c].data()+32; for(size_t x=0;x<span;++x) rows[c][x]=Pixel(9,c,x0-left+x,y); }
      if(render==1) status=pd.AddOneRow(rows.data(),y,x0-left,span,extras);
      else {
       auto stage=jxl::GetPatchesStage(&pd,&extras); jxl::RenderPipelineStage::RowInfo input(3+n),output;
       for(size_t c=0;c<3+n;++c) input[c].push_back(rows[c]+left-jxl::kRenderPipelineXOffset);
       status=(render==3?persistent_stage:stage)->ProcessRow(input,output,left,right,width,x0,y,0);
      }
      std::cout<<" O "<<Code(status); for(size_t c=0;c<3+n;++c) for(size_t x=0;x<span;++x) Float(rows[c][x]);
     }
    }
   }
  }
  return true;
 }
 return false;
}
