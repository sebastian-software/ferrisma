// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// These function bodies are extracted verbatim and hash checked at configure
// time. No framing algorithm or ICC helper is supplied by this adapter.
#include "lib/extras/codec_in_out.h"
#include <jxl/cms.h>
#include "lib/jxl/icc_codec.h"
#include "lib/jxl/test_memory_manager.h"
namespace jxl {namespace test {
#include "original_read_icc.inc"
}namespace {
#include "original_decode_file.inc"
}}
void MetadataObservation(const jxl::ImageMetadata&);
void TransformObservation(const jxl::CustomTransformData&);
bool WholeImageCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm){
  if(command!="wi-decode")return false;
  std::string hex;in>>hex;auto bytes=Unhex(hex);std::vector<uint8_t> backing(bytes.size()+16);std::copy(bytes.begin(),bytes.end(),backing.begin()+8);
  jxl::CodecInOut io(mm);auto status=jxl::DecodeFile(jxl::Bytes(backing.data()+8,bytes.size()),true,&io,nullptr);
  std::vector<uint8_t> pixels;
  if(status){size_t n=io.metadata.m.color_encoding.Channels()+(io.metadata.m.HasAlpha()?1:0),bits=io.metadata.m.bit_depth.bits_per_sample;size_t stride=io.xsize()*n*(bits/8);pixels.resize(stride*io.ysize());jxl::PixelCallback callback;status=jxl::ConvertToExternal(io.Main(),bits,false,n,JXL_NATIVE_ENDIAN,stride,nullptr,pixels.data(),pixels.size(),callback,jxl::Orientation::kIdentity,false);}
  std::cout<<int(status.code())<<" M "<<io.xsize()<<' '<<io.ysize();MetadataObservation(io.metadata.m);TransformObservation(io.metadata.transform_data);
  const auto& image=io.Main();std::cout<<" F "<<image.color_.xsize()<<' '<<image.color_.ysize()<<' '<<image.decoded_bytes()<<" O "<<Hex(jxl::Bytes(pixels));return true;
}
