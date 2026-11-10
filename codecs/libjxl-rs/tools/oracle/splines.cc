// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observe the actual pinned spline implementation. Access-control relaxation
// belongs to this adapter only; no source routines or declarations are changed.
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include "lib/jxl/base/span.h"
#include "lib/jxl/chroma_from_luma.h"
#include "lib/jxl/dec_bit_reader.h"
#include "lib/jxl/enc_ans.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/enc_bit_writer.h"
#include "lib/jxl/enc_splines.h"
#include "lib/jxl/image.h"
#include "lib/jxl/render_pipeline/stage_splines.h"
#include "lib/jxl/splines.h"

jxl::Status Prefix(jxl::BitWriter&, size_t);
jxl::Status Pad(jxl::BitWriter&);
std::string Hex(jxl::Bytes);
namespace {
uint32_t Bits(float x) { uint32_t b; std::memcpy(&b,&x,4); return b; }
float Float(std::istringstream& in) { uint32_t b; in>>b; float x; std::memcpy(&x,&b,4); return x; }
jxl::Spline ReadSpline(std::istringstream& in) {
  jxl::Spline s; size_t n; in>>n;
  for(size_t i=0;i<n;i++) { float x=Float(in); float y=Float(in); s.control_points.emplace_back(x,y); }
  for(auto& c:s.color_dct) for(auto& v:c) v=Float(in);
  for(auto& v:s.sigma_dct) v=Float(in);
  return s;
}
void Quant(const jxl::QuantizedSpline& q) {
  std::cout<<" Q "<<q.control_points_.size();
  for(auto p:q.control_points_) std::cout<<' '<<p.first<<' '<<p.second;
  for(auto& c:q.color_dct_) for(auto v:c) std::cout<<' '<<v;
  for(auto v:q.sigma_dct_) std::cout<<' '<<v;
}
void Spline(const jxl::Spline& s) {
  std::cout<<" S "<<s.control_points.size();
  for(auto p:s.control_points) std::cout<<' '<<Bits(p.x)<<' '<<Bits(p.y);
  for(auto& c:s.color_dct) for(auto v:c) std::cout<<' '<<Bits(v);
  for(auto v:s.sigma_dct) std::cout<<' '<<Bits(v);
}
void State(const jxl::Splines& s) {
  std::cout<<" H "<<s.HasAny()<<' '<<s.GetQuantizationAdjustment()<<' '<<s.QuantizedSplines().size()<<' '<<s.StartingPoints().size()<<' '<<s.splines_storage_.size()<<' '<<s.starting_points_storage_.size()<<' '<<s.segments_.size()<<' '<<bool(s.segment_indices_)<<' '<<bool(s.segment_y_start_);
  for(auto& q:s.splines_storage_) Quant(q);
  for(auto p:s.starting_points_storage_) std::cout<<" P "<<Bits(p.x)<<' '<<Bits(p.y);
}
void Cache(const jxl::Splines& s, size_t h) {
  for(auto& p:s.segments_) { std::cout<<" G "<<Bits(p.center_x)<<' '<<Bits(p.center_y)<<' '<<Bits(p.maximum_distance)<<' '<<Bits(p.inv_sigma)<<' '<<Bits(p.sigma_over_4_times_intensity); for(auto c:p.color) std::cout<<' '<<Bits(c); }
  if(!s.segment_y_start_) return;
  auto starts=s.segment_y_start_.address<ptrdiff_t>();
  std::cout<<" Y"; for(size_t y=0;y<h+2;y++) std::cout<<' '<<starts[y];
  if(!s.segment_indices_) return;
  auto idx=s.segment_indices_.address<size_t>(); std::cout<<" I";
  for(ptrdiff_t i=0;i<starts[h];i++) std::cout<<' '<<idx[i];
}
void Pixels(const jxl::Image3F& img) {
  std::cout<<" F"; for(size_t c=0;c<3;c++) for(size_t y=0;y<img.ysize();y++) for(size_t x=0;x<img.xsize();x++) std::cout<<' '<<Bits(img.PlaneRow(c,y)[x]);
}
}
bool SplinesCommand(const std::string& cmd,std::istringstream& in,JxlMemoryManager* mm) {
  if(cmd.rfind("sp-",0)!=0) return false;
  if(cmd=="sp-create") {
    int32_t a; in>>a; float cx=Float(in),cb=Float(in); uint64_t size,total; in>>size>>total;
    float sx=Float(in),sy=Float(in); auto original=ReadSpline(in);
    auto q=jxl::QuantizedSpline::Create(original,a,cx,cb); std::cout<<static_cast<int>(q.status().code());
    if(q.ok()) { auto value=std::move(q).value_(); Quant(value); jxl::Spline s; for(auto& c:s.color_dct)c.fill(123.25f); s.sigma_dct.fill(-77.5f); auto status=value.Dequantize({sx,sy},a,cx,cb,size,&total,s); std::cout<<" D "<<static_cast<int>(status.code())<<' '<<total; Spline(s); }
  } else if(cmd=="sp-decode" || cmd=="sp-decode-sequence") {
    size_t count=1;if(cmd=="sp-decode-sequence")in>>count;jxl::Splines s(mm);
    for(size_t k=0;k<count;k++){
      size_t pixels,prefix;std::string hex;in>>pixels>>prefix>>hex;
      size_t n=hex=="-"?0:hex.size()/2;std::vector<uint8_t> bytes(n+16);
      for(size_t i=0;i<n;i++)bytes[i+8]=std::stoul(hex.substr(2*i,2),nullptr,16);
      jxl::BitReader br(jxl::Bytes(bytes.data()+8,n));br.SkipBits(prefix);
      auto status=s.Decode(&br,pixels);if(k)std::cout<<" R ";std::cout<<static_cast<int>(status.code())<<' '<<br.TotalBitsConsumed();State(s);auto close=br.Close();std::cout<<" C "<<static_cast<int>(close.code());
    }
  } else if(cmd=="sp-encode-raw") {
    size_t prefix,n;int32_t a;in>>prefix>>a>>n;std::vector<jxl::QuantizedSpline> qs(n);std::vector<jxl::Spline::Point> points(n);
    for(size_t i=0;i<n;i++) {float x=Float(in),y=Float(in);points[i]={x,y};size_t np;in>>np;qs[i].control_points_.resize(np);for(auto& p:qs[i].control_points_)in>>p.first>>p.second;for(auto& c:qs[i].color_dct_)for(auto& v:c)in>>v;for(auto& v:qs[i].sigma_dct_)in>>v;}
    jxl::Splines s(mm);s.SetData({jxl::Span<const jxl::QuantizedSpline>(qs),jxl::Span<const jxl::Spline::Point>(points)});s.quantization_adjustment_=a;
    jxl::BitWriter writer(mm);auto status=Prefix(writer,prefix);if(status)status=jxl::EncodeSplines(s,&writer,jxl::LayerType::Splines,jxl::HistogramParams{},nullptr);size_t bits=writer.BitsWritten();if(status)status=Pad(writer);writer.GetSpan();std::cout<<static_cast<int>(status.code())<<' '<<bits<<' '<<Hex(writer.GetSpan());
  } else if(cmd=="sp-cache") {
    size_t w,h,n;in>>w>>h>>n;std::vector<jxl::QuantizedSpline> qs;std::vector<jxl::Spline::Point> points;
    for(size_t i=0;i<n;i++){auto s=ReadSpline(in);auto q=jxl::QuantizedSpline::Create(s,0,0,1);if(!q.ok())throw std::runtime_error("create");qs.push_back(std::move(q).value_());points.push_back(s.control_points[0]);}
    jxl::Splines s(mm);s.SetData({jxl::Span<const jxl::QuantizedSpline>(qs),jxl::Span<const jxl::Spline::Point>(points)});
    auto status=s.InitializeDrawCache(w,h,jxl::ColorCorrelation{});std::cout<<static_cast<int>(status.code());State(s);Cache(s,h);
    if(status) {
      auto result=jxl::Image3F::Create(mm,w,h);if(!result.ok())throw std::runtime_error("image");auto img=std::move(result).value_();
      for(size_t c=0;c<3;c++)for(size_t y=0;y<h;y++)for(size_t x=0;x<w;x++)img.PlaneRow(c,y)[x]=float(int((x*17+y*31+c*23)%41)-20)*.03125f;
      s.AddTo(&img,jxl::Rect(img));Pixels(img);s.SubtractFrom(&img);Pixels(img);
      jxl::Rect rect(w>2?1:0,h>2?1:0,w>2?w-2:w,h>2?h-2:h);s.AddTo(&img,rect);Pixels(img);
      // Exercise actual stage row descriptors, including extended left/right.
      constexpr size_t off=jxl::kRenderPipelineXOffset;size_t extra=2;
      std::vector<std::vector<float>> row(4,std::vector<float>(off+w+extra*2,0.125f));jxl::RenderPipelineStage::RowInfo ri(4);
      for(size_t c=0;c<4;c++)ri[c].push_back(row[c].data());auto stage=jxl::GetSplineStage(&s);
      auto st=stage->ProcessRow(ri,{},n>1?2:0,extra,w,0,h/2,0);std::cout<<" T "<<static_cast<int>(st.code())<<' '<<stage->GetName();for(size_t c=0;c<4;c++)std::cout<<' '<<int(stage->GetChannelMode(c));
      for(auto& r:row)for(auto v:r)std::cout<<' '<<Bits(v);
    }
  } else throw std::runtime_error("unknown spline request");
  return true;
}
