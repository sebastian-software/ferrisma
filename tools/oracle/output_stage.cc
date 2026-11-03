// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Data-only observer of the actual pristine GetWriteToOutputStage factory.
#include <algorithm>
#include <array>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <map>
#include <memory>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include "lib/jxl/image.h"
#include "lib/jxl/memory_manager_internal.h"
#include "lib/jxl/render_pipeline/stage_write.h"

namespace {
std::string Hex(const uint8_t* p,size_t n) {
 if(!n)return "-";static constexpr char digits[]="0123456789abcdef";
 std::string s;for(size_t i=0;i<n;i++){s+=digits[p[i]>>4];s+=digits[p[i]&15];}return s;
}
size_t SampleSize(JxlDataType type){return type==JXL_TYPE_FLOAT?4:(type==JXL_TYPE_FLOAT16||type==JXL_TYPE_UINT16)?2:1;}
struct State;
struct MemoryCookie {State* state;size_t id;};
struct Cookie {State* state;size_t id;};
struct RunCookie {Cookie* init;size_t id;};
struct Block {std::vector<uint8_t> owner;uint8_t* payload;size_t size,id;};
struct State {
 std::vector<std::string> trace;
 std::map<void*,Block> blocks;
 std::vector<std::unique_ptr<RunCookie>> runs;
 std::vector<Cookie> cookies;
 std::vector<jxl::ImageOutput> outputs;
 size_t allocation_calls=0,init_calls=0,fail_alloc=0,fail_init=0;
 void Add(const std::string& s){trace.push_back(s);}
};
void* Allocate(void* opaque,size_t size) {
 auto& cookie=*static_cast<MemoryCookie*>(opaque);auto& s=*cookie.state;
 const size_t id=++s.allocation_calls;
 if(id==s.fail_alloc){s.Add("A "+std::to_string(cookie.id)+" "+std::to_string(id)+" "+std::to_string(size)+" 0");return nullptr;}
 Block b;b.owner.assign(size+2048,0xa5);b.size=size;b.id=id;
 uintptr_t base=reinterpret_cast<uintptr_t>(b.owner.data());base=(base+2047)&~uintptr_t(2047);
 b.payload=reinterpret_cast<uint8_t*>(base);void* result=b.payload;
 s.blocks.emplace(result,std::move(b));
 s.Add("A "+std::to_string(cookie.id)+" "+std::to_string(id)+" "+std::to_string(size)+" 1");return result;
}
void Release(void* opaque,void* address) {
 auto& cookie=*static_cast<MemoryCookie*>(opaque);auto& s=*cookie.state;
 if(!address){s.Add("F "+std::to_string(cookie.id)+" 0 -");return;}
 auto at=s.blocks.find(address);if(at==s.blocks.end())throw std::runtime_error("output allocator owner mismatch");
 auto& b=at->second;size_t first=0,last=b.size;
 // Address congruence is not a cross-language promise. Discard only unchanged
 // caller-supplied outer sentinel bytes, retaining every changed physical byte
 // and every intervening unchanged byte of this actual returned allocation.
 while(first<last&&b.payload[first]==0xa5)first++;
 while(last>first&&b.payload[last-1]==0xa5)last--;
 s.Add("F "+std::to_string(cookie.id)+" "+std::to_string(b.id)+" "+Hex(b.payload+first,last-first));
 s.blocks.erase(at);
}
void* Allocate2(void* opaque,size_t size) {
 auto& cookie=*static_cast<MemoryCookie*>(opaque);cookie.state->Add("V A");return Allocate(opaque,size);
}
void Release2(void* opaque,void* address) {
 auto& cookie=*static_cast<MemoryCookie*>(opaque);cookie.state->Add("V F");Release(opaque,address);
}
void* Init(void* opaque,size_t threads,size_t pixels) {
 auto* cookie=static_cast<Cookie*>(opaque);auto& s=*cookie->state;const size_t id=++s.init_calls;
 bool failed=id==s.fail_init;
 s.Add("I "+std::to_string(id)+" "+std::to_string(cookie->id)+" "+std::to_string(threads)+" "+std::to_string(pixels)+" "+std::to_string(cookie==&s.cookies[cookie->id])+" "+std::to_string(!failed));
 if(failed)return nullptr;
 auto run=std::make_unique<RunCookie>();run->init=cookie;run->id=id;void* result=run.get();s.runs.push_back(std::move(run));return result;
}
void Run(void* opaque,size_t thread,size_t x,size_t y,size_t n,const void* pixels) {
 auto& run=*static_cast<RunCookie*>(opaque);auto& s=*run.init->state;
 const auto& out=s.outputs[run.init->id];size_t bytes=n*out.format.num_channels*SampleSize(out.format.data_type);
 size_t owner=0;uintptr_t ptr=reinterpret_cast<uintptr_t>(pixels);
 for(const auto& [p,b]:s.blocks){uintptr_t base=reinterpret_cast<uintptr_t>(p);if(ptr>=base&&ptr+bytes<=base+b.size){owner=b.id;break;}}
 s.Add("R "+std::to_string(run.init->id)+" "+std::to_string(run.id)+" "+std::to_string(thread)+" "+std::to_string(x)+" "+std::to_string(y)+" "+std::to_string(n)+" "+std::to_string(owner)+" "+std::to_string(ptr%128==0)+" "+Hex(static_cast<const uint8_t*>(pixels),bytes));
}
void Destroy(void* opaque) {
 auto& run=*static_cast<RunCookie*>(opaque);run.init->state->Add("D "+std::to_string(run.init->id)+" "+std::to_string(run.id));
}
}

