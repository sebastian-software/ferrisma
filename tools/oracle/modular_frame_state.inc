// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Compile this fixture TU with -fno-access-control. The original class,
// methods, layouts, and algorithms are unchanged.
jxl::CompressParams EFReadParams(std::istringstream&);
void EFParams(const jxl::CompressParams&);
void EFHistogram(const jxl::HistogramParams&);
void DimensionsObservation(const jxl::FrameDimensions&);
void MaTree(const jxl::Tree&);
void MaTokens(const std::vector<jxl::Token>&);
void PredictHeaderObservation(const jxl::weighted::Header&);
std::string Hex(jxl::Bytes);
jxl::Status Pad(jxl::BitWriter&);
jxl::Status Prefix(jxl::BitWriter&, size_t);
jxl::Status EFParamsPostInit(jxl::CompressParams*);
jxl::Status EFPrepareLosslessThunder(const jxl::CompressParams&,
                                    jxl::ProgressiveSplitter*, jxl::FrameHeader*);

template <class T> std::vector<T> MFSReadValues(std::istringstream& in) {
  size_t n; in >> n;
  std::vector<T> values(n); for (auto& v : values) in >> v;
  return values;
}
jxl::ModularStreamId MFSReadId(std::istringstream& in) {
  unsigned kind; size_t group, pass; in >> kind >> group >> pass;
  switch (kind) {
    case 0: return jxl::ModularStreamId::Global();
    case 1: return jxl::ModularStreamId::VarDCTDC(group);
    case 2: return jxl::ModularStreamId::ModularDC(group);
    case 3: return jxl::ModularStreamId::ACMetadata(group);
    case 4: {
      auto qt = jxl::ModularStreamId::QuantTable(group);
      if (!qt.ok()) throw std::runtime_error("frame fixture quant table ID");
      return std::move(qt).value_();
    }
    case 5: return jxl::ModularStreamId::ModularAC(group,pass);
    default: throw std::runtime_error("frame fixture stream ID");
  }
}
void MFSOutput(const jxl::BitWriter& writer, const jxl::AuxOut& aux, JxlMemoryManager* mm) {
  jxl::BitWriter copy(mm);
  if (!copy.AppendUnaligned(writer) || !Pad(copy)) throw std::runtime_error("frame fixture writer copy");
  std::cout << " W " << writer.BitsWritten() << ' ' << Hex(copy.GetSpan());
  for (unsigned i=0;i<15;++i) {
    const auto a=aux.layer(static_cast<jxl::LayerType>(i));
    std::cout << ' ' << a.total_bits << ',' << a.histogram_bits << ',' << a.extra_bits << ','
      << a.num_clustered_histograms << ',' << SCBits<uint64_t>(a.clustered_entropy);
  }
}

