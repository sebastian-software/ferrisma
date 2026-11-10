// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Actual image decoder entry points. Only C++ generates the standalone source
// fixtures here; no Rust generic image encoder is substituted by this adapter.
#include "lib/jxl/modular/transform/enc_palette.h"
namespace jxl {
Status ModularDecode(BitReader*,Image&,GroupHeader&,size_t,ModularOptions*,const Tree*,const ANSCode*,const std::vector<uint8_t>*,bool);
}
bool ModularDecodeCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if(command=="md-source" || command=="md-source-global") {
    unsigned kind,predictor,transforms;int preset;bool force,lz;size_t group,prefix;
    in>>kind>>predictor>>preset>>force>>lz>>transforms>>group>>prefix;auto image=TransformReadImage(in,mm);
    if((transforms==1 || transforms==3) && image.channel.size()>=3) {
      jxl::Transform t(jxl::TransformId::kRCT);t.max_delta_error=77;t.rct_type=6;
      if(jxl::FwdRct(image,0,6,nullptr)) image.transform.push_back(t);
    }
    if(transforms==4 || transforms==5) {
      jxl::Transform t(jxl::TransformId::kPalette);t.max_delta_error=77;t.num_c=image.channel.size();t.nb_colors=256;t.nb_deltas=0;t.predictor=jxl::Predictor::Zero;jxl::weighted::Header wh;
      if(jxl::FwdPalette(image,0,t.num_c-1,t.nb_colors,t.nb_deltas,false,false,t.predictor,wh)) image.transform.push_back(t);
    }
    if(transforms==2 || transforms==3 || transforms==5) {
      jxl::Transform t(jxl::TransformId::kSqueeze);t.max_delta_error=77;jxl::DefaultSqueezeParameters(&t.squeezes,image);
      if(jxl::FwdSqueeze(image,t.squeezes,nullptr)) image.transform.push_back(t);
    }
    jxl::ModularOptions options;options.tree_kind=static_cast<jxl::ModularOptions::TreeKind>(kind);options.predictor=static_cast<jxl::Predictor>(predictor);options.wp_mode=preset;
    options.histogram_params.force_huffman=force;options.histogram_params.lz77_method=lz?jxl::HistogramParams::LZ77Method::kRLE:jxl::HistogramParams::LZ77Method::kNone;
    BitWriter writer(mm);BitWriter global_writer(mm);if(!Prefix(writer,prefix)) throw std::runtime_error("modular source prefix");Status status=true;
    if(command=="md-source") status=jxl::ModularGenericCompress(image,options,writer,nullptr,LayerType::ModularGlobal,group);
    else {
      jxl::Tree input_tree;
      if(options.tree_kind==jxl::ModularOptions::TreeKind::kLearn) {
        auto learned=jxl::LearnTree(&image,&options,0,1);if(!learned.ok()) throw std::runtime_error("global learned fixture");input_tree=std::move(learned).value_();
      } else input_tree=jxl::PredefinedTree(options.tree_kind,image.w*image.h*image.channel.size(),image.bitdepth,options.max_properties);
      jxl::Tree tree;std::vector<std::vector<jxl::Token>> tree_tokens(1),tokens(1);status=jxl::TokenizeTree(input_tree,&tree_tokens[0],&tree);
      jxl::EntropyEncodingData tree_code,code;jxl::GroupHeader header;size_t width=0;
      if(status) {auto built=jxl::BuildAndEncodeHistograms(mm,options.histogram_params,jxl::kNumTreeContexts,tree_tokens,&tree_code,&global_writer,LayerType::ModularTree,nullptr);status=built.ok()?Status(true):built.status();}
      if(status) status=jxl::WriteTokens(tree_tokens[0],tree_code,0,&global_writer,LayerType::ModularTree,nullptr);
      if(status) status=jxl::ModularCompress(image,options,group,tree,header,tokens[0],&width);
      auto params=options.histogram_params;params.image_widths.push_back(width);
      if(status) {auto built=jxl::BuildAndEncodeHistograms(mm,params,(tree.size()+1)/2,tokens,&code,&global_writer,LayerType::ModularGlobal,nullptr);status=built.ok()?Status(true):built.status();}
      if(status) status=jxl::Bundle::Write(header,&writer,LayerType::Header,nullptr);
      if(status) status=jxl::WriteTokens(tokens[0],code,0,&writer,LayerType::ModularGlobal,nullptr);
    }
    std::cout<<Code(status)<<' '<<writer.BitsWritten();if(!Pad(writer)) throw std::runtime_error("modular source pad");
    BitReader br(writer.GetSpan());br.SkipBits(prefix);jxl::GroupHeader header;Status read=jxl::Bundle::Read(&br,&header);
    if(!read) throw std::runtime_error("source modular header");std::cout<<' '<<br.TotalBitsConsumed()<<' '<<Hex(writer.GetSpan());if(!br.Close()) throw std::runtime_error("source header close");
    if(command=="md-source-global") {if(!Pad(global_writer)) throw std::runtime_error("global fixture pad");std::cout<<' '<<Hex(global_writer.GetSpan());}
  } else if(command=="md-read") {
    bool generic,header_out,undo,allow,samples;unsigned global_flags;size_t group,max_channel,group_dim,prefix;std::string global_hex,hex;
    in>>generic>>header_out>>undo>>allow>>samples>>group>>max_channel>>group_dim>>global_flags>>prefix>>global_hex>>hex;
    auto image=TransformReadImage(in,mm);jxl::GroupHeader header;MaHeaderSeed(header,3);
    jxl::Tree tree;jxl::ANSCode code;std::vector<uint8_t> map;
    if(global_hex!="-") {
      Input gi(global_hex);BitReader gb(gi.bytes());Status s=jxl::DecodeTree(mm,&gb,&tree,jxl::kMaxTreeSize);
      if(s) s=jxl::DecodeHistograms(mm,&gb,(tree.size()+1)/2,&code,&map);
      if(!s || !gb.Close()) throw std::runtime_error("modular global fixture");
    }
    if(global_flags&8) tree.clear();
    jxl::ModularOptions options;options.max_chan_size=max_channel;options.group_dim=group_dim;Input input(hex);BitReader br(input.bytes());br.SkipBits(prefix);
    Status status=generic?jxl::ModularGenericDecompress(&br,image,header_out?&header:nullptr,group,&options,undo,global_flags&1?&tree:nullptr,global_flags&2?&code:nullptr,global_flags&4?&map:nullptr,allow):
      jxl::ModularDecode(&br,image,header,group,&options,global_flags&1?&tree:nullptr,global_flags&2?&code:nullptr,global_flags&4?&map:nullptr,allow);
    std::cout<<Code(status)<<' '<<br.TotalBitsConsumed()<<' '<<Code(br.Close());MaHeader(header);
    for(auto& t:image.transform) t.max_delta_error=77;
    if(samples) ImageModularObservation(image);else {TransformMetadata(image);std::cout<<' '<<image.transform.size();for(const auto& t:image.transform) ImageTransformObservation(t);}
  } else if(command=="md-dimensions") {
    size_t group_dim;in>>group_dim;auto image=TransformReadImage(in,mm);jxl::ModularOptions options;options.group_dim=group_dim;std::cout<<Code(jxl::ValidateChannelDimensions(image,options));TransformMetadata(image);
  } else return false;
  return true;
}