bool OutputStageCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* original_default) {
 if(command=="wo-mm") {
  bool null_input,alias,opaque,allocate,release;size_t size;
  in>>null_input>>alias>>opaque>>allocate>>release>>size;
  State state;MemoryCookie first{&state,1},second{&state,2};
  JxlMemoryManager input{opaque?static_cast<void*>(&first):nullptr,allocate?Allocate:nullptr,release?Release:nullptr};
  JxlMemoryManager result{&second,Allocate2,Release2};
  if(alias)result=input;
  auto status=jxl::MemoryManagerInit(&result,null_input?nullptr:alias?&result:&input);
  std::cout<<sizeof(result)<<' '<<alignof(JxlMemoryManager)<<' '<<offsetof(JxlMemoryManager,opaque)<<' '<<offsetof(JxlMemoryManager,alloc)<<' '<<offsetof(JxlMemoryManager,free)<<' '<<int(status.code())<<' '<<(result.opaque==nullptr?0:result.opaque==&first?1:2)<<' '<<(result.alloc==nullptr?0:result.alloc==Allocate?1:2)<<' '<<(result.free==nullptr?0:result.free==Release?1:2);
  // A null context is valid for the source default callbacks. The observer's
  // custom callbacks require their actual context; Init itself never rejects it.
  if(status&&(result.opaque||result.alloc!=Allocate)){
   void* block=jxl::MemoryManagerAlloc(&result,size);std::cout<<" N "<<(block!=nullptr);
   if(block)for(size_t i=0;i<std::min<size_t>(size,4);i++)static_cast<uint8_t*>(block)[i]=uint8_t(0x31+i);
   jxl::MemoryManagerFree(&result,nullptr);
   if(block)jxl::MemoryManagerFree(&result,block);
  }
  std::cout<<" E "<<state.trace.size();for(const auto& event:state.trace)std::cout<<" | "<<event;std::cout<<" O "<<state.blocks.size();return true;
 }
 if(command!="wo-run")return false;
 size_t width,height,alpha,orientation,extra_count,manager_kind;bool has_alpha,unpremul;
 in>>width>>height>>has_alpha>>unpremul>>alpha>>orientation>>extra_count>>manager_kind;
 State state;state.outputs.resize(1+extra_count);state.cookies.reserve(1+extra_count);
 std::vector<std::vector<uint8_t>> buffers(1+extra_count);
 const bool transpose=orientation>=5&&orientation<=8;
 for(size_t i=0;i<state.outputs.size();i++) {
  unsigned type,endian,mode;size_t bits,n,padding;in>>type>>bits>>n>>mode>>padding>>endian;
  auto& out=state.outputs[i];out.format={uint32_t(n),static_cast<JxlDataType>(type),static_cast<JxlEndianness>(endian),0};out.bits_per_sample=bits;
  out.stride=(transpose?height:width)*n*SampleSize(out.format.data_type)+padding;
  buffers[i].assign(out.stride*(transpose?width:height)+7,0xa5);
  out.buffer_size=buffers[i].size();out.buffer=(mode==0||mode==2)?buffers[i].data():nullptr;
  state.cookies.push_back({&state,i});
  out.callback=(mode==1||mode==2)?jxl::PixelCallback(Init,Run,Destroy,&state.cookies.back()):jxl::PixelCallback();
 }
 struct Action {size_t kind;std::array<size_t,6> args{};};
 size_t count;in>>count;std::vector<Action> actions(count);
 for(auto& action:actions){in>>action.kind;size_t n=action.kind==1?6:1;for(size_t j=0;j<n;j++)in>>action.args[j];}
 std::array<uint32_t,9> values;for(auto& v:values)in>>v;
 if(!in)throw std::runtime_error("incomplete output-stage wire");
 MemoryCookie first{&state,1},second{&state,2};JxlMemoryManager custom{&first,Allocate,Release},manager;
 if(!jxl::MemoryManagerInit(&manager,manager_kind==1?nullptr:&custom))throw std::runtime_error("output memory init");
 JxlMemoryManager* selected=manager_kind==2?nullptr:&manager;
 std::vector<jxl::ImageOutput> extra(state.outputs.begin()+1,state.outputs.end());
 auto stage=jxl::GetWriteToOutputStage(state.outputs[0],width,height,has_alpha,unpremul,alpha,static_cast<jxl::Orientation>(orientation),extra,selected);
 std::cout<<stage->GetName()<<' '<<stage->settings_.border_x<<' '<<stage->settings_.border_y<<' '<<stage->settings_.shift_x<<' '<<stage->settings_.shift_y<<' '<<int(stage->IsInitialized().code());
 for(size_t c=0;c<5+extra_count;c++)std::cout<<' '<<int(stage->GetChannelMode(c));
 std::cout<<' '<<int(stage->GetChannelMode(SIZE_MAX));
 const size_t row_size=jxl::kRenderPipelineXOffset+width+16;
 std::vector<jxl::ImageF> input;
 for(size_t c=0;c<3+extra_count;c++){
  auto made=jxl::ImageF::Create(original_default,row_size,1);if(!made.ok())throw std::runtime_error("output input row");input.push_back(std::move(made).value_());
  for(size_t x=0;x<row_size;x++){uint32_t word=values[(c*7+x)%values.size()];std::memcpy(input.back().Row(0)+x,&word,4);}
 }
 jxl::RenderPipelineStage::RowInfo rows(input.size()),empty;
 for(size_t c=0;c<input.size();c++)rows[c].push_back(input[c].Row(0));
 for(const auto& action:actions) {
  const auto& a=action.args;
  if(action.kind==0)state.Add("P "+std::to_string(int(stage->PrepareForThreads(a[0]).code())));
  else if(action.kind==1)state.Add("S "+std::to_string(int(stage->ProcessRow(rows,empty,a[0],a[1],a[2],a[3],a[4],a[5]).code())));
  else if(action.kind==2)state.fail_alloc=a[0];
  else if(action.kind==3)state.fail_init=a[0];
  else if(action.kind==4)manager.opaque=a[0]==1?static_cast<void*>(&first):static_cast<void*>(&second);
  else if(action.kind==5)state.outputs[a[0]].buffer=nullptr;
  else if(action.kind==6){manager.alloc=a[0]==1?Allocate:Allocate2;manager.free=a[0]==1?Release:Release2;}
  else throw std::runtime_error("unknown output-stage action");
 }
 stage.reset();std::cout<<" E "<<state.trace.size();for(const auto& event:state.trace)std::cout<<" | "<<event;
 std::cout<<" B "<<buffers.size();for(const auto& buffer:buffers)std::cout<<' '<<Hex(buffer.data(),buffer.size());
 std::cout<<" O "<<state.blocks.size();return true;
}
