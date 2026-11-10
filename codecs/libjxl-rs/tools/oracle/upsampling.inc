// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Run the actual scalar upsampling stage inside the original simple pipeline.
#include "lib/jxl/render_pipeline/stage_upsampling.h"
void UPWeights(jxl::CustomTransformData& factors,unsigned mode){
  if(mode==0)return;
  for(auto [p,n]:{std::pair<float*,size_t>{factors.upsampling2_weights,15},{factors.upsampling4_weights,55},{factors.upsampling8_weights,210}}){for(size_t i=0;i<n;++i)p[i]=mode==1?float(int(i%17)-8)/32.0f:mode==2?0.0f:SCBits<float>(uint32_t{0x7fc98765});}
}
bool UpsamplingCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm){
  if(command=="up-guard"){
    size_t left,right,width;in>>left>>right>>width;jxl::CustomTransformData factors;auto stage=jxl::GetUpsamplingStage(mm,factors,0,1);
    auto ai=jxl::ImageF::Create(mm,64,8),ao=jxl::ImageF::Create(mm,64,8);if(!ai.ok()||!ao.ok())throw std::runtime_error("upsampling guard allocation");auto input=std::move(ai).value_();auto output=std::move(ao).value_();jxl::FillImage(-7.0f,&input);jxl::FillImage(-11.0f,&output);
    jxl::RenderPipelineStage::RowInfo ir(1),orr(1);for(size_t y=0;y<5;++y)ir[0].push_back(input.Row(y));for(size_t y=0;y<2;++y)orr[0].push_back(output.Row(y));
    auto prepared=stage->PrepareForThreads(1);auto status=stage->ProcessRow(ir,orr,left,right,width,0,0,0);std::cout<<int(prepared.code())<<' '<<int(status.code());for(size_t y=0;y<8;++y)for(size_t x=0;x<64;++x)std::cout<<' '<<SCBits<uint32_t>(output.Row(y)[x]);return true;
  }
  if(command=="up-factory"){
    size_t shift,c;in>>shift>>c;jxl::CustomTransformData factors;auto stage=jxl::GetUpsamplingStage(mm,factors,c,shift);std::cout<<bool(stage);
    if(stage){auto s=stage->settings_;std::cout<<' '<<s.shift_x<<' '<<s.shift_y<<' '<<s.border_x<<' '<<s.border_y<<' '<<stage->GetName();for(size_t i=0;i<6;++i)std::cout<<' '<<int(stage->GetChannelMode(i));for(size_t n:{size_t{0},size_t{2},size_t{1}})std::cout<<' '<<int(stage->PrepareForThreads(n).code());}return true;
  }
  if(command!="up-simple")return false;
  size_t w,h,shift,c,threads,passes;unsigned mode,weights;bool reverse;in>>w>>h>>shift>>c>>threads>>mode>>weights>>reverse>>passes;
  jxl::CustomTransformData factors;UPWeights(factors,weights);jxl::FrameDimensions dim;dim.Set(w,h,0,0,0,true,1);jxl::Image3F output;
  {
    jxl::RenderPipeline::Builder builder(mm,3);builder.UseSimpleImplementation();auto added=builder.AddStage(jxl::GetUpsamplingStage(mm,factors,c,shift));std::cout<<" B "<<int(added.code());
    if(!added)return true;if(!builder.AddStage(jxl::GetWriteToImage3FStage(mm,&output)))throw std::runtime_error("upsampling write stage");auto made=std::move(builder).Finalize(dim);if(!made.ok())throw std::runtime_error("upsampling pipeline");auto base=std::move(made).value_();auto& pipeline=*static_cast<jxl::SimpleRenderPipeline*>(base.get());
    auto status=pipeline.PrepareForThreads(threads,true);std::cout<<" T "<<int(status.code());
    if(status)for(size_t pass=0;pass<passes;++pass){for(size_t i=0;i<dim.num_groups;++i){size_t g=reverse?dim.num_groups-1-i:i;auto input=pipeline.GetInputBuffers(g,pass%threads);
      for(size_t cc=0;cc<3;++cc){auto [plane,r]=input.GetBuffer(cc);for(size_t y=0;y<r.ysize();++y)for(size_t x=0;x<r.xsize();++x)r.Row(plane,y)[x]=RPValue(cc,r.x0()+x-jxl::kRenderPipelineXOffset,r.y0()+y-jxl::kRenderPipelineXOffset,pass,mode);}
      status=input.Done();std::cout<<" D "<<int(status.code());if(!status)break;
    }if(!status)break;status=pipeline.PrepareForThreads(threads,true);std::cout<<" A "<<int(status.code());if(!status)break;}
    RPPipeline(pipeline);
  }
  MGOutput(output);return true;
}
