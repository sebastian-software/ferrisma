// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// The original builder/simple pipeline/write stage and unmodified upstream
// interpolation test stages execute all scheduling, mirroring, and copying.
#include "lib/jxl/render_pipeline/simple_render_pipeline.h"
#include "lib/jxl/render_pipeline/stage_write.h"
#include "lib/jxl/render_pipeline/test_render_pipeline_stages.h"
struct RPProbeState {size_t init=0,prepare=0,sizes=0,rows=0,padding=0;};
class RPProbe : public jxl::RenderPipelineStage {
 public:
  RPProbe(RPProbeState* state,unsigned failures,int failrow,bool change,size_t w,size_t h,jxl::FrameOrigin origin)
    :RenderPipelineStage(Settings()),state_(state),failures_(failures),failrow_(failrow),change_(change),w_(w),h_(h),origin_(origin){}
  jxl::Status IsInitialized() const override {++state_->init;return !(failures_&1);}
  jxl::Status PrepareForThreads(size_t) override {++state_->prepare;return !(failures_&2);}
  jxl::Status SetInputSizes(const std::vector<std::pair<size_t,size_t>>&) override {++state_->sizes;if(failures_&16)settings_.border_y=1;return !(failures_&4);}
  jxl::Status ProcessRow(const RowInfo& input,const RowInfo&,size_t,size_t,size_t width,size_t,size_t y,size_t) const override {
    ++state_->rows;if((failures_&8)&&int(y)==failrow_)return false;
    for(size_t c=0;c<input.size();++c){auto* row=GetInputRow(input,c,0);for(size_t x=0;x<width;++x)row[x]=row[x]+1.0f;}return true;
  }
  jxl::RenderPipelineChannelMode GetChannelMode(size_t) const override{return jxl::RenderPipelineChannelMode::kInPlace;}
  bool SwitchToImageDimensions() const override{return change_;}
  void GetImageDimensions(size_t* w,size_t* h,jxl::FrameOrigin* origin) const override{*w=w_;*h=h_;*origin=origin_;}
  void ProcessPaddingRow(const RowInfo& output,size_t width,size_t,size_t y) const override{
    ++state_->padding;for(size_t c=0;c<output.size();++c){float* row=output[c][0]+jxl::kRenderPipelineXOffset;for(size_t x=0;x<width;++x)row[x]=-float(c*19+x*3+y*7);}
  }
  const char* GetName() const override{return "fixture::probe";}
 private:RPProbeState* state_;unsigned failures_;int failrow_;bool change_;size_t w_,h_;jxl::FrameOrigin origin_;
};
float RPValue(size_t c,size_t x,size_t y,size_t pass,unsigned mode){
  if(mode==0)return 0.0f;
  if(mode==3){uint32_t words[]={0,0x80000000,0x7fa12345,0x7fc98765,0x7f800000,0xff800000,1,0x3f800000};return SCBits<float>(words[(c+x+y+pass)%8]);}
  return float(int((c*31+x*17+y*9+pass*11)%257)-128)/16.0f;
}
void RPPipeline(jxl::SimpleRenderPipeline& p){
  std::cout<<" P "<<p.PassesWithAllInput()<<' '<<p.processed_passes_;DimensionsObservation(p.frame_dimensions_);
  std::cout<<' '<<p.group_completed_passes_.size();for(auto v:p.group_completed_passes_)std::cout<<' '<<unsigned(v);
  for(const auto* table:{&p.channel_shifts_,&p.padding_}){std::cout<<' '<<table->size();for(const auto& stage:*table){std::cout<<' '<<stage.size();for(auto [x,y]:stage)std::cout<<' '<<x<<','<<y;}}
  std::cout<<' '<<p.channel_data_.size();for(const auto& ch:p.channel_data_){std::cout<<' '<<ch.xsize()<<','<<ch.ysize()<<','<<ch.bytes_per_row();
    // Before a complete pass, untouched groups still contain undefined source
    // pixels. Row padding is omitted in every state.
    if(p.processed_passes_)for(size_t y=jxl::kRenderPipelineXOffset;y<ch.ysize()-jxl::kRenderPipelineXOffset;++y)for(size_t x=jxl::kRenderPipelineXOffset;x<ch.xsize()-jxl::kRenderPipelineXOffset;++x)std::cout<<' '<<SCBits<uint32_t>(ch.Row(y)[x]);
  }
}
bool RenderPipelineCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm){
  if(command=="rp-set-image"){
    bool gray,profile_gray,extra;size_t w,h,ew,eh;in>>gray>>profile_gray>>w>>h>>extra>>ew>>eh;
    jxl::ImageMetadata metadata;if(!metadata.color_encoding.SetSRGB(gray?jxl::ColorSpace::kGray:jxl::ColorSpace::kRGB,jxl::RenderingIntent::kRelative))throw std::runtime_error("set image metadata");
    jxl::ColorEncoding profile;if(!profile.SetSRGB(profile_gray?jxl::ColorSpace::kGray:jxl::ColorSpace::kRGB,jxl::RenderingIntent::kRelative))throw std::runtime_error("set image profile");
    jxl::ImageBundle bundle(mm,&metadata);auto old=jxl::Image3F::Create(mm,1,1);if(!old.ok())throw std::runtime_error("set image old");auto old_image=std::move(old).value_();jxl::FillImage(1.0f,&old_image);if(!bundle.SetFromImage(std::move(old_image),metadata.color_encoding))throw std::runtime_error("set image old transfer");
    if(extra){auto made=jxl::ImageF::Create(mm,ew,eh);if(!made.ok())throw std::runtime_error("set image EC");auto ec=std::move(made).value_();if(ew&&eh)jxl::FillImage(2.0f,&ec);bundle.extra_channels().push_back(std::move(ec));}
    auto made=jxl::Image3F::Create(mm,w,h);if(!made.ok())throw std::runtime_error("set image input");auto input=std::move(made).value_();if(w&&h)jxl::FillImage(SCBits<float>(uint32_t{0x7fa12345}),&input);
    auto* address=input.Plane(0).bytes();auto status=bundle.SetFromImage(std::move(input),profile);bool transferred=address&&bundle.color_.Plane(0).bytes()==address;
    std::cout<<int(status.code())<<' '<<transferred;if(!transferred)std::cout<<' '<<input.xsize()<<' '<<input.ysize();
    std::cout<<' '<<int(bundle.VerifySizes().code())<<' '<<bundle.xsize()<<' '<<bundle.ysize();
    for(size_t c=0;c<3;++c)for(size_t y=0;y<bundle.color_.ysize();++y)for(size_t x=0;x<bundle.color_.xsize();++x)std::cout<<' '<<SCBits<uint32_t>(bundle.color_.PlaneRow(c,y)[x]);
    ColorBundleObservation(bundle.c_current());return true;
  }
  if(command=="rp-default"){jxl::RenderPipelineInput a;auto s=a.Done();auto b=std::move(a);std::cout<<int(s.code())<<' '<<int(a.Done().code())<<' '<<int(b.Done().code());return true;}
  if(command!="rp-simple")return false;
  size_t w,h,channels,passes,threads,iw,ih;unsigned shifts,mode,failures,build;int failrow;bool group_ids,reverse,move,change;jxl::FrameOrigin origin;
  in>>w>>h>>channels>>shifts>>passes>>threads>>group_ids>>reverse>>move>>mode>>failures>>failrow>>change>>iw>>ih>>origin.x0>>origin.y0>>build;
  jxl::FrameDimensions dim;dim.Set(w,h,0,0,0,true,1);RPProbeState probe;jxl::Image3F output;
  bool use_bundle=(mode&16)!=0,gray=(mode&32)!=0;jxl::ImageMetadata metadata;metadata.num_extra_channels=channels>3?channels-3:0;metadata.extra_channel_info.resize(metadata.num_extra_channels);
  if(!metadata.color_encoding.SetSRGB(gray?jxl::ColorSpace::kGray:jxl::ColorSpace::kRGB,jxl::RenderingIntent::kRelative))throw std::runtime_error("pipeline bundle color");jxl::ImageBundle bundle(mm,&metadata);
  {
    jxl::RenderPipeline::Builder builder(mm,channels);builder.UseSimpleImplementation();
    if(build&1)std::cout<<" N "<<int(builder.AddStage(nullptr).code());
    if(shifts&1){if(!builder.AddStage(std::make_unique<jxl::UpsampleXSlowStage>()))throw std::runtime_error("pipeline X");}
    if(shifts&2){if(!builder.AddStage(std::make_unique<jxl::UpsampleYSlowStage>()))throw std::runtime_error("pipeline Y");}
    if(change||failures){if(!builder.AddStage(std::make_unique<RPProbe>(&probe,failures,failrow,change,iw,ih,origin)))throw std::runtime_error("pipeline probe");}
    if(!(build&2)){if(!builder.AddStage(use_bundle?jxl::GetWriteToImageBundleStage(&bundle,&metadata.color_encoding):jxl::GetWriteToImage3FStage(mm,&output)))throw std::runtime_error("pipeline write");}
    auto made=std::move(builder).Finalize(dim);std::cout<<" B "<<int(made.status().code());
    if(made.ok()){
      auto base=std::move(made).value_();auto& p=*static_cast<jxl::SimpleRenderPipeline*>(base.get());auto initialized=p.IsInitialized();std::cout<<" I "<<int(initialized.code());
      if(initialized){auto prep=p.PrepareForThreads(threads,group_ids);std::cout<<" T "<<int(prep.code());
        if(prep){for(size_t pass=0;pass<passes;++pass){
          // Dimension-changing fixtures use one pass; this is the original
          // source's foreground/background transition, not a new frame reset.
          for(size_t i=0;i<dim.num_groups;++i){size_t group=reverse?dim.num_groups-i-1:i;auto input=p.GetInputBuffers(group,0);
            std::cout<<" R "<<group;
            for(size_t c=0;c<p.channel_data_.size();++c){const auto& [plane,r]=input.GetBuffer(c);std::cout<<' '<<r.x0()<<','<<r.y0()<<','<<r.xsize()<<','<<r.ysize();
              for(size_t y=0;y<r.ysize();++y)for(size_t x=0;x<r.xsize();++x)r.Row(plane,y)[x]=RPValue(gray&&c<3?0:c,r.x0()+x-jxl::kRenderPipelineXOffset,r.y0()+y-jxl::kRenderPipelineXOffset,pass,mode&15);
            }
            jxl::Status status=true;if(move){auto moved=std::move(input);std::cout<<" M "<<int(input.Done().code());status=moved.Done();}else status=input.Done();
            std::cout<<" D "<<int(status.code());RPPipeline(p);p.ClearDone(group);
          }
          auto again=p.PrepareForThreads(threads,group_ids);std::cout<<" A "<<int(again.code());
        }}
      }
    }
  }
  std::cout<<" C "<<probe.init<<' '<<probe.prepare<<' '<<probe.sizes<<' '<<probe.rows<<' '<<probe.padding;
  const auto& color=use_bundle?bundle.color_:output;
  std::cout<<" O "<<color.xsize()<<' '<<color.ysize();for(size_t c=0;c<3;++c)for(size_t y=0;y<color.ysize();++y)for(size_t x=0;x<color.xsize();++x)std::cout<<' '<<SCBits<uint32_t>(color.PlaneRow(c,y)[x]);
  if(use_bundle){std::cout<<" E "<<bundle.extra_channels().size();for(const auto& ec:bundle.extra_channels()){std::cout<<' '<<ec.xsize()<<','<<ec.ysize();for(size_t y=0;y<ec.ysize();++y)for(size_t x=0;x<ec.xsize();++x)std::cout<<' '<<SCBits<uint32_t>(ec.Row(y)[x]);}ColorBundleObservation(bundle.c_current());}
  return true;
}
