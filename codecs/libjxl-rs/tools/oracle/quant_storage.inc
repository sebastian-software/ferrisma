// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observe defined original constructor/ownership state, never matrix pointers.
float EFReadFloat(std::istringstream&);
void QdcState(const jxl::DequantMatrices&);
template<class T> void QSValues(const T& values) {
  for(const auto& value:values) std::cout<<' '<<+value;
}
void QSParams(const jxl::DctQuantWeightParams& p) {
  std::cout<<' '<<p.num_distance_bands;
  for(const auto& channel:p.distance_bands) for(float value:channel) std::cout<<' '<<SCBits<uint32_t>(value);
}
jxl::DctQuantWeightParams QSReadParams(std::istringstream& in) {
  unsigned kind;size_t count;in>>kind>>count;
  jxl::DctQuantWeightParams::DistanceBandsArray bands;
  for(auto& channel:bands) for(auto& value:channel) value=EFReadFloat(in);
  if(kind==0) return jxl::DctQuantWeightParams();
  if(kind==1) return jxl::DctQuantWeightParams(bands,count);
  if(kind==2) {float small[3][1];for(size_t c=0;c<3;++c) small[c][0]=bands[c][0];return jxl::DctQuantWeightParams(small);}
  if(kind==3) {float small[3][3];for(size_t c=0;c<3;++c) for(size_t i=0;i<3;++i) small[c][i]=bands[c][i];return jxl::DctQuantWeightParams(small);}
  float full[3][17];for(size_t c=0;c<3;++c) for(size_t i=0;i<17;++i) full[c][i]=bands[c][i];return jxl::DctQuantWeightParams(full);
}
template<size_t N> std::array<std::array<float,N>,3> QSWeights(const std::array<float,27>& weights) {
  std::array<std::array<float,N>,3> result;
  for(size_t c=0;c<3;++c) for(size_t i=0;i<N;++i) result[c][i]=weights[c*N+i];
  return result;
}
jxl::QuantEncoding QSRead(std::istringstream& in,std::vector<int>& input) {
  unsigned mode;int shift;size_t count;in>>mode;
  auto p=QSReadParams(in),p2=QSReadParams(in);
  std::array<float,27> weights;for(auto& v:weights)v=EFReadFloat(in);
  in>>shift>>count;input.resize(count);for(auto& v:input)in>>v;
  switch(mode) {
    case 0:return jxl::QuantEncoding::Library<0>();
    case 1:return jxl::QuantEncoding::Identity(QSWeights<3>(weights));
    case 2:return jxl::QuantEncoding::DCT2(QSWeights<6>(weights));
    case 3:return jxl::QuantEncoding::DCT4(p,QSWeights<2>(weights));
    case 4:return jxl::QuantEncoding::DCT4X8(p,{weights[0],weights[1],weights[2]});
    case 5:return jxl::QuantEncoding::AFV(p,p2,QSWeights<9>(weights));
    case 6:return jxl::QuantEncoding::DCT(p);
    case 7:return jxl::QuantEncoding::RAW(std::move(input),shift);
    default:throw std::runtime_error("quant constructor fixture mode");
  }
}
template<class T> void QSFloats(const T& weights) {
  for(const auto& channel:weights) for(float v:channel) std::cout<<' '<<SCBits<uint32_t>(v);
}
void QSEncoding(const jxl::QuantEncoding& q) {
  std::cout<<" Q "<<q.mode;QSParams(q.dct_params);
  switch(q.mode) {
    case jxl::QuantEncoding::kQuantModeID:QSFloats(q.idweights);break;
    case jxl::QuantEncoding::kQuantModeDCT2:QSFloats(q.dct2weights);break;
    case jxl::QuantEncoding::kQuantModeDCT4:QSFloats(q.dct4multipliers);break;
    case jxl::QuantEncoding::kQuantModeDCT4X8:for(float v:q.dct4x8multipliers)std::cout<<' '<<SCBits<uint32_t>(v);break;
    case jxl::QuantEncoding::kQuantModeRAW:
      std::cout<<' '<<(q.qraw.qtable!=nullptr)<<' '<<SCBits<uint32_t>(q.qraw.qtable_den);
      if(q.qraw.qtable) {std::cout<<' '<<q.qraw.qtable->size();QSValues(*q.qraw.qtable);} break;
    default:QSFloats(q.afv_weights);break; // active zero-initialized member for Library/DCT
  }
  QSParams(q.dct_params_afv_4x4);std::cout<<' '<<+q.predefined;
}
void QSMutate(jxl::QuantEncoding& q,int value) {
  if(q.mode==jxl::QuantEncoding::kQuantModeRAW && q.qraw.qtable && !q.qraw.qtable->empty()) (*q.qraw.qtable)[0]^=value;
}
void QSMatrices(const jxl::DequantMatrices& m) {
  std::cout<<" M "<<m.computed_mask_<<' '<<static_cast<bool>(m.table_storage_);
  QSValues(m.table_offsets_);QdcState(m);std::cout<<' '<<m.encodings().size();
  for(const auto& q:m.encodings()) QSEncoding(q);
}
bool QuantStorageCommand(const std::string& command,std::istringstream& in) {
  if(command=="qst-owner") {
    std::vector<int> input;auto q=QSRead(in,input);
    std::cout<<"I "<<input.size();QSValues(input);QSEncoding(q);
    jxl::QuantEncoding copy(q);QSMutate(q,0x13579);QSEncoding(q);QSEncoding(copy);
    auto assigned=jxl::QuantEncoding::RAW({101,202});assigned=copy;
    QSMutate(copy,0x2468);QSEncoding(copy);QSEncoding(assigned);
    jxl::QuantEncoding moved(std::move(copy));QSEncoding(copy);QSEncoding(moved);
    jxl::QuantEncoding empty_copy(copy);QSEncoding(empty_copy);
    return true;
  }
  if(command=="qst-matrices") {
    size_t count;in>>count;std::vector<jxl::QuantEncoding> input;
    for(size_t i=0;i<count;++i) {std::vector<int> table;input.push_back(QSRead(in,table));}
    jxl::DequantMatrices m;QSMatrices(m);
    const float dc[]={3.f,7.f,11.f};m.SetDCQuant(dc);
    for(unsigned phase=0;phase<2;++phase) {
      m.computed_mask_=0x5a5a+phase; // SetEncodings invalidates this initialized scalar without reading matrix storage.
      m.SetEncodings(input);for(auto& q:input) QSMutate(q,0x4321+phase);
      QSMatrices(m);
    }
    return true;
  }
  return false;
}
