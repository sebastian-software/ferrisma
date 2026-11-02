// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Genuine public encoder files and the pristine extracted DecodeFile observer.
#include <jxl/encode.h>
#include "lib/jxl/cms/transfer_functions-inl.h"
jxl::Status EFVarDctOneShot(JxlMemoryManager*,const jxl::CompressParams&,
    const jxl::CodecMetadata&,jxl::JxlEncoderChunkedFrameAdapter&,std::vector<uint8_t>&);
bool VarDctImageCommand(const std::string& command, std::istringstream& in,
                       JxlMemoryManager* mm) {
  if (command == "vd-srgb" || command == "vd-srgb-high") {
    uint32_t bits; in >> bits; float v; memcpy(&v, &bits, 4);
    const hwy::HWY_NAMESPACE::ScalableTag<float> d;
    auto input=hwy::HWY_NAMESPACE::Set(d,v);
    auto out=command=="vd-srgb-high"?jxl::HWY_NAMESPACE::TF_SRGB().EncodedFromDisplay(d,input):jxl::HWY_NAMESPACE::FastLinearToSRGB(d,input);
    std::cout << SCBits<uint32_t>(hwy::HWY_NAMESPACE::GetLane(out)); return true;
  }
  if (command == "vd-encode" || command == "vd-encode-raw") {
    uint32_t w,h,depth; bool gray,alpha,gab,progressive; int effort; std::string hex;
    in >> w >> h >> gray >> alpha >> depth >> effort >> gab >> progressive >> hex;
    auto pixels=Unhex(hex);
    if (command=="vd-encode-raw") {
      jxl::CodecMetadata metadata;
      if(!metadata.size.Set(w,h))throw std::runtime_error("VarDCT raw dimensions");
      metadata.m.SetUintSamples(depth);metadata.m.xyb_encoded=true;
      if(alpha)metadata.m.SetAlphaBits(depth,false);
      if(!metadata.m.color_encoding.SetSRGB(gray?jxl::ColorSpace::kGray:jxl::ColorSpace::kRGB,jxl::RenderingIntent::kRelative))throw std::runtime_error("VarDCT raw color");
      jxl::CompressParams p;p.cms={};p.debug_image_opaque=nullptr;p.level=10;p.buffering=0;
      p.speed_tier=static_cast<jxl::SpeedTier>(10-effort);p.butteraugli_distance=1.0f;
      p.patches=jxl::Override::kOff;p.dots=jxl::Override::kOff;p.noise=jxl::Override::kOff;
      p.gaborish=jxl::OverrideFromBool(gab);p.epf=0;p.progressive_dc=0;
      p.progressive_mode=jxl::OverrideFromBool(progressive);
      if(!EFParamsPostInit(&p))throw std::runtime_error("VarDCT raw params");
      jxl::JxlEncoderChunkedFrameAdapter input(w,h,alpha?1:0);
      JxlPixelFormat format{(gray?1u:3u)+(alpha?1u:0u),depth==8?JXL_TYPE_UINT8:JXL_TYPE_UINT16,JXL_NATIVE_ENDIAN,0};
      if(!input.SetFromBuffer(0,pixels.data(),pixels.size(),format))throw std::runtime_error("VarDCT raw input");
      jxl::BitWriter writer(mm);jxl::Status status=jxl::WriteCodestreamHeaders(&metadata,&writer,nullptr);
      if(status)status=writer.WithMaxBits(8,jxl::LayerType::Header,nullptr,[&]{writer.ZeroPadToByte();return true;});
      std::vector<uint8_t> bytes,frame;
      if(status){auto header=std::move(writer).TakeBytes();bytes.assign(header.begin(),header.end());status=EFVarDctOneShot(mm,p,metadata,input,frame);}
      if(status)bytes.insert(bytes.end(),frame.begin(),frame.end());else bytes.clear();
      std::cout<<(status?0:1)<<' '<<Hex(jxl::Bytes(bytes));return true;
    }
    auto* enc=JxlEncoderCreate(nullptr);
    if (!enc) throw std::runtime_error("VarDCT encoder allocation");
    JxlBasicInfo info; JxlEncoderInitBasicInfo(&info); info.xsize=w; info.ysize=h;
    info.bits_per_sample=depth; info.num_color_channels=gray?1:3;
    info.uses_original_profile=JXL_FALSE; info.num_extra_channels=alpha?1:0; info.alpha_bits=alpha?depth:0;
    JxlColorEncoding color; JxlColorEncodingSetToSRGB(&color, gray);
    color.rendering_intent=JXL_RENDERING_INTENT_RELATIVE;
    auto ok=JxlEncoderSetBasicInfo(enc,&info)==JXL_ENC_SUCCESS &&
            JxlEncoderSetColorEncoding(enc,&color)==JXL_ENC_SUCCESS;
    auto* settings=JxlEncoderFrameSettingsCreate(enc,nullptr);
    auto option=[&](JxlEncoderFrameSettingId id,int64_t value) {
      return JxlEncoderFrameSettingsSetOption(settings,id,value)==JXL_ENC_SUCCESS;
    };
    ok=ok && JxlEncoderSetFrameDistance(settings,1.0f)==JXL_ENC_SUCCESS &&
       option(JXL_ENC_FRAME_SETTING_EFFORT,effort) && option(JXL_ENC_FRAME_SETTING_EPF,0) &&
       option(JXL_ENC_FRAME_SETTING_GABORISH,gab) && option(JXL_ENC_FRAME_SETTING_PATCHES,0) &&
       option(JXL_ENC_FRAME_SETTING_DOTS,0) && option(JXL_ENC_FRAME_SETTING_NOISE,0) &&
       option(JXL_ENC_FRAME_SETTING_PROGRESSIVE_DC,0) &&
       option(JXL_ENC_FRAME_SETTING_PROGRESSIVE_AC,progressive);
    JxlPixelFormat format{(gray?1u:3u)+(alpha?1u:0u),depth==8?JXL_TYPE_UINT8:JXL_TYPE_UINT16,JXL_NATIVE_ENDIAN,0};
    ok=ok && JxlEncoderAddImageFrame(settings,&format,pixels.data(),pixels.size())==JXL_ENC_SUCCESS;
    JxlEncoderCloseInput(enc);
    std::vector<uint8_t> bytes(4096); size_t offset=0;
    JxlEncoderStatus status=JXL_ENC_ERROR;
    if(ok) for(;;) {
      uint8_t* next=bytes.data()+offset; size_t available=bytes.size()-offset;
      status=JxlEncoderProcessOutput(enc,&next,&available); offset=next-bytes.data();
      if(status!=JXL_ENC_NEED_MORE_OUTPUT) break;
      bytes.resize(bytes.size()*2);
    }
    JxlEncoderDestroy(enc); bytes.resize(status==JXL_ENC_SUCCESS?offset:0);
    std::cout << int(status) << ' ' << Hex(jxl::Bytes(bytes)); return true;
  }
  if (command != "vd-decode") return false;
  std::string hex; in >> hex; auto bytes=Unhex(hex);
  std::vector<uint8_t> backing(bytes.size()+16); std::copy(bytes.begin(),bytes.end(),backing.begin()+8);
  jxl::CodecInOut io(mm); auto status=jxl::DecodeFile(jxl::Bytes(backing.data()+8,bytes.size()),true,&io,nullptr);
  std::vector<uint8_t> pixels;
  if(status) {
    size_t n=io.metadata.m.color_encoding.Channels()+(io.metadata.m.HasAlpha()?1:0);
    size_t bits=io.metadata.m.bit_depth.bits_per_sample,stride=io.xsize()*n*(bits/8);
    pixels.resize(stride*io.ysize()); jxl::PixelCallback callback;
    status=jxl::ConvertToExternal(io.Main(),bits,false,n,JXL_NATIVE_ENDIAN,stride,nullptr,
      pixels.data(),pixels.size(),callback,jxl::Orientation::kIdentity,false);
  }
  std::cout<<int(status.code())<<" M "<<io.xsize()<<' '<<io.ysize();
  MetadataObservation(io.metadata.m); TransformObservation(io.metadata.transform_data);
  const auto& image=io.Main();
  std::cout<<" F "<<image.color().xsize()<<' '<<image.color().ysize()<<' '<<image.decoded_bytes()<<" O "<<Hex(jxl::Bytes(pixels))<<" R";
  if(status) for(size_t y=0;y<image.ysize();++y) for(size_t x=0;x<image.xsize();++x)
    for(size_t c=0;c<3;++c) std::cout<<' '<<SCBits<uint32_t>(image.color().PlaneRow(c,y)[x]);
  return true;
}
