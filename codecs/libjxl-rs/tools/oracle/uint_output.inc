// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Execute the pristine scalar kernel and integer buffer conversion routines.
#include "lib/jxl/dec_external_image.cc"
bool UintOutputCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm){
  if(command=="ue-kernel"){
    size_t bits,n;in>>bits>>n;auto values=SCRead<float>(in,n);values.resize(n+8,-11.0f);std::vector<uint32_t> output(n+5,0xdeadbeef);jxl::ORACLE_HWY_NS::FloatToU32(values.data(),output.data(),n,float((uint64_t{1}<<bits)-1),bits);for(auto v:output)std::cout<<v<<' ';return true;
  }
  if(command!="ue-channels"&&command!="ue-bundle")return false;
  size_t w,h,n,bits,stride,out_size;unsigned missing,orientation,endian,mode;bool gray=false,alpha=false,associated=false,unpremul=false;
  in>>w>>h>>n>>missing>>bits>>orientation>>endian>>stride>>out_size>>mode;
  if(command=="ue-bundle")in>>gray>>alpha>>associated>>unpremul;
  std::vector<uint8_t> output(1024,0xa7);jxl::PixelCallback callback;
  jxl::Status status=false;
  if(command=="ue-channels"){
    std::vector<jxl::ImageF> planes;std::vector<const jxl::ImageF*> channels;for(size_t c=0;c<n;++c){auto made=jxl::ImageF::Create(mm,w,h);if(!made.ok())throw std::runtime_error("uint plane");planes.push_back(std::move(made).value_());for(size_t y=0;y<h;++y)for(size_t x=0;x<w;++x)planes.back().Row(y)[x]=RPValue(c,x,y,0,mode);}
    for(size_t c=0;c<n;++c)channels.push_back((missing&(1u<<c))?nullptr:&planes[c]);
    status=jxl::ConvertChannelsToExternal(channels.data(),n,bits,false,static_cast<JxlEndianness>(endian),stride,nullptr,output.data(),out_size,callback,static_cast<jxl::Orientation>(orientation));
  }else{
    jxl::ImageMetadata metadata;metadata.SetAlphaBits(alpha?8:0,associated);if(!metadata.color_encoding.SetSRGB(gray?jxl::ColorSpace::kGray:jxl::ColorSpace::kRGB,jxl::RenderingIntent::kRelative))throw std::runtime_error("uint profile");jxl::ImageBundle image(mm,&metadata);auto made=jxl::Image3F::Create(mm,w,h);if(!made.ok())throw std::runtime_error("uint color");auto color=std::move(made).value_();for(size_t c=0;c<3;++c)for(size_t y=0;y<h;++y)for(size_t x=0;x<w;++x)color.PlaneRow(c,y)[x]=RPValue(gray?0:c,x,y,0,mode);if(!image.SetFromImage(std::move(color),metadata.color_encoding))throw std::runtime_error("uint image");if(alpha){auto a=jxl::ImageF::Create(mm,w,h);if(!a.ok())throw std::runtime_error("uint alpha");auto plane=std::move(a).value_();for(size_t y=0;y<h;++y)for(size_t x=0;x<w;++x)plane.Row(y)[x]=RPValue(3,x,y,0,mode);image.extra_channels().push_back(std::move(plane));}
    status=jxl::ConvertToExternal(image,bits,false,n,static_cast<JxlEndianness>(endian),stride,nullptr,output.data(),out_size,callback,static_cast<jxl::Orientation>(orientation),unpremul);
    std::cout<<" M "<<int(image.VerifyMetadata().code())<<' '<<image.HasAlpha()<<' '<<image.AlphaIsPremultiplied()<<' '<<(image.alpha()!=nullptr);
  }
  std::cout<<" S "<<int(status.code())<<' '<<Hex(jxl::Bytes(output));return true;
}
