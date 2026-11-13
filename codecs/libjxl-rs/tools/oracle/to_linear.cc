// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Actual pristine transfer helpers, complete factory/stages and simple pipeline.
#include <array>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <vector>
#include "lib/jxl/render_pipeline/stage_to_linear.cc"
#include "lib/jxl/frame_dimensions.h"
#include "lib/jxl/image_bundle.h"
#include "lib/jxl/image_metadata.h"
#include "lib/jxl/render_pipeline/simple_render_pipeline.h"
#include "lib/jxl/render_pipeline/stage_write.h"
namespace {
template<class T,class U>T TLBits(U v){static_assert(sizeof(T)==sizeof(U),"width");T r;std::memcpy(&r,&v,sizeof(r));return r;}
float TLRead(std::istringstream& in){uint32_t b;in>>b;return TLBits<float>(b);}
float TLPixel(size_t c,size_t x,size_t y,unsigned pattern){
 if(pattern==1){constexpr uint32_t w[]={0,0x80000000,0x3f800000,0xbf800000,0x3f000000,0x40000000,0x7fc12345,0xffc23456,0x7fa12345,0xffa23456,0x7f800000,0xff800000,1,0x80000001,0x3f7fffff,0x3f800001};return TLBits<float>(w[(c*5+x+y*7)%16]);}
 if(pattern==2||pattern==4||pattern==5){constexpr uint32_t w[]={0,0x80000000,1,0x007fffff,0x322bcc77,0x3727c5ab,0x3727c5ac,0x3727c5ad,0x3c9374bb,0x3c9374bc,0x3c9374bd,0x3da5e353,0x3da5e354,0x3da5e355,0x3daaaaaa,0x3daaaaab,0x3daaaaac,0x3effffff,0x3f000000,0x3f000001,0x3f7fffff,0x3f800000,0x3f800001,0x40000000,0x447a0000,0x7f800000};uint32_t word=w[(c*5+x+y*7)%26];return TLBits<float>(pattern==5&&(word==0x7f800000||word==0x447a0000)?0x40000000:(pattern==4&&word==0x7f800000?0x461c4000:word));}
 if(pattern==3 && c<3)return (x+y)%3==c?-.125f:(c==0?.25f:(c==1?.5f:1.f));
 return float((c*7+x*3+y*11)%33)*.03125f;
}
void TLWord(float x){std::cout<<' '<<TLBits<uint32_t>(x);}
}
bool ToLinearCommand(const std::string& cmd,std::istringstream& in,JxlMemoryManager* mm){
 if(cmd!="tl-tf"&&cmd!="tl-row"&&cmd!="tl-simple")return false;
 const hwy::ORACLE_HWY_NS::ScalableTag<float> d;
 if(cmd=="tl-tf"){
  auto result=jxl::ORACLE_HWY_NS::TF_SRGB().DisplayFromEncoded(hwy::ORACLE_HWY_NS::Set(d,TLRead(in)));
  std::cout<<TLBits<uint32_t>(hwy::ORACLE_HWY_NS::GetLane(result));return true;
 }
 unsigned tf,have_gamma;uint32_t gamma;in>>tf>>have_gamma>>gamma;
 jxl::OutputEncodingInfo info{};info.color_encoding.Tf().SetTransferFunction(static_cast<jxl::TransferFunction>(tf));info.color_encoding.Tf().have_gamma=have_gamma;info.color_encoding.Tf().gamma=gamma;
 info.orig_intensity_target=TLRead(in);info.desired_intensity_target=TLRead(in);info.inverse_gamma=TLRead(in);for(auto& x:info.luminances)x=TLRead(in);
 auto stage=jxl::GetToLinearStage(info);
 // Mutate every source constructor field after its state-copy checkpoint.
 info.orig_intensity_target=7;info.desired_intensity_target=3;info.inverse_gamma=2;info.luminances={0,0,0};info.color_encoding.Tf().SetTransferFunction(jxl::TransferFunction::kUnknown);
 std::cout<<bool(stage);if(!stage)return true;
 auto s=stage->settings_;std::cout<<' '<<stage->GetName()<<' '<<int(stage->IsInitialized().code())<<' '<<s.shift_x<<' '<<s.shift_y<<' '<<s.border_x<<' '<<s.border_y;for(size_t c:{0,1,2,3,4,99})std::cout<<' '<<int(stage->GetChannelMode(c));
 if(cmd=="tl-row"){
  size_t width,left,right,offset,extra,passes;unsigned pattern;in>>width>>left>>right>>offset>>extra>>pattern>>passes;
  size_t len=jxl::kRenderPipelineXOffset+offset+width+8;std::vector<std::vector<float>> rows(3+extra,std::vector<float>(len));jxl::RenderPipelineStage::RowInfo ri(3+extra);
  for(size_t c=0;c<rows.size();c++)ri[c].push_back(rows[c].data()+offset);
  for(size_t pass=0;pass<passes;pass++){
   for(size_t c=0;c<rows.size();c++)for(size_t x=0;x<len;x++)rows[c][x]=TLPixel(c,x,pass,pattern);
   auto status=stage->ProcessRow(ri,{},left,right,width,19+pass,23+pass,pass);std::cout<<" R "<<int(status.code());for(auto& row:rows)for(float x:row)TLWord(x);
  }
 }else{
  size_t width,height,extra;unsigned pattern;in>>width>>height>>extra>>pattern;jxl::FrameDimensions dim;dim.Set(width,height,0,0,0,false,1);
  jxl::CodecMetadata metadata;metadata.m.num_extra_channels=extra;metadata.m.extra_channel_info.resize(extra);if(!metadata.size.Set(width,height))throw std::runtime_error("size");if(!metadata.m.color_encoding.SetSRGB(jxl::ColorSpace::kRGB,jxl::RenderingIntent::kRelative))throw std::runtime_error("color");jxl::ImageBundle out(mm,&metadata.m);
  auto builder=jxl::RenderPipeline::Builder(mm,3+extra);builder.UseSimpleImplementation();auto status=builder.AddStage(std::move(stage));if(status)status=builder.AddStage(jxl::GetWriteToImageBundleStage(&out,&metadata.m.color_encoding));std::cout<<" A "<<int(status.code());if(!status)return true;
  auto made=std::move(builder).Finalize(dim);std::cout<<" Z "<<int(made.status().code());if(!made.ok())return true;auto pipeline=std::move(made).value_();status=pipeline->IsInitialized();std::cout<<" I "<<int(status.code());if(status)status=pipeline->PrepareForThreads(2,true);
  if(status)for(size_t i=0;i<dim.num_groups;i++){auto input=pipeline->GetInputBuffers(dim.num_groups-1-i,i%2);for(size_t c=0;c<3+extra;c++){auto b=input.GetBuffer(c);for(size_t y=0;y<b.second.ysize();y++)for(size_t x=0;x<b.second.xsize();x++)b.second.Row(b.first,y)[x]=TLPixel(c,b.second.x0()+x-jxl::kRenderPipelineXOffset,b.second.y0()+y-jxl::kRenderPipelineXOffset,pattern);}status=input.Done();std::cout<<" D "<<int(status.code());if(!status)break;}
  std::cout<<" O "<<out.xsize()<<' '<<out.ysize()<<' '<<out.extra_channels().size();if(out.HasColor())for(size_t c=0;c<3+extra;c++)for(size_t y=0;y<out.ysize();y++)for(size_t x=0;x<out.xsize();x++)TLWord(c<3?out.color()->PlaneRow(c,y)[x]:out.extra_channels()[c-3].Row(y)[x]);
 }
 return true;
}
