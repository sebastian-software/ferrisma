// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Data-only observers shared by actual archive and pristine-include contexts.
#ifndef LIBJXL_RS_DEFAULT_CMS_OBSERVE_H_
#define LIBJXL_RS_DEFAULT_CMS_OBSERVE_H_
#include <jxl/cms.h>
#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

namespace default_cms_observer {
inline uint32_t Word(float x) {uint32_t u;std::memcpy(&u,&x,4);return u;}
inline float Float(uint32_t u) {float x;std::memcpy(&x,&u,4);return x;}
inline std::vector<uint8_t> Unhex(const std::string& hex) {
  if(hex=="-")return {};
  if(hex.size()%2)std::abort();
  std::vector<uint8_t> bytes;
  for(size_t i=0;i<hex.size();i+=2)bytes.push_back(uint8_t(std::stoul(hex.substr(i,2),nullptr,16)));
  return bytes;
}
template<class U> inline void Raw(const void* object,size_t offset) {
  U word;std::memcpy(&word,static_cast<const uint8_t*>(object)+offset,sizeof(word));std::cout<<' '<<word;
}
inline void Encoding(const JxlColorEncoding& c) {
  Raw<uint32_t>(&c,offsetof(JxlColorEncoding,color_space));
  Raw<uint32_t>(&c,offsetof(JxlColorEncoding,white_point));
  for(size_t i=0;i<2;++i)Raw<uint64_t>(&c,offsetof(JxlColorEncoding,white_point_xy)+i*8);
  Raw<uint32_t>(&c,offsetof(JxlColorEncoding,primaries));
  for(size_t i=0;i<2;++i)Raw<uint64_t>(&c,offsetof(JxlColorEncoding,primaries_red_xy)+i*8);
  for(size_t i=0;i<2;++i)Raw<uint64_t>(&c,offsetof(JxlColorEncoding,primaries_green_xy)+i*8);
  for(size_t i=0;i<2;++i)Raw<uint64_t>(&c,offsetof(JxlColorEncoding,primaries_blue_xy)+i*8);
  Raw<uint32_t>(&c,offsetof(JxlColorEncoding,transfer_function));
  Raw<uint64_t>(&c,offsetof(JxlColorEncoding,gamma));
  Raw<uint32_t>(&c,offsetof(JxlColorEncoding,rendering_intent));
}
inline std::string Hex(const uint8_t* bytes,size_t size) {
  if(size==0)return "-";
  const char* digits="0123456789abcdef";std::string text; text.reserve(size*2);
  for(size_t i=0;i<size;++i){text.push_back(digits[bytes[i]>>4]);text.push_back(digits[bytes[i]&15]);}
  return text;
}
inline JXL_BOOL PrefixFail(void* data,const uint8_t* bytes,size_t size,JxlColorEncoding*,JXL_BOOL*) {
  ++*static_cast<size_t*>(data);std::cout<<' '<<size<<' '<<Hex(bytes,size);return 0;
}
struct CallbackState;
struct CallbackData {CallbackState* state;unsigned tag;};
struct CallbackState {
  unsigned mode,calls=0;JxlCmsInterface* interface;JxlColorProfile* source;JxlColorProfile* destination;
  std::vector<uint8_t>* replacement_source;std::vector<uint8_t>* replacement_destination;
  CallbackData first,second;
};
inline JXL_BOOL CallbackSecond(void*,const uint8_t*,size_t,JxlColorEncoding*,JXL_BOOL*);
inline JXL_BOOL Callback(void* opaque,const uint8_t* bytes,size_t size,JxlColorEncoding* output,JXL_BOOL* cmyk) {
  auto* data=static_cast<CallbackData*>(opaque);auto& s=*data->state;const size_t call=++s.calls;
  JxlColorEncoding value;std::memset(&value,0x5a,sizeof(value));JXL_BOOL local_cmyk=0x12345678;
  const auto* original=JxlGetDefaultCms();const auto delegated=original->set_fields_from_icc(original->set_fields_data,bytes,size,&value,&local_cmyk);
  const bool forced=(call==1&&(s.mode&1))||(call==2&&(s.mode&2));
  const JXL_BOOL result=forced?0:delegated?((s.mode&32)?-9:1):0;
  std::cout<<' '<<data->tag<<' '<<(bytes==s.source->icc.data)<<' '<<(bytes==s.destination->icc.data)
    <<' '<<size<<' '<<Hex(bytes,size)<<' '<<delegated<<' '<<result<<' '<<local_cmyk;Encoding(value);
  if(delegated) {
    if(s.mode&64)local_cmyk=-7;
    *cmyk=local_cmyk;
    if(s.mode&128) {
      // Only fields actually read by FromExternal are initialized in the
      // caller's otherwise unknown external carrier.
      output->color_space=value.color_space;output->white_point=value.white_point;
      if(value.white_point==JXL_WHITE_POINT_CUSTOM)std::memcpy(output->white_point_xy,value.white_point_xy,16);
      if(value.color_space==JXL_COLOR_SPACE_RGB||value.color_space==JXL_COLOR_SPACE_UNKNOWN) {
        output->primaries=value.primaries;
        if(value.primaries==JXL_PRIMARIES_CUSTOM) {
          std::memcpy(output->primaries_red_xy,value.primaries_red_xy,16);
          std::memcpy(output->primaries_green_xy,value.primaries_green_xy,16);
          std::memcpy(output->primaries_blue_xy,value.primaries_blue_xy,16);
        }
      }
      output->transfer_function=value.transfer_function;
      if(value.transfer_function==JXL_TRANSFER_FUNCTION_GAMMA)output->gamma=value.gamma;
      output->rendering_intent=value.rendering_intent;
    } else *output=value;
  }
  if(call==1&&(s.mode&4)) {
    s.interface->set_fields_data=&s.second;s.interface->set_fields_from_icc=&CallbackSecond;
  }
  if(call==1&&(s.mode&8))s.destination->icc={s.replacement_destination->data(),s.replacement_destination->size()};
  if(call==2&&(s.mode&16))s.source->icc={s.replacement_source->data(),s.replacement_source->size()};
  return result;
}
inline JXL_BOOL CallbackSecond(void* data,const uint8_t* bytes,size_t size,JxlColorEncoding* output,JXL_BOOL* cmyk) {
  return Callback(data,bytes,size,output,cmyk);
}
inline bool Public(const std::string& command,std::istringstream& in) {
  const JxlCmsInterface* (*volatile getter)()=&JxlGetDefaultCms;
  const auto* cms=getter();
  if(command=="default-cms-init-prefix") {
    unsigned mode;in>>mode;JxlCmsInterface interface;JxlColorProfile source,destination;size_t calls=0;const uint8_t byte=0x5a;
    if(mode==1||mode==2)source.icc.size=0;
    else if(mode>=3) {
      source.icc.size=1;destination.icc.size=mode<=4?0:1;
      if(mode>=5) {
        source.icc.data=&byte;interface.set_fields_data=&calls;interface.set_fields_from_icc=&PrefixFail;
      }
    }
    std::cout<<"calls";
    const size_t dimension=(mode&1)?size_t(-1):0;
    void* context=cms->init(mode==0?nullptr:&interface,dimension,dimension,
      mode==0?nullptr:&source,mode<=2?nullptr:&destination,default_cms_observer::Float(0x7fc12345));
    std::cout<<" end "<<calls<<' '<<(context!=nullptr);if(context)std::abort();return true;
  }
  if(command=="default-cms-callback") {
    unsigned mode;std::string a,b,c,d;in>>mode>>a>>b>>c>>d;
    auto source=Unhex(a),destination=Unhex(b),replacement_source=Unhex(c),replacement_destination=Unhex(d);
    // Inactive interface and descriptor members genuinely remain unknown.
    JxlCmsInterface interface;JxlColorProfile input,output;
    input.icc={source.data(),source.size()};output.icc={destination.data(),destination.size()};
    CallbackState state{mode,0,&interface,&input,&output,&replacement_source,&replacement_destination,{nullptr,1},{nullptr,2}};
    state.first.state=&state;state.second.state=&state;
    interface.set_fields_data=&state.first;interface.set_fields_from_icc=&Callback;
    std::cout<<"events";
    void* context=cms->init(&interface,2,2,&input,&output,80.0f);
    std::cout<<" end "<<state.calls<<' '<<(context!=nullptr)<<' '
      <<(interface.set_fields_data==&state.second)<<' '<<(interface.set_fields_from_icc==&CallbackSecond)
      <<' '<<Hex(input.icc.data,input.icc.size)<<' '<<Hex(output.icc.data,output.icc.size);
    if(context) {
      float values[8]={0.2f,0.7f,1.2f,-0.1f,0.6f,0.3f,0.8f,0.4f};float result[8];
      for(float& x:result)x=Float(0xbf654321);
      std::cout<<' '<<(uintptr_t(cms->get_src_buf(context,1))%128)<<' '<<(uintptr_t(cms->get_dst_buf(context,1))%128)
        <<' '<<cms->run(context,1,values,result,2);
      for(float x:values)std::cout<<' '<<Word(x);for(float x:result)std::cout<<' '<<Word(x);
      cms->destroy(context);
    }
    return true;
  }
  if(command=="default-cms-bindings") {
    std::cout<<reinterpret_cast<uintptr_t>(getter)<<' '
      <<reinterpret_cast<uintptr_t>(cms->set_fields_from_icc)<<' '
      <<reinterpret_cast<uintptr_t>(cms->init)<<' '
      <<reinterpret_cast<uintptr_t>(cms->get_src_buf)<<' '
      <<reinterpret_cast<uintptr_t>(cms->get_dst_buf)<<' '
      <<reinterpret_cast<uintptr_t>(cms->run)<<' '
      <<reinterpret_cast<uintptr_t>(cms->destroy);return true;
  }
  if(command=="default-cms-interface") {
    std::cout<<(cms==getter())<<' '<<(cms->init_data==cms)<<' '<<(cms->set_fields_data==nullptr)
      <<' '<<(cms->set_fields_from_icc!=nullptr)<<' '<<(cms->init!=nullptr)
      <<' '<<(cms->get_src_buf!=nullptr)<<' '<<(cms->get_dst_buf!=nullptr)
      <<' '<<(cms->run!=nullptr)<<' '<<(cms->destroy!=nullptr);
    cms->destroy(nullptr);return true;
  }
  if(command=="default-cms-fields") {
    unsigned flags;std::string hex;in>>flags>>hex;auto bytes=Unhex(hex);const size_t size=bytes.size();
    bytes.resize(size+16);JxlColorEncoding c;std::memset(&c,0x5a,sizeof(c));JXL_BOOL cmyk=0x12345678;
    const auto status=cms->set_fields_from_icc(cms->set_fields_data,(flags&4)?nullptr:bytes.data(),size,
      (flags&1)?nullptr:&c,(flags&2)?nullptr:&cmyk);
    std::cout<<status<<' '<<cmyk;Encoding(c);return true;
  }
  if(command=="default-cms-row") {
    unsigned mode;size_t threads,width,thread,source_extent,destination_extent,count;uint32_t intensity;
    std::string source_hex,destination_hex;
    in>>mode>>threads>>width>>thread>>intensity>>source_extent>>destination_extent>>source_hex>>destination_hex>>count;
    auto source_icc=Unhex(source_hex),destination_icc=Unhex(destination_hex);
    source_icc.resize(source_icc.size()+16);destination_icc.resize(destination_icc.size()+16);
    // The inactive external encoding and channel fields genuinely remain unknown.
    JxlColorProfile source,destination;
    source.icc={source_icc.data(),source_icc.size()-16};destination.icc={destination_icc.data(),destination_icc.size()-16};
    void* context=cms->init((mode&8)?nullptr:cms->init_data,threads,width,
      (mode&8)?nullptr:&source,(mode&8)?nullptr:&destination,Float(intensity));
    std::cout<<(context!=nullptr);
    if(context==nullptr)return true;
    std::vector<float> input(std::max(width*4+16,count),Float(0x3f123456));
    std::vector<float> output(width*4+16,Float(0xbf654321));
    for(size_t i=0;i<count;++i){uint32_t word;in>>word;input[i]=Float(word);}
    std::cout<<' '<<threads;
    float* first_src=nullptr;float* first_dst=nullptr;
    for(size_t i=0;i<threads;++i) {
      float* s=cms->get_src_buf(context,i);float* d=cms->get_dst_buf(context,i);
      if(i==0){first_src=s;first_dst=d;}
      std::cout<<' '<<(s!=nullptr)<<' '<<(d!=nullptr)<<' '<<(uintptr_t(s)%128)<<' '<<(uintptr_t(d)%128)
        <<' '<<uint64_t(uintptr_t(s)-uintptr_t(first_src))<<' '<<uint64_t(uintptr_t(d)-uintptr_t(first_dst))<<' '<<(s==d);
    }
    if(threads) {
      float* s=cms->get_src_buf(context,thread);float* d=cms->get_dst_buf(context,thread);
      if(mode&1)std::memcpy(s,input.data(),count*sizeof(float));
      const float* src=(mode&1)?s:input.data();float* dst=(mode&2)?d:output.data();
      if(mode&4)dst=const_cast<float*>(src);
      std::cout<<' '<<cms->run(context,thread,src,dst,width);
      std::cout<<' '<<input.size();for(float x:input)std::cout<<' '<<Word(x);
      std::cout<<' '<<output.size();for(float x:output)std::cout<<' '<<Word(x);
      for(size_t i=0;i<threads;++i) {
        s=cms->get_src_buf(context,i);d=cms->get_dst_buf(context,i);
        std::cout<<' '<<source_extent;for(size_t j=0;j<source_extent;++j)std::cout<<' '<<Word(s[j]);
        std::cout<<' '<<destination_extent;for(size_t j=0;j<destination_extent;++j)std::cout<<' '<<Word(d[j]);
      }
    }
    cms->destroy(context);return true;
  }
  return false;
}
using PrivateCommand=bool(*)(const std::string&,std::istringstream&);
inline int Main(PrivateCommand private_command) {
  std::string line;
  while(std::getline(std::cin,line)) {
    std::istringstream in(line);std::string command;in>>command;
    if(!Public(command,in)&&!(private_command&&private_command(command,in)))std::abort();
    std::cout<<'\n';
  }
  return 0;
}
} // namespace default_cms_observer
#endif
