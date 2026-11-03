// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Data-only adapters. Transform/profile algorithms bind the pristine CMS archive.
#include <cstdint>
#include <cstring>
#include <cstdlib>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include "third_party/skcms/skcms.h"
#include "third_party/skcms/src/skcms_internals.h"
std::vector<uint8_t> Unhex(const std::string&);
void SkcmsObserveProfile(const skcms_ICCProfile&,const uint8_t*);
namespace {
void Hex(const uint8_t* p,size_t n) {static const char digits[]="0123456789abcdef";for(size_t i=0;i<n;++i)std::cout<<digits[p[i]>>4]<<digits[p[i]&15];}
struct Profile {
    skcms_ICCProfile value;std::vector<uint8_t> bytes;const skcms_ICCProfile* p=nullptr;bool valid=true;
    Profile(int kind,const std::string& data) {
        std::memset(&value,0xa5,sizeof(value));
        if(kind<0)return;
        if(kind==0){p=skcms_sRGB_profile();return;}
        if(kind==1){p=skcms_XYZD50_profile();return;}
        p=&value;
        if(kind==2){bytes=Unhex(data);valid=skcms_Parse(bytes.data(),bytes.size(),&value);return;}
        std::memcpy(&value,kind==4?skcms_XYZD50_profile():skcms_sRGB_profile(),sizeof(value));
        if(kind==5||kind==6){value.has_CICP=true;value.CICP.transfer_characteristics=kind==5?16:18;}
        if(kind==7)for(int i=0;i<3;++i)value.trc[i].parametric={(i==0?1.8f:i==1?2.0f:2.2f),1,0,0,0,0,0};
        if(kind==8){skcms_TransferFunction f;skcms_TransferFunction_makePQish(&f,-107/128.f,1,32/2523.f,2413/128.f,-2392/128.f,8192/1305.f);skcms_SetTransferFunction(&value,&f);}
        if(kind==9){skcms_TransferFunction f;skcms_TransferFunction_makeHLGish(&f,2,2,1/.17883277f,.28466892f,.55991073f);skcms_SetTransferFunction(&value,&f);}
        if(kind==10)skcms_SetTransferFunction(&value,skcms_Identity_TransferFunction());
        if(kind==11)value.has_toXYZD50=false;
        if(kind==12)skcms_SetXYZD50(&value,&skcms_XYZD50_profile()->toXYZD50);
    }
};
void Normalize(std::vector<uint8_t>& bytes,const uint8_t* base) {
    for(size_t i=0;i<3;++i){size_t start=offsetof(skcms_ICCProfile,trc)+i*sizeof(skcms_Curve);uint32_t count;std::memcpy(&count,bytes.data()+start,4);
        if(count)for(size_t offset:{offsetof(skcms_Curve,table_8),offsetof(skcms_Curve,table_16)}) {
            uintptr_t pointer;std::memcpy(&pointer,bytes.data()+start+offset,sizeof(pointer));uintptr_t token=pointer?pointer-uintptr_t(base):UINTPTR_MAX;std::memcpy(bytes.data()+start+offset,&token,sizeof(token));
        }
    }
}
void Changes(const skcms_ICCProfile& value,std::vector<uint8_t> old,const uint8_t* base) {
    std::vector<uint8_t> after(sizeof(value));std::memcpy(after.data(),&value,sizeof(value));Normalize(old,base);Normalize(after,base);
    std::cout<<" D";for(size_t i=0;i<sizeof(value);++i)if(after[i]!=old[i])std::cout<<' '<<i<<':'<<unsigned(after[i]);
}
}
bool SkcmsTransformCommand(const std::string& command,std::istringstream& in) {
    if(command.rfind("skcms-transform",0)!=0)return false;
    if(command=="skcms-transform-unknown") {
        int mode;in>>mode;skcms_ICCProfile value;
        if(mode==0){value.has_B2A=true;std::cout<<skcms_MakeUsableAsDestination(&value);}
        if(mode==1){value.has_B2A=false;value.has_trc=false;std::cout<<skcms_MakeUsableAsDestination(&value);}
        if(mode==2){std::cout<<skcms_TRCs_AreApproximateInverse(nullptr,reinterpret_cast<const skcms_TransferFunction*>(uintptr_t(1)));}
        if(mode==3){std::cout<<skcms_ApproximatelyEqualProfiles(nullptr,nullptr);}
        if(mode==4){value.data_color_space=skcms_Signature_RGB;alignas(16) float pixels[16];for(int i=0;i<16;++i)pixels[i]=float(i)/7;
            bool ok=skcms_Transform(pixels,skcms_PixelFormat_RGBA_ffff,skcms_AlphaFormat_Unpremul,&value,pixels,skcms_PixelFormat_RGBA_ffff,skcms_AlphaFormat_Unpremul,&value,4);
            std::cout<<ok<<' ';Hex(reinterpret_cast<const uint8_t*>(pixels),sizeof(pixels));}
        if(mode==5){value.has_B2A=false;value.has_trc=value.has_toXYZD50=true;value.toXYZD50={{{1,0,0},{0,1,0},{0,0,1}}};value.trc[0].table_entries=0;value.trc[0].parametric.g=-129;std::cout<<skcms_MakeUsableAsDestination(&value);}
        if(mode==6){value.has_trc=true;value.trc[0].table_entries=0;value.trc[0].parametric.g=-129;skcms_TransferFunction inverse;inverse.g=-129;std::cout<<skcms_TRCs_AreApproximateInverse(&value,&inverse);}
        if(mode==7){value.data_color_space=skcms_Signature_RGB;value.has_trc=value.has_toXYZD50=true;value.has_A2B=value.has_CICP=false;value.toXYZD50={{{1,0,0},{0,1,0},{0,0,1}}};for(auto& curve:value.trc){curve.table_entries=0;curve.parametric.g=-129;}alignas(16)float pixels[16];for(int i=0;i<16;++i)pixels[i]=float(i)/7;bool ok=skcms_Transform(pixels,skcms_PixelFormat_RGBA_ffff,skcms_AlphaFormat_Unpremul,&value,pixels,skcms_PixelFormat_RGBA_ffff,skcms_AlphaFormat_Unpremul,skcms_XYZD50_profile(),4);std::cout<<ok<<' ';Hex(reinterpret_cast<const uint8_t*>(pixels),sizeof(pixels));}
        if(mode>=8&&mode<=17){
            const uint32_t counts[]={0xffffffffu,0xfffffffeu,0xffffff80u,0x80000001u,0x80000000u};
            const bool destination=(mode>=12&&mode<=15)||mode==17;
            const uint32_t count=counts[mode>=16?4:(mode-8)%4];
            value.data_color_space=skcms_Signature_RGB;value.pcs=skcms_Signature_XYZ;value.has_CICP=false;
            if(destination){value.has_B2A=true;value.B2A.input_channels=value.B2A.matrix_channels=0;value.B2A.output_channels=count;}
            else{value.has_A2B=true;value.A2B.input_channels=count;value.A2B.matrix_channels=value.A2B.output_channels=0;}
            const auto* xyz=skcms_XYZD50_profile();
            std::cout<<skcms_Transform(nullptr,skcms_PixelFormat_RGB_fff,skcms_AlphaFormat_Opaque,destination?xyz:&value,nullptr,skcms_PixelFormat_RGB_fff,skcms_AlphaFormat_Opaque,destination?&value:xyz,0);
        }
        return true;
    }
    if(command=="skcms-transform") {
        int sf,sa,sk,df,da,dk,mode;size_t n;std::string payload,ps,pd;
        in>>sf>>sa>>sk>>df>>da>>dk>>n>>mode>>payload>>ps>>pd;
        Profile srcp(sk,ps),dstp(dk,pd);
        auto bytes=Unhex(payload);if(bytes.size()>512)std::abort();
        std::vector<uint64_t> source(64,0xa5a5a5a5a5a5a5a5ull),destination(64,0xa5a5a5a5a5a5a5a5ull);
        auto* src=reinterpret_cast<uint8_t*>(source.data());auto* dst=reinterpret_cast<uint8_t*>(destination.data());
        if(!bytes.empty())std::memcpy(src,bytes.data(),bytes.size());
        if(mode==1)dst=src;if(mode==2)dst=src+16;
        bool ok=srcp.valid&&dstp.valid&&skcms_Transform(src,static_cast<skcms_PixelFormat>(sf),static_cast<skcms_AlphaFormat>(sa),srcp.p,dst,static_cast<skcms_PixelFormat>(df),static_cast<skcms_AlphaFormat>(da),dstp.p,n);
        std::cout<<srcp.valid<<' '<<dstp.valid<<' '<<ok<<' ';Hex(mode?src:dst,512);std::cout<<' ';Hex(src,512);return true;
    }
    if(command=="skcms-transform-usable"||command=="skcms-transform-single") {
        int kind;std::string data;in>>kind>>data;Profile profile(kind,data);
        if(!profile.valid){std::cout<<"P";return true;}
        std::vector<uint8_t> before(sizeof(profile.value));std::memcpy(before.data(),&profile.value,sizeof(profile.value));
        bool ok=command=="skcms-transform-single"?skcms_MakeUsableAsDestinationWithSingleCurve(&profile.value):skcms_MakeUsableAsDestination(&profile.value);
        std::cout<<ok<<' ';SkcmsObserveProfile(profile.value,profile.bytes.data());Changes(profile.value,before,profile.bytes.data());return true;
    }
    if(command=="skcms-transform-equal") {
        int a,b;std::string ad,bd;in>>a>>b>>ad>>bd;Profile ap(a,ad),bp(b,bd);
        std::cout<<ap.valid<<' '<<bp.valid<<' '<<(ap.valid&&bp.valid&&skcms_ApproximatelyEqualProfiles(ap.p,bp.p));return true;
    }
    if(command=="skcms-transform-inverse") {
        int kind;std::string data;in>>kind>>data;Profile p(kind,data);skcms_TransferFunction tf;
        uint32_t* words=reinterpret_cast<uint32_t*>(&tf);for(int i=0;i<7;++i){uint32_t word;in>>word;std::memcpy(words+i,&word,4);}
        std::cout<<p.valid<<' '<<(p.valid&&skcms_TRCs_AreApproximateInverse(p.p,&tf));return true;
    }
    std::abort();
}
