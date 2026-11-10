// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.
// Actual public factories/API calls; no source decoder algorithm is included.
#include <brotli/decode.h>
#include "dec/state.h"
#include "common/shared_dictionary_internal.h"
#include <algorithm>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
extern "C" int BrotliStreamAPIAttach(BrotliDecoderState*,uint32_t,size_t,const uint8_t*);
static std::vector<uint8_t> Bytes(const std::string& h) {
  std::vector<uint8_t> v;if(h=="-")return v;
  if(h.size()%2)throw std::runtime_error("odd hex");
  for(size_t i=0;i<h.size();i+=2)v.push_back(static_cast<uint8_t>(std::stoul(h.substr(i,2),nullptr,16)));return v;
}
static void Hex(const uint8_t* p,size_t n){const char*d="0123456789abcdef";if(!n){std::cout<<'-';return;}for(size_t i=0;i<n;++i)std::cout<<d[p[i]>>4]<<d[p[i]&15];}
static void State(BrotliDecoderState* s) {
  std::cout<<" S"<<unsigned(s->state)<<':'<<s->error_code<<':'<<s->pos<<':'<<s->partial_pos_out<<':'<<s->rb_roundtrips<<':'<<s->used_input<<':'<<s->buffer_length
    <<':'<<BrotliDecoderIsUsed(s)<<':'<<BrotliDecoderIsFinished(s)<<':'<<BrotliDecoderHasMoreOutput(s);
}
struct Watch {BrotliDecoderState* state;unsigned mode;bool changed=false;std::ostringstream trace;};
static void WatchStart(void*,size_t);static void WatchChunk(void*,const uint8_t*,size_t);
static void Observe(Watch& w,char kind,size_t n) {
  w.trace<<' '<<kind<<n<<':'<<BrotliDecoderGetErrorCode(w.state)<<':'<<BrotliDecoderIsUsed(w.state)<<':'<<BrotliDecoderIsFinished(w.state)<<':'<<BrotliDecoderHasMoreOutput(w.state);
}
static void WatchStart(void* p,size_t n) {
  auto& w=*static_cast<Watch*>(p);Observe(w,'A',n);
  if(w.mode==1&&!w.changed){BrotliDecoderSetMetadataCallbacks(w.state,nullptr,WatchChunk,p);w.changed=true;w.trace<<" X";}
}
static void WatchChunk(void* p,const uint8_t* data,size_t n) {
  auto& w=*static_cast<Watch*>(p);Observe(w,'C',n);w.trace<<':';const char*d="0123456789abcdef";for(size_t i=0;i<n;++i)w.trace<<d[data[i]>>4]<<d[data[i]&15];
  if(w.mode==2&&!w.changed){BrotliDecoderSetMetadataCallbacks(w.state,WatchStart,nullptr,p);w.changed=true;w.trace<<" X";}
}
int main(){try{std::string line;while(std::getline(std::cin,line)){
  std::istringstream in(line);std::string op,hex;in>>op;
  std::vector<uint8_t> dictionary;
  auto* s=BrotliDecoderCreateInstance(nullptr,nullptr,nullptr);if(!s)throw std::runtime_error("constructor");
  if(op=="api") {
    unsigned mode;size_t first,size;in>>mode>>first>>size>>hex;if(!in||mode>3||first>65536)throw std::runtime_error("invalid API fixture");auto data=Bytes(hex);
    if(mode!=3){size_t ai=data.size(),ao=first,total=999;const uint8_t* ni=data.data();std::vector<uint8_t> output(first,0xcc);uint8_t* no=output.data();
      auto r=BrotliDecoderDecompressStream(s,&ai,&ni,&ao,&no,&total);std::cout<<"R"<<unsigned(r)<<':'<<ai<<':'<<ao<<':'<<total<<':';Hex(output.data(),first-ao);State(s);
    }else{std::cout<<"F";State(s);}
    // These are actual size_t members, not casts between unrelated C types.
    size_t* pointer=mode==1?&s->partial_pos_out:mode==2?&s->rb_roundtrips:&size;
    const uint8_t* bytes=BrotliDecoderTakeOutput(s,pointer);
    std::cout<<" T"<<*pointer<<':'<<(bytes==nullptr)<<':';
    if(bytes){std::cout<<(bytes-s->ringbuffer)<<':';Hex(bytes,*pointer);}else{std::cout<<"-:-";}
    State(s);
  }else if(op=="attach") {
    unsigned type,kind,repeats;size_t size;in>>type>>kind>>size>>repeats>>hex;if(!in||kind>1||size>65536||repeats>17)throw std::runtime_error("invalid attachment fixture");
    dictionary.resize(size);for(size_t i=0;i<size;++i)dictionary[i]=static_cast<uint8_t>(i*97+17);
    const uint8_t* pointer=kind?nullptr:dictionary.data();std::cout<<"D";
    for(unsigned i=0;i<repeats;++i)std::cout<<BrotliStreamAPIAttach(s,type,size,pointer);
    std::cout<<':'<<s->dictionary->num_prefix<<':'<<(s->compound_dictionary?s->compound_dictionary->num_chunks:0)<<':'<<(s->compound_dictionary?s->compound_dictionary->total_size:0);
    auto data=Bytes(hex);size_t ai=data.size(),ao=0,total=999;const uint8_t* ni=data.data();uint8_t* no=nullptr;
    auto r=BrotliDecoderDecompressStream(s,&ai,&ni,&ao,&no,&total);std::cout<<" R"<<unsigned(r)<<':'<<ai<<':'<<ao<<':'<<total;State(s);
    std::cout<<" A"<<BrotliStreamAPIAttach(s,BROTLI_SHARED_DICTIONARY_RAW,size,pointer);
  }else if(op=="watch") {
    unsigned mode;size_t chunk;in>>mode>>chunk>>hex;if(!in||mode>2)throw std::runtime_error("invalid callback watch");auto data=Bytes(hex);Watch watch{s,mode};
    BrotliDecoderSetMetadataCallbacks(s,WatchStart,WatchChunk,&watch);
    size_t offset=0,total=999;
    for(unsigned call=0;call<1000000;++call){size_t extent=data.size()-offset;if(chunk&&extent>chunk)extent=chunk;size_t ai=extent,ao=0;const uint8_t* ni=offset?data.data()+offset:data.data();uint8_t* no=nullptr;
      auto r=BrotliDecoderDecompressStream(s,&ai,&ni,&ao,&no,&total);offset+=extent-ai;
      watch.trace<<" R"<<unsigned(r)<<':'<<ai<<':'<<total;
      if(r!=BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT||offset==data.size())break;
      if(call==999999)throw std::runtime_error("callback watch progress");
    }
    std::cout<<watch.trace.str();State(s);
  }else{throw std::runtime_error("unknown API fixture");}
  BrotliDecoderDestroyInstance(s);std::cout<<'\n';
}}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
