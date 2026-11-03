// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Data-only observations of the pinned original JPEGData bundle and ICC helper.
#include <cstdint>
#include <brotli/encode.h>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include "lib/jxl/base/span.h"
#include "lib/jxl/dec_bit_reader.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/enc_bit_writer.h"
#include "lib/jxl/enc_fields.h"
#include "lib/jxl/fields.h"
#include "lib/jxl/jpeg/enc_jpeg_data_reader.h"
#include "lib/jxl/jpeg/dec_jpeg_data.h"
#include "lib/jxl/jpeg/enc_jpeg_data.h"
#include "lib/jxl/jpeg/jpeg_data.h"
std::string Hex(jxl::Bytes);
std::vector<uint8_t> Unhex(const std::string&);
jxl::Status Prefix(jxl::BitWriter&,size_t);
jxl::Status Pad(jxl::BitWriter&);
namespace {
using namespace jxl::jpeg;
void Bytes(std::istringstream& in,std::vector<uint8_t>& v) {std::string h;in>>h;v=Unhex(h);}
void Vectors(std::istringstream& in,std::vector<std::vector<uint8_t>>& v) {size_t n;in>>n;v.resize(n);for(auto& x:v)Bytes(in,x);}
void PrintBytes(const std::vector<uint8_t>& v) {std::cout<<' '<<Hex(jxl::Bytes(v));}
void PrintVectors(const std::vector<std::vector<uint8_t>>& v) {std::cout<<' '<<v.size();for(auto& x:v)PrintBytes(x);}
void ReadData(std::istringstream& in,JPEGData& j) {
 in>>j.width>>j.height>>j.restart_interval;Vectors(in,j.app_data);size_t n;in>>n;j.app_marker_type.resize(n);for(auto& x:j.app_marker_type){uint32_t v;in>>v;x=static_cast<AppMarkerType>(v);}Vectors(in,j.com_data);
 in>>n;j.quant.resize(n);for(auto& q:j.quant){in>>q.precision>>q.index>>q.is_last;for(auto& x:q.values)in>>x;}
 in>>n;j.huffman_code.resize(n);for(auto& h:j.huffman_code){in>>h.slot_id>>h.is_last;for(auto& x:h.counts)in>>x;for(auto& x:h.values)in>>x;}
 in>>n;j.components.resize(n);for(auto& c:j.components){in>>c.id>>c.h_samp_factor>>c.v_samp_factor>>c.quant_idx>>c.width_in_blocks>>c.height_in_blocks;size_t m;in>>m;c.coeffs.resize(m);for(auto& x:c.coeffs)in>>x;}
 in>>n;j.scan_info.resize(n);for(auto& s:j.scan_info){in>>s.Ss>>s.Se>>s.Ah>>s.Al>>s.num_components>>s.last_needed_pass;for(size_t i=0;i<4;++i)in>>s.components[i].comp_idx>>s.components[i].dc_tbl_idx>>s.components[i].ac_tbl_idx;size_t m;in>>m;s.reset_points.resize(m);for(auto& x:s.reset_points)in>>x;in>>m;s.extra_zero_runs.resize(m);for(auto& x:s.extra_zero_runs)in>>x.block_idx>>x.num_extra_zero_runs;}
 Bytes(in,j.marker_order);Vectors(in,j.inter_marker_data);Bytes(in,j.tail_data);in>>j.has_zero_padding_bit;Bytes(in,j.padding_bits);
 if(!in)throw std::runtime_error("JPEG metadata fixture input");
}
void PrintData(const JPEGData& j) {
 std::cout<<' '<<j.width<<' '<<j.height<<' '<<j.restart_interval;PrintVectors(j.app_data);std::cout<<' '<<j.app_marker_type.size();for(auto x:j.app_marker_type)std::cout<<' '<<static_cast<uint32_t>(x);PrintVectors(j.com_data);
 std::cout<<' '<<j.quant.size();for(auto& q:j.quant){std::cout<<' '<<q.precision<<' '<<q.index<<' '<<q.is_last;for(auto x:q.values)std::cout<<' '<<x;}
 std::cout<<' '<<j.huffman_code.size();for(auto& h:j.huffman_code){std::cout<<' '<<h.slot_id<<' '<<h.is_last;for(auto x:h.counts)std::cout<<' '<<x;for(auto x:h.values)std::cout<<' '<<x;}
 std::cout<<' '<<j.components.size();for(auto& c:j.components){std::cout<<' '<<c.id<<' '<<c.h_samp_factor<<' '<<c.v_samp_factor<<' '<<c.quant_idx<<' '<<c.width_in_blocks<<' '<<c.height_in_blocks<<' '<<c.coeffs.size();for(auto x:c.coeffs)std::cout<<' '<<x;}
 std::cout<<' '<<j.scan_info.size();for(auto& s:j.scan_info){std::cout<<' '<<s.Ss<<' '<<s.Se<<' '<<s.Ah<<' '<<s.Al<<' '<<s.num_components<<' '<<s.last_needed_pass;for(size_t i=0;i<4;++i)std::cout<<' '<<s.components[i].comp_idx<<' '<<s.components[i].dc_tbl_idx<<' '<<s.components[i].ac_tbl_idx;std::cout<<' '<<s.reset_points.size();for(auto x:s.reset_points)std::cout<<' '<<x;std::cout<<' '<<s.extra_zero_runs.size();for(auto& x:s.extra_zero_runs)std::cout<<' '<<x.block_idx<<' '<<x.num_extra_zero_runs;}
 PrintBytes(j.marker_order);PrintVectors(j.inter_marker_data);PrintBytes(j.tail_data);std::cout<<' '<<j.has_zero_padding_bit;PrintBytes(j.padding_bits);
}
struct Input {
 std::vector<uint8_t> backing;size_t size;
 explicit Input(const std::string& h){auto v=Unhex(h);size=v.size();backing.resize(size+16);std::copy(v.begin(),v.end(),backing.begin()+8);}
 jxl::Bytes bytes()const{return jxl::Bytes(backing.data()+8,size);}
};
}  // namespace
bool JpegMetadataCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
 if(command.rfind("jm-",0)!=0)return false;
 if(command=="jm-packed-fixture") {
  std::string h;int effort;in>>h>>effort;auto v=Unhex(h);JPEGData j;
  auto result=ReadJpeg(v.data(),v.size(),JpegReadMode::kReadAll,&j);if(!result)throw std::runtime_error("source packed JPEG fixture parse");
  for(auto& scan:j.scan_info)for(size_t i=scan.num_components;i<4;++i)scan.components[i]={0,0,0};
  jxl::CompressParams params;params.brotli_effort=effort;std::vector<uint8_t> encoded;
  auto status=EncodeJPEGData(mm,j,&encoded,params);
  std::cout<<" P "<<int(status.code())<<' '<<Hex(jxl::Bytes(encoded))<<" S";PrintData(j);return true;
 }
 if(command=="jm-fixture") {
  std::string h;in>>h;auto v=Unhex(h);JPEGData j;
  auto result=ReadJpeg(v.data(),v.size(),JpegReadMode::kReadAll,&j);if(!result)throw std::runtime_error("source JPEG fixture parse");
  j.app_marker_type.resize(j.app_data.size(),AppMarkerType::kUnknown);
  // ReadJpeg initializes only active scan component entries. Seed unused fixture
  // entries explicitly before observing the complete owner representation.
  for(auto& s:j.scan_info)for(size_t i=s.num_components;i<4;++i)s.components[i]={0,0,0};
  PrintData(j);return true;
 }
 JPEGData j;ReadData(in,j);size_t n;in>>n;
 for(size_t step=0;step<n;++step){std::string op;in>>op;
  if(op=="W") {size_t prefix;in>>prefix;jxl::BitWriter w(mm);if(!Prefix(w,prefix))throw std::runtime_error("prefix");auto s=jxl::Bundle::Write(j,&w,jxl::LayerType::Header,nullptr);size_t bits=w.BitsWritten();if(!Pad(w))throw std::runtime_error("pad");std::cout<<" W "<<int(s.code())<<' '<<bits<<' '<<Hex(w.GetSpan());}
  else if(op=="C") {size_t ext=0,total=0;auto s=jxl::Bundle::CanEncode(j,&ext,&total);std::cout<<" C "<<int(s.code())<<' '<<ext<<' '<<total;}
  else if(op=="D") {size_t offset;std::string h;in>>offset>>h;Input data(h);jxl::BitReader reader(data.bytes());reader.SkipBits(offset);auto s=jxl::Bundle::Read(&reader,&j);size_t bits=reader.TotalBitsConsumed();auto close=reader.Close();std::cout<<" D "<<int(s.code())<<' '<<bits<<' '<<int(close.code());}
  else if(op=="I") {std::string h;in>>h;auto v=Unhex(h);if(v.empty())v.reserve(1);auto s=SetJPEGDataFromICC(v,&j);std::cout<<" I "<<int(s.code());}
  else if(op=="E") {int effort;in>>effort;jxl::CompressParams params;params.brotli_effort=effort;std::vector<uint8_t> encoded;auto s=EncodeJPEGData(mm,j,&encoded,params);std::cout<<" E "<<int(s.code())<<' '<<Hex(jxl::Bytes(encoded));}
  else if(op=="Z") {int effort;std::string h;in>>effort>>h;auto payload=Unhex(h);if(payload.empty())payload.reserve(1);jxl::BitWriter w(mm);auto s=jxl::Bundle::Write(j,&w,jxl::LayerType::Header,nullptr);std::vector<uint8_t> encoded;if(s){if(!Pad(w))throw std::runtime_error("source raw fixture pad");jxl::Bytes(w.GetSpan()).AppendTo(encoded);size_t size=BrotliEncoderMaxCompressedSize(payload.size());std::vector<uint8_t> compressed(size);auto ok=BrotliEncoderCompress(effort,BROTLI_DEFAULT_WINDOW,BROTLI_MODE_GENERIC,payload.size(),payload.data(),&size,compressed.data());if(!ok)throw std::runtime_error("source raw fixture encode");encoded.insert(encoded.end(),compressed.begin(),compressed.begin()+size);}std::cout<<" Z "<<int(s.code())<<' '<<Hex(jxl::Bytes(encoded));}
  else if(op=="B") {std::string h;in>>h;Input data(h);auto s=DecodeJPEGData(data.bytes(),&j);std::cout<<" B "<<int(s.code());}
  else if(op=="H") {size_t index;int slot;in>>index>>slot;j.huffman_code.at(index).slot_id=slot;std::cout<<" H "<<index<<' '<<slot;}
  else throw std::runtime_error("unknown JPEG metadata operation");
  std::cout<<" S";PrintData(j);std::cout.flush();
 }
 return true;
}
