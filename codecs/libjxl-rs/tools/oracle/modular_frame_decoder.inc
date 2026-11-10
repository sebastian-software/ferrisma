// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Call the original frame decoder, retaining status, partial owners, and bits.
void ANSCodeObservation(const jxl::ANSCode&,bool);
void MaHeader(jxl::GroupHeader&);
void MFDHeader(std::istringstream& in,jxl::CodecMetadata& metadata,jxl::FrameHeader& header) {
  size_t w,h,extras;unsigned bits,transform,shift,ups,ecups;bool gray,fp,modular,subsample;
  in>>w>>h>>gray>>bits>>fp>>transform>>modular>>shift>>ups>>extras>>ecups>>subsample;
  if(!metadata.size.Set(w,h))throw std::runtime_error("decoder fixture dimensions");
  metadata.m.bit_depth.bits_per_sample=bits;metadata.m.bit_depth.floating_point_sample=fp;metadata.m.xyb_encoded=false;
  metadata.m.num_extra_channels=extras;metadata.m.extra_channel_info.resize(extras);
  if(!metadata.m.color_encoding.SetSRGB(gray?jxl::ColorSpace::kGray:jxl::ColorSpace::kRGB,jxl::RenderingIntent::kRelative))throw std::runtime_error("decoder fixture color");
  header.color_transform=static_cast<jxl::ColorTransform>(transform);header.encoding=modular?jxl::FrameEncoding::kModular:jxl::FrameEncoding::kVarDCT;
  header.group_size_shift=shift;header.upsampling=ups;header.extra_channel_upsampling.assign(extras,ecups);
  if(subsample){const uint8_t h[3]={2,1,1},v[3]={2,1,1};if(!header.chroma_subsampling.Set(h,v))throw std::runtime_error("decoder fixture subsampling");}
}
void MFDObservation(jxl::ModularFrameDecoder& d,bool pixels,JxlMemoryManager* mm) {
  std::cout<<" D "<<(d.memory_manager()!=nullptr)<<' '<<d.do_color<<' '<<d.have_dc()<<' '<<d.UsesFullImage()<<' '<<d.all_same_shift;
  DimensionsObservation(d.frame_dim);MFImage(d.full_image,false);
  bool reading=pixels;
  for(size_t c=0;c<d.full_image.channel.size();++c){
    const auto& ch=d.full_image.channel[c];
    if(c>=d.full_image.nb_meta_channels && (ch.w>d.frame_dim.group_dim || ch.h>d.frame_dim.group_dim))reading=false;
    bool ready=reading && ch.plane.xsize()!=0 && ch.plane.ysize()!=0;
    std::cout<<' '<<ready;
    if(ready)for(size_t y=0;y<ch.h;++y)for(size_t x=0;x<ch.w;++x)std::cout<<' '<<ch.Row(y)[x];
  }
  jxl::Image moved(mm);moved.transform=d.global_transform;MFImage(moved,false);
  MaTree(d.tree);std::cout<<' '<<d.context_map.size();for(auto v:d.context_map)std::cout<<' '<<unsigned(v);
  ANSCodeObservation(d.code,false);auto header=d.global_header;for(auto& t:header.transforms)t.max_delta_error=77;MaHeader(header);
}
bool ModularFrameDecoderCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if(command=="mfd-source" || command=="mfd-source-groups"){
    size_t w,h;unsigned depth,mode;bool gray;in>>w>>h>>gray>>depth>>mode;
    jxl::CodecMetadata metadata;if(!metadata.size.Set(w,h))throw std::runtime_error("decoder source dimensions");metadata.m.SetUintSamples(depth);metadata.m.xyb_encoded=false;
    if(!metadata.m.color_encoding.SetSRGB(gray?jxl::ColorSpace::kGray:jxl::ColorSpace::kRGB,jxl::RenderingIntent::kRelative))throw std::runtime_error("decoder source color");
    jxl::CompressParams p;p.cms={};p.debug_image_opaque=nullptr;p.speed_tier=jxl::SpeedTier::kThunder;p.level=10;p.buffering=0;p.SetLossless();if(!EFParamsPostInit(&p))throw std::runtime_error("decoder source params");
    jxl::PassesEncoderState state(mm);jxl::FrameHeader header(&metadata);if(!EFPrepareLosslessThunder(p,&state.progressive_splitter,&header))throw std::runtime_error("decoder source header");
    auto made=jxl::ModularFrameEncoder::Create(mm,header,p,false);if(!made.ok())throw std::runtime_error("decoder source encoder");auto modular=std::move(made).value_();
    size_t count=w*h*(gray?1:3);std::vector<uint8_t> data(count*(depth==8?1:2));uint64_t random=0x682f91c2fa013589ULL;
    for(size_t i=0;i<count;++i){random^=random<<13;random^=random>>7;random^=random<<17;uint16_t value=mode==0?137:mode==1?uint16_t(i*173+61):uint16_t(random);if(depth==8)data[i]=value;else memcpy(data.data()+i*2,&value,2);}
    jxl::JxlEncoderChunkedFrameAdapter input(w,h,0);JxlPixelFormat format{gray?1u:3u,depth==8?JXL_TYPE_UINT8:JXL_TYPE_UINT16,JXL_NATIVE_ENDIAN,0};if(!input.SetFromBuffer(0,data.data(),data.size(),format))throw std::runtime_error("decoder source input");
    std::vector<std::unique_ptr<jxl::BitWriter>> codes;auto status=EFComputeThunderFrame(p,metadata,input,header,*modular,state,codes,nullptr);
    if(!status)throw std::runtime_error("decoder source frame");
    // The first group begins with the original all-default DC matrix bit.
    std::cout<<header.group_size_shift;
    if(command=="mfd-source-groups"){std::cout<<' '<<codes.size();for(const auto& code:codes)std::cout<<' '<<Hex(code->GetSpan());}
    else std::cout<<' '<<Hex(codes[0]->GetSpan());return true;
  }
  if(command=="mfd-local-source"){
    jxl::CodecMetadata metadata;jxl::FrameHeader header(&metadata);MFDHeader(in,metadata,header);unsigned mode;in>>mode;
    auto dim=header.ToFrameDimensions();size_t colors=header.encoding==jxl::FrameEncoding::kModular?(metadata.m.color_encoding.IsGray()&&header.color_transform==jxl::ColorTransform::kNone?1:3):0;
    auto made=jxl::Image::Create(mm,dim.xsize,dim.ysize,metadata.m.bit_depth.bits_per_sample,colors+metadata.m.num_extra_channels);if(!made.ok())throw std::runtime_error("local source image");auto image=std::move(made).value_();
    for(size_t c=0;c<image.channel.size();++c){auto& ch=image.channel[c];size_t w=dim.xsize,h=dim.ysize;
      if(c<colors && header.color_transform==jxl::ColorTransform::kYCbCr){ch.hshift=header.chroma_subsampling.HShift(c);ch.vshift=header.chroma_subsampling.VShift(c);w=jxl::DivCeil(w,size_t{1}<<ch.hshift);h=jxl::DivCeil(h,size_t{1}<<ch.vshift);}
      if(c>=colors){size_t ecups=header.extra_channel_upsampling[c-colors];w=jxl::DivCeil(dim.xsize_upsampled,ecups);h=jxl::DivCeil(dim.ysize_upsampled,ecups);ch.hshift=ch.vshift=jxl::CeilLog2Nonzero(ecups)-jxl::CeilLog2Nonzero(header.upsampling);}
      if(!ch.shrink(w,h))throw std::runtime_error("local source shrink");for(size_t y=0;y<h;++y)for(size_t x=0;x<w;++x)ch.Row(y)[x]=mode==0?int(c*37):int((x*173+y*29+c*61)%251);
    }
    jxl::ModularOptions options;options.max_chan_size=dim.group_dim;options.group_dim=dim.group_dim;
    jxl::BitWriter writer(mm);if(!writer.WithMaxBits(1,jxl::LayerType::Header,nullptr,[&]{writer.Write(1,0);return true;}))throw std::runtime_error("local source flag");
    if(!jxl::ModularGenericCompress(image,options,writer,nullptr,jxl::LayerType::ModularGlobal,0)||!Pad(writer))throw std::runtime_error("local source compress");std::cout<<Hex(writer.GetSpan());return true;
  }
  if(command!="mfd-global")return false;
  jxl::CodecMetadata metadata;jxl::FrameHeader header(&metadata);MFDHeader(in,metadata,header);bool allow,drop;size_t repeat,prefix;std::string hex;in>>allow>>drop>>repeat>>prefix>>hex;
  auto bytes=Unhex(hex);std::vector<uint8_t> backing(8+bytes.size()+8);std::copy(bytes.begin(),bytes.end(),backing.begin()+8);
  jxl::ModularFrameDecoder decoder(mm);decoder.Init(header.ToFrameDimensions());
  // Give source-indeterminate fields the same explicit fixture values as the
  // safe native constructor. No uninitialized source bytes are observed.
  decoder.do_color=false;decoder.have_something=false;decoder.all_same_shift=false;
  decoder.code.use_prefix_code=false;decoder.code.log_alpha_size=0;decoder.code.lz77.nonserialized_distance_context=0;
  for(size_t i=0;i<repeat;++i){jxl::BitReader reader(jxl::Bytes(backing.data()+8,bytes.size()));reader.SkipBits(prefix);auto status=decoder.DecodeGlobalInfo(&reader,header,allow);auto consumed=reader.TotalBitsConsumed();auto closed=reader.Close();
    if(drop)decoder.MaybeDropFullImage();std::cout<<int(status.code())<<' '<<consumed<<' '<<int(closed.code());MFDObservation(decoder,bool(status),mm);std::cout<<" | ";}
  return true;
}
