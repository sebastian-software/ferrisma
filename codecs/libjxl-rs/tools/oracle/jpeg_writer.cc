// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Data-only adapter. Fixture parsing and serialization invoke the unchanged
// original ReadJpeg, CalculateMcuSize and WriteJpeg archive symbols.
#include <algorithm>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include "lib/jxl/base/span.h"
#include "lib/jxl/jpeg/enc_jpeg_data_reader.h"
#include "lib/jxl/jpeg/dec_jpeg_data_writer.h"
std::string Hex(jxl::Bytes);
std::vector<uint8_t> Unhex(const std::string&);
namespace {
using namespace jxl::jpeg;
template<class T> void Number(std::istringstream& in,T& value) { in>>value; }
template<class T> void Numbers(std::istringstream& in,std::vector<T>& values) {
  size_t n;in>>n;values.resize(n);for(auto& v:values)in>>v;
}
void Bytes(std::istringstream& in,std::vector<uint8_t>& v) { std::string h;in>>h;v=Unhex(h); }
void ByteVectors(std::istringstream& in,std::vector<std::vector<uint8_t>>& values) {
  size_t n;in>>n;values.resize(n);for(auto& v:values)Bytes(in,v);
}
void PrintBytes(const std::vector<uint8_t>& v) { std::cout<<' '<<Hex(jxl::Bytes(v)); }
void PrintByteVectors(const std::vector<std::vector<uint8_t>>& v) {
  std::cout<<' '<<v.size();for(auto& x:v)PrintBytes(x);
}
void ReadData(std::istringstream& in,JPEGData& j) {
  in>>j.width>>j.height>>j.restart_interval;ByteVectors(in,j.app_data);ByteVectors(in,j.com_data);
  size_t n;in>>n;j.quant.resize(n);for(auto& q:j.quant) {
    in>>q.precision>>q.index>>q.is_last;for(auto& v:q.values)in>>v;
  }
  in>>n;j.huffman_code.resize(n);for(auto& h:j.huffman_code) {
    in>>h.slot_id>>h.is_last;for(auto& v:h.counts)in>>v;for(auto& v:h.values)in>>v;
  }
  in>>n;j.components.resize(n);for(auto& c:j.components) {
    in>>c.id>>c.h_samp_factor>>c.v_samp_factor>>c.quant_idx>>c.width_in_blocks>>c.height_in_blocks;
    Numbers(in,c.coeffs);
  }
  in>>n;j.scan_info.resize(n);for(auto& s:j.scan_info) {
    in>>s.Ss>>s.Se>>s.Ah>>s.Al>>s.num_components;
    for(size_t i=0;i<s.num_components;++i)in>>s.components[i].comp_idx>>s.components[i].dc_tbl_idx>>s.components[i].ac_tbl_idx;
    Numbers(in,s.reset_points);in>>n;s.extra_zero_runs.resize(n);
    for(auto& z:s.extra_zero_runs)in>>z.block_idx>>z.num_extra_zero_runs;
  }
  Bytes(in,j.marker_order);ByteVectors(in,j.inter_marker_data);Bytes(in,j.tail_data);
  in>>j.has_zero_padding_bit;Bytes(in,j.padding_bits);bool allocated;in>>allocated;
  if(allocated&&j.padding_bits.empty())j.padding_bits.reserve(1);
}
void PrintData(const JPEGData& j) {
  std::cout<<' '<<j.width<<' '<<j.height<<' '<<j.restart_interval;
  PrintByteVectors(j.app_data);PrintByteVectors(j.com_data);
  std::cout<<' '<<j.quant.size();for(auto& q:j.quant) {
    std::cout<<' '<<q.precision<<' '<<q.index<<' '<<q.is_last;for(auto v:q.values)std::cout<<' '<<v;
  }
  std::cout<<' '<<j.huffman_code.size();for(auto& h:j.huffman_code) {
    std::cout<<' '<<h.slot_id<<' '<<h.is_last;for(auto v:h.counts)std::cout<<' '<<v;for(auto v:h.values)std::cout<<' '<<v;
  }
  std::cout<<' '<<j.components.size();for(auto& c:j.components) {
    std::cout<<' '<<c.id<<' '<<c.h_samp_factor<<' '<<c.v_samp_factor<<' '<<c.quant_idx<<' '<<c.width_in_blocks<<' '<<c.height_in_blocks<<' '<<c.coeffs.size();
    for(auto v:c.coeffs)std::cout<<' '<<v;
  }
  std::cout<<' '<<j.scan_info.size();for(auto& s:j.scan_info) {
    std::cout<<' '<<s.Ss<<' '<<s.Se<<' '<<s.Ah<<' '<<s.Al<<' '<<s.num_components;
    for(size_t i=0;i<s.num_components;++i)std::cout<<' '<<s.components[i].comp_idx<<' '<<s.components[i].dc_tbl_idx<<' '<<s.components[i].ac_tbl_idx;
    std::cout<<' '<<s.reset_points.size();for(auto v:s.reset_points)std::cout<<' '<<v;
    std::cout<<' '<<s.extra_zero_runs.size();for(auto z:s.extra_zero_runs)std::cout<<' '<<z.block_idx<<' '<<z.num_extra_zero_runs;
  }
  PrintBytes(j.marker_order);PrintByteVectors(j.inter_marker_data);PrintBytes(j.tail_data);
  std::cout<<' '<<j.has_zero_padding_bit;PrintBytes(j.padding_bits);
  std::cout<<' '<<(j.padding_bits.data()!=nullptr);
}
void Mutate(JPEGData& j,int mode,size_t arg) {
  switch(mode) {
    case 0:break;
    case 1:j.marker_order.clear();break;
    case 2:j.marker_order.at(arg)=0;break;
    case 3:j.components.at(0).quant_idx=999;break;
    case 4:j.scan_info.at(0).components[0].comp_idx=999;break;
    case 5:j.marker_order.erase(std::remove(j.marker_order.begin(),j.marker_order.end(),0xc4),j.marker_order.end());break;
    case 6:j.has_zero_padding_bit=true;j.padding_bits.assign(arg,2);break;
    case 7:j.has_zero_padding_bit=true;j.padding_bits.assign(arg,0);break;
    case 8:j.app_data.clear();break;
    case 9:for(auto& q:j.quant)q.precision=1;break;
    case 10:for(auto& s:j.scan_info)s.reset_points={0,1,3};break;
    case 11:j.scan_info.at(0).extra_zero_runs={{0,uint32_t(arg)}};break;
    case 12:j.tail_data.reserve(1);j.tail_data.clear();break;
    case 13:
      j.app_data.insert(j.app_data.begin(),{0xef,0,6,0,0xff,0x80,0x7f});
      j.com_data.insert(j.com_data.begin(),{0xfe,0,4,0,0xff});
      j.inter_marker_data.insert(j.inter_marker_data.begin(),{0x12,0xff,0,0x34});
      j.marker_order.insert(j.marker_order.begin(),{0xef,0xfe,0xff});
      j.tail_data={0,0xff,1,2,0x80};break;
    case 15:j.marker_order={uint8_t(arg)};break;
    case 16:case 17:
      for(auto& marker:j.marker_order)if(marker==0xc0||marker==0xc1)marker=0xc2;
      j.scan_info.at(0).Ss=0;j.scan_info.at(0).Se=63;
      j.scan_info.at(0).Ah=(mode==17);j.scan_info.at(0).Al=arg;break;
    default:throw std::runtime_error("unknown JPEG mutation");
  }
}
}
bool JpegWriterCommand(const std::string& command,std::istringstream& in) {
  if(command=="jw-fixture") {
    std::string h;in>>h;auto data=Unhex(h);JPEGData jpg;
    auto s=ReadJpeg(data.data(),data.size(),JpegReadMode::kReadAll,&jpg);
    std::cout<<int(s.code());if(s)PrintData(jpg);return true;
  }
  if(command!="jw-write"&&command!="jw-mcu")return false;
  size_t cap,fail,arg;int mode;
  if(command=="jw-write")in>>cap>>fail>>mode>>arg;
  JPEGData jpg;ReadData(in,jpg);
  if(command=="jw-mcu") {
    for(auto& scan:jpg.scan_info){int x,y;jpg.CalculateMcuSize(scan,&x,&y);std::cout<<' '<<x<<' '<<y;}return true;
  }
  Mutate(jpg,mode,arg);std::vector<uint8_t> written,previous;std::vector<std::string> calls;
  auto s=WriteJpeg(jpg,[&](const uint8_t* data,size_t len){
    size_t n=calls.size()==fail?0:std::min(cap,len);
    // Lossless protocol: a 'p' offer is the preceding offer's exact len-byte
    // prefix; an 'f' offer carries every byte literally. Keep all callbacks,
    // statuses, nullness and bytes without quadratic duplicate hex strings.
    const bool prefix=len<=previous.size()&&(len==0||std::memcmp(data,previous.data(),len)==0);
    std::ostringstream call;call<<len<<' '<<n<<' '<<(data==nullptr)<<' ';
    if(prefix){call<<'p';previous.resize(len);}
    else {call<<'f'<<Hex(jxl::Bytes(data,len));if(len)previous.assign(data,data+len);else previous.clear();}
    calls.push_back(call.str());
    if(n)written.insert(written.end(),data,data+n);return n;
  });
  std::cout<<int(s.code())<<' '<<calls.size();for(auto& call:calls)std::cout<<' '<<call;
  PrintBytes(written);return true;
}
