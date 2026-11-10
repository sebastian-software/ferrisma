// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// The actual pristine implementation is included without copied source bodies.
#include "lib/jxl/cms/jxl_cms.cc"
#include "default_cms_observe.h"
namespace {
void ColorState(const jxl::cms::ColorEncoding& c) {
  std::cout<<' '<<uint32_t(c.white_point)<<' '<<uint32_t(c.primaries)<<' '<<uint32_t(c.rendering_intent)
    <<' '<<c.have_fields<<' '<<uint32_t(c.color_space)<<' '<<c.cmyk<<' '<<c.tf.have_gamma<<' '<<c.tf.gamma<<' '<<uint32_t(c.tf.transfer_function);
  for(const auto* xy:{&c.white,&c.red,&c.green,&c.blue})std::cout<<' '<<uint32_t(xy->x)<<' '<<uint32_t(xy->y);
  std::cout<<' '<<c.icc.size();for(uint8_t x:c.icc)std::cout<<' '<<unsigned(x);
}
bool Private(const std::string& command,std::istringstream& in) {
  if(command=="default-cms-numeric") {
    unsigned which;in>>which;
    if(which==0) {
      jxl::Color xyz;for(float& x:xyz){uint32_t u;in>>u;x=default_cms_observer::Float(u);}
      jxl::cms::CIExy(*volatile fn)(const jxl::Color&)=&jxl::CIExyFromXYZ;const auto xy=fn(xyz);
      uint64_t x,y;std::memcpy(&x,&xy.x,8);std::memcpy(&y,&xy.y,8);std::cout<<x<<' '<<y;
    }else if(which==1) {
      skcms_Matrix3x3 matrix;for(auto& row:matrix.vals)for(float& x:row){uint32_t u;in>>u;x=default_cms_observer::Float(u);}
      bool(*volatile fn)(const skcms_Matrix3x3&)=&jxl::IsAcceptableCHAD;std::cout<<fn(matrix);
    }else if(which==2) {
      uint32_t tf,primaries;in>>tf>>primaries;
      bool(*volatile fn)(jxl::cms::TransferFunction)=&jxl::IsKnownTransferFunction;
      bool(*volatile known)(uint8_t)=&jxl::IsKnownColorPrimaries;
      std::cout<<fn(static_cast<jxl::cms::TransferFunction>(tf))<<' '<<known(uint8_t(primaries));
    }else if(which==3) {
      skcms_ICCProfile profile;in>>profile.data_color_space;
      jxl::cms::ColorSpace(*volatile fn)(const skcms_ICCProfile&)=&jxl::ColorSpaceFromProfile;
      std::cout<<uint32_t(fn(profile));
    }else std::abort();return true;
  }
  if(command=="default-cms-allocate") {
    size_t initial,count;in>>initial>>count;std::vector<float> storage(initial);std::vector<float*> views;
    for(size_t i=0;i<initial;++i)storage[i]=default_cms_observer::Float(0x3f000000+uint32_t(i));
    void(*volatile fn)(size_t,size_t,std::vector<float>*,std::vector<float*>*)=&jxl::AllocateBuffer;
    std::cout<<count;
    for(size_t step=0;step<count;++step) {
      size_t length,threads;in>>length>>threads;fn(length,threads,&storage,&views);
      std::cout<<' '<<storage.size();for(float x:storage)std::cout<<' '<<default_cms_observer::Word(x);
      std::cout<<' '<<views.size();
      for(float* view:views)std::cout<<' '<<(uintptr_t(view)%128)<<' '<<uint64_t(uintptr_t(view)-uintptr_t(views[0]))
        <<' '<<(uintptr_t(view)>=uintptr_t(storage.data())&&uintptr_t(view)<=uintptr_t(storage.data()+storage.size()));
    }return true;
  }
  if(command=="default-cms-luminances"||command=="default-cms-profile-equal") {
    std::string hex,other;unsigned null_output=0;in>>hex;
    if(command=="default-cms-luminances")in>>null_output;else in>>other;
    auto bytes=default_cms_observer::Unhex(hex);jxl::cms::ColorEncoding encoding;
    const auto* cms=JxlGetDefaultCms();const bool fields=encoding.SetFieldsFromICC(std::move(bytes),*cms);std::cout<<fields;
    if(!fields)return true;
    if(command=="default-cms-luminances") {
      float output[3];for(float& x:output)x=default_cms_observer::Float(0x3f123456);
      jxl::Status(*volatile fn)(const jxl::cms::ColorEncoding&,float*)=&jxl::GetPrimariesLuminances;
      std::cout<<' '<<bool(fn(encoding,null_output?nullptr:output));for(float x:output)std::cout<<' '<<default_cms_observer::Word(x);
    }else {
      auto input=default_cms_observer::Unhex(other);skcms_ICCProfile profile;
      const bool parsed=skcms_Parse(input.data(),input.size(),&profile);std::cout<<' '<<parsed;
      if(parsed){bool(*volatile fn)(const skcms_ICCProfile&,const jxl::cms::ColorEncoding&)=&jxl::IsApproximatelyEqual;std::cout<<' '<<fn(profile,encoding);}
    }return true;
  }
  if(command=="default-cms-prefix") {
    unsigned which,mode;std::string hex;in>>which>>mode>>hex;
    skcms_ICCProfile profile;uint8_t backing[160];jxl::Color media,adapted;
    adapted.fill(default_cms_observer::Float(0x3f123456));
    const char* signature="chad";auto bytes=default_cms_observer::Unhex(hex);const size_t size=bytes.size();bytes.resize(size+16);
    if(which==0) {
      if(mode==0)profile.buffer=nullptr;
      else {profile.buffer=backing;profile.size=mode==1?127:128;
        const size_t offsets[]={8,12,13,14,15,36,37,38,39};const uint8_t expected[]={4,'m','n','t','r','a','c','s','p'};
        if(mode>=2)for(size_t i=0;i<9;++i){if(i<mode-2)backing[offsets[i]]=expected[i];else if(i==mode-2){backing[offsets[i]]=0;break;}}
      }
    }else if(which==1||which==2) {
      if(mode==0)profile.buffer=nullptr;
      else {profile.buffer=backing;profile.size=mode==1?131:(mode==2?132:144);
        if(mode>=2){const uint32_t count=mode==2?0:(mode==3?2:1);for(size_t i=0;i<4;++i)backing[128+i]=uint8_t(count>>(24-i*8));}
        if(mode==2||mode==3)signature=nullptr;
        if(mode>=4)std::memcpy(backing+132,mode==4?"chad":(mode==5?"A2B0":(mode==6?"B2D3":"ZZZZ")),4);
      }
    }else if(which==3) {
      for(size_t i=0;i<3;++i){if(i<mode)profile.trc[i].table_entries=0;else if(i==mode){profile.trc[i].table_entries=1;break;}}
    }else if(which==4||which==5) {
      if(mode==0){if(which==5)profile.trc[0].table_entries=1;else profile.buffer=nullptr;}
      else if(mode==1){for(auto& trc:profile.trc)trc.table_entries=0;profile.buffer=nullptr;}
      else {
        if(!skcms_Parse(bytes.data(),size,&profile))std::abort();
        media[0]=mode==2?0.0f:0.96420288f;
        if(mode>=3)media[1]=mode==3?0.0f:1.0f;
        if(mode>=4)media[2]=mode==4?0.0f:0.82490540f;
        if(mode>=6)media={0.95f,1.0f,1.09f};
      }
    }else if(which==6) {
      media[0]=mode==0?0.0f:0.96420288f;if(mode>=1)media[1]=mode==1?0.0f:1.0f;if(mode>=2)media[2]=mode==2?0.0f:0.82490540f;
    }else std::abort();
    jxl::Color& destination=(which==4||which==5)&&mode==7?media:adapted;bool status;
    if(which==0){bool(*volatile fn)(const skcms_ICCProfile&)=&jxl::HasICCVersion4DisplayClass;status=fn(profile);}
    else if(which==1){bool(*volatile fn)(const skcms_ICCProfile&,const char*)=&jxl::HasTag;status=fn(profile,signature);}
    else if(which==2){bool(*volatile fn)(const skcms_ICCProfile&)=&jxl::HasLUTTags;status=fn(profile);}
    else if(which==3){bool(*volatile fn)(const skcms_ICCProfile&)=&jxl::HasParametricTRCs;status=fn(profile);}
    else if(which==4){bool(*volatile fn)(const skcms_ICCProfile&,const jxl::Color&,jxl::Color&)=&jxl::GetNonconformingMatrixProfileWhitePoint;status=fn(profile,media,destination);}
    else if(which==5){bool(*volatile fn)(const skcms_ICCProfile&,const jxl::Color&,jxl::Color&)=&jxl::RecoverMatrixProfileWhitePoint;status=fn(profile,media,destination);}
    else {bool(*volatile fn)(const jxl::Color&)=&jxl::IsApproximatelyD50;status=fn(media);}
    std::cout<<status;for(float x:destination)std::cout<<' '<<default_cms_observer::Word(x);return true;
  }
  if(command=="default-cms-private-bindings") {
    std::cout<<reinterpret_cast<uintptr_t>(&jxl::CIExyFromXYZ)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::DecodeProfile)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::ColorSpaceFromProfile)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::MatrixProduct)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::IsAcceptableCHAD)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::HasICCVersion4DisplayClass)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::HasTag)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::HasLUTTags)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::HasParametricTRCs)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::IsApproximatelyD50)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::GetNonconformingMatrixProfileWhitePoint)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::RecoverMatrixProfileWhitePoint)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::UnadaptedWhitePoint)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::IdentifyPrimaries)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::IsApproximatelyEqual)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::DetectTransferFunction)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::GetPrimariesLuminances)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::ApplyHlgOotf)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::IsKnownTransferFunction)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::IsKnownColorPrimaries)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::ApplyCICP)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::AllocateBuffer)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::N_SCALAR::BeforeTransform)<<' '
      <<reinterpret_cast<uintptr_t>(&jxl::N_SCALAR::AfterTransform);return true;
  }
  if(command=="default-cms-cicp") {
    unsigned primaries,transfer,matrix,range,mode;in>>primaries>>transfer>>matrix>>range>>mode;
    jxl::cms::ColorEncoding c;c.white_point=jxl::cms::WhitePoint::kCustom;c.primaries=jxl::cms::Primaries::kCustom;
    c.rendering_intent=jxl::cms::RenderingIntent::kAbsolute;c.have_fields=false;c.color_space=jxl::cms::ColorSpace::kGray;c.cmyk=true;
    c.tf.have_gamma=true;c.tf.gamma=1234567;c.tf.transfer_function=jxl::cms::TransferFunction::kUnknown;
    c.white={123,-456};c.red={-234,567};c.green={345,-678};c.blue={-456,789};c.icc={17,34,51};
    bool(*volatile fn)(uint8_t,uint8_t,uint8_t,uint8_t,jxl::cms::ColorEncoding*)=&jxl::ApplyCICP;
    const bool status=fn(uint8_t(primaries),uint8_t(transfer),uint8_t(matrix),uint8_t(range),mode==1?nullptr:&c);std::cout<<status;
    if(mode==2) {
      if(status)std::cout<<' '<<uint32_t(c.color_space)<<' '<<c.tf.have_gamma<<' '<<uint32_t(c.tf.transfer_function)<<' '<<uint32_t(c.white_point)<<' '<<uint32_t(c.primaries);
    }else ColorState(c);
    return true;
  }
  if(command=="default-cms-tf") {
    unsigned operation,mask,apply,forward,alias,nulls;int32_t code;uint32_t intensity;
    size_t channels,size,count;in>>operation>>code>>mask>>intensity>>apply>>channels>>forward>>size>>alias>>nulls>>count;
    std::array<float,3> luminances;for(float& x:luminances){uint32_t u;in>>u;x=default_cms_observer::Float(u);}
    // Default initialization constructs the vectors and leaves non-defaulted
    // scalar members genuinely unknown. Only explicit fixture fields are set.
    jxl::JxlCms context;
    if(operation==0)context.preprocess=static_cast<jxl::ExtraTF>(code);
    if(operation==1)context.postprocess=static_cast<jxl::ExtraTF>(code);
    if(mask&1)context.intensity_target=default_cms_observer::Float(intensity);
    if(mask&2)context.apply_hlg_ootf=apply!=0;
    if(mask&4)context.hlg_ootf_num_channels=channels;
    if(mask&8)context.hlg_ootf_luminances=luminances;
    std::vector<float> input(count+16,default_cms_observer::Float(0x3f123456));
    std::vector<float> output(count+16,default_cms_observer::Float(0xbf654321));
    for(size_t i=0;i<count;++i){uint32_t u;in>>u;input[i+1]=default_cms_observer::Float(u);if(operation!=0)output[i+1]=input[i+1];}
    const float* source=input.data()+1;float* destination=output.data()+1;
    if(alias)destination=input.data()+(alias==1?1:(alias==2?2:0));
    if(nulls&1)source=nullptr;if(nulls&2)destination=nullptr;
    bool status;
    if(operation==0){jxl::Status(*volatile fn)(jxl::JxlCms*,const float*,float*,size_t)=&jxl::N_SCALAR::BeforeTransform;status=bool(fn(&context,source,destination,size));}
    else if(operation==1){jxl::Status(*volatile fn)(jxl::JxlCms*,float*,size_t)=&jxl::N_SCALAR::AfterTransform;status=bool(fn(&context,destination,size));}
    else {jxl::Status(*volatile fn)(jxl::JxlCms*,float*,size_t,bool)=&jxl::ApplyHlgOotf;status=bool(fn(&context,destination,size,forward!=0));}
    std::cout<<status<<' '<<input.size();for(float x:input)std::cout<<' '<<default_cms_observer::Word(x);
    std::cout<<' '<<output.size();for(float x:output)std::cout<<' '<<default_cms_observer::Word(x);return true;
  }
  if(command=="default-cms-profile-detail") {
    std::string hex;in>>hex;auto bytes=default_cms_observer::Unhex(hex);const size_t size=bytes.size();bytes.resize(size+16);
    skcms_ICCProfile profile;const bool parsed=skcms_Parse(bytes.data(),size,&profile);std::cout<<parsed;if(!parsed)return true;
    jxl::cms::ColorEncoding encoding;encoding.color_space=jxl::ColorSpaceFromProfile(profile);
    jxl::cms::CIExy white={0.25,0.75};
    jxl::Status (*volatile unadapted)(const skcms_ICCProfile&,jxl::cms::CIExy*)=&jxl::UnadaptedWhitePoint;
    const bool wp=bool(unadapted(profile,&white));std::cout<<' '<<wp;
    default_cms_observer::Raw<uint64_t>(&white,offsetof(jxl::cms::CIExy,x));default_cms_observer::Raw<uint64_t>(&white,offsetof(jxl::cms::CIExy,y));
    if(!wp)return true;
    const bool set=bool(encoding.SetWhitePoint(white));std::cout<<' '<<set;if(!set)return true;
    jxl::Status (*volatile identify)(const skcms_ICCProfile&,const jxl::cms::CIExy&,jxl::cms::ColorEncoding*)=&jxl::IdentifyPrimaries;
    const bool primaries=bool(identify(profile,white,&encoding));std::cout<<' '<<primaries;default_cms_observer::Encoding(encoding.ToExternal());if(!primaries)return true;
    jxl::Status (*volatile detect)(const skcms_ICCProfile&,jxl::cms::ColorEncoding*)=&jxl::DetectTransferFunction;
    std::cout<<' '<<bool(detect(profile,&encoding));default_cms_observer::Encoding(encoding.ToExternal());return true;
  }
  if(command=="default-cms-matrix-product") {
    skcms_Matrix3x3 matrix; jxl::Color input,output;
    for(auto& row:matrix.vals)for(float& x:row){uint32_t u;in>>u;x=default_cms_observer::Float(u);}
    for(float& x:input){uint32_t u;in>>u;x=default_cms_observer::Float(u);}
    unsigned alias;in>>alias;output.fill(default_cms_observer::Float(0x3f123456));
    void (*volatile source)(const skcms_Matrix3x3&,const jxl::Color&,jxl::Color&)=&jxl::MatrixProduct;
    source(matrix,input,alias?input:output);
    for(float x:(alias?input:output))std::cout<<default_cms_observer::Word(x)<<' ';
    return true;
  }
  return false;
}
}
int main(){return default_cms_observer::Main(&Private);}
