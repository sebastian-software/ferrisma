// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Complete-file fixtures call actual original header/one-shot routines and
// DecodeFile. Transform observations parse the genuine generated wire data.
bool ModularImageCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
 if(command=="mi-depth-domain") {
  uint32_t bits,exponent;bool floating;in>>bits>>exponent>>floating;
  jxl::CodecMetadata metadata;if(!metadata.size.Set(3,8))throw std::runtime_error("depth fixture dimensions");
  if(floating){metadata.m.SetFloat32Samples();metadata.m.bit_depth.bits_per_sample=bits;metadata.m.bit_depth.exponent_bits_per_sample=exponent;}
  else metadata.m.SetUintSamples(bits);
  metadata.m.xyb_encoded=false;if(!metadata.m.color_encoding.SetSRGB(jxl::ColorSpace::kRGB,jxl::RenderingIntent::kRelative))throw std::runtime_error("depth fixture color");
  size_t extension_bits=0,total_bits=0;auto can=jxl::Bundle::CanEncode(metadata.m.bit_depth,&extension_bits,&total_bits);
  jxl::BitWriter writer(mm);auto status=jxl::WriteCodestreamHeaders(&metadata,&writer,nullptr);size_t written=writer.BitsWritten();auto padded=Pad(writer);
  std::cout<<int(can.code())<<" W "<<int(status.code())<<' '<<written<<' '<<int(padded.code())<<' '<<Hex(writer.GetSpan())<<" M";MetadataObservation(metadata.m);TransformObservation(metadata.transform_data);return true;
 }
 if(command=="mi-encode") {
  size_t w,h,bits,exponent,effort,group,responsive,colorspace,predictor,palette,epf;bool gray,alpha,xyb,gab,noise;std::string hex;
  in>>w>>h>>gray>>alpha>>bits>>exponent>>xyb>>effort>>group>>responsive>>colorspace>>predictor>>palette>>epf>>gab>>noise>>hex;
  auto pixels=Unhex(hex);jxl::CodecMetadata metadata;
  if(!metadata.size.Set(w,h))throw std::runtime_error("Modular fixture dimensions");
  if(exponent) {metadata.m.SetFloat32Samples();metadata.m.bit_depth.bits_per_sample=bits;metadata.m.bit_depth.exponent_bits_per_sample=exponent;}
  else metadata.m.SetUintSamples(bits);
  metadata.m.xyb_encoded=xyb;
  if(alpha) {metadata.m.SetAlphaBits(bits,false);metadata.m.extra_channel_info[0].bit_depth=metadata.m.bit_depth;}
  if(!metadata.m.color_encoding.SetSRGB(gray?jxl::ColorSpace::kGray:jxl::ColorSpace::kRGB,jxl::RenderingIntent::kRelative))throw std::runtime_error("Modular fixture color");
  jxl::CompressParams p;p.cms={};p.debug_image_opaque=nullptr;p.SetLossless();p.level=10;p.buffering=0;
  p.modular_mode=true;p.speed_tier=static_cast<jxl::SpeedTier>(10-effort);p.modular_group_size_shift=group;p.responsive=responsive;p.colorspace=colorspace;p.options.predictor=static_cast<jxl::Predictor>(predictor);
  p.palette_colors=palette?1024:0;p.channel_colors_pre_transform_percent=palette?100.f:0.f;p.channel_colors_percent=palette?100.f:0.f;
  p.patches=jxl::Override::kOff;p.dots=jxl::Override::kOff;p.noise=jxl::Override::kOff;p.gaborish=jxl::OverrideFromBool(gab);p.epf=epf;p.progressive_dc=0;
  if(xyb) {p.color_transform=jxl::ColorTransform::kXYB;p.butteraugli_distance=1.f;}
  if(noise) {p.noise=jxl::Override::kOn;p.manual_noise={.005f,.01f,.015f,.02f,.025f,.03f,.035f,.04f};}
  if(!EFParamsPostInit(&p))throw std::runtime_error("Modular fixture parameters");
  jxl::JxlEncoderChunkedFrameAdapter input(w,h,alpha?1:0);JxlPixelFormat format{(gray?1u:3u)+(alpha?1u:0u),JXL_TYPE_FLOAT,JXL_NATIVE_ENDIAN,0};
  if(!input.SetFromBuffer(0,pixels.data(),pixels.size(),format))throw std::runtime_error("Modular fixture input");
  jxl::BitWriter writer(mm);auto status=jxl::WriteCodestreamHeaders(&metadata,&writer,nullptr);
  if(status)status=writer.WithMaxBits(8,jxl::LayerType::Header,nullptr,[&]{writer.ZeroPadToByte();return true;});
  std::vector<uint8_t> bytes,frame;
  if(status){auto header=std::move(writer).TakeBytes();bytes.assign(header.begin(),header.end());status=EFVarDctOneShot(mm,p,metadata,input,frame);}
  if(status)bytes.insert(bytes.end(),frame.begin(),frame.end());else bytes.clear();
  std::cout<<int(status.code())<<' '<<Hex(jxl::Bytes(bytes));return true;
 }
 if(command=="mi-inspect") {
  std::string hex;in>>hex;auto bytes=Unhex(hex);std::vector<uint8_t> backing(bytes.size()+16);std::copy(bytes.begin(),bytes.end(),backing.begin()+8);
  jxl::BitReader reader(jxl::Bytes(backing.data()+8,bytes.size()));jxl::CodecMetadata metadata;
  jxl::Status status=reader.ReadFixedBits<16>()==0x0aff?jxl::Status(true):JXL_FAILURE("fixture signature");
  if(status)status=jxl::ReadSizeHeader(&reader,&metadata.size);
  if(status)status=jxl::ReadImageMetadata(&reader,&metadata.m);
  metadata.transform_data.nonserialized_xyb_encoded=metadata.m.xyb_encoded;
  if(status)status=jxl::Bundle::Read(&reader,&metadata.transform_data);
  if(status)status=reader.JumpToByteBoundary();
  jxl::PassesDecoderState state(mm);jxl::ImageBundle output(mm,&metadata.m);jxl::FrameDecoder frame(&state,metadata,nullptr,true);
  if(status)status=state.output_encoding_info.SetFromMetadata(metadata);
  if(status)status=frame.InitFrame(&reader,&output,false);
  if(status)status=frame.InitFrameOutput();
  std::cout<<int(status.code());
  if(status){const auto& h=frame.GetFrameHeader();const auto& dim=frame.frame_dim_;std::cout<<" H "<<unsigned(h.encoding)<<' '<<unsigned(h.color_transform)<<' '<<h.group_size_shift<<' '<<h.flags<<' '<<h.loop_filter.epf_iters<<' '<<h.loop_filter.gab<<' '<<h.passes.num_passes<<" G "<<dim.num_groups<<' '<<dim.num_dc_groups;
   size_t offset=reader.TotalBitsConsumed()/8;size_t size=0;for(auto entry:frame.Toc()) {if(entry.id==0){size=entry.size;break;}offset+=entry.size;}
   jxl::BitReader global(jxl::Bytes(backing.data()+8+offset,size));status=frame.ProcessDCGlobal(&global);std::cout<<" D "<<int(status.code());
   if(status){auto& modular=frame.modular_frame_decoder_;std::cout<<" T "<<modular.global_header.transforms.size();for(const auto& t:modular.global_header.transforms)std::cout<<' '<<unsigned(t.id)<<' '<<t.rct_type<<' '<<t.squeezes.size();uint32_t predictors=0;for(const auto& node:modular.tree)if(node.property<0)predictors|=1u<<unsigned(node.predictor);std::cout<<" P "<<predictors;}
   std::cout<<" C "<<int(global.Close().code());
  }
  std::cout<<" R "<<int(reader.Close().code());return true;
 }
 if(command!="mi-decode")return false;
 unsigned format;std::string hex;in>>format>>hex;auto bytes=Unhex(hex);std::vector<uint8_t> backing(bytes.size()+16);std::copy(bytes.begin(),bytes.end(),backing.begin()+8);
 jxl::CodecInOut io(mm);auto status=jxl::DecodeFile(jxl::Bytes(backing.data()+8,bytes.size()),true,&io,nullptr);std::vector<uint8_t> pixels;
 if(status) {size_t n=io.metadata.m.color_encoding.Channels()+(io.metadata.m.HasAlpha()?1:0);size_t bits=format?32:io.metadata.m.bit_depth.bits_per_sample;size_t stride=io.xsize()*n*((bits+7)/8);pixels.resize(stride*io.ysize());jxl::PixelCallback callback;status=jxl::ConvertToExternal(io.Main(),bits,format,n,JXL_NATIVE_ENDIAN,stride,nullptr,pixels.data(),pixels.size(),callback,jxl::Orientation::kIdentity,false);}
 std::cout<<int(status.code())<<" M "<<io.xsize()<<' '<<io.ysize();MetadataObservation(io.metadata.m);TransformObservation(io.metadata.transform_data);
 const auto& image=io.Main();std::cout<<" F "<<image.color().xsize()<<' '<<image.color().ysize()<<' '<<image.decoded_bytes()<<" O "<<Hex(jxl::Bytes(pixels))<<" R";
 if(status)for(size_t y=0;y<image.ysize();++y)for(size_t x=0;x<image.xsize();++x)for(size_t c=0;c<3;++c)std::cout<<' '<<SCBits<uint32_t>(image.color().PlaneRow(c,y)[x]);return true;
}
