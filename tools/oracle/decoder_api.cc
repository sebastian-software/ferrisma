// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Compile exact hash-checked private helpers, without the unrelated decoder
// state machine. No reference algorithm is reproduced in this adapter.
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <jxl/decode.h>
#include "lib/jxl/base/byte_order.h"
#include "lib/jxl/base/compiler_specific.h"
#include "lib/jxl/base/span.h"
#include "lib/jxl/base/status.h"
#include "lib/jxl/common.h"
#include "lib/jxl/headers.h"
namespace jxl {
namespace decoder_api_reference {
#include "original_decoder_bounds.inc"
#include "original_decoder_size_hint.inc"
#include "original_decoder_errors.inc"
#include "original_decoder_signature.inc"
#include "original_decoder_box_header.inc"
}  // namespace decoder_api_reference
}  // namespace jxl
std::string Hex(jxl::Bytes);
std::vector<uint8_t> Unhex(const std::string&);
bool DecoderApiCommand(const std::string& command,std::istringstream& in,
                       JxlMemoryManager*) {
  namespace source = jxl::decoder_api_reference;
  if(command.rfind("da-",0)!=0)return false;
  if(command=="da-constants") {
    std::cout<<sizeof(JxlSignature)<<' '<<alignof(JxlSignature)<<' '
      <<sizeof(JxlDecoderStatus)<<' '<<alignof(JxlDecoderStatus)<<' '
      <<source::InitialBasicInfoSizeHint();
    for(auto value:{JXL_SIG_NOT_ENOUGH_BYTES,JXL_SIG_INVALID,JXL_SIG_CODESTREAM,JXL_SIG_CONTAINER})
      std::cout<<' '<<unsigned(value);
    for(auto value:{JXL_DEC_SUCCESS,JXL_DEC_ERROR,JXL_DEC_NEED_MORE_INPUT,
      JXL_DEC_NEED_PREVIEW_OUT_BUFFER,JXL_DEC_NEED_IMAGE_OUT_BUFFER,
      JXL_DEC_JPEG_NEED_MORE_OUTPUT,JXL_DEC_BOX_NEED_MORE_OUTPUT,JXL_DEC_BASIC_INFO,
      JXL_DEC_COLOR_ENCODING,JXL_DEC_PREVIEW_IMAGE,JXL_DEC_FRAME,JXL_DEC_FULL_IMAGE,
      JXL_DEC_JPEG_RECONSTRUCTION,JXL_DEC_BOX,JXL_DEC_FRAME_PROGRESSION,JXL_DEC_BOX_COMPLETE})
      std::cout<<' '<<unsigned(value);
  }else if(command=="da-bounds") {
    size_t a,b,size;in>>a>>b>>size;
    std::cout<<source::OutOfBounds(a,b,size);
  }else if(command=="da-signature") {
    size_t pos;std::string hex;in>>pos>>hex;auto data=Unhex(hex);
    auto status=source::ReadSignature(data.data(),data.size(),&pos);
    std::cout<<unsigned(status)<<' '<<pos<<' '
      <<unsigned(JxlSignatureCheck(data.data(),data.size()));
  }else if(command=="da-box") {
    size_t pos,file_pos,box_size,header_size;std::string type,hex;
    in>>pos>>file_pos>>box_size>>header_size>>type>>hex;
    auto tag=Unhex(type),data=Unhex(hex);JxlBoxType box_type;
    std::memcpy(box_type,tag.data(),4);
    auto status=source::ParseBoxHeader(data.data(),data.size(),pos,file_pos,
      box_type,&box_size,&header_size);
    std::cout<<unsigned(status)<<' '<<box_size<<' '<<header_size<<' '
      <<Hex(jxl::Bytes(reinterpret_cast<uint8_t*>(box_type),4));
  }else throw std::runtime_error("unknown decoder API helper");
  return true;
}
