// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Separate observer TU: every private numeric/parser operation below comes from
// this pristine source inclusion. It is never linked into the public oracle.
#include "../../upstream/libjxl/third_party/skcms/skcms.cc"
#include "skcms_parser.cc"

std::vector<uint8_t> Unhex(const std::string& hex) {
  std::vector<uint8_t> bytes;
  if(hex=="-") return bytes;
  for(size_t i=0;i<hex.size();i+=2) bytes.push_back(uint8_t(std::strtoul(hex.substr(i,2).c_str(),nullptr,16)));
  return bytes;
}
namespace {
float FloatWord(uint32_t word) {float value;std::memcpy(&value,&word,4);return value;}
}
int main() {
  std::string line;
  while(std::getline(std::cin,line)) {
    std::istringstream in(line);std::string command;in>>command;
    if(command.empty()) continue;
    if(SkcmsParserCommand(command,in)) {std::cout<<'\n';continue;}
    if(command=="skcms-curve-source-large") {
      unsigned operation;uint32_t count,x;in>>operation>>count>>x;
      // Only the reached source indices need physical storage. Defined controls
      // use x==0 (both indices zero) or the canonical count>INT_MAX guard. Each
      // rejected probe must positively diagnose its earlier arithmetic UB before
      // any out-of-bounds payload access; the audit rejects all other failures.
      const uint8_t table[2]={0x3f,0x80};skcms_Curve c{};c.table_entries=count;
      c.table_8=operation==2?nullptr:table;c.table_16=operation==2?table:nullptr;
      if(operation==0){void(*volatile call)(skcms_Curve*)=canonicalize_identity;call(&c);std::cout<<'C';Curve(c,table);}
      else{float(*volatile call)(const skcms_Curve*,float)=eval_curve;std::cout<<Word(call(&c,FloatWord(x)));}
      std::cout<<'\n';continue;
    }
    if(command=="skcms-curve-read") {
      std::string hex;uint32_t size;unsigned nulls;in>>size>>nulls>>hex;auto bytes=Unhex(hex);
      if(bytes.empty())bytes.resize(1);
      skcms_Curve curve={{0,{2.2f,0.8f,0.1f,0.2f,0.3f,0.4f,0.5f}}};
      uint32_t curve_size=0x87654321;
      bool (*volatile call)(const uint8_t*,uint32_t,skcms_Curve*,uint32_t*)=read_curve;
      bool ok=call((nulls&1)?nullptr:bytes.data(),size,(nulls&2)?nullptr:&curve,(nulls&4)?nullptr:&curve_size);
      std::cout<<int(ok)<<' '<<curve_size;Curve(curve,bytes.data());std::cout<<'\n';continue;
    }
    unsigned kind;in>>kind;skcms_Curve curve;std::memset(&curve,0,sizeof(curve));
    std::vector<uint8_t> bytes;
    if(kind==0) {
      uint32_t words[7];for(auto& word:words)in>>word;std::memcpy(&curve.parametric,words,sizeof(words));
    } else {
      std::string hex;in>>hex;bytes=Unhex(hex);
      curve.table_entries=uint32_t(bytes.size()/(kind==1?1:2));
      curve.table_8=kind==1?bytes.data():nullptr;curve.table_16=kind==2?bytes.data():nullptr;
    }
    if(command=="skcms-curve-eval") {
      uint32_t x;in>>x;float(*volatile call)(const skcms_Curve*,float)=eval_curve;
      std::cout<<Word(call(&curve,FloatWord(x)));
    } else if(command=="skcms-curve-linear") {
      int n;uint32_t tol;unsigned aliases;in>>n>>tol>>aliases;
      float slots[3]={0.125f,0.25f,0.5f};
      float* c=&slots[0];float* d=&slots[(aliases&1)?0:1];
      float* f=(aliases&2)?nullptr:&slots[(aliases&4)?0:((aliases&8)?1:2)];
      int(*volatile call)(const skcms_Curve*,int,float,float*,float*,float*)=fit_linear;
      int points=call(&curve,n,FloatWord(tol),c,d,f);std::cout<<points;
      for(float x:slots)std::cout<<' '<<Word(x);
    } else if(command=="skcms-curve-canonical") {
      void(*volatile call)(skcms_Curve*)=canonicalize_identity;call(&curve);std::cout<<'C';Curve(curve,bytes.data());
    } else {std::cerr<<"unknown private skcms command: "<<command<<'\n';return 2;}
    std::cout<<'\n';
  }
}
