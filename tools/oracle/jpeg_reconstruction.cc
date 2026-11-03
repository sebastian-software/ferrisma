// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Data-only fixture orchestration; actual pristine DecodeFrame and WriteJpeg
// perform coefficient reconstruction and JPEG serialization.
#include <jxl/cms.h>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include "lib/jxl/dec_frame.h"
#include "lib/jxl/headers.h"
#include "lib/jxl/jpeg/jpeg_data.h"
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

void Observe(const JPEGData& j) {
 std::cout<<' '<<j.width<<' '<<j.height<<' '<<j.components.size();
 for(const auto& c:j.components){std::cout<<' '<<c.id<<' '<<c.quant_idx<<' '<<c.h_samp_factor<<' '<<c.v_samp_factor<<' '<<c.width_in_blocks<<' '<<c.height_in_blocks<<' '<<c.coeffs.size();for(auto v:c.coeffs)std::cout<<' '<<v;}
 std::cout<<" Q "<<j.quant.size();for(const auto& q:j.quant){std::cout<<' '<<q.index<<' '<<q.precision;for(auto v:q.values)std::cout<<' '<<v;}
}
}
bool JpegReconstructionCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
 if(command!="jr-frame"&&command!="jr-frame-mut")return false;
 int mode=0;if(command=="jr-frame-mut")in>>mode;
 std::string hex;in>>hex;auto bytes=Unhex(hex);auto jpeg=std::make_unique<jxl::jpeg::JPEGData>();ReadData(in,*jpeg);
 if(mode!=9)for(auto& c:jpeg->components)c.coeffs.clear();
 if(mode==1)jpeg->components.clear();
 if(mode==2)jpeg->components.resize(2);
 if(mode==4)for(auto& c:jpeg->components)c.quant_idx=jpeg->quant.size();
 if(mode==5){jpeg->quant.insert(jpeg->quant.begin(),jpeg->quant[0]);for(auto& c:jpeg->components)++c.quant_idx;}
 if(mode==6)jpeg->quant.push_back(jpeg->quant.back());
 if(mode==7)for(auto& c:jpeg->components)c.quant_idx=0;
 if(mode==10){jpeg->quant.clear();for(auto& c:jpeg->components)c.quant_idx=0;}
 std::vector<uint8_t> backing(bytes.size()+16);std::copy(bytes.begin(),bytes.end(),backing.begin()+8);
 jxl::CodecMetadata metadata;jxl::BitReader reader{jxl::Bytes(backing.data()+8,bytes.size())};
 auto status=jxl::Status(reader.ReadFixedBits<16>()==0x0aff);
 if(status)status=jxl::ReadSizeHeader(&reader,&metadata.size);
 if(status)status=jxl::ReadImageMetadata(&reader,&metadata.m);
 metadata.transform_data.nonserialized_xyb_encoded=metadata.m.xyb_encoded;
 if(status)status=jxl::Bundle::Read(&reader,&metadata.transform_data);
 if(status&&metadata.m.color_encoding.WantICC())throw std::runtime_error("JR fixture needs original ICC observer");
 if(status)status=reader.JumpToByteBoundary();
 size_t offset=reader.TotalBitsConsumed()/8;auto close=reader.Close();
 std::cout<<int(status.code())<<' '<<offset<<' '<<int(close.code());
 if(!status||!close)return true;
 jxl::PassesDecoderState state(mm);jxl::ImageBundle decoded(mm,&metadata.m);decoded.jpeg_data=std::move(jpeg);
 jxl::FrameHeader header(&metadata);
 status=jxl::DecodeFrame(&state,nullptr,bytes.data()+offset,bytes.size()-offset,&header,&decoded,metadata,true);
 std::cout<<" F "<<int(status.code())<<' '<<decoded.decoded_bytes()<<' '<<unsigned(decoded.color_transform);
 for(size_t c=0;c<3;++c)std::cout<<' '<<decoded.chroma_subsampling.HShift(c)<<' '<<decoded.chroma_subsampling.VShift(c);
 Observe(*decoded.jpeg_data);
 if(status){std::vector<uint8_t> result;auto written=jxl::jpeg::WriteJpeg(*decoded.jpeg_data,[&](const uint8_t* p,size_t n){if(n)result.insert(result.end(),p,p+n);return n;});std::cout<<" W "<<int(written.code())<<' '<<Hex(jxl::Bytes(result));}
 return true;
}
