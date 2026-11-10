// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.
// Public pinned Brotli stream API observer; no algorithm bodies are copied.
#include <brotli/decode.h>
#include <brotli/encode.h>
#include "dec/state.h"
#include <algorithm>
#include <cstdlib>
#include <cstring>
#include <cstdint>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
static std::vector<uint8_t> Bytes(const std::string& hex) {
  std::vector<uint8_t> bytes;
  if (hex == "-") return bytes;
  if (hex.size()%2) throw std::runtime_error("odd hex");
  for (size_t i=0;i<hex.size();i+=2)
    bytes.push_back(static_cast<uint8_t>(std::stoul(hex.substr(i,2),nullptr,16)));
  return bytes;
}
static void Hex(const uint8_t* p,size_t n) {
  const char* digits="0123456789abcdef";
  if (!n) {std::cout << '-';return;}
  for(size_t i=0;i<n;++i)std::cout<<digits[p[i]>>4]<<digits[p[i]&15];
}

struct Allocation {void* pointer;size_t id;};
struct Allocator {size_t calls=0,fail=0;unsigned seed=165;std::vector<Allocation> blocks;std::ostringstream events;};
static void* Allocate(void* opaque,size_t size) {
  auto& a=*static_cast<Allocator*>(opaque);size_t id=++a.calls;a.events<<" A"<<id<<':'<<size;
  if(id==a.fail){a.events<<":0";return nullptr;}
  void* p=std::malloc(size?size:1);if(!p)std::abort();std::memset(p,a.seed,size);a.blocks.push_back({p,id});a.events<<":1";return p;
}
static void Free(void* opaque,void* p) {
  auto& a=*static_cast<Allocator*>(opaque);if(!p){a.events<<" F0";return;}
  for(auto& b:a.blocks)if(b.pointer==p){a.events<<" F"<<b.id;b.pointer=nullptr;std::free(p);return;}std::abort();
}
static void State(const BrotliDecoderState* s) {
  // Every field here is initialized by StateInit, independently of a failed
  // first header or missing internal input. No arena/pixel capacity is read.
  std::cout<<" S"<<unsigned(s->state)<<':'<<s->loop_counter<<':'<<s->pos<<':'<<s->buffer_length<<':'<<s->br.val_<<':'<<s->br.bit_pos_
    <<':'<<s->ringbuffer_size<<':'<<s->new_ringbuffer_size<<':'<<s->ringbuffer_mask<<':'<<s->partial_pos_out<<':'<<s->rb_roundtrips<<':'<<s->used_input
    <<':'<<s->max_distance<<':'<<s->dist_rb_idx<<':'<<s->dist_rb[0]<<':'<<s->dist_rb[1]<<':'<<s->dist_rb[2]<<':'<<s->dist_rb[3]
    <<':'<<s->window_bits<<':'<<s->large_window<<':'<<s->is_last_metablock<<':'<<s->is_uncompressed<<':'<<s->is_metadata<<':'<<s->should_wrap_ringbuffer<<':'<<s->canny_ringbuffer_allocation;
}
static std::vector<uint8_t> Metadata(size_t first,size_t second) {
  auto* s=BrotliEncoderCreateInstance(nullptr,nullptr,nullptr);if(!s)std::abort();
  if(!BrotliEncoderSetParameter(s,BROTLI_PARAM_QUALITY,4)||!BrotliEncoderSetParameter(s,BROTLI_PARAM_LGWIN,16))std::abort();
  std::vector<uint8_t> output(131072);uint8_t* no=output.data();size_t ao=output.size();
  for(size_t n:{first,second}) {
    std::vector<uint8_t> metadata(n?n:1);for(size_t i=0;i<n;++i)metadata[i]=static_cast<uint8_t>(i*97+n);
    const uint8_t* ni=metadata.data();size_t ai=n;
    do {if(!BrotliEncoderCompressStream(s,BROTLI_OPERATION_EMIT_METADATA,&ai,&ni,&ao,&no,nullptr)||!ao)std::abort();}while(ai||BrotliEncoderHasMoreOutput(s));
  }
  const uint8_t empty=0;const uint8_t* ni=&empty;size_t ai=0;
  while(!BrotliEncoderIsFinished(s))if(!BrotliEncoderCompressStream(s,BROTLI_OPERATION_FINISH,&ai,&ni,&ao,&no,nullptr)||!ao)std::abort();
  output.resize((no==output.data()?0:static_cast<size_t>(no-output.data())));BrotliEncoderDestroyInstance(s);return output;
}
// Genuine streaming encoder with caller-owned prepared raw dictionaries.
static std::vector<uint8_t> EncodeOperations(std::istringstream& in) {
  unsigned quality,window,mode,large,count;in>>quality>>window>>mode>>large>>count;
  std::vector<std::vector<uint8_t>> dictionaries;for(unsigned i=0;i<count;++i){std::string h;in>>h;dictionaries.push_back(Bytes(h));}
  unsigned n;in>>n;std::vector<std::pair<unsigned,std::vector<uint8_t>>> operations;
  for(unsigned i=0;i<n;++i){unsigned op;std::string h;in>>op>>h;operations.push_back({op,Bytes(h)});}
  if(!in)throw std::runtime_error("invalid encoder operations");
  auto* s=BrotliEncoderCreateInstance(nullptr,nullptr,nullptr);if(!s)std::abort();
  if(!BrotliEncoderSetParameter(s,BROTLI_PARAM_QUALITY,quality)||!BrotliEncoderSetParameter(s,BROTLI_PARAM_LARGE_WINDOW,large)||
     !BrotliEncoderSetParameter(s,BROTLI_PARAM_LGWIN,window)||!BrotliEncoderSetParameter(s,BROTLI_PARAM_MODE,mode))std::abort();
  std::vector<BrotliEncoderPreparedDictionary*> prepared;
  for(auto& data:dictionaries){auto* p=BrotliEncoderPrepareDictionary(BROTLI_SHARED_DICTIONARY_RAW,data.size(),data.data(),quality,nullptr,nullptr,nullptr);if(!p||!BrotliEncoderAttachPreparedDictionary(s,p))std::abort();prepared.push_back(p);}
  size_t cap=65536;for(auto& operation:operations)cap+=operation.second.size()*2;
  std::vector<uint8_t> out(cap);uint8_t* no=out.data();size_t ao=out.size();
  for(auto& operation:operations){size_t ai=operation.second.size();const uint8_t* ni=operation.second.data();
    do {if(!BrotliEncoderCompressStream(s,static_cast<BrotliEncoderOperation>(operation.first),&ai,&ni,&ao,&no,nullptr)||!ao)std::abort();}
    while(ai||BrotliEncoderHasMoreOutput(s)||(operation.first==BROTLI_OPERATION_FINISH&&!BrotliEncoderIsFinished(s)));
  }
  if(!BrotliEncoderIsFinished(s))throw std::runtime_error("encoder fixture missing FINISH");
  out.resize(static_cast<size_t>(no-out.data()));BrotliEncoderDestroyInstance(s);
  for(auto* p:prepared)BrotliEncoderDestroyPreparedDictionary(p);return out;
}
struct CallbackControl;
struct Context {CallbackControl* control;unsigned id;};
struct CallbackControl {BrotliDecoderState* state=nullptr;unsigned mode=0;bool changed=false;Context a{this,0},b{this,1};std::ostringstream trace;};
static void StartA(void*,size_t);static void StartB(void*,size_t);
static void ChunkA(void*,const uint8_t*,size_t);static void ChunkB(void*,const uint8_t*,size_t);
static void Change(CallbackControl& c,bool start,bool chunk) {
  auto sf=start?StartB:nullptr;auto cf=chunk?ChunkB:nullptr;BrotliDecoderSetMetadataCallbacks(c.state,sf,cf,&c.b);c.changed=true;
  c.trace<<" S"<<(c.state->metadata_start_func==sf)<<(c.state->metadata_chunk_func==cf)<<(c.state->metadata_callback_opaque==&c.b);
}
static void Start(void* opaque,size_t size) {
  auto& ctx=*static_cast<Context*>(opaque);auto& c=*ctx.control;
  c.trace<<" A"<<ctx.id<<':'<<size<<':'<<c.state->meta_block_remaining_len<<':'<<c.state->br.bit_pos_<<':'<<BrotliDecoderGetErrorCode(c.state);
  if(ctx.id==0&&!c.changed){if(c.mode==1)Change(c,true,true);if(c.mode==3)Change(c,false,false);}
}
static void Chunk(void* opaque,const uint8_t* data,size_t size) {
  auto& ctx=*static_cast<Context*>(opaque);auto& c=*ctx.control;
  c.trace<<" C"<<ctx.id<<':'<<size<<':'<<c.state->meta_block_remaining_len<<':'<<c.state->br.bit_pos_<<':'<<BrotliDecoderGetErrorCode(c.state)<<':';
  const char* digit="0123456789abcdef";for(size_t i=0;i<size;++i)c.trace<<digit[data[i]>>4]<<digit[data[i]&15];
  if(ctx.id==0&&!c.changed){if(c.mode==2)Change(c,true,true);if(c.mode==4)Change(c,true,false);if(c.mode==5)Change(c,false,true);}
}
static void StartA(void* p,size_t n){Start(p,n);}static void StartB(void* p,size_t n){Start(p,n);}
static void ChunkA(void* p,const uint8_t* d,size_t n){Chunk(p,d,n);}static void ChunkB(void* p,const uint8_t* d,size_t n){Chunk(p,d,n);}
int main() {
  try {
    std::string line;
    while(std::getline(std::cin,line)) {
      std::istringstream in(line);std::string op,hex;in>>op;
      if(op=="encode") {
        unsigned q,w,mode;in>>q>>w>>mode>>hex;auto data=Bytes(hex);
        std::vector<uint8_t> out(BrotliEncoderMaxCompressedSize(data.size()));size_t size=out.size();
        auto ok=BrotliEncoderCompress(q,w,static_cast<BrotliEncoderMode>(mode),data.size(),data.data(),&size,out.data());
        std::cout<<ok<<' ';Hex(out.data(),ok?size:0);std::cout<<'\n';continue;
      }
      if(op=="encode_ops") {auto data=EncodeOperations(in);std::cout<<"1 ";Hex(data.data(),data.size());std::cout<<'\n';continue;}
      if(op=="entry") {
        in>>hex;auto data=Bytes(hex);
        auto* s=BrotliDecoderCreateInstance(nullptr,nullptr,nullptr);size_t ai=0,ao=1,total=7;
        auto r=BrotliDecoderDecompressStream(s,&ai,nullptr,&ao,nullptr,&total);
        std::cout<<"E0:"<<unsigned(r)<<':'<<ai<<':'<<ao<<':'<<total<<':'<<BrotliDecoderGetErrorCode(s);State(s);
        total=8;r=BrotliDecoderDecompressStream(s,&ai,nullptr,nullptr,nullptr,&total);
        std::cout<<" E1:"<<unsigned(r)<<':'<<ai<<':'<<total<<':'<<BrotliDecoderGetErrorCode(s);State(s);BrotliDecoderDestroyInstance(s);
        s=BrotliDecoderCreateInstance(nullptr,nullptr,nullptr);ai=data.size();ao=1;const uint8_t* ni=data.data();uint8_t byte=0xcc;uint8_t* no=&byte;
        r=BrotliDecoderDecompressStream(s,&ai,&ni,&ao,&no,&ai);
        std::cout<<" E2:"<<unsigned(r)<<':'<<ai<<':'<<ao<<':'<<(ni==data.data())<<':'<<unsigned(byte)<<':'<<BrotliDecoderGetErrorCode(s);State(s);BrotliDecoderDestroyInstance(s);
        s=BrotliDecoderCreateInstance(nullptr,nullptr,nullptr);ai=data.size();ao=7;ni=data.data();uint8_t out[7];std::memset(out,0xcc,sizeof(out));no=out;
        r=BrotliDecoderDecompressStream(s,&ai,&ni,&ao,&no,&ao);
        std::cout<<" E3:"<<unsigned(r)<<':'<<ai<<':'<<ao<<':'<<(no==out)<<':'<<BrotliDecoderGetErrorCode(s)<<':';Hex(out,7);State(s);BrotliDecoderDestroyInstance(s);
        s=BrotliDecoderCreateInstance(nullptr,nullptr,nullptr);ai=data.size();ni=data.data();std::vector<uint8_t> output(data.size(),0xcc);no=output.data();total=99;
        r=BrotliDecoderDecompressStream(s,&ai,&ni,&ai,&no,&total);
        std::cout<<" E4:"<<unsigned(r)<<':'<<ai<<':'<<total<<':'<<BrotliDecoderGetErrorCode(s)<<':';Hex(output.data(),(no==output.data()?0:static_cast<size_t>(no-output.data())));State(s);BrotliDecoderDestroyInstance(s);
        std::cout<<'\n';continue;
      }
      if(op=="metadata") {size_t first,second;in>>first>>second;auto data=Metadata(first,second);std::cout<<"1 ";Hex(data.data(),data.size());std::cout<<'\n';continue;}
      if(op=="callbacks") {
        unsigned mode;size_t chunk;in>>mode>>chunk>>hex;if(!in||mode>5)throw std::runtime_error("invalid callback request");
        auto data=Bytes(hex);CallbackControl c;c.mode=mode;c.state=BrotliDecoderCreateInstance(nullptr,nullptr,nullptr);if(!c.state)std::abort();
        BrotliDecoderSetMetadataCallbacks(c.state,StartA,ChunkA,&c.a);
        size_t offset=0,total=123;BrotliDecoderResult result;
        for(size_t call=0;;++call) {
          size_t extent=data.size()-offset;if(chunk&&extent>chunk)extent=chunk;
          size_t ai=extent,ao=0;const uint8_t* ni=data.data()+offset;uint8_t* no=nullptr;
          result=BrotliDecoderDecompressStream(c.state,&ai,&ni,&ao,&no,&total);size_t consumed=extent-ai;
          c.trace<<" R"<<unsigned(result)<<':'<<consumed<<':'<<ai<<':'<<total<<':'<<c.state->br.bit_pos_<<':'<<BrotliDecoderGetErrorCode(c.state)<<':'<<BrotliDecoderIsFinished(c.state);offset+=consumed;
          if(result!=BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT||offset==data.size())break;
          if(ai||call==999999)throw std::runtime_error("callback progress");
        }
        c.trace<<" Z"<<(result==BROTLI_DECODER_RESULT_SUCCESS)<<':'<<offset<<':'<<data.size()<<':'<<total;
        BrotliDecoderDestroyInstance(c.state);std::cout<<c.trace.str()<<'\n';continue;
      }
      if(op!="stream"&&op!="state"&&op!="alloc"&&op!="dict"&&op!="dictalloc"&&op!="take"&&op!="swap"&&op!="replay")throw std::runtime_error("unknown operation");
      size_t chunk_in,chunk_out;unsigned disable,large;in>>chunk_in>>chunk_out>>disable>>large;
      if(!in||!chunk_in||(!chunk_out&&op!="take"))throw std::runtime_error("invalid request");
      Allocator allocator;
      if(op=="alloc"||op=="dictalloc")in>>allocator.fail>>allocator.seed;
      std::vector<std::vector<uint8_t>> dictionaries;
      if(op=="dict"||op=="dictalloc"){unsigned count;in>>count;for(unsigned i=0;i<count;++i){std::string h;in>>h;dictionaries.push_back(Bytes(h));}}
      in>>hex;if(!in)throw std::runtime_error("invalid data request");
      const bool custom=op=="alloc"||op=="dictalloc";
      auto data=Bytes(hex);auto* s=custom?BrotliDecoderCreateInstance(Allocate,Free,&allocator):BrotliDecoderCreateInstance(nullptr,nullptr,nullptr);
      if(!s&&custom){std::cout<<"NULL"<<allocator.events.str()<<'\n';continue;}
      if(!s)throw std::runtime_error("source construction failed");
      std::cout<<"P"<<BrotliDecoderSetParameter(s,BROTLI_DECODER_PARAM_DISABLE_RING_BUFFER_REALLOCATION,disable)
        <<BrotliDecoderSetParameter(s,BROTLI_DECODER_PARAM_LARGE_WINDOW,large);
      for(auto& dictionary:dictionaries)std::cout<<" D"<<BrotliDecoderAttachDictionary(s,BROTLI_SHARED_DICTIONARY_RAW,dictionary.size(),dictionary.data());
      size_t offset=0,total=999;BrotliDecoderResult result=BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT;
      for(size_t call=0;call<1000000;++call) {
        size_t ai=std::min(chunk_in,data.size()-offset),ao=op=="take"?0:chunk_out;
        std::vector<uint8_t> swapped;
        if(op=="swap")swapped.assign(data.begin()+offset,data.begin()+offset+ai);
        const uint8_t* start=op=="swap"?swapped.data():(offset?data.data()+offset:data.data());
        const uint8_t* ni=start;std::vector<uint8_t> out(chunk_out,0xcc);uint8_t* no=out.data();
        result=BrotliDecoderDecompressStream(s,&ai,&ni,&ao,&no,&total);
        size_t consumed=ni==start?0:static_cast<size_t>(ni-start);offset+=consumed;
        std::cout<<" R"<<unsigned(result)<<':'<<ai<<':'<<ao<<':'<<total<<':'<<consumed<<':'
          <<BrotliDecoderGetErrorCode(s)<<':'<<BrotliDecoderIsUsed(s)<<':'<<BrotliDecoderIsFinished(s)<<':'<<BrotliDecoderHasMoreOutput(s)<<':';
        Hex(out.data(),op=="take"?0:chunk_out-ao);
        if(op!="stream")State(s);
        if(op=="take") {
          while(BrotliDecoderHasMoreOutput(s)){size_t size=chunk_out;const uint8_t* bytes=BrotliDecoderTakeOutput(s,&size);std::cout<<" T"<<size<<':';Hex(bytes,size);if(!size)throw std::runtime_error("TakeOutput progress");}
          size_t size=0;const uint8_t* bytes=BrotliDecoderTakeOutput(s,&size);std::cout<<" E"<<size<<':'<<(bytes==nullptr);
        }
        for(size_t i=op=="take"?0:chunk_out-ao;i<out.size();++i)if(out[i]!=0xcc)throw std::runtime_error("output tail changed");
        if(result==BROTLI_DECODER_RESULT_SUCCESS||result==BROTLI_DECODER_RESULT_ERROR||
           (result==BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT&&offset==data.size()))break;
        if(call==999999)throw std::runtime_error("observer call limit");
      }
      if(op=="replay") {
        for(unsigned repeat=0;repeat<3;++repeat){size_t ai=data.size(),ao=chunk_out;const uint8_t* ni=data.data();std::vector<uint8_t> out(chunk_out,0xcc);uint8_t* no=out.data();
          result=BrotliDecoderDecompressStream(s,&ai,&ni,&ao,&no,&total);
          std::cout<<" Q"<<unsigned(result)<<':'<<ai<<':'<<ao<<':'<<total<<':'<<BrotliDecoderGetErrorCode(s)<<':'<<BrotliDecoderSetParameter(s,BROTLI_DECODER_PARAM_LARGE_WINDOW,1)<<':';Hex(out.data(),chunk_out-ao);State(s);
        }
      }
      BrotliDecoderDestroyInstance(s);if(custom){for(auto& b:allocator.blocks)if(b.pointer)throw std::runtime_error("live allocation");std::cout<<allocator.events.str();}std::cout<<'\n';
    }
  }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}
}
