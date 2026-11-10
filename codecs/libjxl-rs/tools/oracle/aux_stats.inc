// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Input/output only; accumulation calls the original methods.
void AuxSeed(jxl::AuxOut& a,uint64_t seed) {
  for(unsigned i=0;i<15;++i) {
    auto& l=a.layer(static_cast<jxl::LayerType>(i));
    l.num_clustered_histograms=seed+i;l.histogram_bits=seed+i*3;l.extra_bits=seed+i*5;l.total_bits=seed+i*7;
    l.clustered_entropy=static_cast<double>(seed+i)/8.0;
  }
  a.num_blocks=seed+1;a.num_small_blocks=seed+2;a.num_dct4x8_blocks=seed+3;a.num_afv_blocks=seed+4;
  a.num_dct8_blocks=seed+5;a.num_dct8x16_blocks=seed+6;a.num_dct8x32_blocks=seed+7;a.num_dct16_blocks=seed+8;
  a.num_dct16x32_blocks=seed+9;a.num_dct32_blocks=seed+10;a.num_dct32x64_blocks=seed+11;a.num_dct64_blocks=seed+12;
  a.num_butteraugli_iters=static_cast<int>(seed);
}
void AuxRead(std::istringstream& in,jxl::AuxOut& a) {
  for(unsigned i=0;i<15;++i) {
    auto& l=a.layer(static_cast<jxl::LayerType>(i));uint64_t bits;
    in>>l.num_clustered_histograms>>l.histogram_bits>>l.extra_bits>>l.total_bits>>bits;
    memcpy(&l.clustered_entropy,&bits,8);
  }
  in>>a.num_blocks>>a.num_small_blocks>>a.num_dct4x8_blocks>>a.num_afv_blocks>>a.num_dct8_blocks>>a.num_dct8x16_blocks
    >>a.num_dct8x32_blocks>>a.num_dct16_blocks>>a.num_dct16x32_blocks>>a.num_dct32_blocks>>a.num_dct32x64_blocks>>a.num_dct64_blocks>>a.num_butteraugli_iters;
}
void AuxObservation(const jxl::AuxOut& a) {
  std::cout<<" X "<<a.TotalBits();
  for(unsigned i=0;i<15;++i) {
    const auto& l=a.layer(static_cast<jxl::LayerType>(i));uint64_t bits;memcpy(&bits,&l.clustered_entropy,8);
    std::cout<<' '<<l.num_clustered_histograms<<' '<<l.histogram_bits<<' '<<l.extra_bits<<' '<<l.total_bits<<' '<<bits;
  }
  for(size_t value:{a.num_blocks,a.num_small_blocks,a.num_dct4x8_blocks,a.num_afv_blocks,a.num_dct8_blocks,a.num_dct8x16_blocks,
      a.num_dct8x32_blocks,a.num_dct16_blocks,a.num_dct16x32_blocks,a.num_dct32_blocks,a.num_dct32x64_blocks,a.num_dct64_blocks}) std::cout<<' '<<value;
  std::cout<<' '<<a.num_butteraugli_iters;
}
bool AuxStatsCommand(const std::string& command,std::istringstream& in) {
  if(command!="aux-stats") return false;
  size_t count;in>>count;jxl::AuxOut a;AuxRead(in,a);AuxObservation(a);
  for(size_t i=0;i<count;++i) {
    int layer;bool self;in>>layer>>self;jxl::AuxOut b;AuxRead(in,b);
    if(layer<0) a.Assimilate(self?a:b);
    else a.layer(static_cast<jxl::LayerType>(layer)).Assimilate((self?a:b).layer(static_cast<jxl::LayerType>(layer)));
    AuxObservation(a);
  }
  return true;
}
