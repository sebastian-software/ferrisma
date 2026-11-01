// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Compose original MFD operations with real encoder sections and the original
// simple pipeline. This fixture does not implement frame/container parsing.
#include "lib/jxl/toc.h"
void MGSeed(jxl::ModularFrameDecoder& d){d.do_color=false;d.have_something=false;d.all_same_shift=false;d.code.use_prefix_code=false;d.code.log_alpha_size=0;d.code.lz77.nonserialized_distance_context=0;}
void MGOutput(const jxl::Image3F& image){std::cout<<" O "<<image.xsize()<<' '<<image.ysize();for(size_t c=0;c<3;++c)for(size_t y=0;y<image.ysize();++y)for(size_t x=0;x<image.xsize();++x)std::cout<<' '<<SCBits<uint32_t>(image.PlaneRow(c,y)[x]);}
bool ModularGroupsCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm){
  if(command=="mg-pixels"){
    jxl::CodecMetadata metadata;jxl::FrameHeader header(&metadata);MFDHeader(in,metadata,header);
    unsigned exp,ecbits,ecexp,mutation;bool ecfp;size_t x,y,w,h;std::string hex;
    in>>exp>>ecbits>>ecfp>>ecexp>>mutation>>x>>y>>w>>h>>hex;
    metadata.m.bit_depth.exponent_bits_per_sample=exp;
    for(auto& ec:metadata.m.extra_channel_info){ec.bit_depth.bits_per_sample=ecbits;ec.bit_depth.floating_point_sample=ecfp;ec.bit_depth.exponent_bits_per_sample=ecexp;}
    auto bytes=Unhex(hex);std::vector<uint8_t> backing(bytes.size()+16);std::copy(bytes.begin(),bytes.end(),backing.begin()+8);
    jxl::BitReader reader(jxl::Bytes(backing.data()+8,bytes.size()));jxl::ModularFrameDecoder d(mm);auto dim=header.ToFrameDimensions();d.Init(dim);MGSeed(d);
    auto status=d.DecodeGlobalInfo(&reader,header,false);std::cout<<" G "<<int(status.code());
    if(status){auto cloned=jxl::Image::Clone(d.full_image);if(!cloned.ok())throw std::runtime_error("pixel fixture clone");auto gi=std::move(cloned).value_();gi.undo_transforms(d.global_header.wp_header,nullptr);
      if(mutation==1)gi.transform.emplace_back(jxl::TransformId::kRCT);
      if(mutation==2)gi.channel.clear();
      if(mutation==3&&!gi.channel.empty())gi.channel[0].w=0;
      if(mutation==4&&gi.channel.size()>1)gi.channel[1].h=0;
      if(mutation==5&&!gi.channel.empty())gi.channel[0].hshift=4;
      if(mutation==6&&!gi.channel.empty())gi.channel.pop_back();
      jxl::PassesDecoderState state(mm);state.shared_storage.frame_dim=dim;state.shared_storage.metadata=&metadata;
      jxl::Image3F output;jxl::RenderPipeline::Builder builder(mm,3+metadata.m.num_extra_channels);builder.UseSimpleImplementation();
      if(!builder.AddStage(jxl::GetWriteToImage3FStage(mm,&output)))throw std::runtime_error("pixel fixture write stage");auto made=std::move(builder).Finalize(dim);if(!made.ok())throw std::runtime_error("pixel fixture pipeline");state.render_pipeline=std::move(made).value_();
      if(!state.render_pipeline->PrepareForThreads(1,false))throw std::runtime_error("pixel fixture prepare");auto input=state.render_pipeline->GetInputBuffers(0,0);
      for(size_t c=0;c<3+metadata.m.num_extra_channels;++c){const auto& b=input.GetBuffer(c);for(size_t yy=0;yy<b.second.ysize();++yy)std::fill_n(b.second.Row(b.first,yy),b.second.xsize(),-7.0f);}
      status=d.ModularImageToDecodedRect(header,gi,&state,nullptr,input,jxl::Rect(x,y,w,h));std::cout<<" C "<<int(status.code());
      for(size_t c=0;c<3+metadata.m.num_extra_channels;++c){const auto& b=input.GetBuffer(c);std::cout<<" I "<<b.second.xsize()<<' '<<b.second.ysize();for(size_t yy=0;yy<b.second.ysize();++yy)for(size_t xx=0;xx<b.second.xsize();++xx)std::cout<<' '<<SCBits<uint32_t>(b.second.Row(b.first,yy)[xx]);}
    }
    auto consumed=reader.TotalBitsConsumed();auto closed=reader.Close();std::cout<<" B "<<consumed<<' '<<int(closed.code());return true;
  }
  if(command!="mg-decode")return false;
  size_t w,h,repeats,n;unsigned depth,kind;bool gray,drop,inplace,zero,has_state,has_input;
  in>>w>>h>>gray>>depth>>drop>>inplace>>repeats>>zero>>has_state>>has_input>>kind>>n;
  std::vector<std::vector<uint8_t>> bytes(n);std::vector<size_t> lengths(n);
  for(size_t i=0;i<n;++i){std::string hex;in>>hex;auto data=Unhex(hex);lengths[i]=data.size();bytes[i].resize(8+data.size()+8);std::copy(data.begin(),data.end(),bytes[i].begin()+8);}
  std::vector<std::unique_ptr<jxl::BitReader>> readers;for(size_t i=0;i<n;++i)readers.push_back(std::make_unique<jxl::BitReader>(jxl::Bytes(bytes[i].data()+8,lengths[i])));
  auto reader=[&](size_t id){return readers[n==1?0:id].get();};
  jxl::CodecMetadata metadata;if(!metadata.size.Set(w,h))throw std::runtime_error("modular group dimensions");metadata.m.SetUintSamples(depth);metadata.m.xyb_encoded=false;
  if(!metadata.m.color_encoding.SetSRGB(gray?jxl::ColorSpace::kGray:jxl::ColorSpace::kRGB,jxl::RenderingIntent::kRelative))throw std::runtime_error("modular group color");
  jxl::CompressParams params;params.cms={};params.debug_image_opaque=nullptr;params.speed_tier=jxl::SpeedTier::kThunder;params.level=10;params.buffering=0;params.SetLossless();if(!EFParamsPostInit(&params))throw std::runtime_error("modular group params");
  jxl::PassesDecoderState state(mm);jxl::ProgressiveSplitter splitter;jxl::FrameHeader header(&metadata);if(!EFPrepareLosslessThunder(params,&splitter,&header))throw std::runtime_error("modular group header");
  const auto dim=header.ToFrameDimensions();state.shared_storage.frame_dim=dim;state.shared_storage.metadata=&metadata;
  jxl::ModularFrameDecoder decoder(mm);decoder.Init(dim);MGSeed(decoder);
  auto status=state.shared_storage.matrices.DecodeDC(reader(0));std::cout<<" Q "<<int(status.code());
  if(status){status=decoder.DecodeGlobalInfo(reader(0),header,false);std::cout<<" G "<<int(status.code());}
  if(status){for(size_t g=0;g<dim.num_dc_groups;++g){jxl::Rect r((g%dim.xsize_dc_groups)*dim.dc_group_dim,(g/dim.xsize_dc_groups)*dim.dc_group_dim,dim.dc_group_dim,dim.dc_group_dim);
    status=decoder.DecodeGroup(header,r,reader(1+g),3,1000,jxl::ModularStreamId::ModularDC(g),false,nullptr,nullptr,false);std::cout<<" D "<<int(status.code());if(!status)break;}}
  jxl::Image3F output;
  if(status){if(drop)decoder.MaybeDropFullImage();jxl::RenderPipeline::Builder builder(mm,3);builder.UseSimpleImplementation();if(!builder.AddStage(jxl::GetWriteToImage3FStage(mm,&output)))throw std::runtime_error("modular group pipeline stage");
    auto made=std::move(builder).Finalize(dim);if(!made.ok())throw std::runtime_error("modular group pipeline");state.render_pipeline=std::move(made).value_();status=state.render_pipeline->PrepareForThreads(1,false);std::cout<<" P "<<int(status.code());
    if(status){for(size_t g=0;g<dim.num_groups;++g){auto input=state.render_pipeline->GetInputBuffers(g,0);bool ready=true;
      jxl::Rect r((g%dim.xsize_groups)*dim.group_dim,(g/dim.xsize_groups)*dim.group_dim,dim.group_dim,dim.group_dim);int min,max;header.passes.GetDownsamplingBracket(0,min,max);
      auto stream=jxl::ModularStreamId::ModularAC(g,0);stream.kind=static_cast<jxl::ModularStreamId::Kind>(kind);
      status=decoder.DecodeGroup(header,r,zero?nullptr:reader(jxl::AcGroupIndex(0,g,dim.num_groups,dim.num_dc_groups)),min,max,stream,zero,has_state?&state:nullptr,has_input?&input:nullptr,false,&ready);
      std::cout<<" A "<<int(status.code())<<' '<<ready;if(!status)break;
      if(!decoder.UsesFullImage()&&ready&&has_input){status=input.Done();std::cout<<" R "<<int(status.code());if(!status)break;}
    }}
    if(status)for(size_t i=0;i<repeats;++i){status=decoder.FinalizeDecoding(header,&state,nullptr,inplace);std::cout<<" F "<<int(status.code());if(!status)break;}
    RPPipeline(*static_cast<jxl::SimpleRenderPipeline*>(state.render_pipeline.get()));
  }
  std::cout<<" S "<<int(status.code());MFDObservation(decoder,false,mm);MGOutput(output);
  std::cout<<" B "<<readers.size();for(auto& r:readers){auto consumed=r->TotalBitsConsumed();auto closed=r->Close();std::cout<<' '<<consumed<<','<<int(closed.code());}return true;
}
