// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Actual RNG and tree sample APIs, including all quantization-map entries.
#include "lib/jxl/base/random.h"
#include <numeric>
namespace jxl {namespace N_SCALAR {float EstimateBits(const int32_t*,size_t);}}

template<class T> std::vector<T> LearnValues(std::istringstream& in) {
  size_t n;in>>n;std::vector<T> result(n);for(auto& v:result) in>>v;return result;
}
void LearnConfigObservation(const jxl::TreeSamples& s) {
  std::cout<<' '<<s.NumPredictors();for(size_t i=0;i<s.NumPredictors();++i) std::cout<<' '<<unsigned(s.PredictorFromIndex(i));
  std::cout<<' '<<s.NumProperties();for(size_t i=0;i<s.NumProperties();++i) std::cout<<' '<<s.PropertyFromIndex(i);
  std::cout<<' '<<s.NumStaticProps()<<' '<<s.NumSamples()<<' '<<s.NumDistinctSamples()<<' '<<s.HasSamples();
}
void LearnSamplesObservation(const jxl::TreeSamples& s) {
  LearnConfigObservation(s);
  for(size_t p=0;p<s.NumProperties();++p) {
    std::cout<<' '<<s.NumPropertyValues(p);for(size_t q=0;q+1<s.NumPropertyValues(p);++q) std::cout<<' '<<s.UnquantizeProperty(p,q);
    uint32_t previous=0;
    for(int v=-520;v<=520;++v) {
      uint32_t q=p<s.NumStaticProps()?s.QuantizeStaticProperty(p,v):s.QuantizeProperty(p,v);
      if(v==-520 || q!=previous) std::cout<<' '<<v<<','<<q;previous=q;
    }
    std::cout<<" E";
  }
  for(size_t i=0;i<s.NumDistinctSamples();++i) {
    std::cout<<' '<<s.Count(i);
    for(size_t p=0;p<s.NumPredictors();++p) {const auto& t=s.RToken(p,i);std::cout<<' '<<unsigned(t.tok)<<','<<unsigned(t.nbits);}
    for(size_t p=0;p<s.NumStaticProps();++p) std::cout<<' '<<s.Property<true>(p,i);
    for(size_t p=0;p<s.NumProperties()-s.NumStaticProps();++p) std::cout<<' '<<s.Property<false>(p,i);
  }
}
jxl::ModularOptions LearnOptions(std::istringstream& in) {
  jxl::ModularOptions o;unsigned kind,predictor,mode,lz,cluster,uint_method,strategy;uint32_t repeats,threshold,fast;
  in>>kind>>predictor>>o.wp_mode>>mode>>o.max_properties>>o.max_property_values>>repeats>>threshold>>fast>>o.skip_encoder_fast_path>>o.zero_tokens>>o.max_chan_size>>o.group_dim>>o.histogram_params.force_huffman>>lz>>cluster>>uint_method>>strategy;
  o.tree_kind=static_cast<jxl::ModularOptions::TreeKind>(kind);o.predictor=static_cast<jxl::Predictor>(predictor);o.wp_tree_mode=static_cast<jxl::ModularOptions::TreeMode>(mode);
  memcpy(&o.nb_repeats,&repeats,4);memcpy(&o.splitting_heuristics_node_threshold,&threshold,4);memcpy(&o.fast_decode_multiplier,&fast,4);
  o.histogram_params.lz77_method=static_cast<jxl::HistogramParams::LZ77Method>(lz);o.histogram_params.clustering=static_cast<jxl::HistogramParams::ClusteringType>(cluster);
  o.histogram_params.uint_method=static_cast<jxl::HistogramParams::HybridUintMethod>(uint_method);o.histogram_params.ans_histogram_strategy=static_cast<jxl::HistogramParams::ANSHistogramStrategy>(strategy);
  o.splitting_heuristics_properties=LearnValues<uint32_t>(in);return o;
}
bool LearnCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if(command=="learn-tree") {
    uint32_t start,stop;size_t count;in>>start>>stop>>count;std::vector<jxl::Image> images;std::vector<jxl::ModularOptions> opts;
    for(size_t i=0;i<count;++i) {opts.push_back(LearnOptions(in));images.push_back(TransformReadImage(in,mm));}
    size_t num_mult;in>>num_mult;std::vector<jxl::ModularMultiplierInfo> mult(num_mult);for(auto& m:mult) {for(auto& r:m.range) in>>r[0]>>r[1];in>>m.multiplier;}
    auto result=jxl::LearnTree(images.data(),opts.data(),start,stop,mult);std::cout<<Code(result.status());if(result.ok()) MaTree(std::move(result).value_());
  } else if(command=="learn-generic") {
    size_t prefix,group;unsigned transforms;bool roundtrip;in>>prefix>>group>>transforms>>roundtrip;auto options=LearnOptions(in);auto image=TransformReadImage(in,mm);
    if((transforms==1 || transforms==3) && image.channel.size()>=3) {jxl::Transform t(jxl::TransformId::kRCT);t.max_delta_error=77;t.rct_type=6;if(jxl::FwdRct(image,0,6,nullptr)) image.transform.push_back(t);}
    if(transforms>=4 && transforms<=7) {jxl::Transform t(jxl::TransformId::kPalette);t.max_delta_error=77;t.num_c=image.channel.size();t.nb_colors=transforms>=6?4:256;t.ordered_palette=transforms>=6;t.lossy_palette=transforms>=6;t.predictor=transforms>=6?jxl::Predictor::Average4:jxl::Predictor::Zero;jxl::weighted::Header header;if(jxl::TransformForward(t,image,header,nullptr)) image.transform.push_back(t);}
    if(transforms==2 || transforms==3 || transforms==5 || transforms==7) {jxl::Transform t(jxl::TransformId::kSqueeze);t.max_delta_error=77;jxl::DefaultSqueezeParameters(&t.squeezes,image);if(jxl::FwdSqueeze(image,t.squeezes,nullptr)) image.transform.push_back(t);}
    BitWriter writer(mm);if(!Prefix(writer,prefix)) throw std::runtime_error("generic fixture prefix");jxl::AuxOut aux;
    Status status=jxl::ModularGenericCompress(image,options,writer,&aux,LayerType::ModularGlobal,group);std::cout<<Code(status)<<' '<<writer.BitsWritten();if(!Pad(writer)) throw std::runtime_error("generic pad");std::cout<<' '<<Hex(writer.GetSpan());
    for(auto layer:{LayerType::ModularGlobal,LayerType::ModularTree}) {const auto& t=aux.layer(layer);uint64_t bits;memcpy(&bits,&t.clustered_entropy,8);std::cout<<' '<<t.num_clustered_histograms<<' '<<t.extra_bits<<' '<<t.histogram_bits<<' '<<t.total_bits<<' '<<bits;}
    if(status && roundtrip) {
      jxl::weighted::Header wp;image.undo_transforms(wp,nullptr);if(image.error) throw std::runtime_error("generic input inverse");
      auto created=jxl::Image::Create(mm,image.w,image.h,image.bitdepth,image.channel.size());if(!created.ok()) throw std::runtime_error("generic decoded image");auto decoded=std::move(created).value_();
      BitReader br(writer.GetSpan());br.SkipBits(prefix);status=jxl::ModularGenericDecompress(&br,decoded,nullptr,group,&options);
      std::cout<<' '<<Code(status)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());for(auto& t:decoded.transform) t.max_delta_error=77;ImageModularObservation(decoded);
    }
  } else if(command=="learn-estimate") {
    auto counts=LearnValues<int32_t>(in);float value=jxl::N_SCALAR::EstimateBits(counts.data(),counts.size());uint32_t bits;memcpy(&bits,&value,4);std::cout<<bits;
  } else if(command=="learn-rng") {
    uint64_t seed,bu,eu;int64_t bi,ei;uint32_t bf,ef,p;size_t count;in>>seed>>bi>>ei>>bu>>eu>>bf>>ef>>p>>count;
    float begin,end,prob;memcpy(&begin,&bf,4);memcpy(&end,&ef,4);memcpy(&prob,&p,4);jxl::Rng rng(seed);auto dist=jxl::Rng::MakeGeometric(prob);uint32_t bits;memcpy(&bits,&dist,4);std::cout<<bits;
    for(size_t i=0;i<count;++i) {std::cout<<' '<<rng()<<' '<<rng.UniformI(bi,ei)<<' '<<rng.UniformU(bu,eu);float f=rng.UniformF(begin,end);memcpy(&bits,&f,4);std::cout<<' '<<bits<<' '<<rng.Bernoulli(prob)<<' '<<rng.Geometric(dist);}
  } else if(command=="learn-shuffle") {
    uint64_t seed;size_t count;in>>seed>>count;std::vector<uint32_t> values(count+3);std::iota(values.begin(),values.end(),0);jxl::Rng rng(seed);rng.Shuffle(values.data(),count);std::cout<<rng();TocValues(values);
  } else if(command=="learn-config") {
    size_t n;in>>n;jxl::TreeSamples s;if(!s.SetPredictor(jxl::Predictor::Zero,jxl::ModularOptions::TreeMode::kDefault) || !s.SetProperties({0,1,9},jxl::ModularOptions::TreeMode::kDefault)) throw std::runtime_error("learning config seed");std::cout<<"C";
    for(size_t i=0;i<n;++i) {unsigned p,mode;in>>p>>mode;auto props=LearnValues<uint32_t>(in);auto m=static_cast<jxl::ModularOptions::TreeMode>(mode);Status a=s.SetPredictor(static_cast<jxl::Predictor>(p),m),b=s.SetProperties(props,m);std::cout<<' '<<Code(a)<<' '<<Code(b);LearnConfigObservation(s);}
  } else if(command=="learn-samples" || command=="learn-split") {
    uint32_t threshold_bits=0,fast_bits=0;if(command=="learn-split") in>>threshold_bits>>fast_bits;
    unsigned predictor,mode;size_t max_values;in>>predictor>>mode>>max_values;auto props=LearnValues<uint32_t>(in);jxl::StaticPropRange range;for(auto& r:range) in>>r[0]>>r[1];
    size_t num_mult;in>>num_mult;std::vector<jxl::ModularMultiplierInfo> mult(num_mult);for(auto& m:mult) {for(auto& r:m.range) in>>r[0]>>r[1];in>>m.multiplier;}
    auto group=LearnValues<uint32_t>(in),channel=LearnValues<uint32_t>(in);auto pixel=LearnValues<int32_t>(in),diff=LearnValues<int32_t>(in);
    jxl::TreeSamples s;auto tm=static_cast<jxl::ModularOptions::TreeMode>(mode);if(!s.SetPredictor(static_cast<jxl::Predictor>(predictor),tm) || !s.SetProperties(props,tm)) throw std::runtime_error("learning fixture setup");
    s.PreQuantizeProperties(range,mult,group,channel,pixel,diff,max_values);std::cout<<"S";TocValues(pixel);TocValues(diff);LearnSamplesObservation(s);
    size_t batches;in>>batches;
    for(size_t b=0;b<batches;++b) {
      size_t extra,count,repeat;in>>extra>>count>>repeat;s.PrepareForSamples(extra);
      struct Sample {int64_t pixel;jxl::Properties props;std::vector<int64_t> preds;};std::vector<Sample> data(count);
      for(auto& v:data) {in>>v.pixel;v.props=LearnValues<int32_t>(in);v.preds=LearnValues<int64_t>(in);}
      for(size_t j=0;j<repeat;++j) for(const auto& v:data) s.AddSample(v.pixel,v.props,v.preds.data());
      LearnSamplesObservation(s);
    }
    size_t swaps;in>>swaps;for(size_t i=0;i<swaps;++i) {if(s.NumDistinctSamples()) s.Swap(0,s.NumDistinctSamples()-1);LearnSamplesObservation(s);}
    if(command=="learn-split") {
      float threshold,fast;memcpy(&threshold,&threshold_bits,4);memcpy(&fast,&fast_bits,4);s.AllSamplesDone();jxl::Tree tree;
      Status status=jxl::ComputeBestTree(s,threshold,mult,range,fast,&tree);std::cout<<' '<<Code(status);MaTree(tree);LearnSamplesObservation(s);
    }
  } else if(command=="learn-collect") {
    uint32_t repeats,group;size_t max;in>>repeats>>group>>max;auto image=TransformReadImage(in,mm);auto gc=LearnValues<uint32_t>(in),cc=LearnValues<uint32_t>(in);auto pixels=LearnValues<int32_t>(in),diff=LearnValues<int32_t>(in);
    jxl::ModularOptions options;memcpy(&options.nb_repeats,&repeats,4);options.max_chan_size=max;jxl::CollectPixelSamples(image,options,group,gc,cc,pixels,diff);std::cout<<"P";TocValues(gc);TocValues(cc);TocValues(pixels);TocValues(diff);
  } else return false;
  return true;
}
