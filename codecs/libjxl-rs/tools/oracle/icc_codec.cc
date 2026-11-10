// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Invoke pristine command decoding, including its private helper boundaries.
#include <cstdint>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include "lib/jxl/icc_codec.cc"
#include "lib/jxl/enc_icc_codec.h"
// Public source symbol intentionally omitted from the production header.
namespace jxl { Status PredictICC(const uint8_t*, size_t, PaddedBytes*); }
std::string Hex(jxl::Bytes);
std::vector<uint8_t> Unhex(const std::string&);
bool IccCodecCommand(const std::string& command, std::istringstream& in,
                     JxlMemoryManager* mm) {
  if (command.rfind("ic-", 0) != 0) return false;
  if (command == "ic-contexts") {
    size_t i; in >> i;
    for (size_t b1=0;b1<256;++b1) for (size_t b2=0;b2<256;++b2)
      std::cout << ' ' << jxl::ICCANSContext(i,b1,b2);
    return true;
  }
  if (command == "ic-bounds") {
    uint64_t a,b,size,value; in >> a >> b >> size >> value;
    std::cout << int(jxl::CheckOutOfBounds(a,b,size).code()) << ' '
              << int(jxl::CheckIs32Bit(value).code());
    return true;
  }
  if (command == "ic-varint") {
    size_t pos; uint64_t out; std::string hex; in >> pos >> out >> hex;
    auto data=Unhex(hex); auto s=jxl::DecodeVarInt(data.data(),data.size(),&pos,&out);
    std::cout << int(s.code()) << ' ' << pos << ' ' << out; return true;
  }
  if (command == "ic-header") {
    uint32_t size; size_t pos; std::string hex; in >> size >> pos >> hex;
    auto data=Unhex(hex); auto header=jxl::ICCInitialHeaderPrediction(size);
    jxl::ICCPredictHeader(data.data(),data.size(),header.data(),pos);
    std::cout << Hex(jxl::Bytes(header.data(),header.size())); return true;
  }
  if (command == "ic-access") {
    size_t pos; std::string hex,taghex; in >> pos >> hex >> taghex;
    auto data=Unhex(hex); auto tagbytes=Unhex(taghex);
    jxl::Tag tag; std::copy_n(tagbytes.begin(),4,tag.begin());
    std::cout << jxl::DecodeUint32(data.data(),data.size(),pos) << ' ';
    auto decoded=jxl::DecodeKeyword(data.data(),data.size(),pos);
    std::cout << Hex(jxl::Bytes(decoded.data(),4)) << ' ';
    jxl::EncodeKeyword(tag,data.data(),data.size(),pos);
    std::cout << Hex(jxl::Bytes(data));
    jxl::PaddedBytes result(mm);
    auto s=jxl::AppendUint32(0x89abcdef,&result); if(s)s=jxl::AppendKeyword(tag,&result);
    std::cout << ' ' << int(s.code()) << ' ' << Hex(jxl::Bytes(result)); return true;
  }
  if (command == "ic-linear") {
    size_t start,stride,width,count; int order; std::string hex;
    in >> start >> stride >> width >> order >> count >> hex; auto data=Unhex(hex);
    for(size_t i=0;i<count;++i) std::cout << ' ' << unsigned(jxl::LinearPredictICCValue(data.data(),start,i,stride,width,order));
    return true;
  }
  if (command == "ic-shuffle") {
    size_t width; std::string hex; in >> width >> hex; auto data=Unhex(hex);
    auto s=jxl::Shuffle(mm,data.data(),data.size(),width);
    std::cout << int(s.code()) << ' ' << Hex(jxl::Bytes(data)); return true;
  }
  if (command == "ic-preamble") {
    size_t size; std::string hex; in >> size >> hex; auto data=Unhex(hex);
    jxl::PaddedBytes buffer(mm); if(!data.empty()&&!buffer.append(data))throw std::runtime_error("ICC fixture allocation");
    std::cout << int(jxl::CheckPreamble(buffer,size).code()); return true;
  }
  if (command == "ic-encode" || command == "ic-unpredict") {
    std::string hex; in >> hex; auto data=Unhex(hex); jxl::PaddedBytes result(mm);
    if(command=="ic-unpredict") { std::string seed;in>>seed;auto bytes=Unhex(seed);
      if(!bytes.empty()&&!result.append(bytes))throw std::runtime_error("ICC seed allocation"); }
    auto s=command=="ic-encode" ? jxl::PredictICC(data.data(),data.size(),&result)
                                : jxl::UnpredictICC(data.data(),data.size(),&result);
    std::cout << int(s.code()) << ' ' << Hex(jxl::Bytes(result)); return true;
  }
  throw std::runtime_error("unknown ICC command");
}
