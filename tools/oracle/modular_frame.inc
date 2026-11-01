// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Included after the pristine enc_modular.cc in sample_conversion.cc to call
// its actual private helpers. Fixture readers live in the main adapter TU.
jxl::Image TransformReadImage(std::istringstream&, JxlMemoryManager*);
jxl::Transform TransformRead(std::istringstream&);
void TransformMetadata(const jxl::Image&);
jxl::Tree MaReadTree(std::istringstream&);

void MFImage(const jxl::Image& image, bool pixels) {
  TransformMetadata(image);
  std::cout << ' ' << pixels;
  if (pixels) for (const auto& c : image.channel) {
    for (size_t y = 0; y < c.plane.ysize(); ++y)
      for (size_t x = 0; x < c.plane.xsize(); ++x) std::cout << ' ' << c.plane.Row(y)[x];
  }
  std::cout << ' ' << image.transform.size();
  for (const auto& t : image.transform) {
    // max_delta_error has no defined initial source value. Omit it.
    std::cout << ' ' << unsigned(t.id) << ',' << t.begin_c << ',' << t.rct_type
      << ',' << t.num_c << ',' << t.nb_colors << ',' << t.nb_deltas << ','
      << unsigned(t.predictor) << ',' << t.ordered_palette << ',' << t.lossy_palette;
    std::cout << ' ' << t.squeezes.size();
    for (const auto& s : t.squeezes)
      std::cout << ' ' << s.horizontal << ',' << s.in_place << ',' << s.begin_c << ',' << s.num_c;
  }
}

bool ModularFrameCommand(const std::string& command, std::istringstream& in, JxlMemoryManager* mm) {
  if (command == "mf-cost") {
    bool wp;
    size_t preset;
    in >> wp >> preset;
    auto image = TransformReadImage(in, mm);
    jxl::StatusOr<float> cost = wp ? jxl::EstimateWPCost(image, preset) : jxl::EstimateCost(image);
    std::cout << static_cast<int>(cost.status().code());
    if (cost.ok()) std::cout << ' ' << SCBits<uint32_t>(std::move(cost).value_());
    MFImage(image, true);
  } else if (command == "mf-quant") {
    int q;
    size_t cw, ch;
    in >> q >> cw >> ch;
    auto image = TransformReadImage(in, mm);
    image.channel[0].w = cw;
    image.channel[0].h = ch;
    jxl::QuantizeChannel(image.channel[0], q);
    std::cout << 'Q';
    MFImage(image, true);
  } else if (command == "mf-transform") {
    bool maybe, force, ordered, lossy;
    int tier, preset;
    uint32_t cost;
    in >> maybe >> force >> tier >> preset >> cost >> ordered >> lossy;
    auto tr = TransformRead(in);
    tr.ordered_palette = ordered;
    tr.lossy_palette = lossy;
    auto image = TransformReadImage(in, mm);
    jxl::CompressParams params;
    params.speed_tier = static_cast<jxl::SpeedTier>(tier);
    jxl::weighted::Header header;
    jxl::weighted::PredictorMode(preset, &header);
    auto result = maybe ? jxl::maybe_do_transform(image, tr, params, header, SCBits<float>(cost), nullptr, force)
                        : jxl::StatusOr<bool>(jxl::do_transform(image, tr, header, nullptr, force));
    std::cout << static_cast<int>(result.status().code());
    if (result.ok()) std::cout << ' ' << std::move(result).value_();
    MFImage(image, !force);
  } else if (command == "mf-palettes") {
    int max_depth, max_value, tier, colors;
    bool lossy, zero;
    uint32_t percent;
    in >> max_depth >> max_value >> tier >> colors >> lossy >> zero >> percent;
    auto image = TransformReadImage(in, mm);
    jxl::CompressParams params;
    params.speed_tier = static_cast<jxl::SpeedTier>(tier);
    params.palette_colors = colors;
    params.lossy_palette = lossy;
    params.options.zero_tokens = zero;
    auto status = jxl::try_palettes(image, max_depth, max_value, params, SCBits<float>(percent), nullptr);
    std::cout << static_cast<int>(status.code()) << ' ' << max_depth << ' ' << max_value;
    MFImage(image, !zero);
  } else if (command == "mf-merge") {
    size_t begin, end, n, ns;
    in >> begin >> end >> n;
    std::vector<jxl::Tree> trees(n);
    for (auto& tree : trees) tree = MaReadTree(in);
    in >> ns;
    std::vector<size_t> splits(ns);
    for (auto& s : splits) in >> s;
    auto tree = MaReadTree(in);
    auto status = jxl::MergeTrees(trees, splits, begin, end, &tree);
    std::cout << static_cast<int>(status.code()) << ' ' << tree.size();
    for (const auto& node : tree)
      std::cout << ' ' << node.property << ',' << node.splitval << ',' << node.lchild << ',' << node.rchild
        << ',' << unsigned(node.predictor) << ',' << node.predictor_offset << ',' << node.multiplier;
  } else return false;
  return true;
}
