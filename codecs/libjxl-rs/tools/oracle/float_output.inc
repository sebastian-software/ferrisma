// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Execute the pristine scalar kernel and floating buffer conversion routines.
// uint_output.inc includes the unchanged dec_external_image.cc above this file.
#if !HWY_HAVE_SCALAR_F16_OPERATORS
#error "Floating output parity requires the pinned Highway native half-cast branch"
#endif
bool FloatOutputCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm){
  if(command=="fe-kernel"){
    size_t n;in>>n;auto values=SCRead<float>(in,n);values.resize(n+8,-11.0f);std::vector<hwy::float16_t> output(n+5,hwy::float16_t::FromBits(0xdead));jxl::ORACLE_HWY_NS::FloatToF16(values.data(),output.data(),n);for(auto v:output)std::cout<<hwy::BitCastScalar<uint16_t>(v)<<' ';return true;
  }
  if(command=="fe-row"){
    size_t n,bits,w,stride,size;unsigned endian;in>>n>>bits>>endian>>w>>stride>>size;
    std::vector<jxl::ImageF> planes;std::vector<const jxl::ImageF*> channels;
    for(size_t c=0;c<n;++c){auto made=jxl::ImageF::Create(mm,w,1);if(!made.ok())throw std::runtime_error("float row");planes.push_back(std::move(made).value_());auto values=SCRead<float>(in,w);for(size_t x=0;x<w;++x)planes.back().Row(0)[x]=values[x];}
    for(const auto& plane:planes)channels.push_back(&plane);
    std::vector<uint8_t> output(1024,0xa7);jxl::PixelCallback callback;
    auto status=jxl::ConvertChannelsToExternal(channels.data(),n,bits,true,static_cast<JxlEndianness>(endian),stride,nullptr,output.data(),size,callback,jxl::Orientation::kIdentity);
    std::cout<<" S "<<int(status.code())<<' '<<Hex(jxl::Bytes(output));return true;
  }
  if(command!="fe-channels"&&command!="fe-bundle")return false;
  size_t w,h,n,bits,stride,out_size;unsigned missing,orientation,endian,mode;bool gray=false,alpha=false,associated=false,unpremul=false;
  in>>w>>h>>n>>missing>>bits>>orientation>>endian>>stride>>out_size>>mode;
  if(command=="fe-bundle")in>>gray>>alpha>>associated>>unpremul;
  std::vector<uint8_t> output(1024,0xa7);jxl::PixelCallback callback;
  jxl::Status status=false;
  if(command=="fe-channels"){
    std::vector<jxl::ImageF> planes;std::vector<const jxl::ImageF*> channels;for(size_t c=0;c<n;++c){auto made=jxl::ImageF::Create(mm,w,h);if(!made.ok())throw std::runtime_error("float plane");planes.push_back(std::move(made).value_());for(size_t y=0;y<h;++y)for(size_t x=0;x<w;++x)planes.back().Row(y)[x]=RPValue(c,x,y,0,mode);}
    for(size_t c=0;c<n;++c)channels.push_back((missing&(1u<<c))?nullptr:&planes[c]);
    status=jxl::ConvertChannelsToExternal(channels.data(),n,bits,true,static_cast<JxlEndianness>(endian),stride,nullptr,output.data(),out_size,callback,static_cast<jxl::Orientation>(orientation));
  }else{
    jxl::ImageMetadata metadata;metadata.SetAlphaBits(alpha?8:0,associated);if(!metadata.color_encoding.SetSRGB(gray?jxl::ColorSpace::kGray:jxl::ColorSpace::kRGB,jxl::RenderingIntent::kRelative))throw std::runtime_error("float profile");jxl::ImageBundle image(mm,&metadata);auto made=jxl::Image3F::Create(mm,w,h);if(!made.ok())throw std::runtime_error("float color");auto color=std::move(made).value_();for(size_t c=0;c<3;++c)for(size_t y=0;y<h;++y)for(size_t x=0;x<w;++x)color.PlaneRow(c,y)[x]=RPValue(gray?0:c,x,y,0,mode);if(!image.SetFromImage(std::move(color),metadata.color_encoding))throw std::runtime_error("float image");if(alpha){auto a=jxl::ImageF::Create(mm,w,h);if(!a.ok())throw std::runtime_error("float alpha");auto plane=std::move(a).value_();for(size_t y=0;y<h;++y)for(size_t x=0;x<w;++x)plane.Row(y)[x]=RPValue(3,x,y,0,mode);image.extra_channels().push_back(std::move(plane));}
    status=jxl::ConvertToExternal(image,bits,true,n,static_cast<JxlEndianness>(endian),stride,nullptr,output.data(),out_size,callback,static_cast<jxl::Orientation>(orientation),unpremul);
    std::cout<<" M "<<int(image.VerifyMetadata().code())<<' '<<image.HasAlpha()<<' '<<image.AlphaIsPremultiplied()<<' '<<(image.alpha()!=nullptr);
  }
  std::cout<<" S "<<int(status.code())<<' '<<Hex(jxl::Bytes(output));return true;
}
