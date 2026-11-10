// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Call the pristine scalar generator and stage bodies, without supplying any
// replacement algorithm. These objects also supply their real public symbols.
#include <cstring>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include "lib/jxl/dec_noise.cc"
#include "lib/jxl/render_pipeline/stage_noise.h"
namespace {
uint32_t NRBits(float f) { uint32_t b; memcpy(&b, &f, 4); return b; }
float NRFloat(std::istringstream& in) { uint32_t b; in >> b; float f; memcpy(&f, &b, 4); return f; }
void NRPlane(const jxl::ImageF& plane) {
  for (size_t y = 0; y < plane.ysize(); ++y)
    for (size_t x = 0; x < plane.xsize(); ++x) std::cout << ' ' << NRBits(plane.Row(y)[x]);
}
}
bool NoiseRenderCommand(const std::string& command, std::istringstream& in, JxlMemoryManager* mm) {
  if (command == "noise-rng") {
    unsigned mode; uint64_t a,b,c,d; size_t n; in >> mode >> a >> b >> c >> d >> n;
    auto observe = [&](jxl::HWY_NAMESPACE::Xorshift128Plus& rng) {
      std::cout << "R";
      for (size_t i = 0; i < n; ++i) { uint64_t values[8]; rng.Fill(values); for (auto value : values) std::cout << ' ' << value; }
    };
    if (mode) { jxl::HWY_NAMESPACE::Xorshift128Plus rng(a,b,c,d); observe(rng); }
    else { jxl::HWY_NAMESPACE::Xorshift128Plus rng(a); observe(rng); }
    return true;
  }
  if (command == "noise-random") {
    size_t visible,nonvisible,x0,y0,w,h; in >> visible >> nonvisible >> x0 >> y0 >> w >> h;
    std::vector<jxl::ImageF> planes;
    for (size_t c=0;c<3;++c) { auto p=jxl::ImageF::Create(mm,w+5,h+3); if(!p.ok())throw std::runtime_error("noise plane"); planes.push_back(std::move(p).value_()); jxl::FillImage(-17.0f-float(c),&planes.back()); }
    jxl::Rect r(2,1,w,h);
    jxl::HWY_NAMESPACE::Random3Planes(visible,nonvisible,x0,y0,{&planes[0],r},{&planes[1],r},{&planes[2],r});
    std::cout << "I"; for (auto& p:planes) NRPlane(p); return true;
  }
  if (command == "noise-decode") {
    size_t skip; std::string hex; in >> skip >> hex;
    std::vector<uint8_t> data; if(hex!="-")for(size_t i=0;i<hex.size();i+=2)data.push_back(std::stoul(hex.substr(i,2),nullptr,16));
    std::vector<uint8_t> backing(data.size()+16); std::copy(data.begin(),data.end(),backing.begin()+8);
    jxl::BitReader br(jxl::Span<const uint8_t>(backing.data()+8,data.size())); br.SkipBits(skip);
    jxl::NoiseParams params; for(auto& v:params.lut)v=NRFloat(in);
    auto status=jxl::DecodeNoise(&br,&params); auto bits=br.TotalBitsConsumed(); auto bounds=br.AllReadsWithinBounds(); auto close=br.Close();
    std::cout << int(status.code()) << ' ' << bits << ' ' << bounds << ' ' << int(close.code()); for(auto v:params.lut)std::cout << ' ' << NRBits(v); return true;
  }
  if (command != "noise-row") return false;
  unsigned kind; size_t first,w,left,right,origin; in >> kind >> first >> w >> left >> right >> origin;
  jxl::NoiseParams params; for(auto& v:params.lut)v=NRFloat(in);
  float values[8];for(auto& v:values)v=NRFloat(in);
  const size_t channels=first+3,len=origin+jxl::kRenderPipelineXOffset+w+right+3;
  std::vector<jxl::ImageF> input,output;
  for(size_t c=0;c<channels;++c){auto ip=jxl::ImageF::Create(mm,len,5),op=jxl::ImageF::Create(mm,len,1);if(!ip.ok()||!op.ok())throw std::runtime_error("noise rows");input.push_back(std::move(ip).value_());output.push_back(std::move(op).value_());for(size_t y=0;y<5;++y)for(size_t x=0;x<len;++x)input[c].Row(y)[x]=values[(c*7+y*3+x)%8];jxl::FillImage(-19.0f-float(c),&output[c]);}
  jxl::ColorCorrelation correlation;
  auto stage=kind?jxl::GetAddNoiseStage(params,correlation,first):jxl::GetConvolveNoiseStage(first);
  jxl::RenderPipelineStage::RowInfo ir(channels),out(channels);
  for(size_t c=0;c<channels;++c){if(kind)ir[c].push_back(input[c].Row(2)+origin);else for(size_t y=0;y<5;++y)ir[c].push_back(input[c].Row(y)+origin);out[c].push_back(output[c].Row(0)+origin);}
  auto status=stage->ProcessRow(ir,out,left,right,w,23,41,17);
  const auto& settings=stage->settings_;
  std::cout<<int(status.code())<<' '<<stage->GetName()<<' '<<settings.border_x<<' '<<settings.border_y;
  for(size_t c=0;c<channels+2;++c)std::cout<<' '<<int(stage->GetChannelMode(c));
  for(auto& p:input)NRPlane(p);for(auto& p:output)NRPlane(p);
  return true;
}
