// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Calls original section/permutation functions. Seed caller storage before
// observing partial outputs; no serialization algorithm lives in this adapter.
#include "lib/jxl/toc.h"
#include "lib/jxl/enc_toc.h"
#include "lib/jxl/coeff_order.h"
#include "lib/jxl/enc_coeff_order.h"
#include "lib/jxl/lehmer_code.h"
template<class T> void TocValues(const std::vector<T>& values) {
  std::cout<<' '<<values.size();for(auto v:values) std::cout<<' '<<v;
}
void TocAux(const jxl::AuxOut& aux) {
  for(auto layer:{LayerType::Header,LayerType::Toc}) {auto a=aux.layer(layer);std::cout<<' '<<a.total_bits<<','<<a.histogram_bits<<','<<a.extra_bits;}
}
template<class T> void TocLehmer(std::istringstream& in,bool decode) {
  size_t n;in>>n;std::vector<T> permutation(n,55);std::vector<uint32_t> code(n,33);
  size_t padded=n?size_t{1}<<jxl::CeilLog2Nonzero(n):1;
  std::vector<uint32_t> temp(decode?padded:n+1,77);
  Status status=false;
  if(decode) {for(auto& v:code) in>>v;status=jxl::DecodeLehmerCode(code.data(),temp.data(),n,permutation.data());}
  else {for(auto& v:permutation) {unsigned x;in>>x;v=x;}status=jxl::ComputeLehmerCode(permutation.data(),temp.data(),n,code.data());}
  std::cout<<Code(status);TocValues(permutation);TocValues(code);TocValues(temp);
}
void TocReadObservation(size_t n,size_t prefix,const std::vector<uint8_t>& bytes,JxlMemoryManager* mm) {
  // Match BitReader::new's non-null empty slice, as the shared Input fixture
  // does. A null span has a separate Close lifecycle in the original reader.
  std::vector<uint8_t> padded=bytes;padded.resize(bytes.size()+8);jxl::Bytes span(padded.data(),bytes.size());
  std::vector<uint32_t> sizes={7,8};std::vector<jxl::coeff_order_t> permutation={2,0,1};
  BitReader br(span);br.SkipBits(prefix);Status read=jxl::ReadToc(mm,n,&br,&sizes,&permutation);
  std::cout<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());TocValues(sizes);TocValues(permutation);
  sizes={7,8};std::vector<uint64_t> offsets={99,88};uint64_t total=77;
  BitReader groups(span);groups.SkipBits(prefix);read=jxl::ReadGroupOffsets(mm,n,&groups,&offsets,&sizes,&total);
  std::cout<<' '<<Code(read)<<' '<<groups.TotalBitsConsumed()<<' '<<Code(groups.Close())<<' '<<total;TocValues(sizes);TocValues(offsets);
}
bool TocCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if(command=="toc-lehmer16" || command=="toc-lehmer32" || command=="toc-lehmer-read16" || command=="toc-lehmer-read32") {
    bool decode=command.find("read")!=std::string::npos;
    if(command.find("16")!=std::string::npos) TocLehmer<uint16_t>(in,decode);else TocLehmer<uint32_t>(in,decode);
  } else if(command=="toc-layout") {
    size_t pass,group,ng,nd,np,n,rows,cols;uint32_t value;in>>pass>>group>>ng>>nd>>np>>n>>rows>>cols>>value;
    size_t r=rows,c=cols;jxl::CoefficientLayout(&r,&c);
    std::cout<<jxl::AcGroupIndex(pass,group,ng,nd)<<' '<<jxl::NumTocEntries(ng,nd,np)<<' '<<jxl::MaxBits(n)<<' '<<jxl::CoefficientRows(rows,cols)<<' '<<jxl::CoefficientColumns(rows,cols)<<' '<<r<<' '<<c<<' '<<jxl::CoeffOrderContext(value);
  } else if(command=="toc-permutation") {
    size_t skip,n,prefix;in>>skip>>n>>prefix;std::vector<uint32_t> order(n);for(auto& v:order) in>>v;
    BitWriter writer(mm);if(!Prefix(writer,prefix)) throw std::runtime_error("permutation prefix");jxl::AuxOut aux;
    Status status=jxl::EncodePermutation(order.data(),skip,n,&writer,LayerType::Header,&aux);
    std::cout<<Code(status)<<' '<<writer.BitsWritten();if(!Pad(writer)) throw std::runtime_error("permutation pad");std::cout<<' '<<Hex(writer.GetSpan());TocAux(aux);
    if(status) {std::vector<uint32_t> decoded(n,55);BitReader br(writer.GetSpan());br.SkipBits(prefix);Status read=jxl::DecodePermutation(mm,skip,n,decoded.data(),&br);std::cout<<' '<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());TocValues(decoded);}
  } else if(command=="toc-permutation-read") {
    size_t skip,n,prefix;bool null;std::string hex;in>>skip>>n>>prefix>>null>>hex;
    std::vector<uint32_t> decoded(n,55);Input input(hex);BitReader br(input.bytes());br.SkipBits(prefix);Status read=jxl::DecodePermutation(mm,skip,n,null?nullptr:decoded.data(),&br);std::cout<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());TocValues(decoded);
  } else if(command=="toc-write") {
    size_t n,prefix;bool perm;in>>n>>perm>>prefix;std::vector<size_t> sizes(n);for(auto& v:sizes) in>>v;
    std::vector<uint32_t> permutation(perm?n:0);for(auto& v:permutation) in>>v;
    BitWriter writer(mm);if(!Prefix(writer,prefix)) throw std::runtime_error("toc prefix");jxl::AuxOut aux;
    Status first=jxl::WriteTocPermutation(permutation,&writer,&aux),second=false;
    if(first) second=jxl::WriteTocSizes(sizes,&writer,&aux);
    std::cout<<Code(first)<<' '<<Code(second)<<' '<<writer.BitsWritten();if(!Pad(writer)) throw std::runtime_error("toc pad");std::cout<<' '<<Hex(writer.GetSpan());TocAux(aux);
    std::vector<uint8_t> bytes(writer.GetSpan().begin(),writer.GetSpan().end());std::cout<<' ';TocReadObservation(n,prefix,bytes,mm);
  } else if(command=="toc-read") {
    size_t n,prefix;std::string hex;in>>n>>prefix>>hex;Input input(hex);std::vector<uint8_t> bytes(input.bytes().begin(),input.bytes().end());TocReadObservation(n,prefix,bytes,mm);
  } else return false;
  return true;
}
