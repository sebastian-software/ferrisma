// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Fixture orchestration only: actual JPEG parsing, public encoder and header
// entry points remain bound to the pristine pinned original static library.
#include <jxl/encode.h>
#include <algorithm>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include "lib/jxl/jpeg/enc_jpeg_data.h"
#include "lib/jxl/frame_header.h"
#include "lib/jxl/headers.h"
#include "lib/jxl/image_metadata.h"
std::string Hex(jxl::Bytes);
std::vector<uint8_t> Unhex(const std::string&);
bool ChromaImageCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
 if(command=="ycc-jpeg-encode") {
  std::string hex;in>>hex;auto bytes=Unhex(hex);auto parsed=jxl::jpeg::ParseJPG(mm,jxl::Bytes(bytes));
  std::cout<<"JPG "<<int(parsed.status().code());if(!parsed.ok())return true;auto jpeg=std::move(parsed).value_();
  std::cout<<' '<<jpeg->width<<' '<<jpeg->height<<' '<<jpeg->components.size();for(const auto& c:jpeg->components)std::cout<<' '<<c.h_samp_factor<<','<<c.v_samp_factor;
  auto* enc=JxlEncoderCreate(nullptr);if(!enc)throw std::runtime_error("JPEG encoder allocation");auto* settings=JxlEncoderFrameSettingsCreate(enc,nullptr);
  std::vector<int> statuses;statuses.push_back(JxlEncoderUseContainer(enc,JXL_FALSE));statuses.push_back(JxlEncoderStoreJPEGMetadata(enc,JXL_FALSE));statuses.push_back(JxlEncoderFrameSettingsSetOption(settings,JXL_ENC_FRAME_SETTING_EFFORT,3));
  statuses.push_back(JxlEncoderAddJPEGFrame(settings,bytes.data(),bytes.size()));JxlEncoderCloseInput(enc);
  std::vector<uint8_t> output(4096);size_t size=0;JxlEncoderStatus status=JXL_ENC_ERROR;
  if(std::all_of(statuses.begin(),statuses.end(),[](int s){return s==JXL_ENC_SUCCESS;}))for(;;){uint8_t* next=output.data()+size;size_t available=output.size()-size;status=JxlEncoderProcessOutput(enc,&next,&available);size=next-output.data();if(status!=JXL_ENC_NEED_MORE_OUTPUT)break;output.resize(output.size()*2);}
  auto error=JxlEncoderGetError(enc);JxlEncoderDestroy(enc);output.resize(status==JXL_ENC_SUCCESS?size:0);
  std::cout<<" API";for(int v:statuses)std::cout<<' '<<v;std::cout<<" W "<<int(status)<<' '<<int(error)<<' '<<Hex(jxl::Bytes(output));return true;
 }
 if(command=="ycc-header") {
  std::string hex;in>>hex;auto bytes=Unhex(hex);jxl::BitReader reader{jxl::Bytes(bytes)};jxl::CodecMetadata metadata;
  if(reader.ReadFixedBits<16>()!=0x0aff)throw std::runtime_error("fixture marker");
  auto status=jxl::ReadSizeHeader(&reader,&metadata.size);if(status)status=jxl::ReadImageMetadata(&reader,&metadata.m);
  metadata.transform_data.nonserialized_xyb_encoded=metadata.m.xyb_encoded;if(status)status=jxl::Bundle::Read(&reader,&metadata.transform_data);
  if(!status||metadata.m.color_encoding.WantICC())throw std::runtime_error("fixture non-ICC header");
  status=reader.JumpToByteBoundary();if(!status)throw std::runtime_error("fixture header alignment");
  const size_t global_bits=reader.TotalBitsConsumed();jxl::FrameHeader header(&metadata);status=jxl::ReadFrameHeader(&reader,&header);
  if(!status)throw std::runtime_error("fixture frame header");const auto d=header.ToFrameDimensions();
  std::cout<<global_bits<<' '<<reader.TotalBitsConsumed()<<' '<<unsigned(header.color_transform)<<' '<<unsigned(header.encoding)<<' '<<header.flags
           <<' '<<metadata.m.bit_depth.bits_per_sample<<' '<<metadata.m.color_encoding.IsGray()<<' '<<metadata.m.num_extra_channels<<' '<<metadata.m.color_encoding.WantICC();
  for(auto v:{d.xsize,d.ysize,d.xsize_padded,d.ysize_padded,d.xsize_blocks,d.ysize_blocks,d.group_dim,d.num_groups,d.xsize_groups,d.ysize_groups})std::cout<<' '<<v;
  for(size_t c=0;c<3;++c)std::cout<<' '<<header.chroma_subsampling.HShift(c)<<' '<<header.chroma_subsampling.VShift(c);
  std::cout<<' '<<int(reader.Close().code());return true;
 }
 return false;
}
