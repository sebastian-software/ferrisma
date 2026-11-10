// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Include the pristine stage to exercise its concrete no-op object as well as
// its public factory. All numeric work calls the original scalar functions.
#include "lib/jxl/render_pipeline/stage_tone_mapping.cc"
float TMFloat(std::istringstream& in){uint32_t bits;in>>bits;return SCBits<float>(bits);}
bool ToneMappingCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm){
  const hwy::N_SCALAR::ScalableTag<float> d;
  auto load=[&](){return hwy::N_SCALAR::Set(d,TMFloat(in));};
  if(command=="tm-fast"){
    unsigned which;in>>which;float target=TMFloat(in);auto x=load(),e=load();auto result=x;
    if(which==0)result=jxl::N_SCALAR::FastPow2f(d,x);
    else if(which==1)result=jxl::N_SCALAR::FastPowf(d,x,e);
    else if(which==2)result=jxl::N_SCALAR::TF_PQ(target).DisplayFromEncoded(d,x);
    else result=jxl::N_SCALAR::TF_PQ(target).EncodedFromDisplay(d,x);
    std::cout<<SCBits<uint32_t>(hwy::N_SCALAR::GetLane(result));return true;
  }
  if(command=="tm-color"){
    unsigned which;in>>which;jxl::Range source{TMFloat(in),TMFloat(in)},target{TMFloat(in),TMFloat(in)};jxl::Vector3 lum{TMFloat(in),TMFloat(in),TMFloat(in)};auto r=load(),g=load(),b=load();float preserve=TMFloat(in);bool gamut=false;
    if(which==0)jxl::N_SCALAR::Rec2408ToneMapper<decltype(d)>(source,target,lum).ToneMap(&r,&g,&b);
    else if(which==4)jxl::N_SCALAR::GamutMap(&r,&g,&b,lum,preserve);
    else{auto ootf=which==1?jxl::N_SCALAR::HlgOOTF(source[1],target[1],lum):which==2?jxl::N_SCALAR::HlgOOTF::FromSceneLight(target[1],lum):jxl::N_SCALAR::HlgOOTF::ToSceneLight(target[1],lum);gamut=ootf.WarrantsGamutMapping();ootf.Apply(&r,&g,&b);}
    std::cout<<gamut;for(auto v:{r,g,b})std::cout<<' '<<SCBits<uint32_t>(hwy::N_SCALAR::GetLane(v));return true;
  }
  if(command!="tm-stage"&&command!="tm-simple")return false;
  unsigned orig,dest,mode;size_t w,h,left,right;in>>orig>>dest;jxl::OutputEncodingInfo info{};info.orig_intensity_target=TMFloat(in);info.desired_intensity_target=TMFloat(in);for(float& v:info.luminances)v=TMFloat(in);in>>w>>h>>left>>right>>mode;
  info.orig_color_encoding.Tf().SetTransferFunction(static_cast<jxl::TransferFunction>(orig));info.color_encoding.Tf().SetTransferFunction(static_cast<jxl::TransferFunction>(dest));
  auto factory=jxl::GetToneMappingStage(info);std::cout<<" H "<<bool(factory);
  if(command=="tm-stage"){
    jxl::N_SCALAR::ToneMappingStage stage(info);auto settings=stage.settings_;std::cout<<' '<<stage.IsNeeded()<<' '<<stage.GetName()<<' '<<settings.shift_x<<' '<<settings.shift_y<<' '<<settings.border_x<<' '<<settings.border_y;for(size_t c=0;c<5;++c)std::cout<<' '<<int(stage.GetChannelMode(c));
    std::vector<jxl::ImageF> planes;jxl::RenderPipelineStage::RowInfo rows(4),unused;for(size_t c=0;c<4;++c){auto made=jxl::ImageF::Create(mm,64,1);if(!made.ok())throw std::runtime_error("tone row");planes.push_back(std::move(made).value_());for(size_t x=0;x<64;++x)planes.back().Row(0)[x]=RPValue(c,x,0,0,mode);rows[c].push_back(planes.back().Row(0));}
    auto status=stage.ProcessRow(rows,unused,left,right,w,19,23,0);std::cout<<" R "<<int(status.code());for(const auto& p:planes)for(size_t x=0;x<64;++x)std::cout<<' '<<SCBits<uint32_t>(p.Row(0)[x]);return true;
  }
  jxl::FrameDimensions dim;dim.Set(w,h,0,0,0,true,1);jxl::Image3F output;
  {
    jxl::RenderPipeline::Builder builder(mm,3);builder.UseSimpleImplementation();if(factory&&!builder.AddStage(std::move(factory)))throw std::runtime_error("tone stage");if(!builder.AddStage(jxl::GetWriteToImage3FStage(mm,&output)))throw std::runtime_error("tone write");auto made=std::move(builder).Finalize(dim);if(!made.ok())throw std::runtime_error("tone pipeline");auto base=std::move(made).value_();auto& pipeline=*static_cast<jxl::SimpleRenderPipeline*>(base.get());auto status=pipeline.PrepareForThreads(2,true);std::cout<<" T "<<int(status.code());
    if(status)for(size_t i=0;i<dim.num_groups;++i){size_t g=dim.num_groups-1-i;auto input=pipeline.GetInputBuffers(g,i%2);for(size_t c=0;c<3;++c){auto [plane,r]=input.GetBuffer(c);for(size_t y=0;y<r.ysize();++y)for(size_t x=0;x<r.xsize();++x)r.Row(plane,y)[x]=RPValue(c,r.x0()+x-jxl::kRenderPipelineXOffset,r.y0()+y-jxl::kRenderPipelineXOffset,0,mode);}status=input.Done();std::cout<<" D "<<int(status.code());if(!status)break;}RPPipeline(pipeline);
  }MGOutput(output);return true;
}
