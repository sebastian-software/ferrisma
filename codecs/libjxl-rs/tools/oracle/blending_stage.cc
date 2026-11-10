// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Include the pristine stage only to observe its initialized private state.
// Actual factories, constructor, row/padding and pipeline algorithms are intact.
#include <array>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <vector>
#include "lib/jxl/render_pipeline/stage_blending.cc"
#include "lib/jxl/render_pipeline/simple_render_pipeline.h"
#include "lib/jxl/render_pipeline/stage_write.h"
int Code(jxl::Status);
namespace {
void Word(float f){uint32_t u;memcpy(&u,&f,4);std::cout<<' '<<u;}
float Pixel(size_t r,size_t c,size_t x,size_t y,unsigned pattern){
  if(pattern){constexpr uint32_t words[]={0,0x80000000,0x3f800000,0xbf800000,0x3f000000,0x40000000,0x7fc12345,0xffc23456,0x7fa12345,0xffa23456,0x7f800000,0xff800000,1,0x80000001,0x3f7fffff,0x3f800001};float f;uint32_t u=words[(r*7+c*5+x+y*3)%16];memcpy(&f,&u,4);return f;}
  return static_cast<float>(static_cast<int>((r*37+c*11+x*3+y*7)%47)-8)*.03125f;
}
void Blend(std::istringstream& in,jxl::BlendingInfo& info){unsigned mode;in>>mode>>info.source>>info.alpha_channel>>info.clamp;info.mode=static_cast<jxl::BlendMode>(mode);}
void Observe(jxl::RenderPipelineStage& stage){
  auto& s=static_cast<jxl::HWY_NAMESPACE::BlendingStage&>(stage);
  size_t w=999,h=999;jxl::FrameOrigin o;stage.GetImageDimensions(&w,&h,&o);
  std::cout<<Code(stage.IsInitialized())<<" H "<<s.image_xsize_<<' '<<s.image_ysize_<<' '<<s.zeroes_.size()<<' '<<s.blending_info_.size()<<' '<<stage.GetName()<<' '<<stage.SwitchToImageDimensions()<<' '<<w<<' '<<h<<' '<<o.x0<<' '<<o.y0;
  for(size_t c:{size_t{0},size_t{1},size_t{2},size_t{3},size_t{6},size_t{99}})std::cout<<' '<<int(stage.GetChannelMode(c));
  for(auto b:s.blending_info_)std::cout<<" B "<<int(b.mode)<<' '<<b.alpha_channel<<' '<<b.clamp;
}
}
bool BlendingStageCommand(const std::string& cmd,std::istringstream& in,JxlMemoryManager* mm){
 if(cmd!="blendstage-row"&&cmd!="blendstage-simple")return false;
 size_t iw,ih,n,assoc;bool xyb,original;int ox,oy;unsigned pattern,ownership;
 in>>iw>>ih>>ox>>oy>>n>>assoc>>xyb>>original>>pattern>>ownership;
 jxl::CodecMetadata metadata;if(!metadata.size.Set(iw,ih))throw std::runtime_error("canvas");metadata.m.xyb_encoded=xyb;metadata.m.num_extra_channels=n;metadata.m.extra_channel_info.resize(n);
 if(!metadata.m.color_encoding.SetSRGB(jxl::ColorSpace::kRGB,jxl::RenderingIntent::kRelative))throw std::runtime_error("color");
 for(size_t c=0;c<n;c++){metadata.m.extra_channel_info[c].type=c%2==0?jxl::ExtraChannel::kAlpha:jxl::ExtraChannel::kDepth;metadata.m.extra_channel_info[c].alpha_associated=(assoc>>c)&1;}
 jxl::FrameHeader header(&metadata);header.frame_origin={ox,oy};Blend(in,header.blending_info);header.extra_channel_blending_info.resize(n);for(auto& b:header.extra_channel_blending_info)Blend(in,b);
 jxl::PassesDecoderState state(mm);state.shared_storage.metadata=&metadata;state.output_encoding_info.color_encoding_is_original=original;
 auto& refs=state.shared_storage.reference_frames;
 for(size_t r=0;r<4;r++){
  size_t w,h;int x,y;bool inxyb;in>>w>>h>>x>>y>>inxyb;refs[r].frame=jxl::make_unique<jxl::ImageBundle>(mm,&metadata.m);refs[r].ib_is_in_xyb=inxyb;
  if(w&&h){auto img=jxl::Image3F::Create(mm,w,h).value_();for(size_t c=0;c<3;c++)for(size_t yy=0;yy<h;yy++)for(size_t xx=0;xx<w;xx++)img.PlaneRow(c,yy)[xx]=Pixel(r,c,xx,yy,pattern);if(!refs[r].frame->SetFromImage(std::move(img),metadata.m.color_encoding))throw std::runtime_error("background");for(size_t c=0;c<n;c++){auto ec=jxl::ImageF::Create(mm,w,h).value_();for(size_t yy=0;yy<h;yy++)for(size_t xx=0;xx<w;xx++)ec.Row(yy)[xx]=Pixel(r,c+3,xx,yy,pattern);refs[r].frame->extra_channels().push_back(std::move(ec));}}
  refs[r].frame->origin={x,y};
 }
 auto stage=jxl::GetBlendingStage(header,&state,jxl::ColorEncoding::SRGB());Observe(*stage);
 if(cmd=="blendstage-row"){
  size_t width,x,y,left,right;unsigned mutate;in>>width>>x>>y>>left>>right>>mutate;
  // Mutate the SAME boxed ImageBundle objects after factory construction.
  if(mutate)for(size_t r=0;r<4;r++)if(refs[r].frame->HasColor()){for(size_t c=0;c<3;c++)for(size_t yy=0;yy<refs[r].frame->ysize();yy++)for(size_t xx=0;xx<refs[r].frame->xsize();xx++)refs[r].frame->color()->PlaneRow(c,yy)[xx]=Pixel(r+mutate,c,xx,yy,pattern);for(size_t c=0;c<n;c++)for(size_t yy=0;yy<refs[r].frame->ysize();yy++)for(size_t xx=0;xx<refs[r].frame->xsize();xx++)refs[r].frame->extra_channels()[c].Row(yy)[xx]=Pixel(r+mutate,c+3,xx,yy,pattern);}
  const size_t len=jxl::kRenderPipelineXOffset+width+8;std::vector<std::vector<float>> rows(3+n,std::vector<float>(len));jxl::RenderPipelineStage::RowInfo ri(3+n);
  for(size_t c=0;c<rows.size();c++){for(size_t xx=0;xx<len;xx++)rows[c][xx]=Pixel(8,c,xx,0,pattern);ri[c].push_back(rows[c].data());}
  auto result=stage->ProcessRow(ri,{},left,right,width,x,y,17);std::cout<<" R "<<Code(result);for(auto& row:rows)for(float v:row)Word(v);
  if(stage->IsInitialized()){
   for(auto& row:rows)std::fill(row.begin(),row.end(),.125f);
   size_t pw=std::min(width,iw),px=iw-pw,py=ih-1;stage->ProcessPaddingRow(ri,pw,px,py);std::cout<<" P";for(auto& row:rows)for(float v:row)Word(v);
  }
 }else{
  size_t fw,fh;in>>fw>>fh;jxl::FrameDimensions dim;dim.Set(fw,fh,0,0,0,false,1);jxl::ImageBundle out(mm,&metadata.m);
  auto builder=jxl::RenderPipeline::Builder(mm,3+n);builder.UseSimpleImplementation();auto added=builder.AddStage(std::move(stage));if(added)added=builder.AddStage(jxl::GetWriteToImageBundleStage(&out,&metadata.m.color_encoding));
  std::cout<<" A "<<Code(added);if(!added)return true;auto built=std::move(builder).Finalize(dim);std::cout<<" Z "<<Code(built.status());if(!built.ok())return true;auto pipe=std::move(built).value_();auto status=pipe->IsInitialized();std::cout<<" I "<<Code(status);
  if(status)status=pipe->PrepareForThreads(1,false);
  if(status)for(size_t g=0;g<dim.num_groups;g++){auto input=pipe->GetInputBuffers(g,0);for(size_t c=0;c<3+n;c++){auto b=input.GetBuffer(c);for(size_t yy=0;yy<b.second.ysize();yy++)for(size_t xx=0;xx<b.second.xsize();xx++)b.second.Row(b.first,yy)[xx]=Pixel(8,c,b.second.x0()-jxl::kRenderPipelineXOffset+xx,b.second.y0()-jxl::kRenderPipelineXOffset+yy,pattern);}status=input.Done();if(!status)break;}
  std::cout<<" D "<<Code(status)<<' '<<out.xsize()<<' '<<out.ysize()<<' '<<out.extra_channels().size();if(out.HasColor())for(size_t c=0;c<3+n;c++)for(size_t yy=0;yy<out.ysize();yy++)for(size_t xx=0;xx<out.xsize();xx++)Word(c<3?out.color()->PlaneRow(c,yy)[xx]:out.extra_channels()[c-3].Row(yy)[xx]);
 }
 return true;
}
