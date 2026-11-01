// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
namespace {
template<class T> void BCValues(const T& values) {
  std::cout << ' ' << values.size();
  for (const auto value : values) std::cout << ' ' << +value;
}
jxl::BlockCtxMap BCReadMap(std::istringstream& in) {
  jxl::BlockCtxMap m; size_t n;
  for (auto& channel : m.dc_thresholds) {
    in >> n; channel.resize(n); for (auto& v : channel) in >> v;
  }
  in >> n; m.qf_thresholds.resize(n); for (auto& v : m.qf_thresholds) in >> v;
  in >> n; m.ctx_map.resize(n);
  for (auto& v : m.ctx_map) { unsigned u; in >> u; v = u; }
  in >> m.num_ctxs >> m.num_dc_ctxs;
  return m;
}
void BCMap(const jxl::BlockCtxMap& m) {
  std::cout << " M " << m.num_ctxs << ' ' << m.num_dc_ctxs;
  for (const auto& v : m.dc_thresholds) BCValues(v);
  BCValues(m.qf_thresholds); BCValues(m.ctx_map);
}
std::vector<uint8_t> BCUnhex(const std::string& hex) {
  std::vector<uint8_t> result;
  if (hex != "-") for (size_t i=0;i<hex.size();i+=2)
    result.push_back(std::stoul(hex.substr(i,2),nullptr,16));
  return result;
}
void BCHex(jxl::Span<const uint8_t> bytes) {
  static constexpr char digits[]="0123456789abcdef";
  if (bytes.empty()) { std::cout << '-'; return; }
  for (const auto b : bytes) std::cout << digits[b>>4] << digits[b&15];
}
void BCDecode(jxl::BlockCtxMap& m, const std::vector<uint8_t>& bytes,
              size_t offset, JxlMemoryManager* mm) {
  // Preserve a non-null logical span and valid end-minus-eight pointer even
  // for empty/truncated inputs. No reader/decoder algorithm is replaced.
  std::vector<uint8_t> backing(bytes.size()+16,0);
  std::copy(bytes.begin(),bytes.end(),backing.begin()+8);
  jxl::BitReader br(jxl::Bytes(backing.data()+8,bytes.size())); br.SkipBits(offset);
  auto status=jxl::DecodeBlockCtxMap(mm,&br,&m);
  std::cout << " R " << static_cast<int>(status.code()) << ' ' << br.TotalBitsConsumed()
            << ' ' << static_cast<int>(br.Close().code());
  BCMap(m);
}
void BCAux(const jxl::AuxOut& aux) {
  std::cout << " A";
  for (const auto& layer : aux.layers) {
    uint64_t bits; std::memcpy(&bits,&layer.clustered_entropy,sizeof(bits));
    std::cout << ' ' << layer.num_clustered_histograms << ' ' << layer.extra_bits
      << ' ' << layer.histogram_bits << ' ' << layer.total_bits << ' ' << bits;
  }
}
}
bool BlockContextCommand(const std::string& command, std::istringstream& in,
                         JxlMemoryManager* mm) {
  if (command=="bc-tables") {
    std::cout << jxl::kDCTOrderContextStart << ' ' << jxl::kNonZeroBuckets << ' '
      << jxl::kZeroDensityContextCount << ' ' << jxl::kZeroDensityContextLimit;
    for (auto v:jxl::kCoeffFreqContext) std::cout << ' ' << v;
    for (auto v:jxl::kCoeffNumNonzeroContext) std::cout << ' ' << v;
    for (auto d:{jxl::kDCThresholdDist,jxl::kQFThresholdDist})
      for (unsigned i=0;i<4;++i) std::cout << ' ' << d.GetDistr(i).d;
    return true;
  }
  if (command=="bc-zero") {
    size_t nz,k,covered,log,prev; in>>nz>>k>>covered>>log>>prev;
    std::cout << jxl::ZeroDensityContext(nz,k,covered,log,prev); return true;
  }
  if (command=="bc-buckets") {
    jxl::BlockCtxMap m; uint32_t nz,block; in>>m.num_ctxs>>nz>>block;
    std::cout << m.NonZeroContext(nz,block) << ' ' << m.ZeroDensityContextsOffset(block)
              << ' ' << m.NumACContexts(); return true;
  }
  if (command=="bc-lookup") {
    auto m=BCReadMap(in); int dc; uint32_t qf; size_t ord,c; in>>dc>>qf>>ord>>c;
    std::cout << m.Context(dc,qf,ord,c); return true;
  }
  if (command=="bc-predict") {
    size_t x; unsigned top; int32_t def,t,l; in>>x>>top>>def>>t>>l;
    std::vector<int32_t> row(x+1,l),above(x+1,t);
    // Fixtures keep both reached signed additions representable.
    std::cout << jxl::PredictFromTopAndLeft(top?above.data():nullptr,row.data(),x,def);
    return true;
  }
  if (command=="bc-read") {
    auto m=BCReadMap(in); size_t n; in>>n;
    for(size_t i=0;i<n;++i) {std::string hex;size_t offset;in>>hex>>offset;BCDecode(m,BCUnhex(hex),offset,mm);}
    return true;
  }
  if (command=="bc-encode") {
    auto m=BCReadMap(in); size_t offset,limit;in>>offset>>limit;
    jxl::BitWriter writer(mm); jxl::AuxOut aux;
    for(size_t i=0;i<aux.layers.size();++i) {
      auto& l=aux.layers[i];l.num_clustered_histograms=7+i;l.extra_bits=11+i;
      l.histogram_bits=13+i;l.total_bits=17+i;l.clustered_entropy=.75;
    }
    if(offset && !writer.WithMaxBits(offset,jxl::LayerType::Header,nullptr,[&] {
      writer.Write(offset,(1ULL<<offset)-1);return true;
    })) throw std::runtime_error("block context prefix allocation");
    auto status=jxl::EncodeBlockCtxMap(m,&writer,&aux);
    std::cout << "W " << static_cast<int>(status.code()) << ' ' << writer.BitsWritten();
    if (!writer.WithMaxBits(7,jxl::LayerType::Header,nullptr,[&] {writer.ZeroPadToByte();return true;})) throw std::runtime_error("block context pad allocation");
    std::cout << ' '; BCHex(writer.GetSpan()); BCAux(aux);
    auto span=writer.GetSpan();std::vector<uint8_t> bytes(span.data(),span.data()+std::min(span.size(),limit));
    auto decoded=BCReadMap(in); BCDecode(decoded,bytes,offset,mm);
    return true;
  }
  return false;
}
