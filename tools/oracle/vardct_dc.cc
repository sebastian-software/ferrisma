// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observe complete unchanged original DC/metadata/sigma boundaries.
#include <cstring>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include "lib/jxl/compressed_dc.cc"
#include "lib/jxl/epf.h"
#include "lib/jxl/dec_modular.h"
#include "lib/jxl/enc_modular.h"
#include "lib/jxl/modular/encoding/enc_encoding.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/enc_bit_writer.h"
#include "lib/jxl/image_metadata.h"
#include "lib/jxl/passes_state.h"
#include "lib/jxl/base/bits.h"
namespace {
float VDCFloat(std::istringstream& in){uint32_t b;in>>b;float f;memcpy(&f,&b,4);return f;}
uint32_t VDCBits(float f){uint32_t b;memcpy(&b,&f,4);return b;}
template<class T>void VDCPlane(const jxl::Plane<T>& p){std::cout<<' '<<p.xsize()<<' '<<p.ysize();for(size_t y=0;y<p.ysize();++y)for(size_t x=0;x<p.xsize();++x){if constexpr(std::is_same<T,float>::value)std::cout<<' '<<VDCBits(p.Row(y)[x]);else std::cout<<' '<<+p.Row(y)[x];}}
void VDCImage(const jxl::Image3F& image){for(size_t c=0;c<3;++c)VDCPlane(image.Plane(c));}
jxl::YCbCrChromaSubsampling VDCChroma(unsigned mode){jxl::YCbCrChromaSubsampling c;if(mode){const uint8_t h[3]={2,1,1},v[3]={uint8_t(mode==2?2:1),1,1};if(!c.Set(h,v))throw std::runtime_error("chroma");}return c;}
std::vector<uint8_t> VDCUnhex(std::string text){std::vector<uint8_t> result;if(text!="-")for(size_t i=0;i<text.size();i+=2)result.push_back(std::stoul(text.substr(i,2),nullptr,16));return result;}
void VDCHex(jxl::Span<const uint8_t> bytes){if(bytes.empty()){std::cout<<'-';return;}const char* h="0123456789abcdef";for(auto b:bytes)std::cout<<h[b>>4]<<h[b&15];}
void VDCSeed(jxl::PassesDecoderState& state){auto& s=state.shared_storage;jxl::FillImage(-19.0f,&s.dc_storage.Plane(0));jxl::FillImage(-23.0f,&s.dc_storage.Plane(1));jxl::FillImage(-29.0f,&s.dc_storage.Plane(2));jxl::FillImage(uint8_t{199},&s.quant_dc);jxl::FillImage(int32_t{-73},&s.raw_quant_field);jxl::FillImage(uint8_t{201},&s.epf_sharpness);jxl::FillImage(int8_t{-117},&s.cmap.ytox_map);jxl::FillImage(int8_t{121},&s.cmap.ytob_map);s.ac_strategy.FillInvalid();state.used_acs=0x80000000u;}
void VDCObservation(jxl::PassesDecoderState& state){auto& s=state.shared_storage;std::cout<<" U "<<state.used_acs.load();VDCImage(s.dc_storage);VDCPlane(s.quant_dc);VDCPlane(s.raw_quant_field);VDCPlane(s.epf_sharpness);VDCPlane(s.cmap.ytox_map);VDCPlane(s.cmap.ytob_map);std::cout<<" S";for(size_t y=0;y<s.ac_strategy.ysize();++y)for(size_t x=0;x<s.ac_strategy.xsize();++x){if(!s.ac_strategy.IsValid(x,y))std::cout<<" 255";else {auto ac=s.ac_strategy.ConstRow(y)[x];std::cout<<' '<<((unsigned(ac.RawStrategy())<<1)|ac.IsFirstBlock());}}if(state.sigma.xsize())VDCPlane(state.sigma);}
}
bool VarDCTDCCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm){
 if(command=="vdc-smooth"){
  size_t w,h;unsigned mode;in>>w>>h>>mode;float factors[3];for(auto& f:factors)f=VDCFloat(in);auto made=jxl::Image3F::Create(mm,w,h);if(!made.ok())throw std::runtime_error("smooth image");auto image=std::move(made).value_();for(size_t c=0;c<3;++c)for(size_t y=0;y<h;++y)for(size_t x=0;x<w;++x)image.PlaneRow(c,y)[x]=VDCFloat(in);
  auto status=jxl::AdaptiveDCSmoothing(mm,factors,&image,VDCChroma(mode),nullptr);std::cout<<int(status.code());for(size_t c=0;c<3;++c){auto cs=VDCChroma(mode);size_t width=w>2&&h>2&&!cs.Is444()?w>>cs.HShift(c):w;size_t height=w>2&&h>2&&!cs.Is444()?h>>cs.VShift(c):h;std::cout<<' '<<width<<' '<<height;for(size_t y=0;y<height;++y)for(size_t x=0;x<width;++x)std::cout<<' '<<VDCBits(image.PlaneRow(c,y)[x]);}return true;
 }
 if(command=="vdc-strategy"){
  size_t w,h,n;in>>w>>h>>n;auto made=jxl::AcStrategyImage::Create(mm,w,h);if(!made.ok())throw std::runtime_error("strategy image");auto s=std::move(made).value_();s.FillInvalid();for(size_t i=0;i<n;++i){size_t x,y;unsigned raw,kind;bool check;in>>x>>y>>raw>>kind>>check;auto status=kind?s.Set(x,y,jxl::AcStrategyType(raw)):s.SetNoBoundsCheck(x,y,jxl::AcStrategyType(raw),check);std::cout<<' '<<int(status.code());}std::cout<<" S";for(size_t y=0;y<h;++y)for(size_t x=0;x<w;++x){if(!s.IsValid(x,y))std::cout<<" 255";else {auto a=s.ConstRow(y)[x];std::cout<<' '<<((unsigned(a.RawStrategy())<<1)|a.IsFirstBlock());}}return true;
 }
 if(command=="vdc-dequant"){
  size_t ox,oy,w,h,iw,ih;unsigned mode;in>>ox>>oy>>w>>h>>iw>>ih>>mode;auto chroma=VDCChroma(mode);float factors[3];for(auto& f:factors)f=VDCFloat(in);float mul=VDCFloat(in),cfl[3]={VDCFloat(in),0,VDCFloat(in)};jxl::BlockCtxMap b;in>>b.num_dc_ctxs;for(auto& v:b.dc_thresholds){size_t n;in>>n;v.resize(n);for(auto& t:v)in>>t;}
  auto dm=jxl::Image3F::Create(mm,iw,ih);auto qm=jxl::ImageB::Create(mm,iw,ih);auto im=jxl::Image::Create(mm,w,h,8,3);if(!dm.ok()||!qm.ok()||!im.ok())throw std::runtime_error("dequant owners");auto dc=std::move(dm).value_();auto q=std::move(qm).value_();auto image=std::move(im).value_();for(size_t c=0;c<3;++c){jxl::FillImage(-19.0f-float(c),&dc.Plane(c));auto& ch=image.channel[c<2?c^1:c];ch.w>>=chroma.HShift(c);ch.h>>=chroma.VShift(c);if(!ch.shrink())throw std::runtime_error("dequant shrink");for(size_t y=0;y<ch.h;++y)for(size_t x=0;x<ch.w;++x)in>>ch.Row(y)[x];}jxl::FillImage(uint8_t{199},&q);jxl::DequantDC(jxl::Rect(ox,oy,w,h),&dc,&q,image,factors,mul,cfl,chroma,b);std::cout<<"D";VDCImage(dc);VDCPlane(q);return true;
 }
 if(command=="vdc-source"){
  size_t w,h;unsigned kind,raw,mutation,precision,mode=0;in>>w>>h>>kind>>raw>>mutation>>precision;if(!in.eof()){in>>std::ws;if(!in.eof())in>>mode;}auto im=jxl::Image::Create(mm,w,h,8,kind?4:3);if(!im.ok())throw std::runtime_error("source image");auto image=std::move(im).value_();size_t count=w*h;
  if(!kind){auto cs=VDCChroma(mode);for(size_t c=0;c<3;++c){auto& ch=image.channel[c<2?c^1:c];ch.w>>=cs.HShift(c);ch.h>>=cs.VShift(c);if(!ch.shrink())throw std::runtime_error("source DC shrink");}}
  if(kind){auto ac=jxl::AcStrategy::FromRawStrategy(jxl::AcStrategyType(raw));count=(w/ac.covered_blocks_x())*(h/ac.covered_blocks_y());auto a=jxl::Channel::Create(mm,(w+7)/8,(h+7)/8,3,3);auto b=jxl::Channel::Create(mm,(w+7)/8,(h+7)/8,3,3);auto c=jxl::Channel::Create(mm,count,2,0,0);if(!a.ok()||!b.ok()||!c.ok())throw std::runtime_error("source channels");image.channel[0]=std::move(a).value_();image.channel[1]=std::move(b).value_();image.channel[2]=std::move(c).value_();}
  for(size_t c=0;c<image.channel.size();++c)for(size_t y=0;y<image.channel[c].h;++y)for(size_t x=0;x<image.channel[c].w;++x){int32_t v=int32_t((x*17+y*11+c*37)%101)-50;if(kind){if(c==0)v=int32_t(x*131+y*19)-160;if(c==1)v=180-int32_t(x*97+y*31);if(c==2)v=y==0?int32_t(raw):int32_t(x*89)-17;if(c==3)v=int32_t((x+y)%8);if(mutation==1&&c==2&&y==0&&x==count-1)v=27;if(mutation==2&&c==3&&x==w-1&&y==h-1)v=8;if(mutation==3&&c==2&&y==0&&x==0)v=26;if(mutation==4&&c==2&&y==0&&x==count-1)v=26;}image.channel[c].Row(y)[x]=v;}
  jxl::ModularOptions options;jxl::BitWriter writer(mm);size_t bits=kind?jxl::CeilLog2Nonzero(w*h):2;size_t val=kind?count-1:precision;if(!writer.WithMaxBits(bits,jxl::LayerType::Header,nullptr,[&]{writer.Write(bits,val);return true;}))throw std::runtime_error("source prefix");if(!jxl::ModularGenericCompress(image,options,writer,nullptr,jxl::LayerType::AcTokens,0))throw std::runtime_error("source compress");writer.ZeroPadToByte();VDCHex(writer.GetSpan());return true;
 }
 if(command=="vdc-global"){
  auto im=jxl::Image::Create(mm,8,8,8,0);if(!im.ok())throw std::runtime_error("global image");auto image=std::move(im).value_();jxl::ModularOptions options;jxl::BitWriter writer(mm);if(!writer.WithMaxBits(1,jxl::LayerType::Header,nullptr,[&]{writer.Write(1,0);return true;}))throw std::runtime_error("global prefix");if(!jxl::ModularGenericCompress(image,options,writer,nullptr,jxl::LayerType::ModularGlobal,0))throw std::runtime_error("global compress");writer.ZeroPadToByte();VDCHex(writer.GetSpan());return true;
 }
 if(command=="vdc-read"){
  size_t w,h,group,offset,repeat;unsigned kind,mode,epf;std::string global,payload;in>>w>>h>>group>>kind>>mode>>epf>>offset>>repeat>>global>>payload;jxl::CodecMetadata metadata;if(!metadata.size.Set(w*8,h*8))throw std::runtime_error("read dimensions");metadata.m.SetUintSamples(8);jxl::FrameHeader header(&metadata);header.encoding=jxl::FrameEncoding::kVarDCT;header.chroma_subsampling=VDCChroma(mode);header.loop_filter.epf_iters=epf;auto dim=header.ToFrameDimensions();jxl::PassesDecoderState state(mm);if(!jxl::InitializePassesSharedState(header,&state.shared_storage,false))throw std::runtime_error("read shared");VDCSeed(state);if(epf){auto made=jxl::ImageF::Create(mm,dim.xsize_blocks+4,dim.ysize_blocks+4);if(!made.ok())throw std::runtime_error("sigma");state.sigma=std::move(made).value_();jxl::FillImage(-17.0f,&state.sigma);}
  jxl::ModularFrameDecoder decoder(mm);decoder.Init(dim);auto gb=VDCUnhex(global);std::vector<uint8_t> gback(gb.size()+16);std::copy(gb.begin(),gb.end(),gback.begin()+8);jxl::BitReader gr(jxl::Bytes(gback.data()+8,gb.size()));auto gs=decoder.DecodeGlobalInfo(&gr,header,false);auto gc=gr.Close();if(!gs||!gc)throw std::runtime_error("read global");auto bytes=VDCUnhex(payload);std::vector<uint8_t> backing(bytes.size()+16);std::copy(bytes.begin(),bytes.end(),backing.begin()+8);
  for(size_t i=0;i<repeat;++i){jxl::BitReader br(jxl::Bytes(backing.data()+8,bytes.size()));br.SkipBits(offset);auto status=kind?decoder.DecodeAcMetadata(header,group,&br,&state):decoder.DecodeVarDCTDC(header,group,&br,&state);auto bits=br.TotalBitsConsumed();auto close=br.Close();std::cout<<" R "<<int(status.code())<<' '<<bits<<' '<<int(close.code());VDCObservation(state);}return true;
 }
 return false;
}
#ifdef VDC_STANDALONE
#include "lib/jxl/memory_manager_internal.h"
int main(){JxlMemoryManager manager;if(!jxl::MemoryManagerInit(&manager,nullptr))return 2;auto* mm=&manager;std::string line;while(std::getline(std::cin,line)){std::istringstream in(line);std::string command;in>>command;if(!VarDCTDCCommand(command,in,mm))throw std::runtime_error("unknown command");std::cout<<'\n';}}
#endif
