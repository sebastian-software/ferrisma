// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Calls the original tree parser, tokenization, filtering, LUT, and bundles.
#include "lib/jxl/modular/encoding/dec_ma.h"
#include "lib/jxl/modular/encoding/enc_ma.h"
#include "lib/jxl/modular/encoding/enc_encoding.h"
#include "lib/jxl/modular/encoding/encoding.h"
#include "lib/jxl/modular/encoding/ma_common.h"
namespace jxl {
Status DecodeModularChannelMAANS(BitReader*,ANSSymbolReader*,const std::vector<uint8_t>&,const Tree&,const weighted::Header&,pixel_type,size_t,TreeLut<uint8_t,false,false>&,Image*,uint32_t&,uint32_t&);
}

jxl::Tree MaReadTree(std::istringstream& in) {
  size_t n; in>>n; jxl::Tree tree;
  for(size_t i=0;i<n;++i) {
    int p,s,l,r;unsigned predictor,mult;int64_t offset;
    in>>p>>s>>l>>r>>predictor>>offset>>mult;
    tree.emplace_back(p,s,l,r,static_cast<jxl::Predictor>(predictor),offset,mult);
  }
  return tree;
}
void MaTree(const jxl::Tree& tree) {
  std::cout<<' '<<tree.size();
  for(const auto& n:tree) std::cout<<' '<<n.property<<','<<n.splitval<<','<<n.lchild<<','<<n.rchild<<','<<unsigned(n.predictor)<<','<<n.predictor_offset<<','<<n.multiplier;
}
void MaTokens(const std::vector<jxl::Token>& tokens) {
  std::cout<<' '<<tokens.size();for(const auto& t:tokens) std::cout<<' '<<t.context<<','<<t.value<<','<<t.is_lz77_length;
}
jxl::Tree MaSeedTree() {return {jxl::PropertyDecisionNode::Leaf(jxl::Predictor::Left,77,3)};}
void MaDecode(BitReader& br,size_t limit,JxlMemoryManager* mm) {
  auto tree=MaSeedTree();Status status=jxl::DecodeTree(mm,&br,&tree,limit);
  std::cout<<' '<<Code(status)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());MaTree(tree);
}
template<typename T,bool O,bool M>
void MaLut(const jxl::FlatTree& flat) {
  jxl::TreeLut<T,O,M> lut;lut.context_lookup.fill(T(197));lut.offsets.fill(73);lut.multipliers.fill(47);
  bool ok=jxl::TreeToLookupTable(flat,lut);std::cout<<' '<<ok;
  // Lossless run-length observation of every table element, including entries
  // left unchanged by a failed conversion. No digest hides a partial state.
  for(size_t i=0;i<lut.context_lookup.size();++i) {
    int off=O?lut.offsets[i]:0,mul=M?lut.multipliers[i]:0;
    if(i==0 || lut.context_lookup[i]!=lut.context_lookup[i-1] ||
       (O && off!=lut.offsets[i-1]) || (M && mul!=lut.multipliers[i-1]))
      std::cout<<' '<<i<<','<<uint64_t(lut.context_lookup[i])<<','<<off<<','<<mul;
  }
  std::cout<<" E";
}
template<typename T>
void MaLuts(unsigned flags,const jxl::FlatTree& flat) {
  switch(flags) {case 0:MaLut<T,false,false>(flat);break;case 1:MaLut<T,true,false>(flat);break;case 2:MaLut<T,false,true>(flat);break;case 3:MaLut<T,true,true>(flat);break;}
}
void MaHeaderSeed(jxl::GroupHeader& h,size_t count) {
  h.use_global_tree=true;jxl::weighted::PredictorMode(2,&h.wp_header);
  h.transforms.resize(count);
  for(size_t i=0;i<count;++i) {
    auto& t=h.transforms[i];t.id=static_cast<jxl::TransformId>(i%3);t.begin_c=i%32;t.rct_type=i%42;
    t.num_c=1+i%4;t.nb_colors=i%257;t.nb_deltas=i%17;t.predictor=static_cast<jxl::Predictor>(i%14);
    t.max_delta_error=77;t.ordered_palette=false;t.lossy_palette=true;
    if(i%3==2) {t.squeezes.resize(i%4);for(auto& s:t.squeezes) {s.horizontal=i%2;s.in_place=i%2==0;s.begin_c=i%16;s.num_c=1+i%16;}}
  }
}
void MaHeader(jxl::GroupHeader& h) {
  std::cout<<' '<<h.use_global_tree;
  const auto& w=h.wp_header;
  std::cout<<' '<<w.all_default<<','<<w.p1C<<','<<w.p2C<<','<<w.p3Ca<<','<<w.p3Cb<<','<<w.p3Cc<<','<<w.p3Cd<<','<<w.p3Ce;
  for(auto v:w.w) std::cout<<' '<<v;
  std::cout<<' '<<h.transforms.size();
  for(auto& t:h.transforms) {t.max_delta_error=77;ImageTransformObservation(t);}
}
void MaRasterDecode(BitReader& br,size_t w,size_t h,int n,size_t group,int preset,const jxl::Tree& tree,JxlMemoryManager* mm) {
  auto created=jxl::Image::Create(mm,w,h,16,n);if(!created.ok()) throw std::runtime_error("ma decoded image");auto image=std::move(created).value_();
  for(auto& c:image.channel) ImagePaint(c.plane,81+c.component);
  jxl::ANSCode code;std::vector<uint8_t> map;jxl::weighted::Header header;jxl::weighted::PredictorMode(preset,&header);
  Status status=jxl::DecodeHistograms(mm,&br,(tree.size()+1)/2,&code,&map);std::cout<<' '<<Code(status)<<' '<<br.TotalBitsConsumed();
  if(status) {
    auto reader_created=jxl::ANSSymbolReader::Create(&code,&br,w);if(!reader_created.ok()) throw std::runtime_error("ma raster reader");auto reader=std::move(reader_created).value_();
    std::cout<<' '<<reader.UsesLZ77()<<' '<<reader.IsHuffRleOnly();jxl::TreeLut<uint8_t,false,false> lut{};uint32_t run=0,value=0;
    for(int c=0;c<n;++c) {
      status=jxl::DecodeModularChannelMAANS(&br,&reader,map,tree,header,c,group,lut,&image,run,value);
      std::cout<<' '<<Code(status)<<' '<<br.TotalBitsConsumed()<<' '<<run<<' '<<value<<' '<<Code(br.AllReadsWithinBounds());
      if(!status) break;
    }
    std::cout<<' '<<reader.CheckANSFinalState();
  }
  std::cout<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());ImageModularObservation(image);
}
bool MaCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if(command=="ma-raster" || command=="ma-raster-read") {
    size_t w,h,group,prefix;int n,preset;in>>w>>h>>n>>group>>preset;
    if(command=="ma-raster") {
      bool force,skip;unsigned lz;in>>force>>lz>>skip>>prefix;
      auto created=jxl::Image::Create(mm,w,h,16,n);if(!created.ok()) throw std::runtime_error("ma raster allocation");auto image=std::move(created).value_();RctFill(image,in);
      auto input_tree=MaReadTree(in);jxl::Tree tree;std::vector<jxl::Token> tt;if(!jxl::TokenizeTree(input_tree,&tt,&tree)) throw std::runtime_error("ma raster tree");
      jxl::ModularOptions opts;opts.predictor=jxl::Predictor::Weighted;opts.wp_mode=preset;opts.skip_encoder_fast_path=skip;
      jxl::GroupHeader header;std::vector<std::vector<jxl::Token>> tokens(1);size_t width=0;
      Status status=jxl::ModularCompress(image,opts,group,tree,header,tokens[0],&width);std::cout<<Code(status);if(!status) return true;
      jxl::HistogramParams params;params.force_huffman=force;params.image_widths.push_back(width);params.lz77_method=lz?jxl::HistogramParams::LZ77Method::kRLE:jxl::HistogramParams::LZ77Method::kNone;
      if(lz==2) params.uint_method=jxl::HistogramParams::HybridUintMethod::k000;
      BitWriter writer(mm);if(!Prefix(writer,prefix)) throw std::runtime_error("ma raster prefix");jxl::EntropyEncodingData code;
      auto built=jxl::BuildAndEncodeHistograms(mm,params,(tree.size()+1)/2,tokens,&code,&writer,LayerType::ModularGlobal,nullptr);
      status=built.ok()?Status(true):built.status();if(status) status=jxl::WriteTokens(tokens[0],code,0,&writer,LayerType::ModularGlobal,nullptr);
      std::cout<<' '<<Code(status)<<' '<<writer.BitsWritten();if(!Pad(writer)) throw std::runtime_error("ma raster padding");std::cout<<' '<<Hex(writer.GetSpan());
      BitReader br(writer.GetSpan());br.SkipBits(prefix);MaRasterDecode(br,w,h,n,group,preset,tree,mm);
    } else {
      in>>prefix;auto input_tree=MaReadTree(in);jxl::Tree tree;std::vector<jxl::Token> tt;if(!jxl::TokenizeTree(input_tree,&tt,&tree)) throw std::runtime_error("ma raster read tree");
      std::string hex;in>>hex;Input input(hex);BitReader br(input.bytes());br.SkipBits(prefix);std::cout<<"R";MaRasterDecode(br,w,h,n,group,preset,tree,mm);
    }
  } else if(command=="ma-predefined") {
    unsigned kind;size_t pixels;int depth,prev;in>>kind>>pixels>>depth>>prev;
    auto tree=jxl::PredefinedTree(static_cast<jxl::ModularOptions::TreeKind>(kind),pixels,depth,prev);std::cout<<"T";MaTree(tree);
  } else if(command=="ma-compress") {
    size_t w,h,group,max_channel,meta;int n,depth,wp;unsigned predictor,variant;bool error,skip,zero;
    in>>w>>h>>n>>depth>>error>>group>>wp>>predictor>>skip>>zero>>max_channel>>meta>>variant;
    auto created=jxl::Image::Create(mm,w,h,depth,n);if(!created.ok()) throw std::runtime_error("ma image allocation");
    auto image=std::move(created).value_();RctFill(image,in);image.error=error;image.nb_meta_channels=meta;
    if(image.channel.size()>1) {if(variant==1) image.channel[1].w=0;if(variant==2) image.channel[1].hshift=1;if(variant==3) image.channel[1].h=0;}
    jxl::Transform transform(jxl::TransformId::kRCT);transform.max_delta_error=77;image.transform.push_back(transform);
    auto tree=MaReadTree(in);jxl::ModularOptions options;options.wp_mode=wp;options.predictor=static_cast<jxl::Predictor>(predictor);
    options.skip_encoder_fast_path=skip;options.zero_tokens=zero;options.max_chan_size=max_channel;
    jxl::GroupHeader header;MaHeaderSeed(header,3);size_t width=314159;std::vector<jxl::Token> tokens{{5,0xdeadbeef}};
    Status status=jxl::ModularCompress(image,options,group,tree,header,tokens,&width);
    std::cout<<Code(status)<<' '<<width;MaHeader(header);MaTokens(tokens);
  } else if(command=="ma-token") {
    auto tree=MaReadTree(in);std::vector<jxl::Token> tokens{{5,0xdeadbeef}};auto decoded=MaSeedTree();
    Status status=jxl::TokenizeTree(tree,&tokens,&decoded);std::cout<<Code(status);MaTokens(tokens);MaTree(decoded);
  } else if(command=="ma-write" || command=="ma-stream") {
    size_t prefix,limit;bool force,lz;in>>prefix>>limit>>force>>lz;
    std::vector<std::vector<jxl::Token>> tokens(1);jxl::Tree decoded;Status status=true;
    if(command=="ma-write") {auto tree=MaReadTree(in);status=jxl::TokenizeTree(tree,&tokens[0],&decoded);}
    else {size_t count;in>>count;for(size_t i=0;i<count;++i) {uint32_t c,v;in>>c>>v;tokens[0].emplace_back(c,v);}}
    std::cout<<Code(status);MaTokens(tokens[0]);MaTree(decoded);
    if(!status) return true;
    BitWriter writer(mm);if(!Prefix(writer,prefix)) throw std::runtime_error("ma prefix");
    jxl::HistogramParams params;params.force_huffman=force;params.lz77_method=lz?jxl::HistogramParams::LZ77Method::kRLE:jxl::HistogramParams::LZ77Method::kNone;
    jxl::EntropyEncodingData code;
    auto built=jxl::BuildAndEncodeHistograms(mm,params,jxl::kNumTreeContexts,tokens,&code,&writer,LayerType::ModularTree,nullptr);
    status=built.ok()?Status(true):built.status();
    if(status) status=jxl::WriteTokens(tokens[0],code,0,&writer,LayerType::ModularTree,nullptr);
    std::cout<<' '<<Code(status)<<' '<<writer.BitsWritten();if(!Pad(writer)) throw std::runtime_error("ma pad");
    std::cout<<' '<<Hex(writer.GetSpan());BitReader br(writer.GetSpan());br.SkipBits(prefix);MaDecode(br,limit,mm);
  } else if(command=="ma-read") {
    size_t prefix,limit;std::string hex;in>>prefix>>limit>>hex;Input input(hex);BitReader br(input.bytes());br.SkipBits(prefix);std::cout<<"R";MaDecode(br,limit,mm);
  } else if(command=="ma-filter") {
    std::array<jxl::pixel_type,2> props;unsigned width,flags;in>>props[0]>>props[1]>>width>>flags;auto tree=MaReadTree(in);
    size_t num;bool wp,only,gradient;auto flat=jxl::FilterTree(tree,props,&num,&wp,&only,&gradient);
    std::cout<<num<<' '<<wp<<' '<<only<<' '<<gradient<<' '<<props[0]<<' '<<props[1]<<' '<<flat.size();
    for(const auto& n:flat) {
      std::cout<<' '<<n.property0<<','<<n.childID;
      if(n.property0==-1) std::cout<<','<<unsigned(n.predictor)<<','<<n.predictor_offset<<','<<n.multiplier;
      else std::cout<<','<<n.splitval0<<','<<n.properties[0]<<','<<n.properties[1]<<','<<n.splitvals[0]<<','<<n.splitvals[1];
    }
    if(width==8) MaLuts<uint8_t>(flags,flat);else if(width==16) MaLuts<uint16_t>(flags,flat);else if(width==32) MaLuts<uint32_t>(flags,flat);
  } else if(command=="ma-group" || command=="ma-group-read") {
    size_t count,prefix;in>>count>>prefix;jxl::GroupHeader h;MaHeaderSeed(h,count);
    if(command=="ma-group-read") {
      std::string hex;in>>hex;Input input(hex);BitReader br(input.bytes());br.SkipBits(prefix);Status read=jxl::Bundle::Read(&br,&h);
      std::cout<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());MaHeader(h);
    } else {
      size_t ext=0,bits=0;Status can=jxl::Bundle::CanEncode(h,&ext,&bits);std::cout<<Code(can)<<' '<<ext<<' '<<bits;
      if(can) {BitWriter writer(mm);if(!Prefix(writer,prefix)) throw std::runtime_error("ma group prefix");Status write=jxl::Bundle::Write(h,&writer,LayerType::Header,nullptr);
        std::cout<<' '<<Code(write)<<' '<<writer.BitsWritten();if(!Pad(writer)) throw std::runtime_error("ma group pad");std::cout<<' '<<Hex(writer.GetSpan());
        jxl::GroupHeader out;MaHeaderSeed(out,3);BitReader br(writer.GetSpan());br.SkipBits(prefix);Status read=jxl::Bundle::Read(&br,&out);
        std::cout<<' '<<Code(read)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());MaHeader(out);
      }
      MaHeader(h);
    }
  } else return false;
  return true;
}
