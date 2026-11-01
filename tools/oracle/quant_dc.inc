// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Call original DC methods; the protocol supplies no quantization algorithm.
#include "lib/jxl/enc_quant_weights.h"

void MFSOutput(const jxl::BitWriter&, const jxl::AuxOut&, JxlMemoryManager*);
void QdcState(const jxl::DequantMatrices& matrices) {
  for (size_t c=0;c<3;++c) std::cout<<' '<<FloatBits(matrices.DCQuant(c))<<','<<FloatBits(matrices.InvDCQuant(c));
}
void QdcRead(std::istringstream& in, float dc[3]) { for(size_t c=0;c<3;++c) dc[c]=EFReadFloat(in); }
bool QuantDcCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if(command!="qdc-chain") return false;
  jxl::DequantMatrices matrices;
  bool seed; size_t prefix,count; in>>seed;
  if(seed) { float dc[3];QdcRead(in,dc);matrices.SetDCQuant(dc); }
  in>>prefix>>count;
  jxl::BitWriter writer(mm);jxl::AuxOut aux;
  if(!Prefix(writer,prefix)) throw std::runtime_error("DC fixture prefix");
  std::cout<<'Q'; QdcState(matrices); MFSOutput(writer,aux,mm);
  for(size_t i=0;i<count;++i) {
    char action;in>>action;jxl::Status status=true;
    if(action=='S') {float dc[3];QdcRead(in,dc);matrices.SetDCQuant(dc);}
    else if(action=='C') {float dc[3];QdcRead(in,dc);status=jxl::DequantMatricesSetCustomDC(mm,&matrices,dc);}
    else if(action=='M') status=jxl::DequantMatricesScaleDC(mm,&matrices,EFReadFloat(in));
    else if(action=='E') {
      unsigned layer;in>>layer;
      status=jxl::DequantMatricesEncodeDC(matrices,&writer,static_cast<jxl::LayerType>(layer),&aux);
    } else if(action=='D') {
      size_t offset;std::string hex;in>>offset>>hex;Input input(hex);
      jxl::BitReader reader(input.bytes());reader.SkipBits(offset);
      status=matrices.DecodeDC(&reader);
      std::cout<<" R "<<reader.TotalBitsConsumed()<<' '<<Code(reader.Close());
    } else throw std::runtime_error("DC fixture action");
    std::cout<<" A "<<Code(status);QdcState(matrices);MFSOutput(writer,aux,mm);
  }
  return true;
}
