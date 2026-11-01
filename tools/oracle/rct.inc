// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Call actual scalar-dispatched RCT APIs with the original null-pool runner.
#include "lib/jxl/modular/transform/rct.h"
#include "lib/jxl/modular/transform/enc_rct.h"
void RctFill(jxl::Image& image,std::istringstream& in) {
  for(auto& c:image.channel) {
    ImagePaint(c.plane,81+c.component);
    for(size_t y=0;y<c.h;++y) for(size_t x=0;x<c.w;++x) in>>c.Row(y)[x];
  }
}
bool RctCommand(const std::string& command,std::istringstream& in,JxlMemoryManager* mm) {
  if(command=="rct-image") {
    unsigned op,mismatch;size_t w,h,begin,type,meta;int n;
    in>>op>>w>>h>>n>>begin>>type>>meta>>mismatch;
    auto result=jxl::Image::Create(mm,w,h,16,n);if(!result.ok()) throw std::runtime_error("RCT image input");
    auto image=std::move(result).value_();RctFill(image,in);image.nb_meta_channels=meta;
    for(size_t c=0;c<image.channel.size();++c) image.channel[c].hshift=c/3;
    if(begin+1<image.channel.size()) {if(mismatch==1) image.channel[begin+1].vshift=1;if(mismatch==2) image.channel[begin+1].w=0;}
    Status status=op==1?jxl::InvRCT(image,begin,type,nullptr):jxl::FwdRct(image,begin,type,nullptr);
    std::cout<<Code(status);ImageModularObservation(image);
    if(op==2) {status=jxl::InvRCT(image,begin,type,nullptr);std::cout<<' '<<Code(status);ImageModularObservation(image);}
  } else if(command=="rct-channels") {
    size_t w,h,type;in>>w>>h>>type;auto a=jxl::Image::Create(mm,w,h,16,3),b=jxl::Image::Create(mm,w,h,16,3);
    if(!a.ok()||!b.ok()) throw std::runtime_error("RCT channel input");
    auto from=std::move(a).value_(),to=std::move(b).value_();RctFill(from,in);
    for(size_t c=0;c<3;++c) {from.channel[c].hshift=c;to.channel[c].hshift=3-c;to.channel[c].component=c+3;ImagePaint(to.channel[c].plane,91+c);}
    Status status=jxl::FwdRct({&from.channel[0],&from.channel[1],&from.channel[2]},{&to.channel[0],&to.channel[1],&to.channel[2]},type,nullptr);
    std::cout<<Code(status);ImageModularObservation(from);ImageModularObservation(to);
  } else if(command=="rct-range-wrap") {
    unsigned op;size_t type;in>>op>>type;auto created=jxl::Image::Create(mm,3,1,16,3);if(!created.ok()) throw std::runtime_error("RCT range input");auto image=std::move(created).value_();
    for(size_t c=0;c<3;++c) {ImagePaint(image.channel[c].plane,101+c);for(size_t x=0;x<3;++x) image.channel[c].Row(0)[x]=c*3+x;image.channel[c].h=size_t{1}<<32;}
    Status status=op?jxl::InvRCT(image,0,type,nullptr):jxl::FwdRct(image,0,type,nullptr);std::cout<<Code(status);ImageModularObservation(image);
  } else return false;
  return true;
}
