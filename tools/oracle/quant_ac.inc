// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
void QACEncoding(const jxl::QuantEncoding& q) {
  if(q.mode!=jxl::QuantEncoding::kQuantModeLibrary) {QSEncoding(q);return;}
  std::cout<<" Q "<<q.mode;QSParams(q.dct_params);
  // Library<0> initialized the whole AFV union backing before Decode wrote
  // another member. Inspect those initialized object bytes, never an inactive
  // floating member, when failure leaves the old Library mode unchanged.
  std::array<float,27> backing;
  std::memcpy(backing.data(),static_cast<const void*>(&q.afv_weights),sizeof(backing));
  for(float value:backing)std::cout<<' '<<QACBits<uint32_t>(value);
  QSParams(q.dct_params_afv_4x4);std::cout<<' '<<+q.predefined;
}
void QACState(const jxl::DequantMatrices& m,unsigned strategy) {
  std::cout<<' '<<m.computed_mask_<<' '<<bool(m.table_storage_);
  if((m.computed_mask_&(1u<<strategy))==0) return;
  size_t kind=size_t(jxl::kAcStrategyToQuantTableMap[strategy]);
  size_t len=jxl::DequantMatrices::required_size_x[kind]*jxl::DequantMatrices::required_size_y[kind]*64;
  for(size_t c=0;c<3;++c) {
    const float* table=m.Matrix(jxl::AcStrategyType(strategy),c);
    const float* inverse=m.InvMatrix(jxl::AcStrategyType(strategy),c);
    for(size_t i=0;i<len;++i) std::cout<<' '<<QACBits<uint32_t>(table[i])<<','<<QACBits<uint32_t>(inverse[i]);
  }
}
std::string QACHex(jxl::Span<const uint8_t> bytes) {
  const char* digits="0123456789abcdef";std::string out;
  for(uint8_t b:bytes){out+=digits[b>>4];out+=digits[b&15];}return out.empty()?"-":out;
}
void QACHeader(jxl::CodecMetadata& metadata,jxl::FrameHeader& header) {
  if(!metadata.size.Set(8,8))throw std::runtime_error("quant fixture size");
  header.encoding=jxl::FrameEncoding::kVarDCT;
}
bool QuantAcCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if(command=="qac-source") {
    unsigned strategy,variant;bool shared;size_t prefix;in>>strategy>>variant>>shared>>prefix;
    size_t kind=size_t(jxl::kAcStrategyToQuantTableMap[strategy]);
    size_t sx=8*jxl::DequantMatrices::required_size_x[kind],sy=8*jxl::DequantMatrices::required_size_y[kind];
    std::vector<int> raw(sx*sy*3);
    for(size_t i=0;i<raw.size();++i) raw[i]=variant==0?3:int(1+(i*13+i/sx*7)%255);
    if(variant==2)raw[raw.size()/2]=0;
    if(variant==3)raw[raw.size()/2]=-7;
    auto q=jxl::QuantEncoding::RAW(std::move(raw),0);
    std::vector<jxl::QuantEncoding> encodings(17,jxl::QuantEncoding::Library<0>());encodings[kind]=q;
    jxl::DequantMatrices m;m.SetEncodings(encodings);
    jxl::CodecMetadata metadata;jxl::FrameHeader header(&metadata);QACHeader(metadata,header);
    jxl::CompressParams params;params.speed_tier=jxl::SpeedTier::kThunder;
    auto made=jxl::ModularFrameEncoder::Create(mm,header,params,false);
    if(!made.ok())throw std::runtime_error("quant fixture encoder");auto encoder=std::move(made).value_();
    jxl::BitWriter global(mm),tables(mm);
    if(shared) {
      if(!encoder->AddQuantTable(sx,sy,q,kind)||!encoder->ComputeTree(nullptr)||!encoder->ComputeTokens(nullptr)||!encoder->EncodeGlobalInfo(false,&global,nullptr))throw std::runtime_error("quant fixture global encoder");
      if(!global.WithMaxBits(7,jxl::LayerType::Header,nullptr,[&]{global.ZeroPadToByte();return true;}))throw std::runtime_error("quant global padding");
    }
    if((prefix!=0 && !tables.WithMaxBits(prefix,jxl::LayerType::Header,nullptr,[&]{tables.Write(prefix,0);return true;}))||!jxl::DequantMatricesEncode(mm,m,&tables,jxl::LayerType::Header,nullptr,shared?encoder.get():nullptr)||!tables.WithMaxBits(7,jxl::LayerType::Header,nullptr,[&]{tables.ZeroPadToByte();return true;}))throw std::runtime_error("quant fixture table encoder");
    std::cout<<(shared?QACHex(global.GetSpan()):"-")<<' '<<QACHex(tables.GetSpan());return true;
  }
  if(command=="qac-library") {
    auto library=jxl::DequantMatrices::Library();std::cout<<"L";
    for(size_t i=0;i<17;++i) QSEncoding(library[i]);
    return true;
  }
  if(command=="qac-compute") {
    unsigned strategy;in>>strategy;std::vector<int> input;
    auto q=QSRead(in,input);
    std::vector<jxl::QuantEncoding> encodings(17,jxl::QuantEncoding::Library<0>());
    encodings[size_t(jxl::kAcStrategyToQuantTableMap[strategy])]=q;
    jxl::DequantMatrices m;m.SetEncodings(encodings);
    size_t n;in>>n;std::cout<<"C";
    for(size_t i=0;i<n;++i) {uint32_t mask;in>>mask;auto status=m.EnsureComputed(mm,mask);std::cout<<' '<<int(status.code());QACState(m,strategy);}
    return true;
  }
  if(command=="qac-decode" || command=="qac-decode-context") {
    unsigned strategy;uint32_t seed;size_t offset;std::string hex;
    in>>strategy>>seed>>offset>>hex;
    jxl::DequantMatrices m;
    if(seed && !m.EnsureComputed(mm,seed)) throw std::runtime_error("quant fixture seed");
    std::vector<uint8_t> bytes;
    if(hex!="-") for(size_t i=0;i<hex.size();i+=2) bytes.push_back(std::stoul(hex.substr(i,2),nullptr,16));
    size_t logical_size=bytes.size();bytes.resize(logical_size+8);
    jxl::BitReader reader{jxl::Span<const uint8_t>(bytes.data(),logical_size)};reader.SkipBits(offset);
    jxl::CodecMetadata metadata;jxl::FrameHeader header(&metadata);QACHeader(metadata,header);
    jxl::ModularFrameDecoder decoder(mm);decoder.Init(header.ToFrameDimensions());
    if(command=="qac-decode-context") {
      std::string global_hex;in>>global_hex;std::vector<uint8_t> global;
      if(global_hex!="-")for(size_t i=0;i<global_hex.size();i+=2)global.push_back(std::stoul(global_hex.substr(i,2),nullptr,16));
      size_t logical=global.size();global.resize(logical+8);
      jxl::BitReader gr(jxl::Span<const uint8_t>(global.data(),logical));
      auto status=decoder.DecodeGlobalInfo(&gr,header,false);auto closed=gr.Close();
      if(!status||!closed)throw std::runtime_error("quant fixture global decoder");
    }
    auto status=m.Decode(mm,&reader,command=="qac-decode-context"?&decoder:nullptr);
    std::cout<<"D "<<int(status.code())<<' '<<reader.TotalBitsConsumed()<<' '<<int(reader.Close().code());
    for(const auto& q:m.encodings()) QACEncoding(q);
    QACState(m,strategy);
    if(status) {auto computed=m.EnsureComputed(mm,1u<<strategy);std::cout<<" C "<<int(computed.code());QACState(m,strategy);}
    return true;
  }
  return false;
}
