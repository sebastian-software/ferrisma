// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Data-only scripts for the pinned original JxlToJpegDecoder. The pristine
// archive supplies decode_to_jpeg.cc, DecodeJPEGData, DecodeFrame, WriteJpeg
// and the public JPEG encoder; this adapter only sequences their calls.
#include <jxl/encode.h>
#include <algorithm>
#include <cstdint>
#include <iostream>
#include <memory>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include "lib/jxl/base/span.h"
#include "lib/jxl/dec_bit_reader.h"
#include "lib/jxl/dec_cache.h"
#include "lib/jxl/dec_frame.h"
#include "lib/jxl/decode_to_jpeg.h"
#include "lib/jxl/frame_header.h"
#include "lib/jxl/headers.h"
#include "lib/jxl/image_bundle.h"
#include "lib/jxl/image_metadata.h"
#include "lib/jxl/jpeg/jpeg_data.h"
std::string Hex(jxl::Bytes);
std::vector<uint8_t> Unhex(const std::string&);
namespace {
using namespace jxl::jpeg;
void PrintBytes(const std::vector<uint8_t>& v) {std::cout<<' '<<Hex(jxl::Bytes(v));}
void PrintVectors(const std::vector<std::vector<uint8_t>>& v) {std::cout<<' '<<v.size();for(auto& x:v)PrintBytes(x);}
// Same complete owner snapshot as the JPEG metadata adapter.
void PrintData(const JPEGData& j) {
 std::cout<<' '<<j.width<<' '<<j.height<<' '<<j.restart_interval;PrintVectors(j.app_data);std::cout<<' '<<j.app_marker_type.size();for(auto x:j.app_marker_type)std::cout<<' '<<static_cast<uint32_t>(x);PrintVectors(j.com_data);
 std::cout<<' '<<j.quant.size();for(auto& q:j.quant){std::cout<<' '<<q.precision<<' '<<q.index<<' '<<q.is_last;for(auto x:q.values)std::cout<<' '<<x;}
 std::cout<<' '<<j.huffman_code.size();for(auto& h:j.huffman_code){std::cout<<' '<<h.slot_id<<' '<<h.is_last;for(auto x:h.counts)std::cout<<' '<<x;for(auto x:h.values)std::cout<<' '<<x;}
 std::cout<<' '<<j.components.size();for(auto& c:j.components){std::cout<<' '<<c.id<<' '<<c.h_samp_factor<<' '<<c.v_samp_factor<<' '<<c.quant_idx<<' '<<c.width_in_blocks<<' '<<c.height_in_blocks<<' '<<c.coeffs.size();for(auto x:c.coeffs)std::cout<<' '<<x;}
 std::cout<<' '<<j.scan_info.size();for(auto& s:j.scan_info){std::cout<<' '<<s.Ss<<' '<<s.Se<<' '<<s.Ah<<' '<<s.Al<<' '<<s.num_components<<' '<<s.last_needed_pass;for(size_t i=0;i<4;++i)std::cout<<' '<<s.components[i].comp_idx<<' '<<s.components[i].dc_tbl_idx<<' '<<s.components[i].ac_tbl_idx;std::cout<<' '<<s.reset_points.size();for(auto x:s.reset_points)std::cout<<' '<<x;std::cout<<' '<<s.extra_zero_runs.size();for(auto& x:s.extra_zero_runs)std::cout<<' '<<x.block_idx<<' '<<x.num_extra_zero_runs;}
 PrintBytes(j.marker_order);PrintVectors(j.inter_marker_data);PrintBytes(j.tail_data);std::cout<<' '<<j.has_zero_padding_bit;PrintBytes(j.padding_bits);
}
// Caller-owned output storage keeps one byte beyond the logical size, so even
// a zero-size buffer has a non-null address. Every buffer stays live until the
// script ends, as decode.cc callers keep their JPEG buffer.
struct Buffer {std::vector<uint8_t> bytes;size_t size;};
JPEGData* Target(int t,jxl::JxlToJpegDecoder& d,std::unique_ptr<JPEGData>& moved) {
 JPEGData* j=t==0?d.GetJpegData():moved.get();
 if(!j)throw std::runtime_error("JPEG decoder script requires a live JPEGData target");
 return j;
}
void Encode(std::istringstream& in) {
 std::string hex;int effort,compress;in>>hex>>effort>>compress;auto bytes=Unhex(hex);
 auto* enc=JxlEncoderCreate(nullptr);if(!enc)throw std::runtime_error("JPEG encoder allocation");
 auto* settings=JxlEncoderFrameSettingsCreate(enc,nullptr);
 std::vector<int> statuses;statuses.push_back(JxlEncoderUseContainer(enc,JXL_TRUE));statuses.push_back(JxlEncoderStoreJPEGMetadata(enc,JXL_TRUE));
 statuses.push_back(JxlEncoderFrameSettingsSetOption(settings,JXL_ENC_FRAME_SETTING_EFFORT,effort));
 statuses.push_back(JxlEncoderFrameSettingsSetOption(settings,JXL_ENC_FRAME_SETTING_JPEG_COMPRESS_BOXES,compress));
 statuses.push_back(JxlEncoderAddJPEGFrame(settings,bytes.data(),bytes.size()));JxlEncoderCloseInput(enc);
 std::vector<uint8_t> output(4096);size_t size=0;JxlEncoderStatus status=JXL_ENC_ERROR;
 if(std::all_of(statuses.begin(),statuses.end(),[](int s){return s==JXL_ENC_SUCCESS;}))for(;;){uint8_t* next=output.data()+size;size_t available=output.size()-size;status=JxlEncoderProcessOutput(enc,&next,&available);size=next-output.data();if(status!=JXL_ENC_NEED_MORE_OUTPUT)break;output.resize(output.size()*2);}
 auto error=JxlEncoderGetError(enc);JxlEncoderDestroy(enc);output.resize(status==JXL_ENC_SUCCESS?size:0);
 std::cout<<"API";for(int v:statuses)std::cout<<' '<<v;std::cout<<" W "<<int(status)<<' '<<int(error)<<' '<<Hex(jxl::Bytes(output));
}
// Parse the codestream headers and decode its first frame into an image
// bundle, after the original SetImageBundleJpegData checkpoint.
void Frame(std::istringstream& in,JxlMemoryManager* mm,jxl::JxlToJpegDecoder& d,std::unique_ptr<JPEGData>& moved) {
 std::string hex;in>>hex;auto bytes=Unhex(hex);
 std::vector<uint8_t> backing(bytes.size()+16);std::copy(bytes.begin(),bytes.end(),backing.begin()+8);
 jxl::CodecMetadata metadata;jxl::BitReader reader{jxl::Bytes(backing.data()+8,bytes.size())};
 auto status=jxl::Status(reader.ReadFixedBits<16>()==0x0aff);
 if(status)status=jxl::ReadSizeHeader(&reader,&metadata.size);
 if(status)status=jxl::ReadImageMetadata(&reader,&metadata.m);
 metadata.transform_data.nonserialized_xyb_encoded=metadata.m.xyb_encoded;
 if(status)status=jxl::Bundle::Read(&reader,&metadata.transform_data);
 if(status&&metadata.m.color_encoding.WantICC())throw std::runtime_error("JPEG decoder script needs a non-ICC codestream");
 if(status)status=reader.JumpToByteBoundary();
 size_t offset=reader.TotalBitsConsumed()/8;auto close=reader.Close();
 std::cout<<" F "<<int(status.code())<<' '<<offset<<' '<<int(close.code());
 if(!status||!close)return;
 jxl::ImageBundle decoded(mm,&metadata.m);
 auto set=d.SetImageBundleJpegData(&decoded);std::cout<<' '<<int(set.code());
 if(!decoded.jpeg_data){std::cout<<" -";return;}
 jxl::PassesDecoderState state(mm);jxl::FrameHeader header(&metadata);
 status=jxl::DecodeFrame(&state,nullptr,bytes.data()+offset,bytes.size()-offset,&header,&decoded,metadata,true);
 std::cout<<' '<<int(status.code());moved=std::move(decoded.jpeg_data);
}
}  // namespace
bool DecodeToJpegCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
 if(command=="jtj-encode"){Encode(in);return true;}
 if(command!="jtj-ops")return false;
 jxl::JxlToJpegDecoder d;std::unique_ptr<JPEGData> moved;
 std::vector<std::unique_ptr<Buffer>> buffers;Buffer* current=nullptr;
 std::string op;
 while(in>>op){
  if(op=="R"){d.Reset();current=nullptr;std::cout<<" R";}
  else if(op=="S"){int eof;size_t size;in>>eof>>size;d.StartBox(eof!=0,size);std::cout<<" S";}
  else if(op=="P"){
   std::string h;in>>h;auto v=Unhex(h);std::vector<uint8_t> backing(v.size()+16);std::copy(v.begin(),v.end(),backing.begin()+8);
   const uint8_t* next_in=backing.data()+8;size_t avail_in=v.size();auto s=d.Process(&next_in,&avail_in);
   std::cout<<" P "<<int(s)<<' '<<(next_in-(backing.data()+8))<<' '<<avail_in;
  }
  else if(op=="O"){
   size_t size;in>>size;buffers.push_back(std::make_unique<Buffer>(Buffer{std::vector<uint8_t>(size+1,0xa5),size}));
   auto s=d.SetOutputBuffer(buffers.back()->bytes.data(),size);if(s==JXL_DEC_SUCCESS)current=buffers.back().get();std::cout<<" O "<<int(s);
  }
  else if(op=="Z"){size_t size;in>>size;auto s=d.SetOutputBuffer(nullptr,size);if(s==JXL_DEC_SUCCESS)current=nullptr;std::cout<<" Z "<<int(s);}
  else if(op=="L"){
   size_t r=d.ReleaseOutputBuffer();std::cout<<" L "<<r;
   if(current)std::cout<<' '<<Hex(jxl::Bytes(current->bytes.data(),current->size));else std::cout<<" -";current=nullptr;
  }
  else if(op=="B"){std::cout<<" B ";if(current)std::cout<<Hex(jxl::Bytes(current->bytes.data(),current->size));else std::cout<<'-';}
  else if(op=="F")Frame(in,mm,d,moved);
  else if(op=="I"){
   std::string h;in>>h;auto icc=Unhex(h);jxl::ImageMetadata m;if(!icc.empty())m.color_encoding.SetICCRaw(std::move(icc));
   jxl::ImageBundle ib(mm,&m);auto s=d.SetImageBundleJpegData(&ib);std::cout<<" I "<<int(s.code())<<' '<<(ib.jpeg_data!=nullptr);
   if(ib.jpeg_data)moved=std::move(ib.jpeg_data);
  }
  else if(op=="N"){
   int t;in>>t;JPEGData* j=Target(t,d,moved);size_t es=777,xs=777;
   auto e=jxl::JxlToJpegDecoder::ExifBoxContentSize(*j,&es);auto x=jxl::JxlToJpegDecoder::XmlBoxContentSize(*j,&xs);
   std::cout<<" N "<<jxl::JxlToJpegDecoder::NumExifMarkers(*j)<<' '<<jxl::JxlToJpegDecoder::NumXmpMarkers(*j)<<' '<<int(e)<<' '<<es<<' '<<int(x)<<' '<<xs;
  }
  else if(op=="X"||op=="M"){
   int t;std::string h;in>>t>>h;auto v=Unhex(h);v.reserve(v.size()+1);JPEGData* j=Target(t,d,moved);
   auto s=op=="X"?jxl::JxlToJpegDecoder::SetExif(v.data(),v.size(),j):jxl::JxlToJpegDecoder::SetXmp(v.data(),v.size(),j);
   std::cout<<' '<<op<<' '<<int(s);
  }
  else if(op=="A"){
   int t;in>>t;JPEGData* j=Target(t,d,moved);std::cout<<" A";PrintVectors(j->app_data);
   std::cout<<' '<<j->app_marker_type.size();for(auto x:j->app_marker_type)std::cout<<' '<<static_cast<uint32_t>(x);
  }
  else if(op=="J"){std::cout<<" J";if(auto* j=d.GetJpegData())PrintData(*j);else std::cout<<" -";}
  else if(op=="W"){
   if(!moved)throw std::runtime_error("JPEG decoder script requires a decoded frame");
   std::cout<<" W "<<int(d.WriteOutput(*moved));
  }
  else throw std::runtime_error("unknown JPEG decoder operation");
  std::cout<<" | "<<d.IsOutputSet()<<' '<<d.IsParsingBox()<<' '<<(d.GetJpegData()!=nullptr);
 }
 in.clear(std::ios::eofbit);
 return true;
}
