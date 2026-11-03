// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Include the pristine original body; do not reimplement its private algorithms.
#include "../../upstream/libjxl/third_party/skcms/skcms.cc"
#include "skcms_transform.cc"
#include <iomanip>
#include <array>
std::vector<uint8_t> Unhex(const std::string& s) {
    if(s=="-")return {};std::vector<uint8_t> out;
    for(size_t i=0;i<s.size();i+=2)out.push_back(static_cast<uint8_t>(std::stoul(s.substr(i,2),nullptr,16)));return out;
}
namespace {
void TfWords(const skcms_TransferFunction& tf) {
    const float words[]={tf.g,tf.a,tf.b,tf.c,tf.d,tf.e,tf.f};
    for(float v:words){uint32_t u;std::memcpy(&u,&v,4);std::cout<<' '<<u;}
}
struct StageContext {skcms_TransferFunction tf;skcms_Curve curve;skcms_Matrix3x3 m3;skcms_Matrix3x4 m4;skcms_A2B a2b;skcms_B2A b2a;std::vector<uint8_t> bytes;};
Op ParseOp(const std::string& text) {
#define M(name) if(text==#name)return Op::name;
    SKCMS_WORK_OPS(M) SKCMS_STORE_OPS(M)
#undef M
    std::abort();
}
void Program(std::istringstream& in) {
    int n;size_t sbpp,dbpp,count;std::string payload;in>>n>>sbpp>>dbpp>>payload>>count;if(count>32)std::abort();
    auto input=Unhex(payload);if(input.size()>512)std::abort();std::vector<uint64_t> src(64,0xa5a5a5a5a5a5a5a5ull),dst(64,0xa5a5a5a5a5a5a5a5ull);
    if(!input.empty())std::memcpy(src.data(),input.data(),input.size());
    std::vector<StageContext> holders(32);Op ops[32];const void* args[32]={};
    for(size_t i=0;i<count;++i) {
        std::string name;in>>name;Op op=ops[i]=ParseOp(name);auto& c=holders[i];
        if(op==Op::matrix_3x3||op==Op::matrix_3x4) {
            float words[12];size_t k=op==Op::matrix_3x3?9:12;for(size_t j=0;j<k;++j){uint32_t u;in>>u;std::memcpy(words+j,&u,4);}
            if(k==9){std::memcpy(&c.m3,words,36);args[i]=&c.m3;}else{std::memcpy(&c.m4,words,48);args[i]=&c.m4;}
        }else if((op>=Op::gamma_r&&op<=Op::hlg_rgb)||(op>=Op::hlginv_r&&op<=Op::hlginv_rgb)) {
            uint32_t words[7];for(auto& u:words)in>>u;std::memcpy(&c.tf,words,28);args[i]=&c.tf;
        }else if(op>=Op::table_r&&op<=Op::table_a) {
            unsigned width;uint32_t entries;std::string bytes;in>>entries>>width>>bytes;c.bytes=Unhex(bytes);c.curve.table_entries=entries;c.curve.table_8=width==1?c.bytes.data():nullptr;c.curve.table_16=width==2?c.bytes.data():nullptr;args[i]=&c.curve;
        }else if(op==Op::clut_A2B||op==Op::clut_B2A) {
            uint32_t dim,outputs;unsigned width;uint8_t points[4];std::string bytes;in>>dim>>outputs>>width;for(auto& p:points){unsigned n;in>>n;p=static_cast<uint8_t>(n);}in>>bytes;c.bytes=Unhex(bytes);const uint8_t* grid=c.bytes.data()+2;
            if(op==Op::clut_A2B){c.a2b.input_channels=dim;c.a2b.output_channels=outputs;std::memcpy(c.a2b.grid_points,points,4);c.a2b.grid_8=width==1?grid:nullptr;c.a2b.grid_16=width==2?grid:nullptr;args[i]=&c.a2b;}
            else {c.b2a.input_channels=dim;c.b2a.output_channels=outputs;std::memcpy(c.b2a.grid_points,points,4);c.b2a.grid_8=width==1?grid:nullptr;c.b2a.grid_16=width==2?grid:nullptr;args[i]=&c.b2a;}
        }
    }
    skcms_private::baseline::run_program(ops,args,ptrdiff_t(count),reinterpret_cast<const char*>(src.data()),reinterpret_cast<char*>(dst.data()),n,sbpp,dbpp);
    Hex(reinterpret_cast<const uint8_t*>(dst.data()),512);std::cout<<' ';Hex(reinterpret_cast<const uint8_t*>(src.data()),512);
}
void PartialSelect(std::istringstream& in) {
    unsigned mode;in>>mode;skcms_Curve c;c.table_entries=0;
    const float markers[]={-129.f,-INFINITY_,-1.f,-1.5f,-127.5f,-128.f,-.5f,-2.f,-3.f,-4.f,-5.f,-6.f,-5.f,-6.f,-5.f,-6.f};
    c.parametric.g=markers[mode];
    if(mode==10)c.parametric.b=1;
    if(mode==11)c.parametric.d=1;
    if(mode==12){c.parametric.b=0;c.parametric.c=1;}
    if(mode==13){c.parametric.d=0;c.parametric.e=1;}
    if(mode==14){c.parametric.b=c.parametric.c=c.parametric.d=c.parametric.e=c.parametric.f=0;}
    if(mode==15){c.parametric.d=c.parametric.e=c.parametric.f=0;}
    auto volatile fn=&select_curve_op;auto result=fn(&c,0);std::cout<<int(result.op)<<' '<<(result.arg!=nullptr);
}
void PartialStage(std::istringstream& in) {
    std::string name;in>>name;Op op=ParseOp(name);skcms_TransferFunction tf;
    if(name.rfind("gamma_",0)==0)tf.g=2.2f;
    else if(name.rfind("pq_",0)==0){tf.a=-107/128.f;tf.b=1;tf.c=32/2523.f;tf.d=2413/128.f;tf.e=-2392/128.f;tf.f=8192/1305.f;}
    else if(name.rfind("hlg_",0)==0){tf.a=2;tf.b=2;tf.c=1/.17883277f;tf.d=.28466892f;tf.e=.55991073f;tf.f=0;}
    else {tf.a=.5f;tf.b=.5f;tf.c=.17883277f;tf.d=.28466892f;tf.e=.55991073f;tf.f=0;}
    alignas(16)float src[16],dst[16];for(int i=0;i<16;++i)src[i]=float(i-3)/19;
    Op ops[]={Op::load_ffff,op,Op::store_ffff};const void* args[]={nullptr,&tf,nullptr};
    skcms_private::baseline::run_program(ops,args,3,reinterpret_cast<const char*>(src),reinterpret_cast<char*>(dst),4,16,16);Hex(reinterpret_cast<const uint8_t*>(dst),64);
}
void PartialGrid(std::istringstream& in) {
    unsigned dim,outputs,width,kind;in>>dim>>outputs>>width>>kind;
    uint8_t bytes[2+16*4*2];for(size_t i=0;i<sizeof(bytes);++i)bytes[i]=uint8_t(i*13+5);skcms_A2B a;skcms_B2A b;
    const void* p;if(!kind){a.input_channels=dim;a.output_channels=outputs;for(unsigned i=0;i<dim;++i)a.grid_points[i]=2;a.grid_8=width==1?bytes+2:nullptr;a.grid_16=width==2?bytes+2:nullptr;p=&a;}
    else {b.input_channels=dim;b.output_channels=outputs;for(unsigned i=0;i<dim;++i)b.grid_points[i]=2;b.grid_8=width==1?bytes+2:nullptr;b.grid_16=width==2?bytes+2:nullptr;p=&b;}
    alignas(16)float src[16],dst[16];for(int i=0;i<16;++i)src[i]=float(i)/19;
    Op ops[]={Op::load_ffff,kind?Op::clut_B2A:Op::clut_A2B,Op::store_ffff};const void* args[]={nullptr,p,nullptr};
    skcms_private::baseline::run_program(ops,args,3,reinterpret_cast<const char*>(src),reinterpret_cast<char*>(dst),4,16,16);Hex(reinterpret_cast<const uint8_t*>(dst),64);
}
bool Private(const std::string& command,std::istringstream& in) {
    if(command=="skcms-transform-private-channel-count"){
        int count;in>>count;auto volatile fn=&select_curve_ops;
        if(count==1){skcms_Curve curve;curve.table_entries=0;curve.parametric.g=-129;OpAndArg ops[4];std::cout<<fn(&curve,count,ops);}
        else std::cout<<fn(nullptr,count,nullptr);
        return true;
    }
    if(command=="skcms-transform-private-partial-select"){PartialSelect(in);return true;}
    if(command=="skcms-transform-private-partial-stage"){PartialStage(in);return true;}
    if(command=="skcms-transform-private-partial-grid"){PartialGrid(in);return true;}
    if(command=="skcms-transform-private-program"){Program(in);return true;}
    if(command=="skcms-transform-private-bytes") {
        int fmt;in>>fmt;auto volatile fn=&bytes_per_pixel;std::cout<<fn(static_cast<skcms_PixelFormat>(fmt));return true;
    }
    if(command=="skcms-transform-private-cicp") {
        bool hlg;in>>hlg;skcms_TransferFunction tf;
        if(hlg){auto volatile fn=&set_sdr_hlg_ish_trc;fn(&tf);}else{auto volatile fn=&set_reference_pq_ish_trc;fn(&tf);}
        std::cout<<'T';TfWords(tf);return true;
    }
    if(command=="skcms-transform-private-select") {
        size_t channels;in>>channels;if(channels>4)std::abort();
        skcms_Curve curves[4];std::memset(curves,0xa5,sizeof(curves));uint8_t tables[4][2]={{0,255},{0,255},{0,255},{0,255}};
        for(size_t c=0;c<channels;++c) {
            uint32_t count;in>>count;float values[7];for(float& value:values){uint32_t u;in>>u;std::memcpy(&value,&u,4);}
            curves[c].table_entries=count;
            if(!count)std::memcpy(&curves[c].parametric,values,sizeof(values));else{curves[c].table_8=tables[c];curves[c].table_16=nullptr;}
        }
        // Also retain and exercise select_curve_op as its actual out-of-line
        // source entry point, rather than an independently copied selector.
        auto volatile one=&select_curve_op;for(size_t c=0;c<channels;++c)(void)one(&curves[c],int(c));
        OpAndArg ops[4];auto volatile fn=&select_curve_ops;int count=fn(curves,int(channels),ops);std::cout<<count;
        for(int i=0;i<count;++i)std::cout<<' '<<int(ops[i].op)<<' '<<intptr_t(uintptr_t(ops[i].arg)-uintptr_t(curves));return true;
    }
    if(command=="skcms-transform-private-prep") {
        int kind;std::string data;in>>kind>>data;Profile p(kind,data);if(!p.valid){std::cout<<'P';return true;}
        skcms_Matrix3x3 matrix;skcms_TransferFunction tf[3];
        for(int i=0;i<9;++i){uint32_t u=0x3e800000u+uint32_t(i)*103;std::memcpy(&matrix.vals[i/3][i%3],&u,4);}
        for(int i=0;i<3;++i){uint32_t words[7];for(int j=0;j<7;++j)words[j]=0x3f000000u+uint32_t(i*7+j)*137;std::memcpy(tf+i,words,28);}
        bool b2a=true,hlg=true;auto volatile fn=&prep_for_destination;
        bool ok=fn(p.p,&matrix,&tf[0],&tf[1],&tf[2],&b2a,&hlg);std::cout<<ok<<' '<<b2a<<' '<<hlg;
        for(auto& row:matrix.vals)for(float v:row){uint32_t u;std::memcpy(&u,&v,4);std::cout<<' '<<u;}
        for(auto& v:tf)TfWords(v);return true;
    }
    return false;
}
}
int main() {
    std::string line;while(std::getline(std::cin,line)){std::istringstream in(line);std::string command;in>>command;
        if(!Private(command,in)&&!SkcmsTransformCommand(command,in))std::abort();std::cout<<'\n';}
}
