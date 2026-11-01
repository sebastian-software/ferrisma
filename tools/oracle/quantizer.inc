// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
void QZParams(const jxl::QuantizerParams& params) {
  std::cout << " P " << params.global_scale << ' ' << params.quant_dc;
}
bool QuantizerCommand(const std::string& command, std::istringstream& in,
                      JxlMemoryManager* mm) {
  if (command == "qz-constants") {
    std::cout << jxl::kGlobalScaleDenom << ' ' << jxl::kGlobalScaleNumerator
              << ' ' << jxl::Quantizer::kQuantMax << ' ' << QZBits<uint32_t>(jxl::kBiasNumerator);
    for (float value : jxl::kZeroBiasDefault) std::cout << ' ' << QZBits<uint32_t>(value);
    for (float value : jxl::kDefaultQuantBias) std::cout << ' ' << QZBits<uint32_t>(value);
    return true;
  }
  if (command == "qz-clamp") {
    size_t count;
    in >> count;
    std::cout << "C";
    for (size_t i = 0; i < count; ++i) std::cout << ' ' << jxl::Quantizer::ClampVal(QZFloat(in));
    return true;
  }
  if (command == "qz-matrix") {
    unsigned strategy;
    in >> strategy;
    jxl::DequantMatrices matrices;
    jxl::Quantizer quantizer(matrices);
    const auto status = matrices.EnsureComputed(mm, 1u << strategy);
    std::cout << int(status.code());
    if (status) {
      size_t kind = size_t(jxl::kAcStrategyToQuantTableMap[strategy]);
      size_t size = jxl::DequantMatrices::required_size_x[kind] *
                    jxl::DequantMatrices::required_size_y[kind] * 64;
      for (size_t c = 0; c < 3; ++c) {
        auto* matrix = quantizer.DequantMatrix(jxl::AcStrategyType(strategy), c);
        auto* inverse = quantizer.InvDequantMatrix(jxl::AcStrategyType(strategy), c);
        std::cout << ' ' << (matrix == matrices.Matrix(jxl::AcStrategyType(strategy), c))
                  << ' ' << (inverse == matrices.InvMatrix(jxl::AcStrategyType(strategy), c)) << ' ' << size;
        for (size_t i = 0; i < size; ++i) std::cout << ' ' << QZBits<uint32_t>(matrix[i]) << ',' << QZBits<uint32_t>(inverse[i]);
      }
    }
    return true;
  }
  if (command == "qz-params") {
    jxl::QuantizerParams params;
    size_t prefix, count;
    in >> prefix >> count;
    jxl::BitWriter writer(mm);
    jxl::AuxOut aux;
    if (!Prefix(writer, prefix)) throw std::runtime_error("quantizer fixture prefix");
    QZParams(params);
    MFSOutput(writer, aux, mm);
    for (size_t i = 0; i < count; ++i) {
      char action;
      in >> action;
      jxl::Status status = true;
      if (action == 'S') {
        in >> params.global_scale >> params.quant_dc;
      } else if (action == 'I') {
        jxl::Bundle::Init(&params);
      } else if (action == 'W') {
        unsigned layer;
        in >> layer;
        size_t extension = 0, total = 0;
        auto can = jxl::Bundle::CanEncode(params, &extension, &total);
        std::cout << " C " << int(can.code()) << ' ' << extension << ' ' << total;
        status = jxl::WriteQuantizerParams(params, &writer, jxl::LayerType(layer), &aux);
      } else if (action == 'D') {
        size_t offset;
        std::string hex;
        in >> offset >> hex;
        QZInput input(hex);
        jxl::BitReader reader(input.bytes());
        reader.SkipBits(offset);
        status = jxl::Bundle::Read(&reader, &params);
        std::cout << " R " << reader.TotalBitsConsumed() << ' ' << int(reader.Close().code());
      } else {
        throw std::runtime_error("quantizer parameter fixture action");
      }
      std::cout << " A " << int(status.code());
      QZParams(params);
      MFSOutput(writer, aux, mm);
    }
    return true;
  }
  if (command != "qz-chain") return false;
  int dc, global;
  size_t prefix, count;
  in >> dc >> global;
  jxl::DequantMatrices matrices;
  float initial[3];
  for (float& value : initial) value = QZFloat(in);
  matrices.SetDCQuant(initial);
  jxl::Quantizer quantizer(matrices, dc, global);
  in >> prefix >> count;
  jxl::BitWriter writer(mm);
  jxl::AuxOut aux;
  if (!Prefix(writer, prefix)) throw std::runtime_error("quantizer fixture prefix");
  size_t slots = 3;
  SSQuant(quantizer, matrices, slots);
  QZParams(quantizer.GetParams());
  MFSOutput(writer, aux, mm);
  for (size_t i = 0; i < count; ++i) {
    char action;
    in >> action;
    jxl::Status status = true;
    if (action == 'D' || action == 'R') {
      size_t offset;
      in >> offset;
      std::string hex;
      if (action == 'D') {
        in >> hex;
      } else {
        jxl::BitWriter copy(mm);
        if (!copy.AppendUnaligned(writer) || !Pad(copy)) throw std::runtime_error("quantizer fixture reader copy");
        hex = Hex(copy.GetSpan());
      }
      QZInput input(hex);
      jxl::BitReader reader(input.bytes());
      reader.SkipBits(offset);
      status = quantizer.Decode(&reader);
      std::cout << " R " << reader.TotalBitsConsumed() << ' ' << int(reader.Close().code());
    } else if (action == 'E') {
      unsigned layer;
      in >> layer;
      status = jxl::WriteQuantizerParams(quantizer.GetParams(), &writer, jxl::LayerType(layer), &aux);
    } else if (action == 'G') {
      std::cout << " G " << QZBits<uint32_t>(quantizer.ScaleGlobalScale(QZFloat(in)));
    } else if (action == 'S') {
      float values[3];
      for (float& value : values) value = QZFloat(in);
      matrices.SetDCQuant(values);
    } else if (action == 'C') {
      quantizer.ClearDCMul();
      slots = 4;
    } else {
      throw std::runtime_error("quantizer fixture action");
    }
    std::cout << " A " << int(status.code());
    SSQuant(quantizer, matrices, slots);
    QZParams(quantizer.GetParams());
    MFSOutput(writer, aux, mm);
  }
  return true;
}
