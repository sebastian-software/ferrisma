// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Genuine complete-file fixtures call the pinned header, encoder and output
// routines. This adapter supplies metadata/input pixels, never codec algorithms.
#include <jxl/cms.h>
#include <array>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include "lib/jxl/dec_bit_reader.h"
#include "lib/jxl/dec_external_image.h"
#include "lib/jxl/dec_frame.h"
#include "lib/jxl/enc_frame.h"
#include "lib/jxl/enc_fields.h"
#include "lib/jxl/enc_toc.h"
#include "lib/jxl/toc.h"
#include "lib/jxl/enc_icc_codec.h"
#include "lib/jxl/icc_codec.h"
#include "lib/jxl/enc_aux_out.h"
void MetadataObservation(const jxl::ImageMetadata&);
std::string Hex(jxl::Bytes);
std::vector<uint8_t> Unhex(const std::string&);
namespace {
float Float(uint32_t word) {float v;std::memcpy(&v,&word,4);return v;}
void Require(jxl::Status s, const char* at="extra-channel source boundary") {if(!s)throw std::runtime_error(at);}
jxl::Status Header(jxl::BitReader& reader,jxl::CodecMetadata& m) {
 JXL_RETURN_IF_ERROR(reader.ReadFixedBits<16>()==0x0aff);
 JXL_RETURN_IF_ERROR(jxl::ReadSizeHeader(&reader,&m.size));
 JXL_RETURN_IF_ERROR(jxl::ReadImageMetadata(&reader,&m.m));
 m.transform_data.nonserialized_xyb_encoded=m.m.xyb_encoded;
 JXL_RETURN_IF_ERROR(jxl::Bundle::Read(&reader,&m.transform_data));
 JXL_RETURN_IF_ERROR(!m.m.color_encoding.WantICC());
 JXL_RETURN_IF_ERROR(reader.JumpToByteBoundary());return reader.AllReadsWithinBounds();
}
}
bool ExtraChannelsCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
 if(command=="ec-meta") {
  std::string hex;in>>hex;auto bytes=Unhex(hex);jxl::CodecMetadata m;jxl::BitReader reader{jxl::Bytes(bytes)};
  auto s=Header(reader,m);auto consumed=reader.TotalBitsConsumed();auto closed=reader.Close();
  std::cout<<int(s.code())<<' '<<consumed<<' '<<int(closed.code())<<" T "<<m.m.extra_channel_info.size();for(auto& ec:m.m.extra_channel_info)std::cout<<' '<<unsigned(ec.type);MetadataObservation(m.m);return true;
 }
 if(command=="ec-mutate-type") {
  std::string hex;unsigned type;in>>hex>>type;auto bytes=Unhex(hex);jxl::CodecMetadata original;jxl::BitReader source{jxl::Bytes(bytes)};Require(Header(source,original));size_t limit=source.TotalBitsConsumed();Require(source.Close());
  if(original.m.extra_channel_info.size()!=1)throw std::runtime_error("one-channel mutation prerequisite");
  for(size_t bit=16;bit+8<=limit;++bit)for(unsigned value=0;value<256;++value) {
   auto changed=bytes;for(unsigned b=0;b<8;++b){size_t at=bit+b;changed[at/8]=(changed[at/8]&~(1u<<(at%8)))|(((value>>b)&1u)<<(at%8));}
   jxl::CodecMetadata m;jxl::BitReader reader{jxl::Bytes(changed)};auto status=Header(reader,m);auto consumed=reader.TotalBitsConsumed();auto close=reader.Close();
   if(!status&&m.xsize()==original.xsize()&&m.ysize()==original.ysize()&&m.m.extra_channel_info.size()==1&&unsigned(m.m.extra_channel_info[0].type)==type&&m.m.bit_depth.bits_per_sample==original.m.bit_depth.bits_per_sample) {
    std::cout<<Hex(jxl::Bytes(changed))<<" B "<<bit<<' '<<value<<" T "<<type<<" S "<<int(status.code())<<' '<<consumed<<' '<<int(close.code());return true;
   }
  }
  throw std::runtime_error("original parser did not reach requested rejected type");
 }
 if(command=="ec-mixed-ups") {
  std::string hex;size_t count;in>>hex>>count;auto bytes=Unhex(hex);jxl::CodecMetadata m;jxl::BitReader reader{jxl::Bytes(bytes)};Require(Header(reader,m));size_t global=reader.TotalBitsConsumed()/8;Require(reader.Close());
  if(m.xsize()!=1||m.ysize()!=1||count!=m.m.extra_channel_info.size())throw std::runtime_error("mixed sampling geometry prerequisite");
  jxl::FrameHeader header(&m);jxl::BitReader frame(jxl::Bytes(bytes.data()+global,bytes.size()-global));Require(jxl::ReadFrameHeader(&frame,&header));auto dims=header.ToFrameDimensions();
  std::vector<uint32_t> sizes;std::vector<jxl::coeff_order_t> permutation;Require(jxl::ReadToc(mm,jxl::NumTocEntries(dims.num_groups,dims.num_dc_groups,header.passes.num_passes),&frame,&sizes,&permutation));size_t payload=global+frame.TotalBitsConsumed()/8;Require(frame.Close());
  auto write=[&](const jxl::FrameHeader& h){jxl::BitWriter writer(mm);Require(jxl::WriteFrameHeader(h,&writer,nullptr));Require(jxl::WriteTocPermutation(permutation,&writer,nullptr));Require(jxl::WriteTocSizes(std::vector<size_t>(sizes.begin(),sizes.end()),&writer,nullptr));Require(writer.AppendByteAligned(jxl::Bytes(bytes.data()+payload,bytes.size()-payload)));auto written=std::move(writer).TakeBytes();std::vector<uint8_t> result(bytes.begin(),bytes.begin()+global);result.insert(result.end(),written.begin(),written.end());return result;};
  if(write(header)!=bytes)throw std::runtime_error("original mixed-sampling writer control");
  for(auto& factor:header.extra_channel_upsampling){in>>factor;if(factor!=1&&factor!=2&&factor!=4&&factor!=8)throw std::runtime_error("source sampling factor domain");if((m.xsize()+factor-1)/factor!=1||(m.ysize()+factor-1)/factor!=1)throw std::runtime_error("source modular dimensions changed");}
  auto written=write(header);std::cout<<Hex(jxl::Bytes(written))<<" C 1 U "<<count;for(auto factor:header.extra_channel_upsampling)std::cout<<' '<<factor;std::cout<<" P "<<Hex(jxl::Bytes(bytes.data()+payload,bytes.size()-payload));return true;
 }
 if(command=="ec-export") {
  std::string hex;size_t bits,n,padding;bool fp,unpremul;in>>hex>>bits>>fp>>n>>unpremul>>padding;
  auto bytes=Unhex(hex);jxl::CodecMetadata m;jxl::BitReader reader{jxl::Bytes(bytes)};
  auto s=Header(reader,m);size_t offset=reader.TotalBitsConsumed()/8;auto close=reader.Close();Require(s);Require(close);
  jxl::PassesDecoderState state(mm);Require(state.output_encoding_info.SetFromMetadata(m));jxl::ImageBundle out(mm,&m.m);bool last=false;
  while(!last){jxl::FrameHeader header(&m);auto decoded=jxl::DecodeFrame(&state,nullptr,bytes.data()+offset,bytes.size()-offset,&header,&out,m,true);if(!decoded){std::cout<<"D "<<int(decoded.code())<<' '<<offset;return true;}offset+=out.decoded_bytes();last=header.is_last;}
  size_t stride=out.xsize()*n*((bits+7)/8)+padding;std::vector<uint8_t> output(stride*out.ysize()+7,0xa5);
  s=jxl::ConvertToExternal(out,bits,fp,n,JXL_LITTLE_ENDIAN,stride,nullptr,output.data(),output.size(),{},jxl::Orientation::kIdentity,unpremul);
  std::cout<<int(s.code())<<' '<<out.HasAlpha()<<' '<<out.AlphaIsPremultiplied()<<' '<<Hex(jxl::Bytes(output));
  std::cout<<" X "<<out.extra_channels().size();
  for(const auto& ec:out.extra_channels()) {
   size_t ecstride=ec.xsize()*((bits+7)/8)+padding;std::vector<uint8_t> data(ecstride*ec.ysize()+7,0xa5);const jxl::ImageF* channels[]={&ec};
   auto status=jxl::ConvertChannelsToExternal(channels,1,bits,fp,JXL_LITTLE_ENDIAN,ecstride,nullptr,data.data(),data.size(),{},jxl::Orientation::kIdentity);
   std::cout<<' '<<int(status.code())<<' '<<Hex(jxl::Bytes(data));
  }
  return true;
 }
 if(command!="ec-encode")return false;
 size_t w,h,ec_count,n;bool gray,xyb,fp;uint32_t bits,exp;
 in>>w>>h>>gray>>xyb>>fp>>bits>>exp>>ec_count;
 jxl::CodecMetadata m;Require(m.size.Set(w,h));m.m.bit_depth.floating_point_sample=fp;m.m.bit_depth.bits_per_sample=bits;m.m.bit_depth.exponent_bits_per_sample=exp;
 m.m.modular_16_bit_buffer_sufficient=bits<=12;m.m.xyb_encoded=xyb;m.m.color_encoding=jxl::ColorEncoding::SRGB(gray);
 m.m.num_extra_channels=ec_count;m.m.extra_channel_info.resize(ec_count);
 for(size_t i=0;i<ec_count;++i){auto& ec=m.m.extra_channel_info[i];uint32_t type;std::string name;
  in>>type>>ec.bit_depth.floating_point_sample>>ec.bit_depth.bits_per_sample>>ec.bit_depth.exponent_bits_per_sample>>ec.dim_shift>>ec.alpha_associated;
  ec.type=static_cast<jxl::ExtraChannel>(type);for(auto& c:ec.spot_color){uint32_t word;in>>word;c=Float(word);}in>>ec.cfa_channel>>name;
  auto decoded=Unhex(name);ec.name.assign(decoded.begin(),decoded.end());
 }
 in>>n;jxl::BitWriter writer(mm);Require(jxl::WriteCodestreamHeaders(&m,&writer,nullptr),"WriteCodestreamHeaders");
 Require(writer.WithMaxBits(8,jxl::LayerType::Header,nullptr,[&]{writer.ZeroPadToByte();return true;}));
 size_t header_bytes=writer.BitsWritten()/8;std::vector<size_t> ends;std::vector<int> statuses;
 for(size_t i=0;i<n;++i){bool modular;size_t fw,fh,seed;int features;unsigned blendmode;
  jxl::FrameInfo info;jxl::CompressParams params;in>>modular>>info.is_last>>info.save_as_reference>>info.source>>info.save_before_color_transform>>info.blend>>blendmode>>info.clamp>>info.origin.x0>>info.origin.y0>>fw>>fh>>seed>>params.resampling>>params.ec_resampling>>params.modular_group_size_shift>>features;
  info.blendmode=static_cast<jxl::BlendMode>(blendmode);info.name="extra-"+std::to_string(i);
  params.cms=*JxlGetDefaultCms();params.debug_image_opaque=nullptr;params.buffering=0;params.level=10;params.speed_tier=features==2?jxl::SpeedTier::kSquirrel:jxl::SpeedTier::kThunder;
  params.patches=features==2?jxl::Override::kOn:jxl::Override::kOff;params.dots=params.noise=jxl::Override::kOff;params.gaborish=features==1?jxl::Override::kOn:jxl::Override::kOff;params.epf=features==1?2:0;
  if(modular&&!xyb){auto res=params.resampling,ecres=params.ec_resampling;params.SetLossless();params.resampling=res;params.ec_resampling=ecres;}
  else{params.modular_mode=modular;params.butteraugli_distance=1.f;params.color_transform=xyb?jxl::ColorTransform::kXYB:jxl::ColorTransform::kNone;}
  auto s=jxl::ParamsPostInit(&params);size_t colors=gray?1:3;std::vector<float> color(fw*fh*colors);
  for(size_t y=0;y<fh;++y)for(size_t x=0;x<fw;++x)for(size_t c=0;c<colors;++c)color[(y*fw+x)*colors+c]=features==2?float((x%24>=5&&x%24<13&&y%24>=5&&y%24<17&&(x%24<7||x%24>=11||y%24==10||y%24==11))?0:1):float((seed*31+x*17+y*23+c*47)%256)/(fp?256.f:255.f);
  jxl::JxlEncoderChunkedFrameAdapter input(fw,fh,ec_count);JxlPixelFormat format{unsigned(colors),JXL_TYPE_FLOAT,JXL_NATIVE_ENDIAN,0};
  Require(input.SetFromBuffer(0,reinterpret_cast<const uint8_t*>(color.data()),color.size()*4,format));
  std::vector<std::vector<float>> channels(ec_count,std::vector<float>(fw*fh));
  for(size_t ec=0;ec<ec_count;++ec){for(size_t y=0;y<fh;++y)for(size_t x=0;x<fw;++x)channels[ec][y*fw+x]=float((seed*13+x*29+y*37+ec*43)%17)/16.f;
   JxlPixelFormat ef{1,JXL_TYPE_FLOAT,JXL_NATIVE_ENDIAN,0};Require(input.SetFromBuffer(ec+1,reinterpret_cast<const uint8_t*>(channels[ec].data()),channels[ec].size()*4,ef));}
  JxlEncoderOutputProcessorWrapper output(mm);if(s)s=jxl::EncodeFrame(mm,params,info,&m,input,*JxlGetDefaultCms(),nullptr,&output,nullptr,nullptr);
  if(s)s=output.SetFinalizedPosition();std::vector<uint8_t> data;if(s)s=output.CopyOutput(data);if(s)s=writer.AppendByteAligned(jxl::Bytes(data));statuses.push_back(int(s.code()));ends.push_back(writer.BitsWritten()/8);Require(s,"EncodeFrame/output");
 }
 auto bytes=std::move(writer).TakeBytes();std::cout<<Hex(jxl::Bytes(bytes))<<' '<<header_bytes<<' '<<ends.size();for(auto e:ends)std::cout<<' '<<e;std::cout<<" S "<<statuses.size();for(auto s:statuses)std::cout<<' '<<s;return true;
}
