// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Input/output and private-state observation only. Methods are original code.
#include "lib/jxl/encode_internal.h"
std::vector<uint8_t> Unhex(const std::string&);
jxl::Status EFCopyThunderColorChannels(jxl::JxlEncoderChunkedFrameAdapter&,const jxl::ImageMetadata&,const jxl::Rect&,jxl::Image3F*,bool*);

JxlPixelFormat FBReadFormat(std::istringstream& in) {
  JxlPixelFormat f;unsigned type,endian;in>>f.num_channels>>type>>endian>>f.align;
  f.data_type=static_cast<JxlDataType>(type);f.endianness=static_cast<JxlEndianness>(endian);return f;
}
void FBState(jxl::JxlEncoderChunkedFrameAdapter& a) {
  std::cout<<" S "<<a.xsize<<' '<<a.ysize<<' '<<a.channels_.size();
  for(size_t i=0;i<a.channels_.size();++i) {
    auto& c=a.channels_[i];auto f=a.GetInputSource();JxlPixelFormat format;
    if(i==0)f.get_color_channels_pixel_format(f.opaque,&format);else f.get_extra_channel_pixel_format(f.opaque,i-1,&format);
    std::cout<<' '<<format.num_channels<<' '<<unsigned(format.data_type)<<' '<<unsigned(format.endianness)<<' '<<format.align
      <<' '<<c.xsize_<<' '<<c.ysize_<<' '<<c.bytes_per_pixel_<<' '<<c.stride_<<' '<<c.buffer_size_
      <<' '<<(c.buffer_!=nullptr)<<' '<<Hex(jxl::Bytes(c.copy_.data(),c.copy_.size()));
    std::cout<<' '<<Hex(c.buffer_?jxl::Bytes(c.buffer_,std::min<size_t>(8,c.buffer_size_)):jxl::Bytes());
    size_t row=123;auto b=i==0?f.get_color_channel_data_at(f.opaque,0,0,0,0,&row)
      :f.get_extra_channel_data_at(f.opaque,i-1,0,0,0,0,&row);
    size_t available=c.copy_.empty()?c.buffer_size_:c.copy_.size();
    std::cout<<' '<<(b!=nullptr)<<' '<<row<<' '<<Hex(b?jxl::Bytes(static_cast<const uint8_t*>(b),std::min<size_t>(8,available)):jxl::Bytes());
    if(b)f.release_buffer(f.opaque,b);
  }
}
bool FrameBuffersCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if(command=="fb-bind") {
    size_t w,h,extras,index,size;bool null;std::string hex;in>>w>>h>>extras>>index>>null>>size;
    auto format=FBReadFormat(in);in>>hex;auto data=Unhex(hex);jxl::JxlEncoderChunkedFrameAdapter a(w,h,extras);
    std::cout<<a.SetFromBuffer(index,null?nullptr:data.data(),size,format)<<' '<<a.channels_.size()<<' '<<jxl::BitsPerChannel(format.data_type)<<' '<<jxl::BytesPerPixel(format);return true;
  }
  if(command=="fb-chain") {
    size_t w,h,extras,n;in>>w>>h>>extras>>n;
    struct Op {size_t index,size;bool null,copy;JxlPixelFormat format;std::vector<uint8_t> data;};
    std::vector<Op> ops(n);for(auto& o:ops) {std::string hex;in>>o.index>>o.null>>o.size;o.format=FBReadFormat(in);in>>o.copy>>hex;o.data=Unhex(hex);}
    jxl::JxlEncoderChunkedFrameAdapter a(w,h,extras);std::vector<uint8_t> initial(512);for(size_t i=0;i<initial.size();++i)initial[i]=i*137+29;
    const JxlPixelFormat format{1,JXL_TYPE_UINT8,JXL_NATIVE_ENDIAN,0};
    for(size_t i=0;i<a.channels_.size();++i)static_cast<void>(a.SetFromBuffer(i,initial.data(),initial.size(),format));
    std::cout<<"I";FBState(a);
    for(const auto& o:ops) {
      auto status=a.SetFromBuffer(o.index,o.null?nullptr:o.data.data(),o.size,o.format);
      std::cout<<" O "<<status;if(o.copy)std::cout<<" C "<<a.CopyBuffers();FBState(a);
    }
    return true;
  }
  if(command!="fb-color")return false;
  size_t w,h,depth,size,rx,ry,rw,rh,cw,ch,sw,sh;int shrink;bool gray,null,copy,alpha;std::string hex;
  in>>w>>h>>gray>>depth;auto format=FBReadFormat(in);
  in>>null>>copy>>size>>rx>>ry>>rw>>rh>>cw>>ch>>shrink>>sw>>sh>>alpha>>hex;
  auto data=Unhex(hex);jxl::JxlEncoderChunkedFrameAdapter input(w,h,0);
  auto bound=input.SetFromBuffer(0,null?nullptr:data.data(),size,format);
  if(copy)input.CopyBuffers();
  jxl::CodecMetadata metadata;if(!metadata.size.Set(w,h))throw std::runtime_error("buffer color dimensions");
  metadata.m.SetUintSamples(depth);metadata.m.xyb_encoded=false;
  if(!metadata.m.color_encoding.SetSRGB(gray?jxl::ColorSpace::kGray:jxl::ColorSpace::kRGB,jxl::RenderingIntent::kRelative))throw std::runtime_error("buffer color encoding");
  auto created=jxl::Image3F::Create(mm,cw,ch);if(!created.ok())throw std::runtime_error("buffer color allocation");auto color=std::move(created).value_();
  for(size_t c=0;c<3;++c)for(size_t y=0;y<ch;++y)for(size_t x=0;x<static_cast<size_t>(color.Plane(c).PixelsPerRow());++x) {uint32_t seed=0x7fa12345;memcpy(&color.Plane(c).Row(y)[x],&seed,4);}
  if(shrink>=0 && !color.Plane(shrink).ShrinkTo(sw,sh))throw std::runtime_error("buffer color shrink");
  auto status=EFCopyThunderColorChannels(input,metadata.m,jxl::Rect(rx,ry,rw,rh),&color,&alpha);
  std::cout<<bound<<' '<<static_cast<int>(status.code())<<' '<<alpha;
  for(size_t c=0;c<3;++c) {
    const auto& p=color.Plane(c);std::cout<<' '<<p.xsize()<<' '<<p.ysize()<<' '<<p.bytes_per_row();
    for(size_t y=0;y<ch;++y)for(size_t x=0;x<static_cast<size_t>(p.PixelsPerRow());++x)std::cout<<' '<<SCBits<uint32_t>(p.Row(y)[x]);
  }
  return true;
}
