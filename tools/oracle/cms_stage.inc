// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Actual original ICC setters, ColorSpaceTransform, and CMS render stage.
// The caller-supplied CMS is a trace/bit-copy probe, not a codec replacement.
#include "lib/jxl/render_pipeline/stage_cms.h"
void ColorStorageObservation(const jxl::cms::ColorEncoding&);
struct CSTProbe;
struct CSTContext {CSTProbe* owner;size_t id,threads,width,src_channels,dst_channels;JxlColorProfile src_profile,dst_profile;bool destroyed=false;std::vector<std::vector<float>> src,dst;};
struct CSTProbe {std::ostringstream trace;std::vector<std::unique_ptr<CSTContext>> contexts;JxlColorEncoding parse_external{};int parse_result=1,parse_cmyk=0,fail_init=0,fail_run=0;bool alias=false;size_t calls=0;};
void CSTExternal(std::ostream& out,const JxlColorEncoding& c){out<<unsigned(c.color_space)<<','<<unsigned(c.white_point)<<','<<unsigned(c.primaries)<<','<<unsigned(c.transfer_function)<<','<<unsigned(c.rendering_intent)<<','<<SCBits<uint64_t>(c.gamma);for(const double* xy:{c.white_point_xy,c.primaries_red_xy,c.primaries_green_xy,c.primaries_blue_xy})for(size_t i=0;i<2;++i)out<<','<<SCBits<uint64_t>(xy[i]);}
std::string CSTICC(const JxlColorProfile& p){return p.icc.size?Hex(jxl::Bytes(p.icc.data,p.icc.size)):"-";}
void CSTProfile(std::ostream& out,const JxlColorProfile& p){out<<' '<<p.num_channels<<' '<<bool(p.icc.data)<<' '<<CSTICC(p)<<' ';CSTExternal(out,p.color_encoding);}
extern "C" int CSTFields(void* data,const uint8_t* icc,size_t size,JxlColorEncoding* output,JXL_BOOL* cmyk){auto& p=*static_cast<CSTProbe*>(data);p.trace<<" P "<<p.parse_result<<' '<<p.parse_cmyk<<' '<<(size?Hex(jxl::Bytes(icc,size)):"-");if(p.parse_result){*output=p.parse_external;*cmyk=p.parse_cmyk;}return p.parse_result;}
extern "C" void* CSTInit(void* data,size_t threads,size_t width,const JxlColorProfile* src,const JxlColorProfile* dst,float intensity){auto& p=*static_cast<CSTProbe*>(data);++p.calls;p.trace<<" I "<<threads<<' '<<width<<' '<<SCBits<uint32_t>(intensity);CSTProfile(p.trace,*src);CSTProfile(p.trace,*dst);if(p.fail_init==1||(p.fail_init==2&&p.calls==1)||(p.fail_init==3&&p.calls==2)){p.trace<<" F";return nullptr;}auto c=std::make_unique<CSTContext>();c->owner=&p;c->id=p.contexts.size();c->threads=threads;c->width=width;c->src_channels=src->num_channels;c->dst_channels=dst->num_channels;c->src_profile=*src;c->dst_profile=*dst;c->src.assign(threads,std::vector<float>(std::max(size_t{1},width*c->src_channels),-11.0f));c->dst.assign(threads,std::vector<float>(std::max(size_t{1},width*c->dst_channels),-13.0f));auto* result=c.get();p.trace<<" C "<<c->id;p.contexts.push_back(std::move(c));return result;}
extern "C" float* CSTSrc(void* data,size_t thread){auto& c=*static_cast<CSTContext*>(data);c.owner->trace<<" S "<<c.id<<' '<<thread;return c.src[thread].data();}
extern "C" float* CSTDst(void* data,size_t thread){auto& c=*static_cast<CSTContext*>(data);c.owner->trace<<" D "<<c.id<<' '<<thread;return c.owner->alias&&c.src_channels>c.dst_channels?c.src[thread].data():c.dst[thread].data();}
extern "C" int CSTRun(void* data,size_t thread,const float* src,float* dst,size_t width){auto& c=*static_cast<CSTContext*>(data);c.owner->trace<<" R "<<c.id<<' '<<thread<<' '<<width<<' '<<(src==dst);for(size_t x=0;x<width;++x)for(size_t ch=0;ch<c.dst_channels;++ch){uint32_t value;memcpy(&value,src+x*c.src_channels+ch%c.src_channels,4);value^=0x80000000u;memcpy(dst+x*c.dst_channels+ch,&value,4);}return c.owner->fail_run?0:1;}
extern "C" void CSTDestroy(void* data){auto& c=*static_cast<CSTContext*>(data);c.destroyed=true;c.owner->trace<<" K "<<c.id<<' '<<CSTICC(c.src_profile)<<' '<<CSTICC(c.dst_profile);}
JxlCmsInterface CSTInterface(CSTProbe& p){return {&p,CSTFields,&p,CSTInit,CSTSrc,CSTDst,CSTRun,CSTDestroy};}
jxl::ColorEncoding CSTColor(CSTProbe& p,unsigned kind){if(kind==9)return jxl::ColorEncoding();bool gray=kind==2||kind==3||kind==6;jxl::ColorEncoding c=(kind==1||kind==3)?jxl::ColorEncoding::LinearSRGB(gray):jxl::ColorEncoding::SRGB(gray);if(kind==4){p.parse_external=c.ToExternal();p.parse_cmyk=1;std::vector<uint8_t> bytes={0xa5,0xc4,7};auto cms=CSTInterface(p);if(!c.SetICC(std::move(bytes),&cms))throw std::runtime_error("CMYK probe profile");}if(kind==5||kind==6)c.SetICCRaw({9,8,7});if(kind==7)c.SetColorSpace(jxl::ColorSpace::kUnknown);if(kind==8&&!c.Tf().SetGamma(0.45))throw std::runtime_error("gamma probe profile");return c;}
void CSTDump(const CSTProbe& p){std::cout<<" X "<<p.contexts.size();for(const auto& c:p.contexts){std::cout<<' '<<c->id<<' '<<c->threads<<' '<<c->width<<' '<<c->src_channels<<' '<<c->dst_channels<<' '<<c->destroyed;for(const auto* all:{&c->src,&c->dst})for(const auto& row:*all){std::cout<<' '<<row.size();for(float v:row)std::cout<<' '<<SCBits<uint32_t>(v);}}std::cout<<" L"<<p.trace.str()<<" ;";}
bool CmsStageCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm){
  if(command=="cst-icc"){
    unsigned kind;bool direct,has_cms;int result,cmyk;std::string hex;in>>kind>>direct>>has_cms>>result>>cmyk>>hex;CSTProbe p;auto color=CSTColor(p,kind);p.parse_external=ReadColorEncoding(in);p.parse_result=result;p.parse_cmyk=cmyk;auto cms=CSTInterface(p);auto data=Unhex(hex);auto* original=data.data();
    if(direct){auto storage=color.View();auto status=storage.SetFieldsFromICC(std::move(data),cms);std::cout<<int(status.code())<<' '<<(!data.empty()||storage.icc.empty()?false:storage.icc.data()==original)<<' '<<Hex(jxl::Bytes(data));ColorStorageObservation(storage);}
    else{auto status=color.SetICC(std::move(data),has_cms?&cms:nullptr);std::cout<<int(status.code())<<' '<<(!data.empty()||color.ICC().empty()?false:color.ICC().data()==original)<<' '<<Hex(jxl::Bytes(data));ColorBundleObservation(color);}
    CSTDump(p);return true;
  }
  if(command=="cst-flow"){
    unsigned sk,dk;size_t width,threads,repeats;CSTProbe p;in>>sk>>dk>>width>>threads>>repeats>>p.fail_init>>p.fail_run>>p.alias;auto src=CSTColor(p,sk),dst=CSTColor(p,dk);auto cms=CSTInterface(p);auto transform=std::make_unique<jxl::ColorSpaceTransform>(cms);
    for(size_t i=0;i<repeats;++i){auto status=transform->Init(src,dst,255.0f,width,threads);std::cout<<" I "<<int(status.code())<<' '<<bool(transform->cms_data_)<<' '<<Hex(jxl::Bytes(transform->icc_src_))<<' '<<Hex(jxl::Bytes(transform->icc_dst_));auto moved=std::move(transform);transform=std::move(moved);
      if(status)for(size_t thread=0;thread<threads;++thread)for(size_t count:{width,width?width-1:0,size_t{0}}){auto* input=transform->BufSrc(thread);auto* output=transform->BufDst(thread);size_t channels=src.IsCMYK()?4:src.Channels();for(size_t x=0;x<count*channels;++x)input[x]=RPValue(0,x,thread,i,3);auto run=transform->Run(thread,input,output,count);std::cout<<" R "<<int(run.code());}}
    transform.reset();CSTDump(p);return true;
  }
  if(command!="cst-stage")return false;
  unsigned sk,dk,mutation,guard,mode;size_t width,threads,repeats;bool enabled,linear;CSTProbe p;in>>sk>>dk>>enabled>>linear>>width>>threads>>repeats>>mutation>>guard>>mode>>p.fail_init>>p.fail_run>>p.alias;auto src=CSTColor(p,sk),dst=CSTColor(p,dk);auto cms=CSTInterface(p);jxl::OutputEncodingInfo info{};info.orig_color_encoding=src;info.linear_color_encoding=src;info.linear_color_encoding.Tf().SetTransferFunction(jxl::TransferFunction::kLinear);info.color_encoding=dst;info.desired_intensity_target=255;info.cms_set=enabled;info.color_management_system=cms;
  auto stage=jxl::GetCmsStage(info,linear);std::cout<<" H "<<bool(stage);
  if(stage){std::cout<<' '<<stage->GetName();for(size_t c=0;c<5;++c)std::cout<<' '<<int(stage->GetChannelMode(c));std::vector<std::pair<size_t,size_t>> sizes(4,{width,1});auto set=stage->SetInputSizes(sizes);std::cout<<" Z "<<int(set.code());
    if(mutation){if(mutation==1)sizes.resize(2);if(mutation==2)++sizes[1].first;if(mutation==3)++sizes[1].second;if(mutation==4)for(auto& s:sizes)s.first=width?width-1:0;set=stage->SetInputSizes(sizes);std::cout<<" Z "<<int(set.code());}
    std::vector<jxl::ImageF> planes;for(size_t c=0;c<4;++c){auto made=jxl::ImageF::Create(mm,64,1);if(!made.ok())throw std::runtime_error("CMS row plane");planes.push_back(std::move(made).value_());for(size_t x=0;x<64;++x)planes.back().Row(0)[x]=RPValue(c,x,0,0,mode);}
    jxl::RenderPipelineStage::RowInfo rows(4),unused;for(size_t c=0;c<4;++c)rows[c].push_back(planes[c].Row(0));
    for(size_t i=0;i<repeats;++i){auto prep=stage->PrepareForThreads(threads);std::cout<<" T "<<int(prep.code());if(prep)for(size_t thread=0;thread<threads;++thread){size_t count=guard==3?width+1:guard==4?0:width;auto status=stage->ProcessRow(rows,unused,guard==1?1:0,guard==2?1:0,count,0,0,thread);std::cout<<" R "<<int(status.code());}}
    for(const auto& plane:planes){std::cout<<" P";for(size_t x=0;x<64;++x)std::cout<<' '<<SCBits<uint32_t>(plane.Row(0)[x]);}
  }
  stage.reset();CSTDump(p);return true;
}
