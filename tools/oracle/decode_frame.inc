// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Complete original DecodeFrame, explicitly selecting its simple pipeline.
#include "lib/jxl/dec_frame.h"
bool DecodeFrameCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm){
  if(command=="df-cache"){
    size_t n;in>>n;jxl::GroupDecCache cache;std::cout<<alignof(jxl::GroupDecCache);for(size_t i=0;i<n;++i){auto status=cache.InitDCBufferOnce(mm);std::cout<<' '<<int(status.code());}SSPlane(cache.dc_buffer,false);for(const auto& im:cache.num_nzeroes)for(size_t c=0;c<3;++c)SSPlane(im.Plane(c),false);return true;
  }
  if(command!="df-source"&&command!="df-decode")return false;
  size_t w,h;bool gray;unsigned depth;std::string hex;in>>w>>h>>gray>>depth>>hex;
  jxl::CodecMetadata metadata;if(!metadata.size.Set(w,h))throw std::runtime_error("decode frame dimensions");metadata.m.SetUintSamples(depth);metadata.m.xyb_encoded=false;if(!metadata.m.color_encoding.SetSRGB(gray?jxl::ColorSpace::kGray:jxl::ColorSpace::kRGB,jxl::RenderingIntent::kRelative))throw std::runtime_error("decode frame color");
  auto bytes=Unhex(hex);
  if(command=="df-source"){
    jxl::CompressParams p;p.speed_tier=jxl::SpeedTier::kThunder;p.level=10;p.buffering=0;p.SetLossless();if(!EFParamsPostInit(&p))throw std::runtime_error("decode source params");jxl::JxlEncoderChunkedFrameAdapter input(w,h,0);JxlPixelFormat format{gray?1u:3u,depth==8?JXL_TYPE_UINT8:JXL_TYPE_UINT16,JXL_NATIVE_ENDIAN,0};if(!input.SetFromBuffer(0,bytes.data(),bytes.size(),format))throw std::runtime_error("decode source input");std::vector<uint8_t> frame;auto status=EFThunderOneShot(mm,p,metadata,input,frame,nullptr);std::cout<<int(status.code())<<' '<<Hex(jxl::Bytes(frame));return true;
  }
  // Backing bytes preserve the real non-null empty-span and end-minus-eight
  // reader preconditions without changing the supplied frame span or length.
  std::vector<uint8_t> backing(bytes.size()+16);std::copy(bytes.begin(),bytes.end(),backing.begin()+8);
  jxl::PassesDecoderState state(mm);if(!state.output_encoding_info.SetFromMetadata(metadata))throw std::runtime_error("decode frame output profile");jxl::ImageBundle output(mm,&metadata.m);jxl::FrameHeader header(&metadata);
  // DecodeFrame initializes its own header with is_preview=false. A valid bool
  // marker on this separate output object detects only the actual copy.
  header.nonserialized_is_preview=true;
  auto status=jxl::DecodeFrame(&state,nullptr,backing.data()+8,bytes.size(),&header,&output,metadata,true);bool copied=!header.nonserialized_is_preview;
  std::cout<<int(status.code())<<" H "<<copied;
  if(copied)std::cout<<' '<<unsigned(header.encoding)<<' '<<unsigned(header.frame_type)<<' '<<header.flags<<' '<<unsigned(header.color_transform)<<' '<<header.group_size_shift<<' '<<header.upsampling<<' '<<header.is_last;
  std::cout<<" O "<<output.color_.xsize()<<' '<<output.color_.ysize()<<' '<<output.decoded_bytes()<<' '<<output.duration<<' '<<output.origin.x0<<' '<<output.origin.y0<<' '<<output.extra_channels().size();ColorBundleObservation(output.c_current());
  // Error paths can leave allocated, undefined source pixels. Their dimensions,
  // profiles, and other initialized partial state remain part of observations.
  if(status)for(size_t c=0;c<3;++c)for(size_t y=0;y<output.color_.ysize();++y)for(size_t x=0;x<output.color_.xsize();++x)std::cout<<' '<<SCBits<uint32_t>(output.color_.PlaneRow(c,y)[x]);
  return true;
}
