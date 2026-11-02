// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// The pristine scalar implementation supplies its actual public factories.
// Scoped -O0 retains Highway operator boundaries with all sanitizers active.
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
float RPValue(size_t,size_t,size_t,size_t,unsigned);
void RPPipeline(jxl::SimpleRenderPipeline&);
void MGOutput(const jxl::Image3F&);
#ifdef JXL_EPF_DEFAULT_REFERENCE
#include "lib/jxl/render_pipeline/stage_epf.h"
#else
#include "lib/jxl/render_pipeline/stage_epf.cc"
#endif
namespace {
template<class T,class U> T Bits(U value) {T out;static_assert(sizeof(T)==sizeof(U));std::memcpy(&out,&value,sizeof(out));return out;}
}
bool EPFCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
 if(command.rfind("epf-",0)!=0)return false;
 unsigned pass,iters;in>>pass>>iters;jxl::LoopFilter lf;lf.epf_iters=iters;
 for(float* p:{&lf.epf_pass0_sigma_scale,&lf.epf_pass2_sigma_scale,&lf.epf_border_sad_mul,&lf.epf_channel_scale[0],&lf.epf_channel_scale[1],&lf.epf_channel_scale[2],&lf.epf_pass1_zeroflush,&lf.epf_pass2_zeroflush}){uint32_t b;in>>b;*p=Bits<float>(b);}
 auto made=jxl::ImageF::Create(mm,128,64);if(!made.ok())throw std::runtime_error("sigma allocation");auto sigma=std::move(made).value_();
  for(size_t y=0;y<64;y++)for(size_t x=0;x<128;x++)sigma.Row(y)[x]= (x+y)%3==0 ? -10.0f : ((x+y)%3==1 ? -3.0f : -0.1f);
 auto stage=jxl::GetEPFStage(lf,sigma,static_cast<jxl::EpfStage>(pass));
 if(command=="epf-factory") {std::cout<<bool(stage);if(stage){auto s=stage->settings_;std::cout<<' '<<s.border_x<<' '<<s.border_y<<' '<<stage->GetName();for(size_t c=0;c<5;c++)std::cout<<' '<<int(stage->GetChannelMode(c));std::cout<<' '<<int(stage->IsInitialized().code())<<' '<<int(stage->PrepareForThreads(2).code());}return true;}
 if(command=="epf-simple") {
  size_t w, h, threads, passes; unsigned mode; bool reverse;
  in >> w >> h >> threads >> mode >> reverse >> passes;
  jxl::FrameDimensions dim; dim.Set(w, h, 0, 0, 0, true, 1); jxl::Image3F output;
  {
    jxl::RenderPipeline::Builder builder(mm, 3); builder.UseSimpleImplementation();
    auto added = builder.AddStage(std::move(stage)); std::cout << " B " << int(added.code());
    if (!added) return true;
    if (!builder.AddStage(jxl::GetWriteToImage3FStage(mm, &output))) throw std::runtime_error("EPF writer");
    auto made = std::move(builder).Finalize(dim); if (!made.ok()) throw std::runtime_error("EPF pipeline");
    auto base = std::move(made).value_(); auto& pipeline = *static_cast<jxl::SimpleRenderPipeline*>(base.get());
    auto status = pipeline.PrepareForThreads(threads, true); std::cout << " T " << int(status.code());
    if (status) for (size_t pass = 0; pass < passes; ++pass) {
      for (size_t i = 0; i < dim.num_groups; ++i) {
        size_t g = reverse ? dim.num_groups - 1 - i : i; auto input = pipeline.GetInputBuffers(g, pass % threads);
        for (size_t c = 0; c < 3; ++c) {
          auto [plane, r] = input.GetBuffer(c);
          for (size_t y = 0; y < r.ysize(); ++y)
            for (size_t x = 0; x < r.xsize(); ++x)
              r.Row(plane, y)[x] = RPValue(c, r.x0() + x - jxl::kRenderPipelineXOffset, r.y0() + y - jxl::kRenderPipelineXOffset, pass, mode);
        }
        status = input.Done(); std::cout << " D " << int(status.code()); if (!status) break;
      }
      if (!status) break;
      status = pipeline.PrepareForThreads(threads, true); std::cout << " A " << int(status.code()); if (!status) break;
    }
    RPPipeline(pipeline);
  }
  MGOutput(output); return true;
 }
 size_t left,right,width,origin,xpos,ypos;in>>left>>right>>width>>origin>>xpos>>ypos;
 std::array<float,9> values;for(float& v:values){uint32_t b;in>>b;v=Bits<float>(b);}std::array<float,3> sigmas;for(float& v:sigmas){uint32_t b;in>>b;v=Bits<float>(b);}
 for(size_t y=0;y<64;y++)for(size_t x=0;x<128;x++)sigma.Row(y)[x]=sigmas[(x+y)%3];
 const size_t len=origin+jxl::kRenderPipelineXOffset+width+right+5;
 std::vector<jxl::ImageF> input,output;
 for(size_t c=0;c<5;c++){auto ai=jxl::ImageF::Create(mm,len,9);auto ao=jxl::ImageF::Create(mm,len,3);if(!ai.ok()||!ao.ok())throw std::runtime_error("row allocation");input.push_back(std::move(ai).value_());output.push_back(std::move(ao).value_());for(size_t y=0;y<9;y++)for(size_t x=0;x<len;x++)input[c].Row(y)[x]=values[(c*7+y*3+x)%9];for(size_t y=0;y<3;y++)for(size_t x=0;x<len;x++)output[c].Row(y)[x]=Bits<float>(uint32_t(0x7fa00000+c*4096+y*256+x));}
 jxl::RenderPipelineStage::RowInfo ir(5),out(5);const size_t border=3-pass;
 for(size_t c=0;c<5;c++){for(size_t y=0;y<2*border+1;y++)ir[c].push_back(input[c].Row(y+1)+origin);out[c].push_back(output[c].Row(1)+origin);}
 auto status=stage->ProcessRow(ir,out,left,right,width,xpos,ypos,17);std::cout<<int(status.code());
 for(const auto& p:output)for(size_t y=0;y<3;y++)for(size_t x=0;x<len;x++)std::cout<<' '<<Bits<uint32_t>(p.Row(y)[x]);
 return true;
}
