// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Whole-image color output fixtures. The original public encoder produces every
// file; profile splicing uses the original header/ICC writers. The decoder adapter
// drives the original FrameDecoder after performing the OutputEncodingInfo
// operations of decode.cc's header, CMS, intensity-target and output-profile
// entry points in their source order. A public-decoder control runs the actual
// JxlDecoder API on the same request so the adapter's sequence is checked against
// the original library's own output buffer.
#include <jxl/cms.h>
#include <jxl/decode.h>
#include <jxl/encode.h>
#include <algorithm>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <memory>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include "lib/jxl/dec_frame.h"
#include "lib/jxl/enc_fields.h"
#include "lib/jxl/enc_icc_codec.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/enc_bit_writer.h"
#include "lib/jxl/icc_codec.h"
#include "lib/jxl/padded_bytes.h"
std::string Hex(jxl::Bytes);
std::vector<uint8_t> Unhex(const std::string&);
JxlColorEncoding ReadColorEncoding(std::istringstream& in);
void MetadataObservation(const jxl::ImageMetadata&);
void ColorBundleObservation(const jxl::ColorEncoding&);
void OEIObservation(const jxl::OutputEncodingInfo&);
namespace {
float FloatFromBits(uint32_t bits) {float value;std::memcpy(&value,&bits,4);return value;}
uint32_t BitsFromFloat(float value) {uint32_t bits;std::memcpy(&bits,&value,4);return bits;}

// Deterministic input samples; the encoder owns every codestream decision.
std::vector<uint8_t> Pixels(size_t w,size_t h,size_t channels,unsigned bits,bool floating,uint64_t seed) {
 const size_t bytes=floating?4:(bits<=8?1:2);std::vector<uint8_t> out(w*h*channels*bytes);
 const uint32_t max=bits>=32?0xffffffffu:((1u<<bits)-1);
 for(size_t y=0;y<h;++y)for(size_t x=0;x<w;++x)for(size_t c=0;c<channels;++c){
  const uint64_t v=(x*37+y*101+c*59+seed*13+(x*y)%23+(x*x*7+y*y*3)%61)*2654435761u;
  const size_t at=((y*w+x)*channels+c)*bytes;
  if(floating){float f=float(v%4096)/4095.0f;std::memcpy(&out[at],&f,4);}
  else if(bytes==1)out[at]=uint8_t((v>>7)&max);
  else{uint16_t s=uint16_t((v>>5)&max);std::memcpy(&out[at],&s,2);}
 }
 return out;
}

void Encode(std::istringstream& in) {
 size_t w,h;unsigned gray,alpha,bits,floating,mode,effort;uint32_t target;uint64_t seed;std::string kind;
 in>>w>>h>>gray>>alpha>>bits>>floating>>mode>>effort>>target>>seed>>kind;
 JxlColorEncoding color{};std::vector<uint8_t> icc;
 if(kind=="e")color=ReadColorEncoding(in);else if(kind=="i"){std::string hex;in>>hex;icc=Unhex(hex);}else throw std::runtime_error("color fixture kind");
 JxlEncoder* enc=JxlEncoderCreate(nullptr);if(!enc)throw std::runtime_error("encoder allocation");
 std::vector<int> statuses;statuses.push_back(JxlEncoderUseContainer(enc,JXL_FALSE));
 JxlBasicInfo info;JxlEncoderInitBasicInfo(&info);info.xsize=w;info.ysize=h;info.bits_per_sample=bits;
 info.exponent_bits_per_sample=floating?(bits==16?5:8):0;info.uses_original_profile=mode!=1;info.num_color_channels=gray?1:3;
 info.num_extra_channels=alpha;info.alpha_bits=alpha?bits:0;info.alpha_exponent_bits=alpha?info.exponent_bits_per_sample:0;info.intensity_target=FloatFromBits(target);
 statuses.push_back(JxlEncoderSetBasicInfo(enc,&info));
 statuses.push_back(kind=="e"?JxlEncoderSetColorEncoding(enc,&color):JxlEncoderSetICCProfile(enc,icc.data(),icc.size()));
 auto* settings=JxlEncoderFrameSettingsCreate(enc,nullptr);
 statuses.push_back(JxlEncoderFrameSettingsSetOption(settings,JXL_ENC_FRAME_SETTING_EFFORT,effort));
 if(mode==0)statuses.push_back(JxlEncoderSetFrameLossless(settings,JXL_TRUE));else statuses.push_back(JxlEncoderSetFrameDistance(settings,1.0f));
 const size_t channels=(gray?1:3)+alpha;JxlPixelFormat format{unsigned(channels),floating?JXL_TYPE_FLOAT:(bits<=8?JXL_TYPE_UINT8:JXL_TYPE_UINT16),JXL_NATIVE_ENDIAN,0};
 auto pixels=Pixels(w,h,channels,bits,floating,seed);
 const bool ready=std::all_of(statuses.begin(),statuses.end(),[](int s){return s==JXL_ENC_SUCCESS;});
 if(ready)statuses.push_back(JxlEncoderAddImageFrame(settings,&format,pixels.data(),pixels.size()));
 JxlEncoderCloseInput(enc);
 std::vector<uint8_t> output(4096);size_t size=0;JxlEncoderStatus status=JXL_ENC_ERROR;
 if(std::all_of(statuses.begin(),statuses.end(),[](int s){return s==JXL_ENC_SUCCESS;}))for(;;){uint8_t* next=output.data()+size;size_t available=output.size()-size;status=JxlEncoderProcessOutput(enc,&next,&available);size=next-output.data();if(status!=JXL_ENC_NEED_MORE_OUTPUT)break;output.resize(output.size()*2);}
 auto error=JxlEncoderGetError(enc);JxlEncoderDestroy(enc);output.resize(status==JXL_ENC_SUCCESS?size:0);
 // Level-10 output is always boxed by the public encoder. Take its unchanged
 // jxlc/jxlp codestream bytes; the raw sequence decoder reads codestreams.
 int level=5;std::vector<uint8_t> codestream;
 if(output.size()>=12&&output[0]==0&&output[1]==0&&output[2]==0&&output[3]==12){
  for(size_t at=0;at<output.size();){
   if(output.size()-at<8)throw std::runtime_error("encoder box header");
   uint64_t box=0;for(size_t i=0;i<4;++i)box=box<<8|output[at+i];std::string type(output.begin()+at+4,output.begin()+at+8);size_t header=8;
   if(box==1){if(output.size()-at<16)throw std::runtime_error("encoder box size");box=0;for(size_t i=8;i<16;++i)box=box<<8|output[at+i];header=16;}
   if(box==0)box=output.size()-at;
   if(box<header||box>output.size()-at)throw std::runtime_error("encoder box bounds");
   if(type=="jxll")level=output[at+header];
   if(type=="jxlc")codestream.insert(codestream.end(),output.begin()+at+header,output.begin()+at+box);
   if(type=="jxlp"){if(box<header+4)throw std::runtime_error("encoder jxlp index");codestream.insert(codestream.end(),output.begin()+at+header+4,output.begin()+at+box);}
   at+=box;
  }
 }else codestream=output;
 std::cout<<int(status)<<' '<<Hex(jxl::Bytes(codestream))<<" L "<<level<<" API";for(int v:statuses)std::cout<<' '<<v;std::cout<<" W "<<int(error);
}

// decode.cc's header sequence: the ICC stream is stored with SetICCRaw.
jxl::Status Header(jxl::BitReader& reader,jxl::CodecMetadata& metadata,JxlMemoryManager* mm) {
 if(reader.ReadFixedBits<16>()!=0x0aff)return JXL_FAILURE("signature");
 JXL_RETURN_IF_ERROR(jxl::ReadSizeHeader(&reader,&metadata.size));
 JXL_RETURN_IF_ERROR(jxl::ReadImageMetadata(&reader,&metadata.m));
 metadata.transform_data.nonserialized_xyb_encoded=metadata.m.xyb_encoded;
 JXL_RETURN_IF_ERROR(jxl::Bundle::Read(&reader,&metadata.transform_data));
 if(metadata.m.color_encoding.WantICC()){
  jxl::ICCReader icc(mm);jxl::PaddedBytes decoded(mm);JXL_RETURN_IF_ERROR(icc.Init(&reader));JXL_RETURN_IF_ERROR(icc.Process(&reader,&decoded));
  if(decoded.empty())return JXL_FAILURE("empty ICC");
  metadata.m.color_encoding.SetICCRaw(std::vector<uint8_t>(decoded.begin(),decoded.end()));
 }
 JXL_RETURN_IF_ERROR(reader.JumpToByteBoundary());return reader.AllReadsWithinBounds();
}

// JxlDecoderSetOutputColorProfile's body after its header/post-header checks,
// with no image output buffer set. Returns 0 or the failing source step.
int SetOutputColorProfile(jxl::PassesDecoderState& state,jxl::ImageMetadata& image_metadata,const JxlColorEncoding* color_encoding,const std::vector<uint8_t>* icc_data) {
 auto& output_encoding=state.output_encoding_info;auto& orig_encoding=image_metadata.color_encoding;
 jxl::ColorEncoding c_out;bool same_encoding=false;
 if(color_encoding){if(!c_out.FromExternal(*color_encoding))return 1;same_encoding=c_out.SameColorEncoding(output_encoding.color_encoding);}
 if(!output_encoding.cms_set&&(icc_data!=nullptr||(!image_metadata.xyb_encoded&&!same_encoding)))return 2;
 if(!orig_encoding.HaveFields()&&output_encoding.cms_set){
  std::vector<uint8_t> tmp_icc=orig_encoding.ICC();
  if(!orig_encoding.SetICC(std::move(tmp_icc),&output_encoding.color_management_system))return 3;
  output_encoding.orig_color_encoding=orig_encoding;
 }
 if(color_encoding){
  if(color_encoding->color_space==JXL_COLOR_SPACE_UNKNOWN)return 4;
  if(c_out.ICC().empty())return 5;
  if(!output_encoding.MaybeSetColorEncoding(c_out))return 6;
  image_metadata.color_encoding=output_encoding.color_encoding;return 0;
 }
 jxl::ColorEncoding c_dst;std::vector<uint8_t> padded_icc(icc_data->begin(),icc_data->end());
 if(!c_dst.SetICC(std::move(padded_icc),&output_encoding.color_management_system))return 7;
 if(!output_encoding.MaybeSetColorEncoding(c_dst))return 8;
 return 0;
}

struct Request {
 std::vector<uint8_t> bytes;bool cms=false;uint32_t target=0;std::string desired="-";JxlColorEncoding color{};std::vector<uint8_t> icc;
};
Request ReadRequest(std::istringstream& in) {
 Request r;std::string hex;in>>hex>>r.cms>>r.target>>r.desired;r.bytes=Unhex(hex);
 if(r.desired=="e")r.color=ReadColorEncoding(in);else if(r.desired=="i"){std::string icc;in>>icc;r.icc=Unhex(icc);}else if(r.desired!="-")throw std::runtime_error("desired profile kind");
 return r;
}

void Planes(const jxl::ImageBundle& image) {
 std::cout<<" I "<<image.xsize()<<' '<<image.ysize();ColorBundleObservation(image.c_current());
 for(size_t c=0;c<3;++c)for(size_t y=0;y<image.ysize();++y)for(size_t x=0;x<image.xsize();++x)std::cout<<' '<<BitsFromFloat(image.color().PlaneRow(c,y)[x]);
 std::cout<<" X "<<image.extra_channels().size();
 for(const auto& ec:image.extra_channels()){std::cout<<' '<<ec.xsize()<<' '<<ec.ysize();for(size_t y=0;y<ec.ysize();++y)for(size_t x=0;x<ec.xsize();++x)std::cout<<' '<<BitsFromFloat(ec.Row(y)[x]);}
}

void Decode(std::istringstream& in,JxlMemoryManager* mm) {
 const Request r=ReadRequest(in);
 std::vector<uint8_t> backing(r.bytes.size()+16);std::copy(r.bytes.begin(),r.bytes.end(),backing.begin()+8);const auto* data=backing.data()+8;
 jxl::CodecMetadata metadata;jxl::BitReader reader(jxl::Bytes(data,r.bytes.size()));auto status=Header(reader,metadata,mm);size_t offset=reader.TotalBitsConsumed()/8;auto close=reader.Close();
 std::cout<<" H "<<int(status.code())<<' '<<int(close.code());if(!status||!close)return;
 MetadataObservation(metadata.m);
 // Value-initialize the aggregate's constructor-indeterminate scalars, as
 // the output-encoding fixtures do; safe native storage receives the same seeds.
 jxl::PassesDecoderState state(mm);state.output_encoding_info=jxl::OutputEncodingInfo{};status=state.output_encoding_info.SetFromMetadata(metadata);std::cout<<" P "<<int(status.code());if(!status)return;
 if(r.target!=0)state.output_encoding_info.desired_intensity_target=FloatFromBits(r.target);
 if(r.cms){state.output_encoding_info.color_management_system=*JxlGetDefaultCms();state.output_encoding_info.cms_set=true;}
 jxl::ImageMetadata image_metadata=metadata.m;
 if(r.desired!="-"){int step=SetOutputColorProfile(state,image_metadata,r.desired=="e"?&r.color:nullptr,r.desired=="i"?&r.icc:nullptr);std::cout<<" C "<<step;if(step)return;}
 OEIObservation(state.output_encoding_info);
 jxl::ImageBundle output(mm,&metadata.m);bool preview=metadata.m.have_preview,last_main=false;size_t frames=0;
 while(status&&!last_main&&frames<64){
  jxl::FrameDecoder frame(&state,metadata,nullptr,true);
  jxl::BitReader header(jxl::Bytes(data+offset,r.bytes.size()-offset));
  status=frame.InitFrame(&header,&output,preview);
  if(status)status=frame.InitFrameOutput();
  if(status)status=header.AllReadsWithinBounds();
  size_t header_bytes=status?header.TotalBitsConsumed()/8:0;
  close=header.Close();if(status)status=close;
  std::cout<<" F "<<frames++<<' '<<preview<<' '<<int(status.code());if(!status)break;
  const auto& h=frame.GetFrameHeader();
  size_t position=offset+header_bytes;std::vector<std::unique_ptr<jxl::BitReader>> readers;std::vector<jxl::FrameDecoder::SectionInfo> sections;std::vector<jxl::FrameDecoder::SectionStatus> section_status;
  for(const auto& entry:frame.Toc()){if(entry.size>r.bytes.size()-position){status=false;break;}readers.emplace_back(std::make_unique<jxl::BitReader>(jxl::Bytes(data+position,entry.size)));sections.push_back({readers.back().get(),entry.id,sections.size()});position+=entry.size;}
  if(status){section_status.resize(sections.size());status=frame.ProcessSections(sections.data(),sections.size(),section_status.data());}
  for(auto& section:readers){auto closed=section->Close();if(!closed&&status)status=closed;}
  bool all_done=status;for(auto v:section_status)if(v!=jxl::FrameDecoder::kDone)all_done=false;
  if(all_done){status=frame.FinalizeFrame();if(status){output.SetDecodedBytes(position-offset);offset=position;last_main=!preview&&h.is_last;}}
  else if(status)status=false;
  std::cout<<" S "<<int(status.code());preview=false;
 }
 bool eof=offset==r.bytes.size();if(status&&(!last_main||!eof))status=false;
 std::cout<<" E "<<int(status.code())<<' '<<offset<<' '<<frames;
 OEIObservation(state.output_encoding_info);
 if(status)Planes(output);
}

// The actual public decoder for the same request, with float output buffers.
void Public(std::istringstream& in) {
 const Request r=ReadRequest(in);
 JxlDecoder* dec=JxlDecoderCreate(nullptr);if(!dec)throw std::runtime_error("decoder allocation");
 std::vector<int> statuses;
 statuses.push_back(JxlDecoderSubscribeEvents(dec,JXL_DEC_BASIC_INFO|JXL_DEC_COLOR_ENCODING|JXL_DEC_FULL_IMAGE));
 if(r.cms)statuses.push_back(JxlDecoderSetCms(dec,*JxlGetDefaultCms()));
 if(r.target!=0)statuses.push_back(JxlDecoderSetDesiredIntensityTarget(dec,FloatFromBits(r.target)));
 statuses.push_back(JxlDecoderSetInput(dec,r.bytes.data(),r.bytes.size()));JxlDecoderCloseInput(dec);
 JxlBasicInfo info{};std::vector<float> pixels;JxlPixelFormat format{0,JXL_TYPE_FLOAT,JXL_NATIVE_ENDIAN,0};JxlDecoderStatus status;
 for(;;){
  status=JxlDecoderProcessInput(dec);
  if(status==JXL_DEC_BASIC_INFO){statuses.push_back(JxlDecoderGetBasicInfo(dec,&info));}
  else if(status==JXL_DEC_COLOR_ENCODING){
   if(r.desired=="e")statuses.push_back(JxlDecoderSetOutputColorProfile(dec,&r.color,nullptr,0));
   else if(r.desired=="i")statuses.push_back(JxlDecoderSetOutputColorProfile(dec,nullptr,r.icc.data(),r.icc.size()));
   if(statuses.back()!=JXL_DEC_SUCCESS)break;
  }
  else if(status==JXL_DEC_NEED_IMAGE_OUT_BUFFER){
   JxlColorEncoding out{};const bool known=JxlDecoderGetColorAsEncodedProfile(dec,JXL_COLOR_PROFILE_TARGET_DATA,&out)==JXL_DEC_SUCCESS;
   size_t color_channels=info.num_color_channels;if(known)color_channels=out.color_space==JXL_COLOR_SPACE_GRAY?1:3;
   format.num_channels=unsigned(color_channels+(info.alpha_bits?1:0));size_t size=0;
   statuses.push_back(JxlDecoderImageOutBufferSize(dec,&format,&size));pixels.assign(size/4,0.0f);
   statuses.push_back(JxlDecoderSetImageOutBuffer(dec,&format,pixels.data(),size));
   if(statuses.back()!=JXL_DEC_SUCCESS)break;
  }
  else break;
 }
 JxlDecoderDestroy(dec);
 std::cout<<int(status)<<' '<<format.num_channels<<' '<<pixels.size()<<" API";for(int v:statuses)std::cout<<' '<<v;
 if(status==JXL_DEC_FULL_IMAGE){std::cout<<" O";for(float v:pixels)std::cout<<' '<<BitsFromFloat(v);}
}

// Replace the header ICC profile with the original writers and keep the genuine
// frame bytes. Splicing the parsed profile itself must reproduce the input.
void Splice(std::istringstream& in,JxlMemoryManager* mm) {
 std::string hex,icc_hex;in>>hex>>icc_hex;auto bytes=Unhex(hex);auto icc=Unhex(icc_hex);
 jxl::CodecMetadata metadata;jxl::BitReader reader{jxl::Bytes(bytes)};auto status=Header(reader,metadata,mm);size_t header_bytes=reader.TotalBitsConsumed()/8;auto close=reader.Close();
 if(!status||!close)throw std::runtime_error("splice source header");
 metadata.m.color_encoding.SetICCRaw(std::move(icc));
 jxl::BitWriter writer(mm);status=jxl::WriteCodestreamHeaders(&metadata,&writer,nullptr);
 if(status)status=jxl::WriteICC(jxl::Bytes(metadata.m.color_encoding.ICC()),&writer,jxl::LayerType::Header,nullptr);
 if(status)status=writer.WithMaxBits(8,jxl::LayerType::Header,nullptr,[&]{writer.ZeroPadToByte();return true;});
 if(status)status=writer.AppendByteAligned(jxl::Bytes(bytes.data()+header_bytes,bytes.size()-header_bytes));
 std::cout<<int(status.code());if(!status)return;
 auto result=std::move(writer).TakeBytes();std::cout<<' '<<Hex(jxl::Bytes(result));
}
}  // namespace
bool ColorOutputCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
 if(command=="co-encode")Encode(in);
 else if(command=="co-decode")Decode(in,mm);
 else if(command=="co-public")Public(in);
 else if(command=="co-splice")Splice(in,mm);
 else return false;
 return true;
}
