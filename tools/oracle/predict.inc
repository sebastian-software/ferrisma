// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Original inline predictors/tree lookup/state. Never observe the source's
// indeterminate non-tree PredictionResult::multiplier.
#include "lib/jxl/modular/encoding/context_predict.h"
void PredictHeaderObservation(const jxl::weighted::Header& h) {
  std::cout<<' '<<h.all_default<<','<<h.p1C<<','<<h.p2C<<','<<h.p3Ca<<','<<h.p3Cb<<','<<h.p3Cc<<','<<h.p3Cd<<','<<h.p3Ce;
  for(auto w:h.w) std::cout<<' '<<w;
}
void PredictStateObservation(const jxl::weighted::State& s) {
  std::cout<<' '<<s.pred;for(auto p:s.prediction) std::cout<<' '<<p;
  for(const auto& errors:s.pred_errors) TocValues(errors);TocValues(s.error);
}
jxl::FlatTree PredictFixtureTree() {
  jxl::FlatTree tree(5);auto& root=tree[0];root.property0=9;root.splitval0=0;root.childID=1;root.properties[0]=4;root.properties[1]=8;root.splitvals[0]=10000;root.splitvals[1]=0;
  const jxl::Predictor predictors[4]={jxl::Predictor::Left,jxl::Predictor::Gradient,jxl::Predictor::Weighted,jxl::Predictor::Average4};
  const int offsets[4]={-17,0,9,127};const uint32_t contexts[4]={11,19,23,31};
  for(size_t i=0;i<4;++i) {auto& node=tree[i+1];node.property0=-1;node.predictor=predictors[i];node.childID=contexts[i];node.multiplier=1<<i;node.predictor_offset=offsets[i];}return tree;
}
bool PredictCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if(command=="pred-one") {
    uint32_t which;int64_t v[8];in>>which;for(auto& x:v) in>>x;auto p=static_cast<jxl::Predictor>(which);
    std::cout<<jxl::PredictorHasWeighted(p)<<' '<<jxl::detail::PredictOne(p,v[0],v[1],v[2],v[3],v[4],v[5],v[6],v[7])<<' '<<jxl::Select(v[0],v[1],v[3])<<' '<<jxl::ClampedGradient(static_cast<int32_t>(v[1]),static_cast<int32_t>(v[0]),static_cast<int32_t>(v[3]));
  } else if(command=="pred-preset") {
    int preset;in>>preset;jxl::weighted::Header h;jxl::weighted::PredictorMode(preset,&h);std::cout<<"H";PredictHeaderObservation(h);
  } else if(command=="pred-header"||command=="pred-header-read") {
    size_t prefix;in>>prefix;jxl::weighted::Header h;
    if(command=="pred-header-read") {
      jxl::weighted::PredictorMode(1,&h);std::string hex;in>>hex;Input input(hex);BitReader br(input.bytes());br.SkipBits(prefix);Status read=jxl::Bundle::Read(&br,&h);std::cout<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());PredictHeaderObservation(h);
    } else {
      in>>h.p1C>>h.p2C>>h.p3Ca>>h.p3Cb>>h.p3Cc>>h.p3Cd>>h.p3Ce;for(auto& w:h.w) in>>w;
      size_t ext=0,bits=0;Status can=jxl::Bundle::CanEncode(h,&ext,&bits);std::cout<<Code(can)<<' '<<ext<<' '<<bits;
      if(can) {BitWriter writer(mm);if(!Prefix(writer,prefix)) throw std::runtime_error("weighted prefix");Status write=jxl::Bundle::Write(h,&writer,LayerType::Header,nullptr);std::cout<<' '<<Code(write)<<' '<<writer.BitsWritten();if(!Pad(writer)) throw std::runtime_error("weighted pad");std::cout<<' '<<Hex(writer.GetSpan());jxl::weighted::Header decoded;BitReader br(writer.GetSpan());br.SkipBits(prefix);Status read=jxl::Bundle::Read(&br,&decoded);std::cout<<' '<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());PredictHeaderObservation(decoded);}
      PredictHeaderObservation(h);
    }
  } else if(command=="pred-math") {
    uint64_t x;uint32_t max;int64_t p[4];std::array<uint32_t,4> w;in>>x>>max;for(auto& v:p) in>>v;for(auto& v:w) in>>v;jxl::weighted::Header h;jxl::weighted::State s(h,1,1);
    std::cout<<s.ErrorWeight(x,max)<<' '<<s.WeightedAverage(p,w)<<' '<<s.AddBits(p[0]);for(auto d:s.divlookup) std::cout<<' '<<d;
  } else if(command=="pred-lookup") {
    size_t n,count;in>>n>>count;jxl::FlatTree tree(n);
    for(auto& node:tree) {int32_t a,b,c,t0,t1;in>>node.property0>>a>>b>>c>>node.childID>>t0>>t1;if(node.property0<0) {node.predictor=static_cast<jxl::Predictor>(static_cast<uint32_t>(a));node.multiplier=b;node.predictor_offset=t0;} else {node.splitval0=a;node.splitvals[0]=b;node.splitvals[1]=c;node.properties[0]=t0;node.properties[1]=t1;}}
    jxl::Properties properties(count);for(auto& p:properties) in>>p;jxl::MATreeLookup lookup(tree);auto result=lookup.Lookup(properties);
    std::cout<<sizeof(jxl::FlatDecisionNode)<<' '<<alignof(jxl::FlatDecisionNode)<<' '<<result.context<<','<<unsigned(result.predictor)<<','<<result.offset<<','<<result.multiplier;
  } else if(command=="pred-references") {
    size_t w,h,refw,y;int n;uint32_t current,mode;in>>w>>h>>n>>current>>y>>refw>>mode;
    auto created=jxl::Image::Create(mm,w,h,16,n);if(!created.ok()) throw std::runtime_error("reference image");auto image=std::move(created).value_();RctFill(image,in);
    for(uint32_t i=0;i<current;++i) {if(mode&1 && i%3==0) image.channel[i].w=w-1;if(mode&2 && i%3==1) image.channel[i].hshift=1;if(mode&4 && i%3==2) image.channel[i].vshift=1;}
    auto r=jxl::Channel::Create(mm,refw,w);if(!r.ok()) throw std::runtime_error("reference output");auto references=std::move(r).value_();ImagePaint(references.plane,141);
    jxl::PrecomputeReferences(image.channel[current],y,image,current,&references);std::cout<<"R";ImageChannelObservation(references,w,true);
  } else if(command=="pred-stream") {
    unsigned mode,predictor;int preset,static0,static1;size_t w,h,refw;in>>mode>>w>>h>>predictor>>preset>>refw>>static0>>static1;
    auto created=jxl::Image::Create(mm,w,h,16,3);if(!created.ok()) throw std::runtime_error("prediction image");auto image=std::move(created).value_();RctFill(image,in);
    auto r=jxl::Channel::Create(mm,refw,w);if(!r.ok()) throw std::runtime_error("prediction references");auto refs=std::move(r).value_();ImagePaint(refs.plane,151);
    jxl::weighted::Header header;jxl::weighted::PredictorMode(preset,&header);jxl::weighted::State state(header,w,h);
    jxl::Properties props(jxl::kNumNonrefProperties+refw);for(size_t i=0;i<props.size();++i) props[i]=i*17-100;
    auto tree=PredictFixtureTree();jxl::MATreeLookup lookup(tree);std::cout<<"S";PredictHeaderObservation(header);
    const auto& ch=image.channel[2];const ptrdiff_t stride=ch.plane.PixelsPerRow();
    for(size_t y=0;y<h;++y) {jxl::PrecomputeReferences(ch,y,image,2,&refs);jxl::InitPropsRow(&props,{static0,static1},y);
      for(size_t x=0;x<w;++x) {
        std::vector<int64_t> predictions(jxl::kNumModularPredictors,-999);const int32_t* pp=ch.Row(y)+x;bool nec=y>1&&x>1&&x<w-2;
        bool all=mode==5||mode==9||mode==10,uses_wp=mode==1||mode==3||mode==4||mode==5||mode==7||mode==8||mode==9,uses_tree=mode==2||mode==3||mode==6||mode==7;
        auto predictor_value=static_cast<jxl::Predictor>(predictor);
        if(all) {
          if(mode==10) jxl::PredictAllNoWP(w,pp,stride,x,y,predictions.data());
          else if(mode==9&&nec) jxl::PredictLearnAllNEC(&props,w,pp,stride,x,y,refs,&state,predictions.data());
          else jxl::PredictLearnAll(&props,w,pp,stride,x,y,refs,&state,predictions.data());
          std::cout<<" A";TocValues(predictions);
        } else {
          const auto print=[&](const jxl::PredictionResult& result) {std::cout<<" P "<<result.context<<','<<result.guess<<','<<unsigned(result.predictor);if(uses_tree) std::cout<<','<<result.multiplier;};
          switch(mode) {
            case 0:print(jxl::PredictNoTreeNoWP(w,pp,stride,x,y,predictor_value));break;
            case 1:print(jxl::PredictNoTreeWP(w,pp,stride,x,y,predictor_value,&state));break;
            case 2:print(jxl::PredictTreeNoWP(&props,w,pp,stride,x,y,lookup,refs));break;
            case 3:print(jxl::PredictTreeWP(&props,w,pp,stride,x,y,lookup,refs,&state));break;
            case 4:print(jxl::PredictLearn(&props,w,pp,stride,x,y,predictor_value,refs,&state));break;
            case 6:print(nec?jxl::PredictTreeNoWPNEC(&props,w,pp,stride,x,y,lookup,refs):jxl::PredictTreeNoWP(&props,w,pp,stride,x,y,lookup,refs));break;
            case 7:print(nec?jxl::PredictTreeWPNEC(&props,w,pp,stride,x,y,lookup,refs,&state):jxl::PredictTreeWP(&props,w,pp,stride,x,y,lookup,refs,&state));break;
            default:print(nec?jxl::PredictLearnNEC(&props,w,pp,stride,x,y,predictor_value,refs,&state):jxl::PredictLearn(&props,w,pp,stride,x,y,predictor_value,refs,&state));break;
          }
        }
        if(uses_wp) state.UpdateErrors(ch.Row(y)[x],x,y,w);TocValues(props);PredictStateObservation(state);
      }
    }
  } else return false;
  return true;
}
