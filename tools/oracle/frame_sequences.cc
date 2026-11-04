// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Fixture orchestration calls actual pinned encoder/decoder entry points.
// Section order and close checkpoints follow dec_frame.cc::DecodeFrame; preview
// selection uses the production FrameDecoder::InitFrame argument directly. The
// incremental driver follows decode.cc around the same production FrameDecoder.
#include <jxl/cms.h>
#include <array>
#include <cstdint>
#include <cstring>
#include <deque>
#include <iostream>
#include <memory>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include "lib/jxl/dec_frame.h"
#include "lib/jxl/enc_fields.h"
#include "lib/jxl/enc_icc_codec.h"
#include "lib/jxl/enc_frame.h"
#include "lib/jxl/enc_toc.h"
#include "lib/jxl/toc.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/icc_codec.h"
#include "lib/jxl/image_ops.h"
#include "frame_sequence_writes.h"
std::string Hex(jxl::Bytes);
std::vector<uint8_t> Unhex(const std::string&);
void MetadataObservation(const jxl::ImageMetadata&);
void ColorBundleObservation(const jxl::ColorEncoding&);
namespace {
void Image(const jxl::Image3F& image,bool pixels) {
 std::cout<<" I "<<image.xsize()<<' '<<image.ysize();
 if(pixels)for(size_t c=0;c<3;++c)for(size_t y=0;y<image.ysize();++y)for(size_t x=0;x<image.xsize();++x){uint32_t word;auto value=image.PlaneRow(c,y)[x];std::memcpy(&word,&value,4);std::cout<<' '<<word;}
}
void Bundle(const jxl::ImageBundle& image,bool pixels) {
 std::cout<<" O "<<image.origin.x0<<' '<<image.origin.y0<<' '<<image.duration<<' '<<image.timecode<<' '<<image.use_for_next_frame<<' '<<image.blend<<' '<<unsigned(image.blendmode)<<' '<<Hex(jxl::Bytes(reinterpret_cast<const uint8_t*>(image.name.data()),image.name.size()))<<' '<<image.decoded_bytes()<<' '<<(image.metadata()!=nullptr)<<' '<<unsigned(image.color_transform);
 Image(image.color(),pixels);ColorBundleObservation(image.c_current());std::cout<<" X "<<image.extra_channels().size();
 for(const auto& ec:image.extra_channels()){std::cout<<' '<<ec.xsize()<<' '<<ec.ysize();if(pixels)for(size_t y=0;y<ec.ysize();++y)for(size_t x=0;x<ec.xsize();++x){uint32_t word;auto value=ec.Row(y)[x];std::memcpy(&word,&value,4);std::cout<<' '<<word;}}
}
void CoveredBundle(const jxl::ImageBundle& image,const FrameSequenceBundleCoverage& coverage) {
 Bundle(image,false);std::cout<<" L "<<coverage.planes.size();
 for(size_t c=0;c<coverage.planes.size();++c){const auto& plane=coverage.planes[c];size_t count=0;for(bool ready:plane.initialized)count+=ready;
  std::cout<<' '<<c<<' '<<count;for(size_t y=0;y<plane.height;++y)for(size_t x=0;x<plane.width;++x)if(plane.initialized[y*plane.width+x]){
   float sample=c<3?image.color().PlaneRow(c,y)[x]:image.extra_channels()[c-3].Row(y)[x];uint32_t word;std::memcpy(&word,&sample,4);std::cout<<' '<<x<<' '<<y<<' '<<word;
  }
 }
}

void State(const jxl::PassesDecoderState& state,bool pixels,const std::array<const jxl::ImageBundle*,4>& identities,const FrameSequenceWriteObservation& writes) {
 const auto& pd=state.shared_storage.image_features.patches;std::cout<<" PD "<<pd.HasAny()<<' '<<pd.GetReferences()<<' '<<pd.positions_.size()<<' '<<pd.ref_positions_.size();for(const auto& p:pd.ref_positions_)std::cout<<" PR "<<p.ref<<' '<<p.x0<<' '<<p.y0<<' '<<p.xsize<<' '<<p.ysize;
 unsigned reference_mask=0,dc_mask=0;for(size_t i=0;i<4;++i){if(state.shared_storage.reference_frames[i].frame->HasColor())reference_mask|=1u<<i;if(state.shared_storage.dc_frames[i].xsize())dc_mask|=16u<<i;}
 std::cout<<" Q "<<state.visible_frame_index<<' '<<state.nonvisible_frame_index<<' '<<reference_mask<<' '<<dc_mask;
 for(size_t i=0;i<4;++i){const auto& reference=state.shared_storage.reference_frames[i];std::cout<<" R "<<i<<' '<<(reference.frame.get()==identities[i])<<' '<<reference.ib_is_in_xyb;Bundle(*reference.frame,reference.frame->HasColor());}
 std::cout<<" T "<<writes.bundle_coverage[1].moved_color_slot;CoveredBundle(state.frame_storage_for_referencing,writes.bundle_coverage[1]);
 for(size_t i=0;i<4;++i){std::cout<<" C "<<i;const auto& dc=state.shared_storage.dc_frames[i];Image(dc,false);const auto& coverage=writes.coverage[i];size_t count=0;for(bool ready:coverage.initialized)count+=ready;std::cout<<" V "<<count;for(size_t y=0;y<coverage.height;++y)for(size_t x=0;x<coverage.width;++x)if(coverage.initialized[y*coverage.width+x]){std::cout<<' '<<x<<' '<<y;for(size_t c=0;c<3;++c){uint32_t word;float sample=dc.PlaneRow(c,y)[x];std::memcpy(&word,&sample,4);std::cout<<' '<<word;}}}
}
jxl::Status Header(jxl::BitReader& reader,jxl::CodecMetadata& metadata,JxlMemoryManager* mm) {
 JXL_ENSURE(reader.ReadFixedBits<16>()==0x0aff);
 JXL_RETURN_IF_ERROR(jxl::ReadSizeHeader(&reader,&metadata.size));
 JXL_RETURN_IF_ERROR(jxl::ReadImageMetadata(&reader,&metadata.m));
 metadata.transform_data.nonserialized_xyb_encoded=metadata.m.xyb_encoded;
 JXL_RETURN_IF_ERROR(jxl::Bundle::Read(&reader,&metadata.transform_data));
 if(metadata.m.color_encoding.WantICC()){jxl::ICCReader icc(mm);jxl::PaddedBytes bytes(mm);JXL_RETURN_IF_ERROR(icc.Init(&reader));JXL_RETURN_IF_ERROR(icc.Process(&reader,&bytes));std::vector<uint8_t> profile(bytes.begin(),bytes.end());JXL_RETURN_IF_ERROR(metadata.m.color_encoding.SetICC(std::move(profile),JxlGetDefaultCms()));}
 JXL_RETURN_IF_ERROR(reader.JumpToByteBoundary());return reader.AllReadsWithinBounds();
}
void Decode(const std::vector<uint8_t>& bytes,JxlMemoryManager* mm) {
 std::vector<uint8_t> backing(bytes.size()+16);std::copy(bytes.begin(),bytes.end(),backing.begin()+8);const auto* data=backing.data()+8;
 jxl::CodecMetadata metadata;jxl::BitReader reader(jxl::Bytes(data,bytes.size()));auto status=Header(reader,metadata,mm);size_t offset=reader.TotalBitsConsumed()/8;auto close=reader.Close();std::cout<<" H "<<int(status.code())<<' '<<reader.TotalBitsConsumed()<<' '<<int(close.code());if(!status||!close)return;
 std::cout<<" M "<<metadata.size.xsize()<<' '<<metadata.size.ysize();MetadataObservation(metadata.m);
 jxl::PassesDecoderState state(mm);status=state.output_encoding_info.SetFromMetadata(metadata);std::cout<<" P "<<int(status.code());if(!status)return;
 std::array<const jxl::ImageBundle*,4> identities;for(size_t i=0;i<4;++i)identities[i]=state.shared_storage.reference_frames[i].frame.get();
 jxl::ImageBundle output(mm,&metadata.m);
 FrameSequenceWriteObservation writes(state.shared_storage,output,state.frame_storage_for_referencing);bool preview=metadata.m.have_preview,last_main=false;size_t frames=0;
 while(status&&!last_main&&frames<64){
  jxl::FrameDecoder frame(&state,metadata,nullptr,true);
  jxl::BitReader header(jxl::Bytes(data+offset,bytes.size()-offset));
  status=frame.InitFrame(&header,&output,preview);
  writes.AfterInitFrame();
  const auto init_status=status;
  bool output_attempted=bool(status);
  jxl::Status output_status=true;
  // DecodeFrame initializes output before the deferred header bounds check.
  // Preserve its partial decoder state even when that later check fails.
  if(status){output_status=frame.InitFrameOutput();status=output_status;}
  if(status)status=header.AllReadsWithinBounds();
  size_t header_bytes=status?header.TotalBitsConsumed()/8:0;
  // On source early returns the observer still discharges the reader's close
  // obligation and records its result independently of the earlier status.
  close=header.Close();
  std::cout<<" F "<<frames++<<' '<<preview<<' '<<int(init_status.code())<<' '<<header.TotalBitsConsumed()<<' '<<int(close.code());
  if(output_attempted)std::cout<<" A "<<int(output_status.code());
  if(!status||!close){if(status)status=close;CoveredBundle(output,writes.bundle_coverage[0]);State(state,false,identities,writes);break;}
  const auto& h=frame.GetFrameHeader();std::cout<<" K "<<unsigned(h.frame_type)<<' '<<unsigned(h.encoding)<<' '<<h.flags<<' '<<h.dc_level<<' '<<h.is_last<<' '<<h.save_as_reference<<' '<<h.save_before_color_transform<<' '<<jxl::FrameDecoder::SavedAs(h)<<' '<<h.animation_frame.duration<<' '<<h.animation_frame.timecode<<' '<<h.frame_origin.x0<<' '<<h.frame_origin.y0<<' '<<h.default_xsize()<<' '<<h.default_ysize()<<' '<<unsigned(h.blending_info.mode)<<' '<<h.blending_info.source<<' '<<unsigned(h.color_transform)<<' '<<h.custom_size_or_origin;
  const auto dimensions=h.ToFrameDimensions();std::cout<<' '<<dimensions.xsize<<' '<<dimensions.ysize;std::cout<<" U "<<h.upsampling<<' '<<h.extra_channel_upsampling.size();for(auto factor:h.extra_channel_upsampling)std::cout<<' '<<factor;std::cout<<' '<<dimensions.num_groups<<' '<<dimensions.num_dc_groups<<' '<<h.group_size_shift;
  size_t position=offset+header_bytes;std::vector<std::unique_ptr<jxl::BitReader>> readers;std::vector<jxl::FrameDecoder::SectionInfo> sections;std::vector<jxl::FrameDecoder::SectionStatus> section_status;
  for(const auto& entry:frame.Toc()){if(entry.size>bytes.size()-position){status=false;break;}readers.emplace_back(std::make_unique<jxl::BitReader>(jxl::Bytes(data+position,entry.size)));sections.push_back({readers.back().get(),entry.id,sections.size()});position+=entry.size;}
  if(status){section_status.resize(sections.size());status=frame.ProcessSections(sections.data(),sections.size(),section_status.data());}
  writes.AfterSections();
  std::cout<<" S "<<int(status.code())<<' '<<section_status.size();for(auto v:section_status)std::cout<<' '<<unsigned(v);
  for(auto& section:readers){auto consumed=section->TotalBitsConsumed();auto closed=section->Close();std::cout<<" B "<<consumed<<' '<<int(closed.code());if(!closed&&status)status=closed;}
  bool all_done=status;for(auto v:section_status)if(v!=jxl::FrameDecoder::kDone)all_done=false;
  if(all_done){std::cout<<" D "<<frame.References()<<' '<<frame.HasDecodedDC()<<' '<<frame.HasDecodedAll();status=frame.FinalizeFrame();std::cout<<" Z "<<int(status.code());if(status){if(h.CanBeReferenced())writes.AfterReferenceMove(h.save_as_reference);output.SetDecodedBytes(position-offset);offset=position;last_main=!preview&&h.is_last;}}
  else if(status)status=false;
  CoveredBundle(output,writes.bundle_coverage[0]);State(state,bool(status),identities,writes);preview=false;
 }
 bool eof=offset==bytes.size();if(status&&(!last_main||!eof))status=false;std::cout<<" E "<<int(status.code())<<' '<<offset<<' '<<bytes.size()<<' '<<frames<<' '<<last_main<<' '<<eof<<' '<<state.visible_frame_index<<' '<<state.nonvisible_frame_index;
}

// Drive the production FrameDecoder incrementally as decode.cc's
// JxlDecoderProcessCodestream/ProcessSections/FlushImage do: one decoder and
// image bundle per frame, partial input, section selection from next_section,
// reader bounds/close checks, progressive pause events and Flush. Delivery 2
// and 3 offer sections in a seeded permutation (one per call, or all at once).
size_t IncrementalBytesPerChannel(JxlDataType type){return type==JXL_TYPE_UINT8?1:type==JXL_TYPE_FLOAT?4:2;}
void IncrementalBuffer(const char* tag,const std::vector<uint8_t>& buffer){std::cout<<' '<<tag<<' '<<buffer.size()<<' '<<Hex(jxl::Bytes(buffer));}
// Pixel callback records: rows are stored at the same buffer layout.
struct IncrementalCallback{uint8_t* out;size_t size,stride,pixel_bytes,inits=0,threads=0,pixels=0,runs=0,destroys=0;};
void* IncrementalInit(void* opaque,size_t threads,size_t pixels){auto* r=static_cast<IncrementalCallback*>(opaque);++r->inits;r->threads=threads;r->pixels=pixels;return opaque;}
void IncrementalRun(void* opaque,size_t,size_t x,size_t y,size_t n,const void* pixels){auto* r=static_cast<IncrementalCallback*>(opaque);size_t at=y*r->stride+x*r->pixel_bytes;if(at>r->size||n*r->pixel_bytes>r->size-at)std::abort();std::memcpy(r->out+at,pixels,n*r->pixel_bytes);++r->runs;}
void IncrementalDestroy(void* opaque){++static_cast<IncrementalCallback*>(opaque)->destroys;}
void Incremental(const std::vector<uint8_t>& bytes,std::istringstream& in,JxlMemoryManager* mm) {
 size_t limit,delivery,seed,detail,flush,skip,channels,data_type;bool coalescing,spot,unpremul,undo,extra,callback;
 in>>limit>>delivery>>seed>>coalescing>>spot>>detail>>flush>>skip>>channels>>data_type>>unpremul>>undo>>extra>>callback;
 if(!in||limit>bytes.size()||delivery>3||flush>2)throw std::runtime_error("incremental request");
 // Callback records and output buffers outlive the state's retained pipelines.
 std::deque<IncrementalCallback> records;std::vector<std::vector<uint8_t>> keep;
 std::vector<uint8_t> backing(limit+16);std::copy(bytes.begin(),bytes.begin()+limit,backing.begin()+8);const auto* data=backing.data()+8;
 jxl::CodecMetadata metadata;jxl::BitReader reader(jxl::Bytes(data,limit));auto status=Header(reader,metadata,mm);size_t offset=reader.TotalBitsConsumed()/8;auto close=reader.Close();std::cout<<" H "<<int(status.code())<<' '<<reader.TotalBitsConsumed()<<' '<<int(close.code());if(!status||!close)return;
 std::cout<<" M "<<metadata.size.xsize()<<' '<<metadata.size.ysize();MetadataObservation(metadata.m);
 jxl::PassesDecoderState state(mm);status=state.output_encoding_info.SetFromMetadata(metadata);std::cout<<" P "<<int(status.code());if(!status)return;
 std::array<const jxl::ImageBundle*,4> identities;for(size_t i=0;i<4;++i)identities[i]=state.shared_storage.reference_frames[i].frame.get();
 jxl::ImageBundle placeholder(mm,&metadata.m);std::unique_ptr<jxl::ImageBundle> ib;
 FrameSequenceWriteObservation writes(state.shared_storage,placeholder,state.frame_storage_for_referencing);

 uint64_t rng=seed*2+1;
 bool preview=metadata.m.have_preview,last_total=false;size_t frames=0;
 while(frames<64&&!last_total){
  if(offset>limit){std::cout<<" @N";break;}
  ib=jxl::make_unique<jxl::ImageBundle>(mm,&metadata.m);writes.bundles[0]=ib.get();
  jxl::FrameDecoder frame(&state,metadata,nullptr,true);
  jxl::BitReader header(jxl::Bytes(data+offset,limit-offset));
  status=frame.InitFrame(&header,ib.get(),preview);writes.AfterInitFrame();
  bool inb=bool(header.AllReadsWithinBounds());size_t bits=header.TotalBitsConsumed();close=header.Close();
  std::cout<<" @F "<<frames++<<' '<<preview<<' '<<int(status.code())<<' '<<inb<<' '<<bits<<' '<<int(close.code());
  if(!inb||status.code()==jxl::StatusCode::kNotEnoughBytes){std::cout<<" @N";break;}
  if(!status)break;
  offset+=bits/8;
  const auto& h=frame.GetFrameHeader();const auto d=h.ToFrameDimensions();
  std::cout<<" @K "<<unsigned(h.frame_type)<<' '<<unsigned(h.encoding)<<' '<<h.flags<<' '<<h.dc_level<<' '<<h.is_last<<' '<<h.save_as_reference<<' '<<h.save_before_color_transform<<' '<<jxl::FrameDecoder::SavedAs(h)<<' '<<h.animation_frame.duration<<' '<<h.custom_size_or_origin<<' '<<unsigned(h.blending_info.mode)<<' '<<h.passes.num_passes<<' '<<h.passes.num_downsample<<' '<<d.xsize<<' '<<d.ysize<<' '<<d.num_groups<<' '<<d.num_dc_groups;
  status=frame.InitFrameOutput();std::cout<<" @A "<<int(status.code());if(!status)break;
  const auto& toc=frame.Toc();const size_t n=toc.size();const size_t remaining=frame.SumSectionSizes();
  std::cout<<" @T "<<n<<' '<<remaining;for(const auto& entry:toc)std::cout<<' '<<entry.size<<' '<<entry.id;
  bool skipping=false,last_still=false;
  if(!preview){
   last_total=h.is_last;last_still=h.is_last||h.animation_frame.duration>0||(!coalescing&&h.frame_type==jxl::FrameType::kRegularFrame);
   if(skip>0){skipping=true;if(last_still)--skip;}
   if(skipping&&!(h.CanBeReferenced()||h.frame_type==jxl::FrameType::kDCFrame)){std::cout<<" @X";offset+=remaining;continue;}
  }
  frame.SetRenderSpotcolors(spot);frame.SetCoalescing(coalescing);
  JxlProgressiveDetail effective=preview?JxlProgressiveDetail::kFrames:frame.SetPauseAtProgressive(static_cast<JxlProgressiveDetail>(detail));
  std::cout<<" @G "<<int(effective);
  const bool want=channels!=0&&(preview||(last_still&&!skipping));
  size_t xsize=0,ysize=0;JxlPixelFormat format{uint32_t(channels),JxlDataType(data_type),JXL_NATIVE_ENDIAN,0};
  JxlPixelFormat ec_format{1,JxlDataType(data_type),JXL_NATIVE_ENDIAN,0};const size_t bpc=IncrementalBytesPerChannel(JxlDataType(data_type));
  size_t main_index=0,first_extra=0;IncrementalCallback* record=nullptr;
  if(want){
   if(preview){xsize=metadata.oriented_preview_xsize(!undo);ysize=metadata.oriented_preview_ysize(!undo);}
   else{xsize=metadata.oriented_xsize(!undo);ysize=metadata.oriented_ysize(!undo);
    if(!coalescing){xsize=d.xsize_upsampled;ysize=d.ysize_upsampled;if(undo&&static_cast<int>(metadata.m.GetOrientation())>4)std::swap(xsize,ysize);}}
   main_index=keep.size();keep.emplace_back(bpc*channels*xsize*ysize,0xA5);first_extra=keep.size();
   if(callback){records.push_back({keep[main_index].data(),keep[main_index].size(),bpc*channels*xsize,bpc*channels});record=&records.back();}
   if(extra)for(size_t i=0;i<metadata.m.extra_channel_info.size();++i)keep.emplace_back(bpc*xsize*ysize,0x5A);
  }
  const size_t extras=want&&extra?metadata.m.extra_channel_info.size():0;
  std::cout<<" @W "<<want<<' '<<xsize<<' '<<ysize;
  const auto dump=[&]{IncrementalBuffer("@Y",keep[main_index]);if(record)std::cout<<" @C "<<record->inits<<' '<<record->threads<<' '<<record->pixels<<' '<<record->runs<<' '<<record->destroys;for(size_t i=0;i<extras;++i)IncrementalBuffer("@YE",keep[first_extra+i]);};
  std::vector<size_t> starts(n);size_t position=offset;for(size_t i=0;i<n;++i){starts[i]=position;position+=toc[i].size;}
  std::vector<size_t> order(n);for(size_t i=0;i<n;++i)order[i]=i;
  if(delivery>=2)for(size_t i=n;i-->1;){rng^=rng<<13;rng^=rng>>7;rng^=rng<<17;std::swap(order[i],order[rng%(i+1)]);}
  std::cout<<" @Q";for(size_t i:order)std::cout<<' '<<i;
  std::vector<uint8_t> processed(n);size_t next_section=0,available=1,cursor=0,idle=0;bool dc_done=false,done=false;
  const auto end_of=[&](size_t k){return std::min(limit,k==0?offset:starts[k-1]+toc[k-1].size);};
  size_t avail_end=delivery==1?end_of(available):limit;
  for(size_t call=0;;++call){
   if(call==4096)throw std::runtime_error("incremental call limit");
   if(want){
    // decode.cc installs the output on every kFull iteration.
    auto set=record?frame.SetImageOutput(jxl::PixelCallback{IncrementalInit,IncrementalRun,IncrementalDestroy,record},nullptr,0,xsize,ysize,format,bpc*8,unpremul,undo):frame.SetImageOutput(jxl::PixelCallback{},keep[main_index].data(),keep[main_index].size(),xsize,ysize,format,bpc*8,unpremul,undo);
    std::cout<<" @I "<<int(set.code());
    for(size_t i=0;i<extras;++i){auto added=frame.AddExtraChannelOutput(keep[first_extra+i].data(),keep[first_extra+i].size(),xsize,ec_format,bpc*8);std::cout<<' '<<int(added.code());}
   }
   const size_t next_pause=frame.NextNumPassesToPause();
   std::vector<size_t> chosen;
   if(delivery<=1){for(size_t i=next_section;i<n;++i){if(processed[i])continue;if(starts[i]+toc[i].size>avail_end)break;chosen.push_back(i);}}
   else if(delivery==2){for(size_t k=0;k<n;++k){size_t i=order[(cursor+k)%n];if(!processed[i]&&starts[i]+toc[i].size<=limit){chosen.push_back(i);cursor=(cursor+k+1)%n;break;}}}
   else{for(size_t i:order)if(!processed[i]&&starts[i]+toc[i].size<=limit)chosen.push_back(i);}
   std::vector<std::unique_ptr<jxl::BitReader>> readers;std::vector<jxl::FrameDecoder::SectionInfo> sections;std::vector<jxl::FrameDecoder::SectionStatus> section_status;
   for(size_t i:chosen){readers.emplace_back(jxl::make_unique<jxl::BitReader>(jxl::Bytes(data+starts[i],toc[i].size)));sections.push_back({readers.back().get(),toc[i].id,i});section_status.emplace_back();}
   status=frame.ProcessSections(sections.data(),sections.size(),section_status.data());writes.AfterSections();
   std::cout<<" @S "<<int(status.code())<<' '<<sections.size();for(size_t i=0;i<sections.size();++i)std::cout<<' '<<sections[i].index<<' '<<sections[i].id<<' '<<unsigned(section_status[i]);
   bool out_of_bounds=false,close_error=false;
   for(auto& section:readers){bool within=bool(section->AllReadsWithinBounds());if(!within)out_of_bounds=true;size_t consumed=section->TotalBitsConsumed();auto closed=section->Close();if(!closed)close_error=true;std::cout<<" @B "<<consumed<<' '<<within<<' '<<int(closed.code());}
   int outcome=close_error?1:out_of_bounds?2:!status?3:0;size_t progress=0;
   if(!outcome)for(size_t i=0;i<section_status.size();++i){if(section_status[i]==jxl::FrameDecoder::kDone){processed[sections[i].index]=1;++progress;}else if(section_status[i]!=jxl::FrameDecoder::kSkipped){outcome=4;break;}}
   std::cout<<" @O "<<outcome;if(outcome)break;
   while(next_section<n&&processed[next_section])++next_section;
   const bool all=frame.HasDecodedAll(),dc=frame.HasDecodedDC();const size_t passes=frame.NumCompletePasses();
   std::cout<<" @D "<<dc<<' '<<all<<' '<<passes<<' '<<frame.NextNumPassesToPause()<<' '<<next_section;
   int event=0;
   if(effective>=JxlProgressiveDetail::kDC&&!dc_done&&!all&&dc){dc_done=true;event=1;std::cout<<" @V 1 8";}
   else if(!all&&effective>=JxlProgressiveDetail::kLastPasses&&passes>=next_pause){event=2;std::cout<<" @V 2 "<<h.passes.GetDownsamplingTargetForCompletedPasses(passes);}
   if(want&&((event&&flush>=1)||(!event&&!all&&flush==2&&dc))){auto flushed=frame.Flush();writes.AfterSections();std::cout<<" @L "<<int(flushed.code());dump();}
   if(event)continue;
   if(all){done=true;break;}
   bool more=false;
   if(delivery==1){if(available<n&&avail_end<limit){++available;avail_end=end_of(available);more=true;}}
   else if(delivery==2){idle=progress?0:idle+1;size_t left=0;for(auto v:processed)left+=!v;more=left!=0&&idle<left;}
   if(!more){std::cout<<" @N";if(want&&flush>=1&&dc){auto flushed=frame.Flush();writes.AfterSections();std::cout<<" @L "<<int(flushed.code());dump();}break;}
  }
  if(!done){CoveredBundle(*ib,writes.bundle_coverage[0]);State(state,false,identities,writes);break;}
  if(!preview)std::cout<<" @R "<<frame.References();
  status=frame.FinalizeFrame();std::cout<<" @Z "<<int(status.code());
  if(status&&h.CanBeReferenced())writes.AfterReferenceMove(h.save_as_reference);
  if(want)dump();
  CoveredBundle(*ib,writes.bundle_coverage[0]);State(state,bool(status),identities,writes);
  if(!status)break;
  offset+=remaining;preview=false;writes.bundles[0]=&placeholder;ib.reset();
 }
 std::cout<<" @E "<<frames<<' '<<offset<<' '<<last_total<<' '<<state.visible_frame_index<<' '<<state.nonvisible_frame_index;
}
}
bool FrameSequencesCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
 if(command=="ifd-decode"){std::string hex;in>>hex;Incremental(Unhex(hex),in,mm);return true;}
 if(command=="fs-decode"){std::string hex;in>>hex;Decode(Unhex(hex),mm);return true;}
 if(command=="fs-dc-state"){
  size_t w,h,level;bool use,existing;in>>w>>h>>level>>use>>existing;jxl::CodecMetadata metadata;if(!metadata.size.Set(w,h))throw std::runtime_error("DC dimensions");jxl::FrameHeader header(&metadata);header.frame_type=jxl::FrameType::kDCFrame;header.dc_level=level;header.UpdateFlag(use,jxl::FrameHeader::kUseDcFrame);jxl::PassesSharedState state(mm);
  if(existing&&level<4){auto made=jxl::Image3F::Create(mm,2,3);if(!made.ok())throw std::runtime_error("DC slot");state.dc_frames[level]=std::move(made).value_();jxl::ZeroFillImage(&state.dc_frames[level]);}
  auto status=jxl::InitializePassesSharedState(header,&state);std::cout<<int(status.code())<<' '<<(state.dc==&state.dc_storage);for(const auto& dc:state.dc_frames)std::cout<<' '<<(state.dc==&dc);Image(state.dc_storage,false);Image(*state.dc,bool(status)&&use&&existing);return true;
 }
 if(command=="fs-dc-ladder") {
  std::string hex;size_t highest;in>>hex>>highest;auto bytes=Unhex(hex);
  if(highest<2||highest>4)throw std::runtime_error("DC ladder experiment level");
  jxl::CodecMetadata metadata;jxl::BitReader global{jxl::Bytes(bytes)};
  auto status=Header(global,metadata,mm);size_t header_bytes=global.TotalBitsConsumed()/8;auto closed=global.Close();
  if(!status||!closed||metadata.xsize()!=1||metadata.ysize()!=1)throw std::runtime_error("DC ladder 1x1 metadata");
  std::vector<jxl::FrameHeader> headers;std::vector<std::vector<size_t>> sizes;
  std::vector<std::vector<jxl::coeff_order_t>> permutations;std::vector<std::vector<uint8_t>> payloads;
  size_t offset=header_bytes;
  while(offset<bytes.size()) {
   jxl::FrameHeader header(&metadata);jxl::BitReader reader(jxl::Bytes(bytes.data()+offset,bytes.size()-offset));
   status=jxl::ReadFrameHeader(&reader,&header);
   if(!status){auto ignored=reader.Close();(void)ignored;throw std::runtime_error("DC ladder original frame");}
   auto dimensions=header.ToFrameDimensions();std::vector<uint32_t> source_sizes;std::vector<jxl::coeff_order_t> permutation;
   status=jxl::ReadToc(mm,jxl::NumTocEntries(dimensions.num_groups,dimensions.num_dc_groups,header.passes.num_passes),&reader,&source_sizes,&permutation);
   size_t begin=reader.TotalBitsConsumed()/8;closed=reader.Close();
   if(!status||!closed)throw std::runtime_error("DC ladder original TOC");
   size_t total=0;for(auto size:source_sizes)total+=size;
   if(offset+begin+total>bytes.size())throw std::runtime_error("DC ladder original payload");
   headers.push_back(header);sizes.emplace_back(source_sizes.begin(),source_sizes.end());permutations.push_back(permutation);
   payloads.emplace_back(bytes.begin()+offset+begin,bytes.begin()+offset+begin+total);offset+=begin+total;
  }
  if(headers.size()!=3||headers[0].dc_level!=2||headers[1].dc_level!=1||headers[2].dc_level!=0||
    (headers[0].flags&jxl::FrameHeader::kUseDcFrame)||!(headers[1].flags&jxl::FrameHeader::kUseDcFrame)||!(headers[2].flags&jxl::FrameHeader::kUseDcFrame))
   throw std::runtime_error("DC ladder genuine source prerequisites");
  const auto frame_bytes=[&](jxl::FrameHeader header,size_t original) {
   jxl::BitWriter writer(mm);auto written=jxl::WriteFrameHeader(header,&writer,nullptr);
   if(written)written=jxl::WriteTocPermutation(permutations[original],&writer,nullptr);
   if(written)written=jxl::WriteTocSizes(sizes[original],&writer,nullptr);
   if(written)written=writer.AppendByteAligned(jxl::Bytes(payloads[original]));
   if(!written)throw std::runtime_error("DC ladder actual source writers");
   auto made=std::move(writer).TakeBytes();return std::vector<uint8_t>(made.begin(),made.end());
  };
  std::vector<uint8_t> control(bytes.begin(),bytes.begin()+header_bytes);
  for(size_t i=0;i<headers.size();++i){auto encoded=frame_bytes(headers[i],i);control.insert(control.end(),encoded.begin(),encoded.end());}
  if(control!=bytes)throw std::runtime_error("DC ladder source writer control is not exact");
  std::vector<uint8_t> result(bytes.begin(),bytes.begin()+header_bytes);std::ostringstream provenance;std::vector<size_t> ladder_ends;
  for(size_t next=highest+1;next-->0;) {
   size_t original=next==highest?0:(next==0?2:1);auto header=headers[original];header.dc_level=next;
   auto original_dimensions=headers[original].ToFrameDimensions();auto dimensions=header.ToFrameDimensions();
   const auto geometry=[](const jxl::FrameDimensions& d) {return std::array<size_t,18>{d.xsize,d.ysize,d.xsize_upsampled,d.ysize_upsampled,d.xsize_upsampled_padded,d.ysize_upsampled_padded,d.xsize_padded,d.ysize_padded,d.xsize_blocks,d.ysize_blocks,d.xsize_groups,d.ysize_groups,d.xsize_dc_groups,d.ysize_dc_groups,d.num_groups,d.num_dc_groups,d.group_dim,d.dc_group_dim};};
   if(geometry(original_dimensions)!=geometry(dimensions)||dimensions.xsize!=1||dimensions.ysize!=1)throw std::runtime_error("DC ladder payload geometry changed");
   auto encoded=frame_bytes(header,original);
   // Reparse the real emitted header/TOC, check payload identity, and restore
   // only dc_level. The exact original source writers must then reproduce the
   // original frame bytes, proving no other serialized field changed.
   jxl::FrameHeader reparsed(&metadata);jxl::BitReader reader{jxl::Bytes(encoded)};
   auto checked=jxl::ReadFrameHeader(&reader,&reparsed);
   std::vector<uint32_t> re_sizes;std::vector<jxl::coeff_order_t> re_permutation;
   if(checked)checked=jxl::ReadToc(mm,jxl::NumTocEntries(dimensions.num_groups,dimensions.num_dc_groups,reparsed.passes.num_passes),&reader,&re_sizes,&re_permutation);
   size_t payload_begin=reader.TotalBitsConsumed()/8;auto close=reader.Close();
   if(!checked||!close||reparsed.dc_level!=next||std::vector<size_t>(re_sizes.begin(),re_sizes.end())!=sizes[original]||re_permutation!=permutations[original]||std::vector<uint8_t>(encoded.begin()+payload_begin,encoded.end())!=payloads[original])throw std::runtime_error("DC ladder emitted provenance");
   reparsed.dc_level=headers[original].dc_level;
   if(frame_bytes(reparsed,original)!=frame_bytes(headers[original],original))throw std::runtime_error("DC ladder changed a field besides dc_level");
   result.insert(result.end(),encoded.begin(),encoded.end());ladder_ends.push_back(result.size());
   provenance<<" F "<<original<<' '<<headers[original].dc_level<<' '<<next<<' '<<header.flags
     <<' '<<dimensions.xsize<<' '<<dimensions.ysize<<' '<<dimensions.xsize_padded<<' '<<dimensions.ysize_padded
     <<' '<<dimensions.xsize_blocks<<' '<<dimensions.ysize_blocks<<' '<<dimensions.num_groups<<' '<<dimensions.num_dc_groups
     <<' '<<payloads[original].size()<<' '<<Hex(jxl::Bytes(payloads[original]));
   provenance<<" G";for(size_t field:geometry(dimensions))provenance<<' '<<field;
  }
  std::cout<<Hex(jxl::Bytes(result))<<" D "<<(highest+1)<<" C 1"<<provenance.str()<<" E "<<ladder_ends.size();for(auto end:ladder_ends)std::cout<<' '<<end;return true;
 }

 if(command=="fs-frame-ends") {
  std::string hex;in>>hex;auto bytes=Unhex(hex);jxl::CodecMetadata metadata;
  jxl::BitReader global{jxl::Bytes(bytes)};auto status=Header(global,metadata,mm);size_t offset=global.TotalBitsConsumed()/8;auto close=global.Close();
  if(!status||!close)throw std::runtime_error("source frame-ends metadata");
  std::vector<size_t> ends;bool preview=metadata.m.have_preview;
  while(offset<bytes.size()) {
   jxl::FrameHeader header(&metadata);header.nonserialized_is_preview=preview;
   jxl::BitReader reader(jxl::Bytes(bytes.data()+offset,bytes.size()-offset));status=jxl::ReadFrameHeader(&reader,&header);
   if(!status){auto ignored=reader.Close();(void)ignored;throw std::runtime_error("source frame-ends header");}
   auto dimensions=header.ToFrameDimensions();std::vector<uint32_t> sizes;std::vector<jxl::coeff_order_t> permutation;
   status=jxl::ReadToc(mm,jxl::NumTocEntries(dimensions.num_groups,dimensions.num_dc_groups,header.passes.num_passes),&reader,&sizes,&permutation);
   size_t begin=reader.TotalBitsConsumed()/8;close=reader.Close();if(!status||!close)throw std::runtime_error("source frame-ends TOC");
   size_t total=0;for(auto size:sizes)total+=size;
   if(begin>bytes.size()-offset||total>bytes.size()-offset-begin)throw std::runtime_error("source frame-ends payload");
   offset+=begin+total;ends.push_back(offset);preview=false;
  }
  std::cout<<ends.size();for(auto end:ends)std::cout<<' '<<end;return true;
 }

 if(command=="fs-section-size") {
  std::string hex;size_t selected,entry,remove;in>>hex>>selected>>entry>>remove;auto bytes=Unhex(hex);
  jxl::CodecMetadata metadata;jxl::BitReader global{jxl::Bytes(bytes)};auto status=Header(global,metadata,mm);size_t offset=global.TotalBitsConsumed()/8;auto close=global.Close();
  if(!status||!close)throw std::runtime_error("source section-size metadata");
  for(size_t i=0;i<=selected;++i){
   jxl::FrameHeader header(&metadata);header.nonserialized_is_preview=i==0&&metadata.m.have_preview;
   jxl::BitReader reader(jxl::Bytes(bytes.data()+offset,bytes.size()-offset));status=jxl::ReadFrameHeader(&reader,&header);
   if(!status){auto ignored=reader.Close();(void)ignored;throw std::runtime_error("source section-size frame header");}
   auto dimensions=header.ToFrameDimensions();std::vector<uint32_t> sizes;std::vector<jxl::coeff_order_t> permutation;
   status=jxl::ReadToc(mm,jxl::NumTocEntries(dimensions.num_groups,dimensions.num_dc_groups,header.passes.num_passes),&reader,&sizes,&permutation);
   size_t begin=reader.TotalBitsConsumed()/8;close=reader.Close();if(!status||!close)throw std::runtime_error("source section-size TOC");
   size_t total=0;for(auto size:sizes)total+=size;
   if(i!=selected){offset+=begin+total;continue;}
   if(entry>=sizes.size()||remove>sizes[entry])throw std::runtime_error("source section-size request");
   std::vector<size_t> changed(sizes.begin(),sizes.end());changed[entry]-=remove;
   jxl::BitWriter writer(mm);status=jxl::WriteFrameHeader(header,&writer,nullptr);
   if(status)status=jxl::WriteTocPermutation(permutation,&writer,nullptr);
   if(status)status=jxl::WriteTocSizes(changed,&writer,nullptr);
   if(!status)throw std::runtime_error("source section-size writer");
   auto replacement=std::move(writer).TakeBytes();std::vector<uint8_t> result(bytes.begin(),bytes.begin()+offset);
   result.insert(result.end(),replacement.begin(),replacement.end());result.insert(result.end(),bytes.begin()+offset+begin,bytes.end());
   std::cout<<Hex(jxl::Bytes(result));return true;
  }
  throw std::runtime_error("source section-size frame absent");
 }
 const bool with_passes=command=="fs-encode-passes",extended=command=="fs-encode-ext"||with_passes,patches=command=="fs-encode-patches";
 if(command!="fs-encode"&&!extended&&!patches)return false;
 size_t w,h,preview_w,preview_h,n;bool gray,alpha,xyb,animation;in>>w>>h>>gray>>alpha>>xyb>>animation>>preview_w>>preview_h>>n;
 jxl::CodecMetadata metadata;if(!metadata.size.Set(w,h))throw std::runtime_error("sequence size");metadata.m.SetUintSamples(8);metadata.m.xyb_encoded=xyb;if(alpha)metadata.m.SetAlphaBits(8,false);metadata.m.color_encoding=jxl::ColorEncoding::SRGB(gray);metadata.m.have_animation=animation;metadata.m.animation.have_timecodes=animation;metadata.m.animation.tps_numerator=1000;metadata.m.animation.tps_denominator=1;metadata.m.animation.num_loops=3;
 metadata.m.have_preview=preview_w!=0;if(metadata.m.have_preview&&!metadata.m.preview_size.Set(preview_w,preview_h))throw std::runtime_error("preview size");
 jxl::BitWriter writer(mm);jxl::Status status=jxl::WriteCodestreamHeaders(&metadata,&writer,nullptr);if(status&&metadata.m.color_encoding.WantICC())status=jxl::WriteICC(jxl::Bytes(metadata.m.color_encoding.ICC()),&writer,jxl::LayerType::Header,nullptr);if(status)status=writer.WithMaxBits(8,jxl::LayerType::Header,nullptr,[&]{writer.ZeroPadToByte();return true;});
 size_t header_bytes=writer.BitsWritten()/8;std::vector<size_t> ends;std::vector<int> statuses;std::vector<std::array<int,3>> postinit_resampling;
 for(size_t i=0;i<n;++i){bool modular,preview;unsigned type,blendmode;size_t fw,fh,seed;int progressive;jxl::FrameInfo info;in>>modular>>type>>info.dc_level>>info.is_last>>info.save_as_reference>>info.source>>info.save_before_color_transform>>info.blend>>blendmode>>info.clamp>>info.origin.x0>>info.origin.y0>>fw>>fh>>info.duration>>info.timecode>>seed>>progressive>>preview;info.frame_type=static_cast<jxl::FrameType>(type);info.blendmode=static_cast<jxl::BlendMode>(blendmode);info.is_preview=preview;info.name="sequence-"+std::to_string(i);
  int requested_resampling=1,requested_ec_resampling=1,requested_group_shift=1;if(extended)in>>requested_resampling>>requested_ec_resampling>>requested_group_shift;
  int progressive_mode=-1,qprogressive_mode=-1;if(with_passes)in>>progressive_mode>>qprogressive_mode;
  if(!status){statuses.push_back(-1);continue;}
  jxl::CompressParams params;params.cms=*JxlGetDefaultCms();params.debug_image_opaque=nullptr;params.buffering=0;params.level=10;params.speed_tier=patches?jxl::SpeedTier::kSquirrel:jxl::SpeedTier::kThunder;params.patches=patches?jxl::Override::kOn:jxl::Override::kOff;params.dots=params.noise=jxl::Override::kOff;params.gaborish=jxl::Override::kOff;params.epf=0;params.progressive_dc=progressive;
  if(modular&&!xyb)params.SetLossless();else{params.modular_mode=modular;params.butteraugli_distance=1.f;params.color_transform=xyb?jxl::ColorTransform::kXYB:jxl::ColorTransform::kNone;}
  if(extended){params.resampling=requested_resampling;params.ec_resampling=requested_ec_resampling;params.modular_group_size_shift=requested_group_shift;}
  // Actual encoder progressive AC pass selection (enc_frame.cc).
  if(with_passes){params.progressive_mode=static_cast<jxl::Override>(progressive_mode);params.qprogressive_mode=static_cast<jxl::Override>(qprogressive_mode);}
  status=jxl::ParamsPostInit(&params);if(extended)postinit_resampling.push_back({params.resampling,params.ec_resampling,params.modular_group_size_shift});size_t channels=(gray?1:3)+(alpha?1:0);std::vector<uint8_t> pixels(fw*fh*channels);for(size_t y=0;y<fh;++y)for(size_t x=0;x<fw;++x)for(size_t c=0;c<channels;++c)pixels[(y*fw+x)*channels+c]=patches?uint8_t((x%24>=5&&x%24<13&&y%24>=5&&y%24<17&&(x%24<7||x%24>=11||y%24==10||y%24==11))?0:255):uint8_t((seed*31+x*17+y*23+c*47)%256);
  jxl::JxlEncoderChunkedFrameAdapter input(fw,fh,alpha?1:0);JxlPixelFormat format{unsigned(channels),JXL_TYPE_UINT8,JXL_NATIVE_ENDIAN,0};if(status)status=input.SetFromBuffer(0,pixels.data(),pixels.size(),format);
  JxlEncoderOutputProcessorWrapper output(mm);if(status)status=jxl::EncodeFrame(mm,params,info,&metadata,input,*JxlGetDefaultCms(),nullptr,&output,nullptr,nullptr);if(status)status=output.SetFinalizedPosition();std::vector<uint8_t> bytes;if(status)status=output.CopyOutput(bytes);if(status)status=writer.AppendByteAligned(jxl::Bytes(bytes));statuses.push_back(int(status.code()));ends.push_back(writer.BitsWritten()/8);
 }
 auto bytes=std::move(writer).TakeBytes();std::cout<<int(status.code())<<' '<<Hex(jxl::Bytes(bytes))<<' '<<header_bytes<<' '<<ends.size();for(auto end:ends)std::cout<<' '<<end;std::cout<<" S "<<statuses.size();for(auto s:statuses)std::cout<<' '<<s;if(extended){std::cout<<" P "<<postinit_resampling.size();for(auto factors:postinit_resampling)for(auto value:factors)std::cout<<' '<<value;}return true;
}
