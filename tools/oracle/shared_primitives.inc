// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observe original classes and the actual complete ComputeEncodingData caller.
#include "lib/jxl/chroma_from_luma.h"
#include "lib/jxl/quantizer.h"
float EFReadFloat(std::istringstream&);
void SSCorr(const jxl::ColorCorrelation& c) {
  std::cout<<" C "<<c.color_factor_<<' '<<SCBits<uint32_t>(c.color_scale_)<<' '<<SCBits<uint32_t>(c.GetColorFactor())
    <<' '<<SCBits<uint32_t>(c.GetBaseCorrelationX())<<' '<<SCBits<uint32_t>(c.GetBaseCorrelationB())<<' '<<c.GetYToXDC()<<' '<<c.GetYToBDC()<<' '<<c.IsJPEGCompatible();
  for(size_t i=0;i<4;++i)std::cout<<' '<<SCBits<uint32_t>(c.DCFactors()[i]);
}
void SSQuant(const jxl::Quantizer& q,const jxl::DequantMatrices& d,size_t slots) {
  std::cout<<" Q "<<q.global_scale_<<' '<<q.quant_dc_<<' '<<SCBits<uint32_t>(q.Scale())<<' '<<SCBits<uint32_t>(q.InvGlobalScale())<<' '<<SCBits<uint32_t>(q.inv_quant_dc());
  for(size_t c=0;c<slots;++c)std::cout<<' '<<SCBits<uint32_t>(q.MulDC()[c])<<' '<<SCBits<uint32_t>(q.InvMulDC()[c]);
  for(float value:q.zero_bias_)std::cout<<' '<<SCBits<uint32_t>(value);
  for(size_t c=0;c<3;++c)std::cout<<' '<<SCBits<uint32_t>(q.GetDcStep(c))<<' '<<SCBits<uint32_t>(q.GetInvDcStep(c));
  for(int n:{0,1,-1,64,INT32_MIN,INT32_MAX})std::cout<<' '<<SCBits<uint32_t>(q.inv_quant_ac(n));
}
template<class T> void SSPlane(const jxl::Plane<T>& p,bool samples) {
  std::cout<<" P "<<p.xsize()<<' '<<p.ysize()<<' '<<p.bytes_per_row()<<' '<<(p.memory_manager()!=nullptr);
  if(samples)for(size_t y=0;y<p.ysize();++y)for(size_t x=0;x<p.xsize();++x)std::cout<<' '<<+p.Row(y)[x];
}
jxl::Status EFComputeThunderFrame(const jxl::CompressParams&,const jxl::CodecMetadata&,jxl::JxlEncoderChunkedFrameAdapter&,jxl::FrameHeader&,jxl::ModularFrameEncoder&,jxl::PassesEncoderState&,std::vector<std::unique_ptr<jxl::BitWriter>>&,jxl::AuxOut*);
jxl::Status EFThunderOneShot(JxlMemoryManager*,const jxl::CompressParams&,const jxl::CodecMetadata&,jxl::JxlEncoderChunkedFrameAdapter&,std::vector<uint8_t>&,jxl::AuxOut*);
bool SharedPrimitivesCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if(command=="ss-quant") {
    bool def,custom,clear;int dc,global;in>>def>>dc>>global>>custom>>clear;float first[3],second[3];for(float& v:first)v=EFReadFloat(in);for(float& v:second)v=EFReadFloat(in);
    jxl::DequantMatrices d;if(custom)d.SetDCQuant(first);
    auto q=def?std::make_unique<jxl::Quantizer>(d):std::make_unique<jxl::Quantizer>(d,dc,global);
    if(clear)q->ClearDCMul();SSQuant(*q,d,clear?4:3);
    d.SetDCQuant(second);SSQuant(*q,d,clear?4:3);
    q->RecomputeFromGlobalScale();SSQuant(*q,d,clear?4:3);return true;
  }
  if(command=="ss-corr") {
    uint32_t factor;int x,b,probe,jpeg;size_t prefix;std::string hex;in>>factor>>x>>b>>probe>>jpeg>>prefix>>hex;
    jxl::ColorCorrelation c;SSCorr(c);c.SetColorFactor(factor);SSCorr(c);c.SetYToXDC(x);SSCorr(c);c.SetYToBDC(b);SSCorr(c);
    std::cout<<" R "<<SCBits<uint32_t>(c.YtoXRatio(probe))<<' '<<SCBits<uint32_t>(c.YtoBRatio(probe))<<' '<<jxl::ColorCorrelation::RatioJPEG(jpeg);
    auto bytes=Unhex(hex);std::vector<uint8_t> storage(8+bytes.size()+8);std::copy(bytes.begin(),bytes.end(),storage.begin()+8);jxl::BitReader reader(jxl::Bytes(storage.data()+8,bytes.size()));reader.ReadBits(prefix);auto status=c.DecodeDC(&reader);size_t consumed=reader.TotalBitsConsumed();auto closed=reader.Close();
    std::cout<<" D "<<static_cast<int>(status.code())<<' '<<consumed<<' '<<static_cast<int>(closed.code());SSCorr(c);return true;
  }
  if(command=="ss-map") {
    bool def,xyb;size_t w,h,prefix;std::string hex;in>>def>>w>>h>>xyb>>prefix>>hex;jxl::ColorCorrelationMap c;
    if(!def) {auto result=jxl::ColorCorrelationMap::Create(mm,w,h,xyb);std::cout<<static_cast<int>(result.status().code());if(!result.ok())return true;c=std::move(result).value_();}else std::cout<<0;
    SSPlane(c.ytox_map,true);SSPlane(c.ytob_map,true);SSCorr(c.base());
    auto bytes=Unhex(hex);std::vector<uint8_t> storage(8+bytes.size()+8);std::copy(bytes.begin(),bytes.end(),storage.begin()+8);jxl::BitReader reader(jxl::Bytes(storage.data()+8,bytes.size()));reader.ReadBits(prefix);auto status=c.DecodeDC(&reader);auto bits=reader.TotalBitsConsumed();auto closed=reader.Close();
    std::cout<<" D "<<static_cast<int>(status.code())<<' '<<bits<<' '<<static_cast<int>(closed.code());SSCorr(c.base());return true;
  }
  if(command=="ss-acs") {
    bool def,move;size_t w,h,rx,ry,rw,rh;int mode;uint32_t kind;in>>def>>w>>h>>mode>>rx>>ry>>rw>>rh>>kind>>move;jxl::AcStrategyImage c;
    if(!def) {auto result=jxl::AcStrategyImage::Create(mm,w,h);std::cout<<static_cast<int>(result.status().code());if(!result.ok())return true;c=std::move(result).value_();c.FillInvalid();if(mode==1)c.FillDCT8();if(mode==2)c.FillDCT8(jxl::Rect(rx,ry,rw,rh));}
    else std::cout<<0;
    if(move){auto moved=std::move(c);c=std::move(moved);}
    std::cout<<' '<<c.CountBlocks(static_cast<jxl::AcStrategyType>(kind));SSPlane(c.layers_,true);
    for(size_t y=0;y<c.ysize();++y)for(size_t x=0;x<c.xsize();++x)std::cout<<' '<<c.IsValid(x,y);return true;
  }
  if(command!="ss-frame")return false;
  size_t w,h;bool gray,null,has_aux,copy;unsigned depth;std::string hex;in>>w>>h>>gray>>depth>>null>>has_aux>>copy>>hex;
  jxl::CodecMetadata metadata;if(!metadata.size.Set(w,h))throw std::runtime_error("shared frame metadata");metadata.m.SetUintSamples(depth);metadata.m.xyb_encoded=false;
  if(!metadata.m.color_encoding.SetSRGB(gray?jxl::ColorSpace::kGray:jxl::ColorSpace::kRGB,jxl::RenderingIntent::kRelative))throw std::runtime_error("shared frame color");
  jxl::CompressParams p;p.cms={};p.debug_image_opaque=nullptr;p.speed_tier=jxl::SpeedTier::kThunder;p.level=10;p.buffering=0;p.SetLossless();if(!EFParamsPostInit(&p))throw std::runtime_error("shared frame params");
  jxl::PassesEncoderState state(mm);jxl::FrameHeader header(&metadata);if(!EFPrepareLosslessThunder(p,&state.progressive_splitter,&header))throw std::runtime_error("shared frame header");
  auto made=jxl::ModularFrameEncoder::Create(mm,header,p,false);if(!made.ok())throw std::runtime_error("shared frame modular");auto modular=std::move(made).value_();
  auto data=Unhex(hex);jxl::JxlEncoderChunkedFrameAdapter input(w,h,0);JxlPixelFormat f{gray?1u:3u,depth==8?JXL_TYPE_UINT8:JXL_TYPE_UINT16,JXL_NATIVE_ENDIAN,0};if(!input.SetFromBuffer(0,null?nullptr:data.data(),data.size(),f))throw std::runtime_error("shared frame buffer");if(copy)input.CopyBuffers();
  jxl::AuxOut aux;AuxSeed(aux,7);std::vector<std::unique_ptr<jxl::BitWriter>> codes;auto status=EFComputeThunderFrame(p,metadata,input,header,*modular,state,codes,has_aux?&aux:nullptr);
  std::cout<<static_cast<int>(status.code())<<' '<<header.flags<<' '<<state.shared.coeff_order_size;
  SSPlane(state.shared.ac_strategy.layers_,false);SSPlane(state.shared.raw_quant_field,false);SSPlane(state.shared.epf_sharpness,false);SSPlane(state.shared.cmap.ytox_map,true);SSPlane(state.shared.cmap.ytob_map,true);SSCorr(state.shared.cmap.base());SSPlane(state.shared.quant_dc,false);for(size_t c=0;c<3;++c)SSPlane(state.shared.dc_storage.Plane(c),false);
  SSQuant(state.shared.quantizer,state.shared.matrices,3);MFSGroupCodes(codes,mm);AuxObservation(aux);
  jxl::AuxOut full_aux;AuxSeed(full_aux,7);std::vector<uint8_t> frame;auto full=EFThunderOneShot(mm,p,metadata,input,frame,has_aux?&full_aux:nullptr);
  std::cout<<" F "<<static_cast<int>(full.code())<<' '<<Hex(jxl::Bytes(frame.data(),frame.size()));AuxObservation(full_aux);return true;
}