template <class T> void MFSValues(const std::vector<T>& values) {
  std::cout << ' ' << values.size();
  for (const auto& v : values) std::cout << ' ' << +v;
}
void MFSOptions(const jxl::ModularOptions& o) {
  std::cout << ' ' << o.max_chan_size << ' ' << o.group_dim << ' ' << SCBits<uint32_t>(o.nb_repeats)
    << ' ' << o.max_properties;
  MFSValues(o.splitting_heuristics_properties);
  std::cout << ' ' << SCBits<uint32_t>(o.splitting_heuristics_node_threshold) << ' ' << o.max_property_values
    << ' ' << unsigned(o.predictor) << ' ' << o.wp_mode << ' ' << SCBits<uint32_t>(o.fast_decode_multiplier)
    << ' ' << int(o.wp_tree_mode) << ' ' << o.skip_encoder_fast_path << ' ' << unsigned(o.tree_kind) << ' ' << o.zero_tokens;
  EFHistogram(o.histogram_params);
}
jxl::CompressParams MFSReadParams(std::istringstream& in) {
  auto p = EFReadParams(in);
  uint32_t repeats, fast, pre, percent;
  int mode; unsigned kind;
  in >> p.colorspace >> p.palette_colors >> pre >> percent >> repeats >> p.options.max_properties
     >> p.options.wp_mode >> mode >> kind >> fast >> p.options.zero_tokens;
  p.channel_colors_pre_transform_percent = SCBits<float>(pre);
  p.channel_colors_percent = SCBits<float>(percent);
  p.options.nb_repeats = SCBits<float>(repeats);
  p.options.fast_decode_multiplier = SCBits<float>(fast);
  p.options.wp_tree_mode = static_cast<jxl::ModularOptions::TreeMode>(mode);
  p.options.tree_kind = static_cast<jxl::ModularOptions::TreeKind>(kind);
  p.custom_fixed_tree = MaReadTree(in);
  return p;
}
void MFSHeaderInput(std::istringstream& in, jxl::CodecMetadata& metadata, jxl::FrameHeader& header) {
  size_t w, h, extras;
  bool gray, modular;
  in >> w >> h >> header.group_size_shift >> header.passes.num_passes >> gray >> extras >> modular;
  if (!metadata.size.Set(w,h)) throw std::runtime_error("frame state metadata size");
  metadata.m.num_extra_channels = extras;
  metadata.m.extra_channel_info.resize(extras);
  if (!metadata.m.color_encoding.SetSRGB(gray ? jxl::ColorSpace::kGray : jxl::ColorSpace::kRGB,
                                       jxl::RenderingIntent::kRelative)) throw std::runtime_error("frame state color");
  header.encoding = modular ? jxl::FrameEncoding::kModular : jxl::FrameEncoding::kVarDCT;
}
void MFSState(const jxl::ModularFrameEncoder& e) {
  std::cout << " E";
  DimensionsObservation(e.frame_dim_);
  EFParams(e.cparams_);
  MaTree(e.cparams_.custom_fixed_tree);
  MFSValues(e.ac_metadata_size); MFSValues(e.extra_dc_precision);
  std::cout << ' ' << e.stream_options_.size();
  for (const auto& o : e.stream_options_) MFSOptions(o);
  std::cout << ' ' << e.stream_images_.size();
  for (const auto& image : e.stream_images_) MFImage(image,true);
  MFSValues(e.quants_); MaTree(e.tree_);
  std::cout << ' ' << e.tree_tokens_.size(); for (const auto& t : e.tree_tokens_) MaTokens(t);
  std::cout << ' ' << e.stream_headers_.size();
  for (const auto& h : e.stream_headers_) {
    std::cout << ' ' << h.use_global_tree;
    PredictHeaderObservation(h.wp_header);
    // Reuse the transform-only observation without reading source scratch.
    jxl::Image view(e.memory_manager()); view.transform = h.transforms;
    MFImage(view,false);
  }
  std::cout << ' ' << e.tokens_.size(); for (const auto& t : e.tokens_) MaTokens(t);
  MFSValues(e.context_map_); MFSValues(e.tree_splits_);
  std::cout << ' ' << e.gi_channel_.size(); for (const auto& c : e.gi_channel_) MFSValues(c);
  MFSValues(e.image_widths_);
  std::cout << ' ' << e.stream_params_.size();
  for (const auto& group : e.stream_params_)
    std::cout << ' ' << group.rect.x0() << ',' << group.rect.y0() << ',' << group.rect.xsize() << ',' << group.rect.ysize()
      << ',' << group.minShift << ',' << group.maxShift << ',' << group.id.ID(e.frame_dim_);
  // The source code scalars are indeterminate before histogram generation.
  std::cout << ' ' << e.code_.encoding_info.size() << ' ' << e.code_.uint_config.size()
    << ' ' << e.code_.encoded_histograms.size();
  MFSValues(e.code_.context_map);
}
bool ModularFrameStateCommand(const std::string& command, std::istringstream& in, JxlMemoryManager* mm) {
  if (command == "mfs-thunder") {
    size_t w, h, cw, ch; unsigned depth; bool gray;
    in >> w >> h >> gray >> depth >> cw >> ch;
    jxl::CodecMetadata metadata;
    auto status = metadata.size.Set(w,h);
    if (!status) { std::cout << "S " << static_cast<int>(status.code()); return true; }
    metadata.m.SetUintSamples(depth);
    metadata.m.xyb_encoded = false;
    if (!metadata.m.color_encoding.SetSRGB(gray ? jxl::ColorSpace::kGray : jxl::ColorSpace::kRGB,
                                         jxl::RenderingIntent::kRelative))
      throw std::runtime_error("Thunder fixture color");
    jxl::CompressParams p;
    p.cms={}; p.debug_image_opaque=nullptr;
    p.speed_tier=jxl::SpeedTier::kThunder; p.level=10; p.buffering=0;
    p.SetLossless();
    if (!EFParamsPostInit(&p)) throw std::runtime_error("Thunder fixture parameters");
    jxl::PassesEncoderState enc_state(mm);
    jxl::FrameHeader header(&metadata);
    if (!EFPrepareLosslessThunder(p,&enc_state.progressive_splitter,&header))
      throw std::runtime_error("Thunder fixture header");
    enc_state.shared.frame_dim=header.ToFrameDimensions();
    auto created=jxl::ModularFrameEncoder::Create(mm,header,p,false);
    if (!created.ok()) throw std::runtime_error("Thunder fixture frame allocation");
    auto e=std::move(created).value_();
    auto image=jxl::Image3F::Create(mm,cw,ch);
    if (!image.ok()) throw std::runtime_error("Thunder fixture image allocation");
    auto color=std::move(image).value_();
    for (size_t c=0;c<3;++c) for(size_t y=0;y<ch;++y) for(size_t x=0;x<cw;++x) {
      uint32_t bits; in>>bits; color.PlaneRow(c,y)[x]=SCBits<float>(bits);
    }
    JxlCmsInterface cms={};
    const jxl::Rect group_rect(0,0,cw,ch), frame_area_rect(0,0,w,h);
    status=e->ComputeEncodingData(header,metadata.m,&color,{},group_rect,
                                 enc_state.shared.frame_dim,frame_area_rect,
                                 &enc_state,cms,nullptr,nullptr,true);
    std::cout<<static_cast<int>(status.code());
    MFSState(*e);
    // The source path does not mutate Image3F in this configuration.
    for(size_t c=0;c<3;++c) for(size_t y=0;y<ch;++y) for(size_t x=0;x<cw;++x)
      std::cout<<' '<<SCBits<uint32_t>(color.PlaneRow(c,y)[x]);
    return true;
  }
  if (command != "mfs-init" && command != "mfs-chain") return false;
  bool streaming;
  in >> streaming;
  auto p = MFSReadParams(in);
  jxl::CodecMetadata metadata;
  jxl::FrameHeader header(&metadata);
  MFSHeaderInput(in,metadata,header);
  auto created = jxl::ModularFrameEncoder::Create(mm,header,p,streaming);
  std::cout << static_cast<int>(created.status().code());
  if (created.ok()) {
    auto encoder = std::move(created).value_(); auto& e=*encoder;
    if (command == "mfs-chain") {
      e.stream_images_[0] = TransformReadImage(in,mm);
      e.gi_channel_.resize(e.stream_images_.size());
      e.quants_ = MFSReadValues<uint32_t>(in);
      size_t prefix, actions; in >> prefix >> actions;
      jxl::BitWriter writer(mm); jxl::AuxOut aux;
      if (!Prefix(writer,prefix)) throw std::runtime_error("frame fixture prefix");
      MFSState(e); MFSOutput(writer,aux,mm);
      for (size_t action=0;action<actions;++action) {
        unsigned op; in >> op;
        jxl::Status status=true;
        std::cout << " O " << op;
        if (op==0) {
          auto id=MFSReadId(in);
          size_t x,y,w,h; int min,max; bool color,groupwise,record;
          in >> x >> y >> w >> h >> min >> max >> color >> groupwise >> record;
          jxl::Rect rect(x,y,w,h);
          if (record) e.stream_params_.push_back({rect,min,max,id});
          // The original ComputeEncodingData caller seeds group options from
          // the global options before invoking the private preparation method.
          if (id.ID(e.frame_dim_) != 0) e.stream_options_[id.ID(e.frame_dim_)] = e.stream_options_[0];
          status=e.PrepareStreamParams(rect,e.cparams_,min,max,id,color,groupwise);
        } else if (op==1) status=e.ComputeTree(nullptr);
        else if (op==2) status=e.ComputeTokens(nullptr);
        else if (op==3) {
          bool stream,use_aux; in >> stream >> use_aux;
          status=e.EncodeGlobalInfo(stream,&writer,use_aux?&aux:nullptr);
        } else if (op==4) {
          auto id=MFSReadId(in); unsigned layer; bool use_aux; in >> layer >> use_aux;
          status=e.EncodeStream(&writer,use_aux?&aux:nullptr,static_cast<jxl::LayerType>(layer),id);
        } else if (op==5) {auto id=MFSReadId(in); e.ClearStreamData(id);}
        else if (op==6) e.ClearModularStreamData();
        else if (op==7) {
          size_t dc,ac,x,y; in >> dc >> ac >> x >> y;
          jxl::FrameDimensions patch; patch.xsize_groups=x; patch.ysize_groups=y;
          std::cout << " I " << e.ComputeStreamingAbsoluteAcGroupId(dc,ac,patch);
        } else if (op==8) {
          auto id=MFSReadId(in); unsigned kind,predictor; uint32_t repeats; int props; size_t max;
          in >> kind >> predictor >> repeats >> props >> max;
          auto& o=e.stream_options_[id.ID(e.frame_dim_)];
          o.tree_kind=static_cast<jxl::ModularOptions::TreeKind>(kind);o.predictor=static_cast<jxl::Predictor>(predictor);
          o.nb_repeats=SCBits<float>(repeats);o.max_properties=props;o.max_chan_size=max;
        } else if (op==9) {
          auto id=MFSReadId(in);bool error;in >> error;e.stream_images_[id.ID(e.frame_dim_)].error=error;
        } else if (op==10) {
          auto id=MFSReadId(in);e.stream_images_[id.ID(e.frame_dim_)]=TransformReadImage(in,mm);
        } else throw std::runtime_error("unknown frame fixture action");
        std::cout << ' ' << static_cast<int>(status.code());
        MFSState(e); MFSOutput(writer,aux,mm);
      }
    } else MFSState(e);
  }
  std::cout << " P"; EFParams(p);
  return true;
}
