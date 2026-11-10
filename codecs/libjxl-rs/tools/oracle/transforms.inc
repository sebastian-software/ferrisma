// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Call actual source transforms; metadata observations omit undefined pixels.
#include "lib/jxl/modular/transform/palette.h"
#include "lib/jxl/modular/transform/squeeze.h"
#include "lib/jxl/modular/transform/enc_squeeze.h"
#include "lib/jxl/modular/transform/enc_transform.h"
// Include the unmodified source to observe its private scalar palette helpers.
// This .cc is deliberately not separately compiled into the oracle.
#include "lib/jxl/modular/transform/enc_palette.cc"
std::vector<jxl::SqueezeParams> SqueezeReadParams(std::istringstream& in) {
  size_t count;in>>count;std::vector<jxl::SqueezeParams> result(count);
  for(auto& p:result) in>>p.horizontal>>p.in_place>>p.begin_c>>p.num_c;
  return result;
}
void SqueezeParamsObservation(const std::vector<jxl::SqueezeParams>& params) {
  std::cout<<' '<<params.size();for(const auto& p:params) std::cout<<' '<<p.horizontal<<','<<p.in_place<<','<<p.begin_c<<','<<p.num_c;
}
jxl::Transform TransformRead(std::istringstream& in) {
  unsigned id,p;in>>id;jxl::Transform t(static_cast<jxl::TransformId>(id));t.max_delta_error=77;
  in>>t.begin_c>>t.num_c>>t.rct_type>>t.nb_colors>>t.nb_deltas>>p;t.predictor=static_cast<jxl::Predictor>(p);t.squeezes=SqueezeReadParams(in);return t;
}
jxl::Image TransformReadImage(std::istringstream& in,JxlMemoryManager* mm) {
  size_t w,h,meta,count;int depth;bool error;in>>w>>h>>depth>>meta>>error>>count;
  auto created=jxl::Image::Create(mm,w,h,depth,0);if(!created.ok()) throw std::runtime_error("transform image allocation");auto image=std::move(created).value_();
  image.nb_meta_channels=meta;image.error=error;
  for(size_t i=0;i<count;++i) {
    size_t cw,ch;int hs,vs,component;in>>cw>>ch>>hs>>vs>>component;
    auto cc=jxl::Channel::Create(mm,cw,ch,hs,vs);if(!cc.ok()) throw std::runtime_error("transform channel allocation");auto c=std::move(cc).value_();c.component=component;
    ImagePaint(c.plane,121+i);for(size_t y=0;y<ch;++y) for(size_t x=0;x<cw;++x) in>>c.Row(y)[x];image.channel.push_back(std::move(c));
  }
  return image;
}
void TransformMetadata(const jxl::Image& image) {
  std::cout<<' '<<image.w<<','<<image.h<<','<<image.bitdepth<<','<<image.nb_meta_channels<<','<<image.error<<','<<image.empty()<<' '<<image.channel.size();
  for(const auto& c:image.channel) std::cout<<' '<<c.w<<','<<c.h<<','<<c.hshift<<','<<c.vshift<<','<<c.component<<','<<c.plane.xsize()<<','<<c.plane.ysize()<<','<<c.plane.bytes_per_row()<<','<<c.plane.PixelsPerRow()<<','<<(c.plane.bytes()!=nullptr);
}
bool TransformsCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if(command=="pal-distance") {
    size_t count;in>>count;std::vector<float> a(count);std::vector<int32_t> b(count);
    for(auto& v:a) {uint32_t bits;in>>bits;memcpy(&v,&bits,4);}for(auto& v:b) in>>v;
    float value=jxl::palette_internal::ColorDistance(a,b);uint32_t bits;memcpy(&bits,&value,4);std::cout<<bits;
  } else if(command=="pal-quant") {
    size_t count;int size,depth;bool high;in>>count>>size>>depth>>high;std::vector<int32_t> color(count);for(auto& v:color) in>>v;
    std::cout<<jxl::palette_internal::QuantizeColorToImplicitPaletteIndex(color,size,depth,high);
  } else if(command=="pal-round") {
    int value,div;in>>value>>div;std::cout<<jxl::RoundInt(value,div);
  } else if(command=="pal-forward") {
    uint32_t begin,end,colors,deltas,predictor;bool ordered,lossy,undo;int preset;in>>begin>>end>>colors>>deltas>>ordered>>lossy>>predictor>>preset>>undo;
    auto image=TransformReadImage(in,mm);jxl::weighted::Header header;jxl::weighted::PredictorMode(preset,&header);auto pred=static_cast<jxl::Predictor>(predictor);
    Status status=jxl::FwdPalette(image,begin,end,colors,deltas,ordered,lossy,pred,header);
    std::cout<<Code(status)<<' '<<colors<<' '<<deltas<<' '<<unsigned(pred);ImageModularObservation(image);
    if(status && undo) {status=jxl::InvPalette(image,begin,colors,deltas,pred,header,nullptr);std::cout<<' '<<Code(status);ImageModularObservation(image);}
  } else if(command=="pal-sort") {
    bool heap,reverse;size_t count;in>>heap>>reverse>>count;std::vector<std::array<int64_t,2>> values(count);
    for(size_t i=0;i<count;++i) {in>>values[i][0];values[i][1]=i;}
    auto cmp=[&](const auto& a,const auto& b){return reverse?a[0]>b[0]:a[0]<b[0];};
    if(heap) std::partial_sort(values.begin(),values.end(),values.end(),cmp);else std::sort(values.begin(),values.end(),cmp);
    std::cout<<count;for(const auto& v:values) std::cout<<' '<<v[0]<<','<<v[1];
  } else if(command=="tr-forward") {
    int preset;bool ordered,lossy;in>>preset>>ordered>>lossy;auto t=TransformRead(in);t.ordered_palette=ordered;t.lossy_palette=lossy;auto image=TransformReadImage(in,mm);jxl::weighted::Header header;jxl::weighted::PredictorMode(preset,&header);
    Status status=jxl::TransformForward(t,image,header,nullptr);std::cout<<Code(status);ImageTransformObservation(t);ImageModularObservation(image);
  } else if(command=="tr-minmax") {
    bool min,max;in>>min>>max;auto image=TransformReadImage(in,mm);int32_t lo=17,hi=23;jxl::compute_minmax(image.channel[0],min?&lo:nullptr,max?&hi:nullptr);std::cout<<lo<<' '<<hi;
  } else if(command=="tr-meta" || command=="tr-inverse") {
    int preset;in>>preset;auto t=TransformRead(in);auto image=TransformReadImage(in,mm);jxl::weighted::Header header;jxl::weighted::PredictorMode(preset,&header);
    Status s=command=="tr-meta"?t.MetaApply(image):t.Inverse(image,header,nullptr);std::cout<<Code(s);ImageTransformObservation(t);
    if(command=="tr-meta") TransformMetadata(image);else ImageModularObservation(image);
  } else if(command=="tr-undo") {
    int preset;size_t count;in>>preset>>count;std::vector<jxl::Transform> t;for(size_t i=0;i<count;++i) t.push_back(TransformRead(in));
    auto image=TransformReadImage(in,mm);image.transform=std::move(t);jxl::weighted::Header header;jxl::weighted::PredictorMode(preset,&header);image.undo_transforms(header,nullptr);std::cout<<"U";ImageModularObservation(image);
  } else if(command=="tr-roundtrip") {
    size_t kind;in>>kind;auto image=TransformReadImage(in,mm);jxl::Transform rct(jxl::TransformId::kRCT);rct.max_delta_error=77;rct.rct_type=kind;
    Status s=jxl::FwdRct(image,0,kind,nullptr);std::cout<<Code(s);if(s) image.transform.push_back(rct);
    jxl::Transform squeeze(jxl::TransformId::kSqueeze);squeeze.max_delta_error=77;jxl::DefaultSqueezeParameters(&squeeze.squeezes,image);
    s=jxl::FwdSqueeze(image,squeeze.squeezes,nullptr);std::cout<<' '<<Code(s);if(s) image.transform.push_back(squeeze);ImageModularObservation(image);
    jxl::weighted::Header header;image.undo_transforms(header,nullptr);std::cout<<" U";ImageModularObservation(image);
  } else if(command=="sq-tendency") {
    int64_t b,a,n;in>>b>>a>>n;std::cout<<jxl::SmoothTendency(b,a,n);
  } else if(command=="sq-check") {
    jxl::SqueezeParams p;int count;in>>p.begin_c>>p.num_c>>count;std::cout<<Code(jxl::CheckMetaSqueezeParams(p,count));
  } else if(command=="sq-default") {
    auto image=TransformReadImage(in,mm);std::vector<jxl::SqueezeParams> params(1);jxl::DefaultSqueezeParameters(&params,image);std::cout<<"P";SqueezeParamsObservation(params);TransformMetadata(image);
  } else if(command=="sq-meta" || command=="sq-forward" || command=="sq-inverse" || command=="sq-roundtrip") {
    auto params=SqueezeReadParams(in);auto image=TransformReadImage(in,mm);
    if(command=="sq-roundtrip" && params.empty()) jxl::DefaultSqueezeParameters(&params,image);
    Status status=true;
    if(command=="sq-meta") status=jxl::MetaSqueeze(image,&params);
    else if(command=="sq-inverse") status=jxl::InvSqueeze(image,params,nullptr);
    else status=jxl::FwdSqueeze(image,params,nullptr);
    std::cout<<Code(status);SqueezeParamsObservation(params);
    if(command=="sq-meta") TransformMetadata(image);else ImageModularObservation(image);
    if(command=="sq-roundtrip" && status) {status=jxl::InvSqueeze(image,params,nullptr);std::cout<<' '<<Code(status);ImageModularObservation(image);}
  } else if(command=="pal-value") {
    int index,size,stride,depth;size_t c;in>>index>>c>>size>>stride>>depth;std::vector<jxl::pixel_type> palette(6*stride);
    for(size_t i=0;i<palette.size();++i) palette[i]=static_cast<int32_t>((i*17)^(i<<3))-8192;
    std::cout<<jxl::palette_internal::GetPaletteValue(palette.data(),index,c,size,stride,depth);
  } else if(command=="pal-scale") {
    uint64_t value,depth;in>>value>>depth;std::cout<<jxl::palette_internal::Scale<4>(value,depth);
  } else if(command=="pal-meta") {
    uint32_t begin,end,colors,deltas;bool lossy;in>>begin>>end>>colors>>deltas>>lossy;auto image=TransformReadImage(in,mm);
    Status status=jxl::MetaPalette(image,begin,end,colors,deltas,lossy);std::cout<<Code(status);TransformMetadata(image);
  } else if(command=="pal-inverse") {
    uint32_t begin,colors,deltas,predictor;int preset;in>>begin>>colors>>deltas>>predictor>>preset;auto image=TransformReadImage(in,mm);
    jxl::weighted::Header h;jxl::weighted::PredictorMode(preset,&h);Status status=jxl::InvPalette(image,begin,colors,deltas,static_cast<jxl::Predictor>(predictor),h,nullptr);
    std::cout<<Code(status);ImageModularObservation(image);
  } else return false;
  return true;
}
