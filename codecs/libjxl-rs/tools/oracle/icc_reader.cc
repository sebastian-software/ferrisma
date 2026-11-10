// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Fixture framing only. Real ICCReader symbols come from the pristine complete
// icc_codec.cc observation unit; this file defines no source codec body.
#include <algorithm>
#include <cstdint>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include "lib/jxl/icc_codec.h"
#include "lib/jxl/icc_codec_common.h"
#include "lib/jxl/enc_icc_codec.h"
#include "lib/jxl/enc_ans.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/enc_bit_writer.h"
#include "lib/jxl/fields.h"

std::string Hex(jxl::Bytes);
std::vector<uint8_t> Unhex(const std::string&);
jxl::Status Prefix(jxl::BitWriter&, size_t);
jxl::Status Pad(jxl::BitWriter&);
namespace {
struct DefinedPrefix {
  size_t prefix=0;
  void Update(size_t old_size, size_t old_capacity,
              const jxl::ICCReader& after, unsigned operation, int status,
              size_t old_i, size_t initial_bits, size_t input_size) {
    if (old_capacity != after.decompressed_.capacity()) {
      prefix=std::min(prefix,old_size);
      // PaddedBytes::change_capacity copies old_size bytes and initializes
      // exactly the following byte, retaining a contiguous defined prefix.
      if (after.decompressed_.data() && prefix==old_size) ++prefix;
    }
    size_t written=after.i_;
    if ((operation==1 || operation==2) && status==-1) {
      // Failed CheckEOI restores the logical index/ANS checkpoint but leaves
      // every speculative decompressed byte written up to the next check.
      if (!(old_i>0 && old_i%512==0 && initial_bits>input_size*8))
        written=std::min<size_t>((after.i_/512+1)*512,after.enc_size_);
    }
    if(operation!=3 && operation!=4) prefix=std::max(prefix,written);
  }
};
void Snapshot(const jxl::ICCReader& reader,size_t prefix,
              const jxl::PaddedBytes& output,const uint32_t* previous_window,
              const uint8_t* previous_storage,size_t previous_capacity) {
  const auto& ans=reader.ans_reader_;
  std::cout << ' ' << reader.i_ << ' ' << reader.bits_to_skip_ << ' '
            << reader.used_bits_base_ << ' ' << reader.enc_size_ << ' '
            << reader.decompressed_.size() << ' ' << reader.decompressed_.capacity();
  size_t defined=std::min(prefix,reader.decompressed_.size());
  std::cout << ' ' << defined << ' ' << Hex(jxl::Bytes(reader.decompressed_.data(),defined))
            << ' ' << output.size() << ' ' << Hex(jxl::Bytes(output))
            << ' ' << reader.context_map_.size();
  for(uint8_t context:reader.context_map_) std::cout << ' ' << unsigned(context);
  std::cout << ' ' << reader.code_.huffman_data.size() << ' '
            << reader.code_.uint_config.size() << ' ' << reader.code_.degenerate_symbols.size();
  for(uint32_t value:{ans.state_,ans.log_alpha_size_,ans.log_entry_size_,
        ans.entry_size_minus_1_,ans.num_decoded_,ans.num_to_copy_,ans.copy_pos_,
        ans.lz77_ctx_,ans.lz77_min_length_,ans.lz77_threshold_,
        ans.lz77_length_uint_.split_exponent,ans.lz77_length_uint_.split_token,
        ans.lz77_length_uint_.msb_in_token,ans.lz77_length_uint_.lsb_in_token,
        ans.num_special_distances_}) std::cout << ' ' << value;
  size_t window=ans.lz77_window_ ? std::min<size_t>(ans.num_decoded_,jxl::kWindowSize):0;
  std::cout << ' ' << (ans.lz77_window_!=nullptr) << ' ' << window;
  for(size_t i=0;i<window;++i) std::cout << ' ' << ans.lz77_window_[i];
  std::cout << ' ' << (previous_window==ans.lz77_window_)
            << ' ' << (previous_capacity==reader.decompressed_.capacity()
                ? (previous_storage==reader.decompressed_.data()):2);
}
}
bool IccReaderCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if(command.rfind("ir-",0)!=0) return false;
  if(command=="ir-write" || command=="ir-pack" || command=="ir-size") {
    size_t prefix;in>>prefix;jxl::BitWriter writer(mm);auto status=Prefix(writer,prefix);
    if(command=="ir-write") {
      std::string hex;in>>hex;auto bytes=Unhex(hex);
      if(status)status=jxl::WriteICC(jxl::Bytes(bytes),&writer,jxl::LayerType::Header,nullptr);
    } else {
      uint64_t size;in>>size;
      if(status)status=writer.WithMaxBits(128,jxl::LayerType::Header,nullptr,[&]{return jxl::U64Coder::Write(size,&writer);});
      if(command=="ir-pack") {
        unsigned huffman,lz;std::string hex;in>>huffman>>lz>>hex;auto bytes=Unhex(hex);
        std::vector<std::vector<jxl::Token>> tokens(1);
        for(size_t i=0;i<bytes.size();++i)tokens[0].emplace_back(jxl::ICCANSContext(i,i>0?bytes[i-1]:0,i>1?bytes[i-2]:0),bytes[i]);
        jxl::HistogramParams params;params.force_huffman=huffman;
        params.lz77_method=lz==0?jxl::HistogramParams::LZ77Method::kNone:
            lz==1?jxl::HistogramParams::LZ77Method::kRLE:jxl::HistogramParams::LZ77Method::kLZ77b3w3f;
        jxl::EntropyEncodingData code;
        if(status){auto cost=jxl::BuildAndEncodeHistograms(mm,params,jxl::kNumICCContexts,tokens,&code,&writer,jxl::LayerType::Header,nullptr);if(!cost.ok())status=cost.status();}
        if(status)status=jxl::WriteTokens(tokens[0],code,0,&writer,jxl::LayerType::Header,nullptr);
      }
    }
    size_t bits=writer.BitsWritten();if(status)status=Pad(writer);
    std::cout<<int(status.code())<<' '<<bits<<' '<<Hex(writer.GetSpan());return true;
  }
  if(command=="ir-run" || command=="ir-cache") {
    std::string seed;size_t count;in>>seed>>count;auto bytes=Unhex(seed);
    jxl::PaddedBytes output(mm);if(!bytes.empty()&&!output.append(bytes))throw std::runtime_error("ICC output seed");
    jxl::ICCReader reader(mm);DefinedPrefix initialized;int previous_status=0;
    for(size_t step=0;step<count;++step) {
      unsigned op;size_t prefix;std::string hex;in>>op>>prefix>>hex;auto input=Unhex(hex);
      std::vector<uint8_t> backing(input.size()+16);std::copy(input.begin(),input.end(),backing.begin()+8);
      jxl::BitReader bits(jxl::Bytes(backing.data()+8,input.size()));bits.SkipBits(prefix);
      auto old_window=reader.ans_reader_.lz77_window_;auto old_data=reader.decompressed_.data();
      size_t old_size=reader.decompressed_.size(),old_capacity=reader.decompressed_.capacity();
      size_t old_i=reader.i_,initial_bits=bits.TotalBitsConsumed();
      // Caller recovery is explicit: the prior fatal step and its complete
      // state are emitted before this real Reset, whose flag is also emitted.
      bool reset=op==5 && previous_status>0;
      if(reset)reader.Reset();
      jxl::Status init=true,status=true;
      if(op==0 || op==2 || op==5) init=reader.Init(&bits);
      if(op==1 || ((op==2 || op==5) && init)) {
        old_i=reader.i_;initial_bits=bits.TotalBitsConsumed();status=reader.Process(&bits,&output);
      } else if(op==3)reader.Reset();
      else if(op==4)status=jxl::ICCReader::CheckEOI(&bits);
      auto active_status=!init?init:status;
      // A failed Init never enters Process, even for operation2.
      unsigned operation=(op==2 || op==5)?(init?2:0):op;
      initialized.Update(reset?0:old_size,old_capacity,reader,
          operation,int(active_status.code()),old_i,initial_bits,input.size());
      previous_status=int(active_status.code());
      size_t consumed=bits.TotalBitsConsumed();auto close=bits.Close();
      std::cout<<(step?" S ":"S ")<<op<<' '<<int(init.code())<<' '<<int(status.code())
               <<' '<<consumed<<' '<<int(close.code());
      Snapshot(reader,initialized.prefix,output,old_window,old_data,old_capacity);
      std::cout<<' '<<reset;
      if(command=="ir-cache") {
        // These additive fixtures establish Create at the first Init (i>0),
        // then inspect the real cached views after failed subsequent Init.
        if(step==0 && reader.i_==0) throw std::runtime_error("cache fixture requires Create");
        const auto& ans=reader.ans_reader_;
        std::cout<<" C "<<ans.use_prefix_code_<<' '<<reader.code_.use_prefix_code
                 <<' '<<(ans.alias_tables_==reader.code_.alias_tables.address<jxl::AliasTable::Entry>())
                 <<' '<<(ans.huffman_data_==reader.code_.huffman_data.data())
                 <<' '<<(ans.configs==reader.code_.uint_config.data());
      }
    }
    return true;
  }
  throw std::runtime_error("unknown ICC reader observation");
}
