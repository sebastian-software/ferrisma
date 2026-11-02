// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Independent observations of original moves and actual public write factories.
#include <array>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <memory>
#include <sstream>
#include <string>
#include <vector>
#include "lib/jxl/image_bundle.h"
#include "lib/jxl/jpeg/jpeg_data.h"
#include "lib/jxl/passes_state.h"
#include "lib/jxl/render_pipeline/stage_write.h"
void SSBundle(const jxl::ImageBundle&);
namespace {
uint32_t Word(size_t c,size_t x,size_t y,size_t pass) {
  static const uint32_t values[]={0,0x80000000,1,0x80000001,0x7f7fffff,0xff7fffff,0x7f800000,0xff800000,0x7fa12345,0xffa54321,0x7fc98765,0xffc76543,0x3f800000};
  return values[(c*7+x*3+y*11+pass)%13];
}
float Value(size_t c,size_t x,size_t y,size_t pass) {auto bits=Word(c,x,y,pass);float value;std::memcpy(&value,&bits,4);return value;}
void Pixels(const jxl::Image3F& image) {
  std::cout<<" I "<<image.xsize()<<' '<<image.ysize();
  for(size_t c=0;c<3;++c)for(size_t y=0;y<image.ysize();++y)for(size_t x=0;x<image.xsize();++x){uint32_t v;auto f=image.PlaneRow(c,y)[x];std::memcpy(&v,&f,4);std::cout<<' '<<v;}
}
void Extras(const jxl::ImageBundle& image) {
  std::cout<<" X "<<image.extra_channels().size();
  for(const auto& ec:image.extra_channels()){std::cout<<' '<<ec.xsize()<<' '<<ec.ysize();for(size_t y=0;y<ec.ysize();++y)for(size_t x=0;x<ec.xsize();++x){uint32_t v;auto f=ec.Row(y)[x];std::memcpy(&v,&f,4);std::cout<<' '<<v;}}
}
void Fill(jxl::Image3F& image,size_t pass) {for(size_t c=0;c<3;++c)for(size_t y=0;y<image.ysize();++y)for(size_t x=0;x<image.xsize();++x)image.PlaneRow(c,y)[x]=Value(c,x,y,pass);}
jxl::Image3F Make(JxlMemoryManager* mm,size_t w,size_t h) {auto made=jxl::Image3F::Create(mm,w,h);if(!made.ok())throw std::runtime_error("owner image");return std::move(made).value_();}
void Init(jxl::ImageBundle& b,JxlMemoryManager* mm,const jxl::ImageMetadata& metadata,size_t w,size_t h,size_t extras,size_t profile,size_t name,size_t seed,bool jpeg) {
  auto color=Make(mm,w+2,h+1);Fill(color,seed);if(!color.ShrinkTo(w,h))throw std::runtime_error("owner shrink");
  jxl::ColorEncoding c=profile%2?jxl::ColorEncoding::LinearSRGB(metadata.color_encoding.IsGray()):jxl::ColorEncoding::SRGB(metadata.color_encoding.IsGray());
  if(profile==2){jxl::IccBytes raw={0,1,127,128,255,31};c.SetICCRaw(std::move(raw));}
  if(!b.SetFromImage(std::move(color),c))throw std::runtime_error("owner bundle");
  for(size_t e=0;e<extras;++e){auto made=jxl::ImageF::Create(mm,w,h);if(!made.ok())throw std::runtime_error("owner extra");auto ec=std::move(made).value_();for(size_t y=0;y<h;++y)for(size_t x=0;x<w;++x)ec.Row(y)[x]=Value(e+3,x,y,seed);b.extra_channels().push_back(std::move(ec));}
  b.color_transform=static_cast<jxl::ColorTransform>(profile%3);uint8_t hs[]={uint8_t(1+(profile%2)),1,1},vs[]={uint8_t(1+(profile==2)),1,1};if(!b.chroma_subsampling.Set(hs,vs))throw std::runtime_error("owner subsampling");
  b.origin={-int32_t(seed+3),int32_t(seed+7)};b.duration=seed*313;b.timecode=seed*117;b.use_for_next_frame=seed%2;b.blend=!(seed%2);b.blendmode=jxl::BlendMode::kAdd;b.name.resize(name);for(size_t i=0;i<name;++i)b.name[i]=char((i*37+seed)%256);b.SetDecodedBytes(seed*1931);
  if(jpeg){b.jpeg_data=std::make_unique<jxl::jpeg::JPEGData>();b.jpeg_data->width=w+11;b.jpeg_data->height=h+13;}
}
}
bool RetainedFrameOwnersCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
 if(command=="ow-move") {
  size_t w,h,extras,profile,name;bool gray,jpeg,construct;in>>w>>h>>extras>>profile>>name>>gray>>jpeg>>construct;
  jxl::ImageMetadata metadata,old_metadata;metadata.color_encoding=gray?jxl::ColorEncoding::SRGB(true):jxl::ColorEncoding::SRGB();
  jxl::ImageBundle source(mm,&metadata),destination(mm,&old_metadata);Init(source,mm,metadata,w,h,extras,profile,name,7,jpeg);Init(destination,mm,old_metadata,2,3,1,0,5,2,false);
  auto* identity=&destination;auto* allocation=source.color_.Plane(0).bytes();if(construct){auto intermediate=std::move(source);destination=std::move(intermediate);}else destination=std::move(source);
  std::cout<<(&destination==identity)<<' '<<(destination.metadata()==&metadata)<<' '<<(destination.color_.Plane(0).bytes()==allocation)<<' '<<(source.color_.Plane(0).bytes()==allocation);
  SSBundle(destination);Pixels(destination.color_);Extras(destination);SSBundle(source);Pixels(source.color_);
  auto ds=destination.color_.ShrinkTo(w+2,h+1);auto ss=source.color_.ShrinkTo(w+2,h+1);std::cout<<" G "<<int(ds.code())<<' '<<int(ss.code());SSBundle(destination);Pixels(destination.color_);SSBundle(source);
  destination.RemoveColor();SSBundle(destination);Extras(destination);std::cout<<" S "<<destination.xsize()<<' '<<destination.ysize()<<' '<<destination.HasColor();return true;
 }
 if(command!="ow-write")return false;
 size_t kind,slot,w,h,channels,guard;in>>kind>>slot>>w>>h>>channels>>guard;
 jxl::ImageMetadata metadata;metadata.color_encoding=jxl::ColorEncoding::SRGB();metadata.num_extra_channels=channels-3;metadata.extra_channel_info.resize(channels-3);
 auto dc=std::make_unique<std::array<jxl::Image3F,4>>();auto owned=std::make_unique<jxl::Image3F>();
 auto bundle=std::make_unique<jxl::ImageBundle>(mm,&metadata);jxl::PassesSharedState shared(mm);for(auto& ref:shared.reference_frames)*ref.frame=jxl::ImageBundle(mm,&metadata);
 jxl::Image3F* image=kind==0?owned.get():&(*dc)[slot];jxl::ImageBundle* ib=kind==2?bundle.get():shared.reference_frames[slot].frame.get();
 auto* identity=kind<2?static_cast<void*>(image):static_cast<void*>(ib);
 jxl::ColorEncoding write_profile=guard==6?jxl::ColorEncoding::SRGB(true):metadata.color_encoding;
 if(kind>=2&&(guard==5||guard==10)){auto made=jxl::ImageF::Create(mm,guard==5?w+1:w,h);if(!made.ok())throw std::runtime_error("old EC");ib->extra_channels().push_back(std::move(made).value_());}
 auto stage=kind<2?jxl::GetWriteToImage3FStage(mm,image):jxl::GetWriteToImageBundleStage(ib,&write_profile);
 std::vector<std::pair<size_t,size_t>> sizes(channels,{w,h});if(guard==3)sizes[1].first++;if(guard==4&&channels>3)sizes.back().second++;
 if(guard==9)sizes.resize(2);auto status=stage->SetInputSizes(sizes);std::cout<<int(status.code())<<' '<<stage->GetName();for(size_t c=0;c<channels+1;++c)std::cout<<' '<<int(stage->GetChannelMode(c));
 if(!status){if(kind<2)std::cout<<" I "<<image->xsize()<<' '<<image->ysize();else SSBundle(*ib);return true;}
 if(w==0||h==0){if(kind<2)Pixels(*image);else SSBundle(*ib);return true;}
 // Move the enclosing owners while retaining the original object addresses.
 auto moved_dc=std::move(dc);auto moved_owned=std::move(owned);auto moved_bundle=std::move(bundle);
 std::cout<<' '<<(identity==(kind<2?static_cast<void*>(image):static_cast<void*>(ib)));
 for(size_t pass=0;pass<2;++pass){
  if(pass){if(kind<2)*image=Make(mm,w,h);else{jxl::ImageBundle replacement(mm,&metadata);Init(replacement,mm,metadata,w,h,channels-3,1,23,3,false);*ib=std::move(replacement);}}
  if(kind>=2&&guard==7&&channels>3){auto made=jxl::ImageF::Create(mm,w-1,h);if(!made.ok())throw std::runtime_error("short EC");ib->extra_channels().back()=std::move(made).value_();}
  auto& color=kind<2?*image:ib->color_;if(kind<2)Fill(*image,pass+5);else{Fill(ib->color_,pass+5);for(size_t e=0;e<ib->extra_channels().size();++e)for(size_t y=0;y<h;++y)for(size_t x=0;x<ib->extra_channels()[e].xsize();++x)ib->extra_channels()[e].Row(y)[x]=Value(e+3,x,y,pass+5);}
  std::vector<std::vector<float>> rows(channels,std::vector<float>(jxl::kRenderPipelineXOffset+w));jxl::RenderPipelineStage::RowInfo input(channels);
  for(size_t c=0;c<channels;++c){input[c]={rows[c].data()};for(size_t x=0;x<w;++x)rows[c][jxl::kRenderPipelineXOffset+x]=Value(c,x,pass,1);}
  size_t x=w>2?1:0,width=w>2?w-2:w;if(guard==7){x=0;width=w;}if(guard==8){x=w;width=0;}auto row_status=stage->ProcessRow(input,{},guard==1,guard==2,width,x,pass%h,0);std::cout<<" R "<<int(row_status.code());Pixels(color);if(kind>=2){SSBundle(*ib);Extras(*ib);}
 }
 return true;
}
