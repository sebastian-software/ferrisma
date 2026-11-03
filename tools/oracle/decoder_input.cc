// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Exact hash-bound original member methods; this adapter supplies fields and
// observations, not another implementation of the input algorithm.
#include <jxl/decode.h>
#include <algorithm>
#include <cstdlib>
#include <deque>
#include <iostream>
#include <map>
#include <memory>
#include <sstream>
#include <string>
#include <vector>
#include "lib/jxl/base/span.h"
#include "lib/jxl/base/status.h"
#include "lib/jxl/padded_bytes.h"
std::string Hex(jxl::Bytes);
std::vector<uint8_t> Unhex(const std::string&);
namespace decoder_input_source {
#include "original_decoder_errors.inc"
#include "original_decoder_input_status.inc"
struct JxlDecoder {
  explicit JxlDecoder(JxlMemoryManager* mm) : codestream_copy(mm) {}
  bool last_codestream_seen = false;
  size_t file_pos = 0;
  size_t box_contents_end = 0;
  bool box_contents_unbounded = true;
  jxl::PaddedBytes codestream_copy;
  size_t codestream_unconsumed = 0;
  size_t codestream_pos = 0;
  uint32_t next_jxlp_index = 0;
  std::map<uint32_t, std::pair<std::unique_ptr<jxl::PaddedBytes>, bool>> jxlp_ooo_buffer;
  size_t jxlp_ooo_buffer_total = 0;
  const uint8_t* next_in = nullptr;
  size_t avail_in = 0;
  bool input_closed = false;
#include "original_decoder_input_buffer.inc"
#include "original_decoder_input_methods.inc"
};
#include "original_decoder_input_public.inc"
struct Allocator {
 size_t calls=0, fail=0, next=0;
 uint8_t* mutation=nullptr;uint8_t mutation_value=0;
 std::map<void*,size_t> live;
 std::vector<std::string> events;
 static void* Alloc(void* opaque,size_t size) {
  auto& a=*static_cast<Allocator*>(opaque);++a.calls;
  if(a.mutation)*a.mutation=a.mutation_value;
  a.events.push_back("A"+std::to_string(size));
  if(a.calls==a.fail)return nullptr;
  auto* p=std::malloc(size);if(p)a.live[p]=++a.next;return p;
 }
 static void Free(void* opaque,void* p) {
  auto& a=*static_cast<Allocator*>(opaque);
  a.events.push_back("F"+std::to_string(p?a.live.at(p):0));
  a.live.erase(p);std::free(p);
 }
};
}
namespace decoder_input_cursor_source {
struct JxlDecoder { const uint8_t* next_in;size_t avail_in;bool input_closed; };
#include "original_decoder_input_public.inc"
}
bool DecoderInputCommand(const std::string& command,std::istringstream& in,JxlMemoryManager*) {
 if(command=="di-short") {
  size_t mode,n;in>>mode>>n;uint8_t byte=0;
  decoder_input_cursor_source::JxlDecoder cursor;
  if(mode==0){cursor.next_in=&byte;auto s=decoder_input_cursor_source::JxlDecoderSetInput(&cursor,nullptr,n);std::cout<<unsigned(s)<<' '<<(cursor.next_in==&byte);}
  else if(mode==1){cursor.next_in=nullptr;cursor.input_closed=true;auto s=decoder_input_cursor_source::JxlDecoderSetInput(&cursor,&byte,n);std::cout<<unsigned(s)<<' '<<(cursor.next_in!=nullptr)<<' '<<cursor.input_closed;}
  else if(mode==2){cursor.avail_in=n;auto s=decoder_input_cursor_source::JxlDecoderReleaseInput(&cursor);std::cout<<s<<' '<<(cursor.next_in!=nullptr)<<' '<<cursor.avail_in;}
  else if(mode==3){decoder_input_cursor_source::JxlDecoderCloseInput(&cursor);std::cout<<cursor.input_closed;}
  else{cursor.next_in=nullptr;cursor.input_closed=false;auto s=decoder_input_cursor_source::JxlDecoderSetInput(&cursor,&byte,0);std::cout<<unsigned(s)<<' '<<(cursor.next_in==&byte)<<' '<<cursor.avail_in;}
  return true;
 }
 if(command!="di-run")return false;
 using namespace decoder_input_source;
 Allocator allocator;in>>allocator.fail;
 JxlMemoryManager mm{&allocator,Allocator::Alloc,Allocator::Free};
 std::deque<std::vector<uint8_t>> buffers;
 auto dec=std::make_unique<decoder_input_source::JxlDecoder>(&mm);
 jxl::Bytes span;
 std::string op;
 while(in>>op) {
  int status=-99;std::string detail="-";
  if(op=="set") {std::string hex;bool null;in>>null>>hex;buffers.push_back(Unhex(hex));auto& b=buffers.back();size_t n=b.size();b.resize(n+1);status=decoder_input_source::JxlDecoderSetInput(dec.get(),null?nullptr:b.data(),n);}
  else if(op=="mutate"){size_t index,offset;unsigned value;in>>index>>offset>>value;allocator.mutation=&buffers.at(index).at(offset);allocator.mutation_value=uint8_t(value);status=0;}
  else if(op=="release")status=int(decoder_input_source::JxlDecoderReleaseInput(dec.get()));
  else if(op=="close"){decoder_input_source::JxlDecoderCloseInput(dec.get());status=0;}
  else if(op=="advance"){size_t n;in>>n;dec->AdvanceCodestream(n);status=0;}
  else if(op=="raw"){size_t n;in>>n;dec->AdvanceInput(n);status=0;}
  else if(op=="request")status=dec->RequestMoreInput();
  else if(op=="get"||op=="get-check") {auto old=span;status=dec->GetCodestreamInput(&span);if(status==0)detail=Hex(span);if(op=="get-check"){if(status!=0)detail="U"+std::to_string(span.data()==old.data()&&span.size()==old.size());else detail=std::to_string(span.data()!=nullptr)+":"+detail+":"+std::to_string(span.data()==(dec->codestream_copy.data()?dec->codestream_copy.data()+dec->codestream_pos:nullptr));}}
  else if(op=="inject"){bool success=true;auto s=dec->InjectNextBufferedJxlpBox(success);status=int(s.code());detail=std::to_string(success);}
  else if(op=="queue") {uint32_t index;bool last;std::string hex;in>>index>>last>>hex;auto data=std::make_unique<jxl::PaddedBytes>(&mm);auto bytes=Unhex(hex);auto s=data->append(bytes);status=int(s.code());if(s){dec->jxlp_ooo_buffer_total+=data->size();dec->jxlp_ooo_buffer.emplace(index,std::make_pair(std::move(data),last));}}
  else if(op=="box"){in>>dec->box_contents_unbounded>>dec->box_contents_end;status=0;}
  else if(op=="seed"){in>>dec->codestream_pos>>dec->codestream_unconsumed>>dec->jxlp_ooo_buffer_total>>dec->next_jxlp_index>>dec->file_pos;status=0;}
  else if(op=="limit"){size_t n;in>>n;status=dec->CanAddBuffer(n);}
  else throw std::runtime_error("input operation");
  long owner=-1;size_t offset=0;
  if(dec->next_in)for(size_t i=0;i<buffers.size();++i){auto lo=reinterpret_cast<uintptr_t>(buffers[i].data()),hi=lo+buffers[i].size();auto p=reinterpret_cast<uintptr_t>(dec->next_in);if(p>=lo&&p<hi){owner=long(i);offset=p-lo;}}
  std::cout<<" [ "<<op<<' '<<status<<' '<<detail<<' '<<owner<<' '<<offset<<' '<<dec->avail_in<<' '<<dec->input_closed<<' '<<dec->file_pos<<' '<<dec->box_contents_unbounded<<' '<<dec->box_contents_end<<' '<<dec->codestream_copy.size()<<' '<<dec->codestream_copy.capacity()<<' '<<Hex(jxl::Bytes(dec->codestream_copy.data(),dec->codestream_copy.size()))<<' '<<dec->codestream_pos<<' '<<dec->codestream_unconsumed<<' '<<dec->jxlp_ooo_buffer_total<<' '<<dec->next_jxlp_index<<' '<<dec->last_codestream_seen<<' '<<dec->jxlp_ooo_buffer.size();
  for(auto& e:dec->jxlp_ooo_buffer)std::cout<<' '<<e.first<<':'<<e.second.second<<':'<<Hex(jxl::Bytes(e.second.first->data(),e.second.first->size()));
  std::cout<<" ]";
 }
 if(!in.eof())throw std::runtime_error("input parse");in.clear();
 dec.reset();std::cout<<" E";for(auto& e:allocator.events)std::cout<<' '<<e;
 return true;
}
