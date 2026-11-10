// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#include "lib/jxl/base/float.h"
#include "lib/jxl/enc_external_image.h"

bool ExternalImageCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if(command=="ex-layout") {
    std::cout<<sizeof(JxlPixelFormat)<<' '<<alignof(JxlPixelFormat)
      <<' '<<offsetof(JxlPixelFormat,num_channels)<<' '<<offsetof(JxlPixelFormat,data_type)
      <<' '<<offsetof(JxlPixelFormat,endianness)<<' '<<offsetof(JxlPixelFormat,align);
    return true;
  }
  if(command=="ex-row") {
    unsigned type;bool little;size_t count,stride;uint32_t scale_bits,sentinel;std::string hex;
    in>>type>>little>>count>>stride>>scale_bits>>hex>>sentinel;
    auto data=Unhex(hex);float scale,seed;memcpy(&scale,&scale_bits,4);memcpy(&seed,&sentinel,4);
    std::vector<float> output(count+3,seed);size_t calls=0;
    auto status=jxl::LoadFloatRow(data.data(),count,stride,static_cast<JxlDataType>(type),little,scale,
      [&](size_t i,float value){output[i]=value;++calls;});
    std::cout<<Code(status)<<' '<<calls;
    for(float value:output)std::cout<<' '<<FloatBits(value);
    return true;
  }
  if(command!="ex-plane")return false;
  bool checked;size_t w,h,pw,ph,stride,bits,c,size;unsigned type,endian;JxlPixelFormat format;std::string hex;
  in>>checked>>w>>h>>pw>>ph>>stride>>bits>>format.num_channels>>type>>endian>>format.align>>c>>size>>hex;
  format.data_type=static_cast<JxlDataType>(type);format.endianness=static_cast<JxlEndianness>(endian);
  auto created=jxl::ImageF::Create(mm,pw,ph);if(!created.ok())throw std::runtime_error("external plane allocation");
  auto plane=std::move(created).value_();
  for(size_t y=0;y<ph;++y)for(size_t x=0;x<static_cast<size_t>(plane.PixelsPerRow());++x) {
    uint32_t seed=0x7fa12345u;memcpy(&plane.Row(y)[x],&seed,4);
  }
  auto data=Unhex(hex);
  auto status=checked?jxl::ConvertFromExternal(data.data(),size,w,h,bits,format,c,nullptr,&plane)
    :jxl::ConvertFromExternalPlaneNoSizeCheck(data.data(),w,h,stride,bits,format,c,nullptr,&plane);
  std::cout<<Code(status)<<' '<<plane.xsize()<<' '<<plane.ysize()<<' '<<plane.bytes_per_row();
  for(size_t y=0;y<ph;++y)for(size_t x=0;x<static_cast<size_t>(plane.PixelsPerRow());++x)std::cout<<' '<<FloatBits(plane.Row(y)[x]);
  return true;
}
