// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observe the real scalar factories and stages linked from the original archive.
#include <array>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include "lib/jxl/image.h"
#include "lib/jxl/frame_dimensions.h"
#include "lib/jxl/render_pipeline/simple_render_pipeline.h"
#include "lib/jxl/render_pipeline/stage_write.h"
#include "lib/jxl/render_pipeline/stage_ycbcr.h"
#include "lib/jxl/render_pipeline/stage_chroma_upsampling.h"
float RPValue(size_t,size_t,size_t,size_t,unsigned);
void RPPipeline(jxl::SimpleRenderPipeline&);
void MGOutput(const jxl::Image3F&);
namespace {
template<class T,class U>T Bits(U v){T out;static_assert(sizeof(T)==sizeof(U));std::memcpy(&out,&v,sizeof(out));return out;}
}
bool ChromaStagesCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm){
 if(command.rfind("cs-",0)!=0)return false;
 if(command=="cs-simple") {
 unsigned mask;size_t w,h,threads,passes;unsigned mode;bool reverse;in>>mask>>w>>h>>threads>>mode>>reverse>>passes;
 jxl::FrameDimensions dim;dim.Set(w,h,0,0,0,true,1);jxl::Image3F image;
 {
 jxl::RenderPipeline::Builder builder(mm,3);builder.UseSimpleImplementation();
 for(size_t c=0;c<3;c++){if(mask&(1u<<c)){auto s=builder.AddStage(jxl::GetChromaUpsamplingStage(c,true));std::cout<<" B "<<int(s.code());if(!s)return true;}if(mask&(1u<<(c+3))){auto s=builder.AddStage(jxl::GetChromaUpsamplingStage(c,false));std::cout<<" B "<<int(s.code());if(!s)return true;}}
 if(mask&64){auto s=builder.AddStage(jxl::GetYCbCrStage());std::cout<<" B "<<int(s.code());if(!s)return true;}
 if(!builder.AddStage(jxl::GetWriteToImage3FStage(mm,&image)))throw std::runtime_error("chroma writer");auto made=std::move(builder).Finalize(dim);if(!made.ok())throw std::runtime_error("chroma pipeline");auto base=std::move(made).value_();auto& pipeline=*static_cast<jxl::SimpleRenderPipeline*>(base.get());
 auto status=pipeline.PrepareForThreads(threads,true);std::cout<<" T "<<int(status.code());
 if(status)for(size_t pass=0;pass<passes;pass++){for(size_t i=0;i<dim.num_groups;i++){size_t g=reverse?dim.num_groups-1-i:i;auto input=pipeline.GetInputBuffers(g,pass%threads);for(size_t c=0;c<3;c++){auto [p,r]=input.GetBuffer(c);for(size_t y=0;y<r.ysize();y++)for(size_t x=0;x<r.xsize();x++)r.Row(p,y)[x]=RPValue(c,r.x0()+x-jxl::kRenderPipelineXOffset,r.y0()+y-jxl::kRenderPipelineXOffset,pass,mode);}status=input.Done();std::cout<<" D "<<int(status.code());if(!status)break;}if(!status)break;status=pipeline.PrepareForThreads(threads,true);std::cout<<" A "<<int(status.code());if(!status)break;}
 RPPipeline(pipeline);
 }MGOutput(image);return true;
 }
 unsigned type;size_t channel;in>>type>>channel;auto stage=type==0?jxl::GetYCbCrStage():jxl::GetChromaUpsamplingStage(channel,type==1);
 if(command=="cs-factory"){auto s=stage->settings_;std::cout<<s.shift_x<<' '<<s.shift_y<<' '<<s.border_x<<' '<<s.border_y<<' '<<stage->GetName();for(size_t c:{size_t{0},size_t{1},size_t{2},size_t{3},size_t{4},SIZE_MAX})std::cout<<' '<<int(stage->GetChannelMode(c));for(size_t n:{size_t{0},size_t{2},size_t{1}})std::cout<<' '<<int(stage->PrepareForThreads(n).code());std::cout<<' '<<int(stage->IsInitialized().code());return true;}
 size_t left,right,width,origin;in>>left>>right>>width>>origin;std::array<float,9> values;for(float& v:values){uint32_t b;in>>b;v=Bits<float>(b);}
 size_t len=origin+jxl::kRenderPipelineXOffset+2*(width+right)+5;std::vector<jxl::ImageF> input,output;
 for(size_t c=0;c<5;c++){auto a=jxl::ImageF::Create(mm,len,7),b=jxl::ImageF::Create(mm,len,5);if(!a.ok()||!b.ok())throw std::runtime_error("chroma rows");input.push_back(std::move(a).value_());output.push_back(std::move(b).value_());for(size_t y=0;y<7;y++)for(size_t x=0;x<len;x++)input[c].Row(y)[x]=values[(c*7+y*3+x)%9];for(size_t y=0;y<5;y++)for(size_t x=0;x<len;x++)output[c].Row(y)[x]=Bits<float>(uint32_t(0x7fa00000+c*8192+y*1024+x));}
 jxl::RenderPipelineStage::RowInfo ir(5),out(5);for(size_t c=0;c<5;c++){if(type==2){for(size_t y:{size_t{1},size_t{3},size_t{5}})ir[c].push_back(input[c].Row(y)+origin);for(size_t y:{size_t{1},size_t{3}})out[c].push_back(output[c].Row(y)+origin);}else{ir[c].push_back(input[c].Row(3)+origin);out[c].push_back(output[c].Row(2)+origin);}}
 auto status=stage->ProcessRow(ir,out,left,right,width,23,41,17);std::cout<<int(status.code());for(const auto& planes:{&input,&output})for(const auto& p:*planes)for(size_t y=0;y<p.ysize();y++)for(size_t x=0;x<len;x++)std::cout<<' '<<Bits<uint32_t>(p.Row(y)[x]);return true;
}
