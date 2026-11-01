// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Original storage/field APIs only. Explicitly paint caller-owned bytes before
// observing them; a cloned image exposes valid samples, never uninitialized pad.
#include "lib/jxl/image.h"
#include "lib/jxl/image_ops.h"
#include "lib/jxl/modular/modular_image.h"
#include "lib/jxl/modular/options.h"
#include "lib/jxl/modular/transform/transform.h"

template<class T> void ImagePaint(jxl::Plane<T>& p,uint64_t seed) {
  if(!p.bytes()) return;size_t n=p.bytes_per_row()*p.ysize();
  for(size_t i=0;i<n;++i) p.bytes()[i]=static_cast<uint8_t>((i*17+seed*13)^(i>>5));
}
template<class T> void ImagePlaneObservation(const jxl::Plane<T>& p,size_t capacity_rows,bool full) {
  std::cout<<' '<<p.xsize()<<','<<p.ysize()<<','<<p.bytes_per_row()<<','<<p.PixelsPerRow()<<','<<(p.bytes()!=nullptr)<<','<<(reinterpret_cast<uintptr_t>(p.bytes())%128);
  std::vector<uint8_t> samples;
  if(p.bytes()) {
    if(full) samples.assign(p.bytes(),p.bytes()+capacity_rows*p.bytes_per_row());
    else for(size_t y=0;y<p.ysize();++y) {const uint8_t* row=reinterpret_cast<const uint8_t*>(p.Row(y));samples.insert(samples.end(),row,row+p.xsize()*sizeof(T));}
  }
  std::cout<<' '<<Hex(jxl::Bytes(samples));
}
template<class T> void ImagePlaneCommand(std::istringstream& in,JxlMemoryManager* mm) {
  size_t x,y,pre,sx,sy,ux,uy;in>>x>>y>>pre>>sx>>sy>>ux>>uy;
  auto created=jxl::Plane<T>::Create(mm,x,y,pre);std::cout<<Code(created.status());if(!created.ok()) return;
  auto p=std::move(created).value_();ImagePaint(p,7);ImagePlaneObservation(p,y,true);
  Status s=p.ShrinkTo(sx,sy);std::cout<<' '<<Code(s);ImagePlaneObservation(p,y,true);
  s=p.ShrinkTo(ux,uy);std::cout<<' '<<Code(s);ImagePlaneObservation(p,y,true);
  jxl::Plane<T> empty;p.Swap(empty);ImagePlaneObservation(p,0,true);
  jxl::Plane<T> moved(std::move(empty));ImagePlaneObservation(moved,y,true);
}
template<class T> void Image3Command(std::istringstream& in,JxlMemoryManager* mm) {
  size_t x,y,sx,sy,ux,uy;in>>x>>y>>sx>>sy>>ux>>uy;
  auto created=jxl::Image3<T>::Create(mm,x,y);std::cout<<Code(created.status());if(!created.ok()) return;
  auto p=std::move(created).value_();for(size_t c=0;c<3;++c) ImagePaint(p.Plane(c),10+c);
  Status s=p.ShrinkTo(sx,sy);std::cout<<' '<<Code(s);s=p.ShrinkTo(ux,uy);std::cout<<' '<<Code(s);
  jxl::Image3<T> empty;p.Swap(empty);jxl::Image3<T> moved(std::move(empty));
  for(size_t c=0;c<3;++c) {ImagePlaneObservation(p.Plane(c),0,true);ImagePlaneObservation(moved.Plane(c),y,true);}
}
template<class T> T ImageValue(uint64_t bits) {T value;std::memcpy(&value,&bits,sizeof(T));return value;}
template<class T> void ImageCopyCommand(std::istringstream& in,JxlMemoryManager* mm) {
  unsigned mode;size_t w,h,tw,th,x,y,rw,rh,tx,ty,padding;uint64_t value;
  in>>mode>>w>>h>>tw>>th>>x>>y>>rw>>rh>>tx>>ty>>padding>>value;
  auto a=jxl::Plane<T>::Create(mm,w,h),b=jxl::Plane<T>::Create(mm,tw,th);
  std::cout<<Code(a.status())<<' '<<Code(b.status());if(!a.ok() || !b.ok()) return;
  auto from=std::move(a).value_(),to=std::move(b).value_();ImagePaint(from,11);ImagePaint(to,19);
  jxl::Rect rf(x,y,rw,rh),rt(tx,ty,rw,rh);Status status=true;
  if(mode==0) status=jxl::CopyImageTo(from,&to);
  if(mode==1) status=jxl::CopyImageTo(rf,from,rt,&to);
  if(mode==2) status=jxl::CopyImageToWithPadding(rf,from,padding,rt,&to);
  if(mode==3) jxl::FillImage(ImageValue<T>(value),&to);
  if(mode==4) jxl::ZeroFillImage(&to);
  if(mode==5) jxl::FillPlane(ImageValue<T>(value),&to,rt);
  std::cout<<' '<<Code(status);ImagePlaneObservation(from,h,true);ImagePlaneObservation(to,th,true);
}
void ImageTransformObservation(const jxl::Transform& t) {
  std::cout<<' '<<unsigned(t.id)<<','<<t.begin_c<<','<<t.rct_type<<','<<t.num_c<<','<<t.nb_colors<<','<<t.nb_deltas<<','<<t.max_delta_error<<','<<unsigned(t.predictor)<<','<<t.ordered_palette<<','<<t.lossy_palette;
  std::cout<<' '<<t.squeezes.size();for(const auto& s:t.squeezes) std::cout<<' '<<s.horizontal<<','<<s.in_place<<','<<s.begin_c<<','<<s.num_c;
}
void ImageChannelObservation(const jxl::Channel& c,size_t rows,bool full) {
  std::cout<<' '<<c.w<<','<<c.h<<','<<c.hshift<<','<<c.vshift<<','<<c.component;ImagePlaneObservation(c.plane,rows,full);
}
void ImageModularObservation(const jxl::Image& image) {
  std::cout<<' '<<image.w<<','<<image.h<<','<<image.bitdepth<<','<<image.nb_meta_channels<<','<<image.error<<','<<image.empty();
  std::cout<<' '<<image.channel.size();for(const auto& c:image.channel) ImageChannelObservation(c,c.plane.ysize(),false);
  std::cout<<' '<<image.transform.size();for(const auto& t:image.transform) ImageTransformObservation(t);
}
bool ImageCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if(command=="image-pad" || command=="image-pad-partial") {
    size_t cw,ch,w,h,block; in>>cw>>ch>>w>>h>>block;
    auto result=jxl::Image3F::Create(mm,cw,ch);
    if(!result.ok()) throw std::runtime_error("padding fixture allocation");
    auto image=std::move(result).value_();
    const bool partial=command=="image-pad-partial";
    if(partial) {
      auto replacement=jxl::ImageF::Create(mm,20,12);
      if(!replacement.ok()) throw std::runtime_error("padding fixture replacement");
      auto plane=std::move(replacement).value_(); image.Plane(1).Swap(plane);
    }
    for(size_t c=0;c<3;++c) ImagePaint(image.Plane(c),51+c);
    auto initial=image.ShrinkTo(w,h);
    std::cout<<Code(initial);
    if(initial) for(unsigned phase=0;phase<2;++phase) {
      auto status=jxl::PadImageToBlockMultipleInPlace(&image,block);
      std::cout<<' '<<Code(status);
      for(size_t c=0;c<3;++c) ImagePlaneObservation(image.Plane(c),partial&&c==1?12:ch,true);
      if(image.xsize()==0 || image.ysize()==0) break; // no second call with source-undefined empty input
    }
  } else if(command=="image-downsample") {
    size_t w,h,factor; bool three; unsigned mode;
    in>>three>>w>>h>>factor>>mode;
    auto result=jxl::Image3F::Create(mm,mode>=2?w+8:w,mode>=2?h+8:h);
    if(!result.ok()) throw std::runtime_error("downsample fixture allocation");
    auto image=std::move(result).value_();
    if(mode>=2 && !image.ShrinkTo(w,h)) throw std::runtime_error("downsample fixture shrink");
    const uint32_t values[]={0,0x80000000,1,0x80000001,0x3f800000,0xbf800000,
      0x7f7fffff,0xff7fffff,0x7f800000,0xff800000,0x7fc12345,0x7fa23456};
    for(size_t c=0;c<3;++c) for(size_t y=0;y<h;++y) for(size_t x=0;x<w;++x) {
      if(mode%2==0) image.PlaneRow(c,y)[x]=static_cast<float>((x*13+y*67+c*997)%4093)-2051.f;
      else { uint32_t bits=values[(x+y*7+c*3)%12]; memcpy(&image.PlaneRow(c,y)[x],&bits,4); }
    }
    if(three) {
      auto down=jxl::DownsampleImage(image,factor); std::cout<<Code(down.status());
      if(down.ok()) { auto output=std::move(down).value_(); for(size_t c=0;c<3;++c) ImagePlaneObservation(output.Plane(c),output.ysize(),false); }
    } else {
      auto down=jxl::DownsampleImage(image.Plane(0),factor); std::cout<<Code(down.status());
      if(down.ok()) { auto output=std::move(down).value_(); ImagePlaneObservation(output,output.ysize(),false); }
    }
    for(size_t c=0;c<3;++c) ImagePlaneObservation(image.Plane(c),h,false);
  } else if(command=="image-row-size") {
    size_t x,size;in>>x>>size;auto result=jxl::BytesPerRow(x,size);std::cout<<Code(result.status())<<' '<<(result.ok()?std::move(result).value_():0);
  } else if(command=="image-plane" || command=="image-three" || command=="image-copy") {
    unsigned type;in>>type;
    if(command=="image-plane") {switch(type) {case 0:ImagePlaneCommand<uint8_t>(in,mm);break;case 1:ImagePlaneCommand<int16_t>(in,mm);break;case 2:ImagePlaneCommand<int32_t>(in,mm);break;case 3:ImagePlaneCommand<float>(in,mm);break;case 4:ImagePlaneCommand<uint64_t>(in,mm);break;case 5:ImagePlaneCommand<uint16_t>(in,mm);break;default:ImagePlaneCommand<double>(in,mm);}}
    if(command=="image-three") {switch(type) {case 0:Image3Command<uint8_t>(in,mm);break;case 1:Image3Command<int16_t>(in,mm);break;case 2:Image3Command<int32_t>(in,mm);break;case 3:Image3Command<float>(in,mm);break;case 4:Image3Command<uint64_t>(in,mm);break;case 5:Image3Command<uint16_t>(in,mm);break;default:Image3Command<double>(in,mm);}}
    if(command=="image-copy") {switch(type) {case 0:ImageCopyCommand<uint8_t>(in,mm);break;case 1:ImageCopyCommand<int16_t>(in,mm);break;case 2:ImageCopyCommand<int32_t>(in,mm);break;case 3:ImageCopyCommand<float>(in,mm);break;case 4:ImageCopyCommand<uint64_t>(in,mm);break;case 5:ImageCopyCommand<uint16_t>(in,mm);break;default:ImageCopyCommand<double>(in,mm);}}
  } else if(command=="image-three-partial" || command=="image-three-stride") {
    bool partial=command=="image-three-partial";auto created=jxl::Image3I::Create(mm,partial?17:1,partial?9:3);if(!created.ok()) throw std::runtime_error("three create");auto p=std::move(created).value_();
    auto replacement=jxl::ImageI::Create(mm,partial?3:256,partial?2:3);if(!replacement.ok()) throw std::runtime_error("three replacement");auto replacement_plane=std::move(replacement).value_();p.Plane(1).Swap(replacement_plane);
    for(size_t c=0;c<3;++c) ImagePaint(p.Plane(c),21+c);
    Status status=partial?p.ShrinkTo(7,4):Status(true);std::cout<<Code(status);
    for(size_t c=0;c<3;++c) ImagePlaneObservation(p.Plane(c),partial?(c==1?2:9):3,true);
    if(!partial) for(size_t c=0;c<3;++c) for(size_t y=0;y<3;++y) std::cout<<' '<<p.PlaneRow(c,y)[0];
  } else if(command=="image-three-ops") {
    auto a=jxl::Image3I::Create(mm,9,4),b=jxl::Image3I::Create(mm,9,4);if(!a.ok()||!b.ok()) throw std::runtime_error("three ops allocation");auto from=std::move(a).value_(),to=std::move(b).value_();
    for(size_t c=0;c<3;++c) {ImagePaint(from.Plane(c),61+c);ImagePaint(to.Plane(c),67+c);}
    std::cout<<"P";
    for(unsigned phase=0;phase<5;++phase) {
      Status status=true;
      if(phase==0) status=jxl::CopyImageTo(from,&to);
      if(phase==1) jxl::FillImage(int32_t{1234},&to);
      if(phase==2) jxl::ZeroFillImage(&to);
      if(phase==3) status=jxl::CopyImageTo(jxl::Rect(1,1,3,2),from,jxl::Rect(2,1,3,2),&to);
      if(phase==4) {if(!to.Plane(1).ShrinkTo(1,1)) throw std::runtime_error("three ops shrink");status=jxl::CopyImageTo(jxl::Rect(0,0,9,4),from,jxl::Rect(0,0,9,4),&to);}
      std::cout<<' '<<Code(status);for(size_t c=0;c<3;++c) ImagePlaneObservation(to.Plane(c),4,true);
    }
  } else if(command=="image-channel") {
    size_t w,h;int hs,vs,nw,nh;in>>w>>h>>hs>>vs>>nw>>nh;auto result=jxl::Channel::Create(mm,w,h,hs,vs);std::cout<<Code(result.status());if(!result.ok()) return true;
    auto c=std::move(result).value_();ImagePaint(c.plane,31);ImageChannelObservation(c,h,true);
    bool changed=c.plane.xsize()!=static_cast<size_t>(nw)||c.plane.ysize()!=static_cast<size_t>(nh);Status status=c.shrink(nw,nh);
    size_t rows=h;if(status&&changed) {rows=c.plane.ysize();ImagePaint(c.plane,37);}std::cout<<' '<<Code(status);ImageChannelObservation(c,rows,true);
  } else if(command=="image-modular") {
    size_t w,h,meta;int depth,n;unsigned variant;in>>w>>h>>depth>>n>>meta>>variant;
    auto result=jxl::Image::Create(mm,w,h,depth,n);std::cout<<Code(result.status());if(!result.ok()) return true;auto image=std::move(result).value_();image.nb_meta_channels=meta;
    for(auto& c:image.channel) ImagePaint(c.plane,41+c.component);
    if(image.channel.size()>1) {if(variant==1) image.channel[1].w=0;if(variant==2) image.channel[1].hshift=3;}
    jxl::Transform t(jxl::TransformId::kRCT);t.max_delta_error=77;t.squeezes.resize(1);t.squeezes[0].horizontal=true;image.transform.push_back(t);
    ImageModularObservation(image);
    for(auto pair:{std::pair<uint32_t,uint32_t>{0,0},{0,1},{1,2},{0,3},{2,1},{~0u,~0u}}) std::cout<<' '<<Code(jxl::CheckEqualChannels(image,pair.first,pair.second));
    auto clone=jxl::Image::Clone(image);std::cout<<' '<<Code(clone.status());if(clone.ok()) ImageModularObservation(std::move(clone).value_());
  } else if(command=="image-transform" || command=="image-transform-read") {
    jxl::Transform t(jxl::TransformId::kRCT);t.max_delta_error=77;t.ordered_palette=false;t.lossy_palette=true;size_t prefix;
    if(command=="image-transform-read") {
      t.begin_c=21;t.rct_type=17;t.num_c=7;t.nb_colors=99;t.nb_deltas=8;t.predictor=jxl::Predictor::Select;t.squeezes.resize(2);
      std::string hex;in>>prefix>>hex;Input input(hex);BitReader br(input.bytes());br.SkipBits(prefix);Status read=jxl::Bundle::Read(&br,&t);std::cout<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());ImageTransformObservation(t);
    } else {
      unsigned id,predictor;size_t count;in>>id>>t.begin_c>>t.rct_type>>t.num_c>>t.nb_colors>>t.nb_deltas>>predictor>>count>>prefix;t.id=static_cast<jxl::TransformId>(id);t.predictor=static_cast<jxl::Predictor>(predictor);t.squeezes.resize(count);
      for(size_t i=0;i<count;++i) {auto& s=t.squeezes[i];s.horizontal=i%2;s.in_place=(i/2)%2;s.begin_c=i%3;s.num_c=2;}
      size_t ext=0,bits=0;Status can=jxl::Bundle::CanEncode(t,&ext,&bits);std::cout<<Code(can)<<' '<<ext<<' '<<bits;
      if(can) {BitWriter writer(mm);if(!Prefix(writer,prefix)) throw std::runtime_error("transform prefix");Status write=jxl::Bundle::Write(t,&writer,LayerType::Header,nullptr);std::cout<<' '<<Code(write)<<' '<<writer.BitsWritten();if(!Pad(writer)) throw std::runtime_error("transform pad");std::cout<<' '<<Hex(writer.GetSpan());jxl::Transform decoded;decoded.max_delta_error=77;BitReader br(writer.GetSpan());br.SkipBits(prefix);Status read=jxl::Bundle::Read(&br,&decoded);std::cout<<' '<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());ImageTransformObservation(decoded);}
      ImageTransformObservation(t);
    }
  } else if(command=="image-options") {
    int32_t a,b;in>>a>>b;std::cout<<jxl::PixelAdd(a,b);jxl::ModularOptions o;
    std::cout<<' '<<o.max_chan_size<<','<<o.group_dim<<','<<FloatBits(o.nb_repeats)<<','<<o.max_properties<<','<<FloatBits(o.splitting_heuristics_node_threshold)<<','<<o.max_property_values<<','<<unsigned(o.predictor)<<','<<o.wp_mode<<','<<FloatBits(o.fast_decode_multiplier)<<','<<unsigned(o.wp_tree_mode)<<','<<o.skip_encoder_fast_path<<','<<unsigned(o.tree_kind)<<','<<o.zero_tokens;
    TocValues(o.splitting_heuristics_properties);
    const auto& p=o.histogram_params;std::cout<<' '<<unsigned(p.clustering)<<','<<unsigned(p.uint_method)<<','<<unsigned(p.lz77_method)<<','<<unsigned(p.ans_histogram_strategy)<<','<<p.max_histograms<<','<<p.force_huffman<<','<<p.initialize_global_state<<','<<p.streaming_mode<<','<<p.add_missing_symbols<<','<<p.add_fixed_histograms;TocValues(p.image_widths);
  } else if(command=="image-mirror") {int64_t x,size;in>>x>>size;std::cout<<jxl::Mirror(x,size);}
  else return false;
  return true;
}
