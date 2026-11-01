// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observe the original finalization stages at EncodeFrameOneShot's local-writer
// boundary. Header/TOC/permutation/writer algorithms remain in upstream code.
bool FrameOutputCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if(command!="finalize-frame")return false;
  size_t w,h,n;unsigned depth;bool gray,has_aux;uint64_t seed;
  in>>w>>h>>gray>>depth>>has_aux>>seed>>n;
  jxl::CodecMetadata metadata;
  if(!metadata.size.Set(w,h))throw std::runtime_error("frame output dimensions");
  metadata.m.SetUintSamples(depth);metadata.m.xyb_encoded=false;
  if(!metadata.m.color_encoding.SetSRGB(gray?jxl::ColorSpace::kGray:jxl::ColorSpace::kRGB,jxl::RenderingIntent::kRelative))throw std::runtime_error("frame output color");
  jxl::CompressParams p;p.speed_tier=jxl::SpeedTier::kThunder;p.level=10;p.buffering=0;p.SetLossless();
  if(!jxl::ParamsPostInit(&p))throw std::runtime_error("frame output params");
  jxl::PassesEncoderState state(mm);jxl::FrameHeader header(&metadata);
  if(!EFPrepareLosslessThunder(p,&state.progressive_splitter,&header))throw std::runtime_error("frame output header");
  state.shared.frame_dim=header.ToFrameDimensions();
  std::vector<std::unique_ptr<jxl::BitWriter>> groups;
  for(size_t i=0;i<n;++i) {
    std::string data;size_t prefix;in>>data>>prefix;
    auto writer=std::make_unique<jxl::BitWriter>(mm);auto bytes=Unhex(data);
    if(!writer->AppendByteAligned(jxl::Bytes(bytes.data(),bytes.size())) || !Prefix(*writer,prefix))throw std::runtime_error("frame output group");
    groups.push_back(std::move(writer));
  }
  jxl::AuxOut aux;AuxSeed(aux,seed);auto* stats=has_aux?&aux:nullptr;
  jxl::BitWriter writer(mm);
  const auto finalize=[&]()->jxl::Status {
    JXL_RETURN_IF_ERROR(writer.AppendByteAligned(state.special_frames));
    JXL_RETURN_IF_ERROR(jxl::WriteFrameHeader(header,&writer,stats));
    std::vector<jxl::coeff_order_t> permutation;
    JXL_RETURN_IF_ERROR(jxl::PermuteGroups(p,state.shared.frame_dim,state.progressive_splitter.GetNumPasses(),0,&permutation,&groups));
    std::vector<size_t> sizes;sizes.reserve(groups.size());
    for(const auto& bw:groups) {JXL_ENSURE(bw->BitsWritten()%8==0);sizes.push_back(bw->BitsWritten()/8);}
    JXL_RETURN_IF_ERROR(jxl::WriteTocPermutation(permutation,&writer,stats));
    JXL_RETURN_IF_ERROR(jxl::WriteTocSizes(sizes,&writer,stats));
    return writer.AppendByteAligned(groups);
  };
  auto status=finalize();std::cout<<static_cast<int>(status.code());
  if(status)std::cout<<' '<<writer.BitsWritten()<<' '<<Hex(writer.GetSpan());
  std::cout<<' '<<header.all_default;AuxObservation(aux);return true;
}
