// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Original DequantMatricesLibraryDef constants from lib/jxl/quant_weights.cc.
//! Exact f32 words retain V(float) rounding of each original double expression.

use super::{DctQuantWeightParams, QuantEncoding};

fn dct() -> QuantEncoding {
    QuantEncoding::dct(&DctQuantWeightParams::from_bands(&[
        [
            f32::from_bits(0x4544e000), /* V(3150.0) */
            f32::from_bits(0x00000000), /* V(0.0) */
            f32::from_bits(0xbecccccd), /* V(-0.4) */
            f32::from_bits(0xbecccccd), /* V(-0.4) */
            f32::from_bits(0xbecccccd), /* V(-0.4) */
            f32::from_bits(0xc0000000), /* V(-2.0) */
        ],
        [
            f32::from_bits(0x440c0000), /* V(560.0) */
            f32::from_bits(0x00000000), /* V(0.0) */
            f32::from_bits(0xbe99999a), /* V(-0.3) */
            f32::from_bits(0xbe99999a), /* V(-0.3) */
            f32::from_bits(0xbe99999a), /* V(-0.3) */
            f32::from_bits(0xbe99999a), /* V(-0.3) */
        ],
        [
            f32::from_bits(0x44000000), /* V(512.0) */
            f32::from_bits(0xc0000000), /* V(-2.0) */
            f32::from_bits(0xbf800000), /* V(-1.0) */
            f32::from_bits(0x00000000), /* V(0.0) */
            f32::from_bits(0xbf800000), /* V(-1.0) */
            f32::from_bits(0xc0000000), /* V(-2.0) */
        ],
    ]))
}

fn identity() -> QuantEncoding {
    QuantEncoding::identity(&[
        [
            f32::from_bits(0x438c0000), /* V(280.0) */
            f32::from_bits(0x45458000), /* V(3160.0) */
            f32::from_bits(0x45458000), /* V(3160.0) */
        ],
        [
            f32::from_bits(0x42700000), /* V(60.0) */
            f32::from_bits(0x44580000), /* V(864.0) */
            f32::from_bits(0x44580000), /* V(864.0) */
        ],
        [
            f32::from_bits(0x41900000), /* V(18.0) */
            f32::from_bits(0x43480000), /* V(200.0) */
            f32::from_bits(0x43480000), /* V(200.0) */
        ],
    ])
}

fn dct2x2() -> QuantEncoding {
    QuantEncoding::dct2(&[
        [
            f32::from_bits(0x45700000), /* V(3840.0) */
            f32::from_bits(0x45200000), /* V(2560.0) */
            f32::from_bits(0x44a00000), /* V(1280.0) */
            f32::from_bits(0x44200000), /* V(640.0) */
            f32::from_bits(0x43f00000), /* V(480.0) */
            f32::from_bits(0x43960000), /* V(300.0) */
        ],
        [
            f32::from_bits(0x44700000), /* V(960.0) */
            f32::from_bits(0x44200000), /* V(640.0) */
            f32::from_bits(0x43a00000), /* V(320.0) */
            f32::from_bits(0x43340000), /* V(180.0) */
            f32::from_bits(0x430c0000), /* V(140.0) */
            f32::from_bits(0x42f00000), /* V(120.0) */
        ],
        [
            f32::from_bits(0x44200000), /* V(640.0) */
            f32::from_bits(0x43a00000), /* V(320.0) */
            f32::from_bits(0x43000000), /* V(128.0) */
            f32::from_bits(0x42800000), /* V(64.0) */
            f32::from_bits(0x42000000), /* V(32.0) */
            f32::from_bits(0x41800000), /* V(16.0) */
        ],
    ])
}

fn dct4x4() -> QuantEncoding {
    QuantEncoding::dct4(
        &DctQuantWeightParams::from_bands(&[
            [
                f32::from_bits(0x45098000), /* V(2200.0) */
                f32::from_bits(0x00000000), /* V(0.0) */
                f32::from_bits(0x00000000), /* V(0.0) */
                f32::from_bits(0x00000000), /* V(0.0) */
            ],
            [
                f32::from_bits(0x43c40000), /* V(392.0) */
                f32::from_bits(0x00000000), /* V(0.0) */
                f32::from_bits(0x00000000), /* V(0.0) */
                f32::from_bits(0x00000000), /* V(0.0) */
            ],
            [
                f32::from_bits(0x42e00000), /* V(112.0) */
                f32::from_bits(0xbe800000), /* V(-0.25) */
                f32::from_bits(0xbe800000), /* V(-0.25) */
                f32::from_bits(0xbf000000), /* V(-0.5) */
            ],
        ]),
        &[
            [
                f32::from_bits(0x3f800000), /* V(1.0) */
                f32::from_bits(0x3f800000), /* V(1.0) */
            ],
            [
                f32::from_bits(0x3f800000), /* V(1.0) */
                f32::from_bits(0x3f800000), /* V(1.0) */
            ],
            [
                f32::from_bits(0x3f800000), /* V(1.0) */
                f32::from_bits(0x3f800000), /* V(1.0) */
            ],
        ],
    )
}

fn dct16x16() -> QuantEncoding {
    QuantEncoding::dct(&DctQuantWeightParams::from_bands(&[
        [
            f32::from_bits(0x460c937e), /* V(8996.8725711814115328) */
            f32::from_bits(0xbfa668f3), /* V(-1.3000777393353804) */
            f32::from_bits(0xbefd0db8), /* V(-0.49424529824571225) */
            f32::from_bits(0xbee0d0e6), /* V(-0.439093774457103443) */
            f32::from_bits(0xbf229007), /* V(-0.6350101832695744) */
            f32::from_bits(0xbf66da92), /* V(-0.90177264050827612) */
            f32::from_bits(0xbfcedff7), /* V(-1.6162099239887414) */
        ],
        [
            f32::from_bits(0x454777bd), /* V(3191.48366296844234752) */
            f32::from_bits(0xbf2c9b60), /* V(-0.67424582104194355) */
            f32::from_bits(0xbf4eb594), /* V(-0.80745813428471001) */
            f32::from_bits(0xbee60532), /* V(-0.44925837484843441) */
            f32::from_bits(0xbeb7a18d), /* V(-0.35865440981033403) */
            f32::from_bits(0xbea05ee2), /* V(-0.31322389111877305) */
            f32::from_bits(0xbec096c4), /* V(-0.37615025315725483) */
        ],
        [
            f32::from_bits(0x4490b021), /* V(1157.50408145487200256) */
            f32::from_bits(0xc00366af), /* V(-2.0531423165804414) */
            f32::from_bits(0xbfb33333), /* V(-1.4) */
            f32::from_bits(0xbf01c251), /* V(-0.50687130033378396) */
            f32::from_bits(0xbedaab30), /* V(-0.42708730624733904) */
            f32::from_bits(0xbfbe2ae0), /* V(-1.4856834539296244) */
            f32::from_bits(0xc09d7821), /* V(-4.9209142884401604) */
        ],
    ]))
}

fn dct32x32() -> QuantEncoding {
    QuantEncoding::dct(&DctQuantWeightParams::from_bands(&[
        [
            f32::from_bits(0x467599a2), /* V(15718.40830982518931456) */
            f32::from_bits(0xbf833333), /* V(-1.025) */
            f32::from_bits(0xbf7ae148), /* V(-0.98) */
            f32::from_bits(0xbf66b50b), /* V(-0.9012) */
            f32::from_bits(0xbecccccd), /* V(-0.4) */
            f32::from_bits(0xbef9f48f), /* V(-0.48819395464) */
            f32::from_bits(0xbed795b3), /* V(-0.421064) */
            f32::from_bits(0xbe8a3d71), /* V(-0.27) */
        ],
        [
            f32::from_bits(0x45e44e1c), /* V(7305.7636810695983104) */
            f32::from_bits(0xbf4ddfc7), /* V(-0.8041958212306401) */
            f32::from_bits(0xbf4367de), /* V(-0.7633036457487539) */
            f32::from_bits(0xbf0e7d96), /* V(-0.55660379990111464) */
            f32::from_bits(0xbefee698), /* V(-0.49785304658857626) */
            f32::from_bits(0xbedfbdee), /* V(-0.43699592683512467) */
            f32::from_bits(0xbecdb9de), /* V(-0.40180866526242109) */
            f32::from_bits(0xbe8be314), /* V(-0.27321683125358037) */
        ],
        [
            f32::from_bits(0x456db882), /* V(3803.53173721215041536) */
            f32::from_bits(0xc043e30f), /* V(-3.060733579805728) */
            f32::from_bits(0xc002a51a), /* V(-2.0413270132490346) */
            f32::from_bits(0xc0018217), /* V(-2.0235650159727417) */
            f32::from_bits(0xbf0cae96), /* V(-0.5495389509954993) */
            f32::from_bits(0xbecccccd), /* V(-0.4) */
            f32::from_bits(0xbecccccd), /* V(-0.4) */
            f32::from_bits(0xbe99999a), /* V(-0.3) */
        ],
    ]))
}

fn dct8x16() -> QuantEncoding {
    QuantEncoding::dct(&DctQuantWeightParams::from_bands(&[
        [
            f32::from_bits(0x45e24630), /* V(7240.7734393502) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbe4ccccd), /* V(-0.2) */
            f32::from_bits(0xbe4ccccd), /* V(-0.2) */
            f32::from_bits(0xbe4ccccd), /* V(-0.2) */
            f32::from_bits(0xbf000000), /* V(-0.5) */
        ],
        [
            f32::from_bits(0x44b504f3), /* V(1448.15468787004) */
            f32::from_bits(0xbf000000), /* V(-0.5) */
            f32::from_bits(0xbf000000), /* V(-0.5) */
            f32::from_bits(0xbf000000), /* V(-0.5) */
            f32::from_bits(0xbe4ccccd), /* V(-0.2) */
            f32::from_bits(0xbe4ccccd), /* V(-0.2) */
            f32::from_bits(0xbe4ccccd), /* V(-0.2) */
        ],
        [
            f32::from_bits(0x43fd6d54), /* V(506.854140754517) */
            f32::from_bits(0xbfb33333), /* V(-1.4) */
            f32::from_bits(0xbe4ccccd), /* V(-0.2) */
            f32::from_bits(0xbf000000), /* V(-0.5) */
            f32::from_bits(0xbf000000), /* V(-0.5) */
            f32::from_bits(0xbfc00000), /* V(-1.5) */
            f32::from_bits(0xc0666666), /* V(-3.6) */
        ],
    ]))
}

fn dct8x32() -> QuantEncoding {
    QuantEncoding::dct(&DctQuantWeightParams::from_bands(&[
        [
            f32::from_bits(0x467e6cff), /* V(16283.2494710648897) */
            f32::from_bits(0xbfe40122), /* V(-1.7812845336559429) */
            f32::from_bits(0xbfd0c186), /* V(-1.6309059012653515) */
            f32::from_bits(0xbf84e453), /* V(-1.0382179034313539) */
            f32::from_bits(0xbf59999a), /* V(-0.85) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbf666666), /* V(-0.9) */
            f32::from_bits(0xbf9e3757), /* V(-1.2360638576849587) */
        ],
        [
            f32::from_bits(0x459f0943), /* V(5089.15750884921511936) */
            f32::from_bits(0xbea3dd84), /* V(-0.320049391452786891) */
            f32::from_bits(0xbeb50ecb), /* V(-0.35362849922161446) */
            f32::from_bits(0xbe9b573f), /* V(-0.30340000000000003) */
            f32::from_bits(0xbf1c28f6), /* V(-0.61) */
            f32::from_bits(0xbf000000), /* V(-0.5) */
            f32::from_bits(0xbf000000), /* V(-0.5) */
            f32::from_bits(0xbf19999a), /* V(-0.6) */
        ],
        [
            f32::from_bits(0x45545c6b), /* V(3397.77603275308720128) */
            f32::from_bits(0xbea48505), /* V(-0.321327362693153371) */
            f32::from_bits(0xbeb0add4), /* V(-0.34507619223117997) */
            f32::from_bits(0xbf341206), /* V(-0.70340000000000003) */
            f32::from_bits(0xbf666666), /* V(-0.9) */
            f32::from_bits(0xbf800000), /* V(-1.0) */
            f32::from_bits(0xbf800000), /* V(-1.0) */
            f32::from_bits(0xbf96757e), /* V(-1.1754605576265209) */
        ],
    ]))
}

fn dct16x32() -> QuantEncoding {
    QuantEncoding::dct(&DctQuantWeightParams::from_bands(&[
        [
            f32::from_bits(0x465853e2), /* V(13844.97076442300573) */
            f32::from_bits(0xbf789c80), /* V(-0.97113799999999995) */
            f32::from_bits(0xbf2872b0), /* V(-0.658) */
            f32::from_bits(0xbed72c52), /* V(-0.42026) */
            f32::from_bits(0xbe689225), /* V(-0.22712) */
            f32::from_bits(0xbe61e4f7), /* V(-0.2206) */
            f32::from_bits(0xbe676c8b), /* V(-0.226) */
            f32::from_bits(0xbf19999a), /* V(-0.6) */
        ],
        [
            f32::from_bits(0x4595f7b6), /* V(4798.964084220744293) */
            f32::from_bits(0xbf1c7b15), /* V(-0.61125308982767057) */
            f32::from_bits(0xbf567406), /* V(-0.83770786552491361) */
            f32::from_bits(0xbf4a472e), /* V(-0.79014862079498627) */
            f32::from_bits(0xbe89de1e), /* V(-0.2692727459704829) */
            f32::from_bits(0xbec3f4e2), /* V(-0.38272769465388551) */
            f32::from_bits(0xbe6abe79), /* V(-0.22924222653091453) */
            f32::from_bits(0xbe5429e0), /* V(-0.20719098826199578) */
        ],
        [
            f32::from_bits(0x44e1e795), /* V(1807.236946760964614) */
            f32::from_bits(0xbf99999a), /* V(-1.2) */
            f32::from_bits(0xbf99999a), /* V(-1.2) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbecccccd), /* V(-0.4) */
            f32::from_bits(0xbf000000), /* V(-0.5) */
        ],
    ]))
}

fn dct4x8() -> QuantEncoding {
    QuantEncoding::dct4x8(
        &DctQuantWeightParams::from_bands(&[
            [
                f32::from_bits(0x450960cf), /* V(2198.050556016380522) */
                f32::from_bits(0xbf767343), /* V(-0.96269623020744692) */
                f32::from_bits(0xbf430eaa), /* V(-0.76194253026666783) */
                f32::from_bits(0xbf27b58e), /* V(-0.6551140670773547) */
            ],
            [
                f32::from_bits(0x443f1765), /* V(764.3655248643528689) */
                f32::from_bits(0xbf6d2221), /* V(-0.92630200888366945) */
                f32::from_bits(0xbf77af96), /* V(-0.9675229603596517) */
                f32::from_bits(0xbe8e9161), /* V(-0.27845290869168118) */
            ],
            [
                f32::from_bits(0x4403c6e2), /* V(527.107573587542228) */
                f32::from_bits(0xbfbacee2), /* V(-1.4594385811273854) */
                f32::from_bits(0xbfb99c4a), /* V(-1.450082094097871593) */
                f32::from_bits(0xbfcaccb6), /* V(-1.5843722511996204) */
            ],
        ]),
        &[
            f32::from_bits(0x3f800000), /* V(1.0) */
            f32::from_bits(0x3f800000), /* V(1.0) */
            f32::from_bits(0x3f800000), /* V(1.0) */
        ],
    )
}

fn afv0() -> QuantEncoding {
    QuantEncoding::afv(
        &dct4x8().dct_params,
        &dct4x4().dct_params,
        &[
            [
                f32::from_bits(0x45400000), /* V(3072.0) */
                f32::from_bits(0x45400000), /* V(3072.0) */
                f32::from_bits(0x43800000), /* V(256.0) */
                f32::from_bits(0x43800000), /* V(256.0) */
                f32::from_bits(0x43800000), /* V(256.0) */
                f32::from_bits(0x43cf0000), /* V(414.0) */
                f32::from_bits(0x00000000), /* V(0.0) */
                f32::from_bits(0x00000000), /* V(0.0) */
                f32::from_bits(0x00000000), /* V(0.0) */
            ],
            [
                f32::from_bits(0x44800000), /* V(1024.0) */
                f32::from_bits(0x44800000), /* V(1024.0) */
                f32::from_bits(0x42480000), /* V(50) */
                f32::from_bits(0x42480000), /* V(50) */
                f32::from_bits(0x42480000), /* V(50) */
                f32::from_bits(0x42680000), /* V(58.0) */
                f32::from_bits(0x00000000), /* V(0.0) */
                f32::from_bits(0x00000000), /* V(0.0) */
                f32::from_bits(0x00000000), /* V(0.0) */
            ],
            [
                f32::from_bits(0x43c00000), /* V(384.0) */
                f32::from_bits(0x43c00000), /* V(384.0) */
                f32::from_bits(0x41400000), /* V(12.0) */
                f32::from_bits(0x41400000), /* V(12.0) */
                f32::from_bits(0x41400000), /* V(12.0) */
                f32::from_bits(0x41b00000), /* V(22.0) */
                f32::from_bits(0xbe800000), /* V(-0.25) */
                f32::from_bits(0xbe800000), /* V(-0.25) */
                f32::from_bits(0xbe800000), /* V(-0.25) */
            ],
        ],
    )
}

fn dct64x64() -> QuantEncoding {
    QuantEncoding::dct(&DctQuantWeightParams::from_bands(&[
        [
            f32::from_bits(0x46bb3c55), /* V(0.9 * 26629.073922049845) */
            f32::from_bits(0xbf833333), /* V(-1.025) */
            f32::from_bits(0xbf47ae14), /* V(-0.78) */
            f32::from_bits(0xbf266e44), /* V(-0.65012) */
            f32::from_bits(0xbe42fc58), /* V(-0.19041574084286472) */
            f32::from_bits(0xbe5530cc), /* V(-0.20819395464) */
            f32::from_bits(0xbed795b3), /* V(-0.421064) */
            f32::from_bits(0xbea798e8), /* V(-0.32733845535848671) */
        ],
        [
            f32::from_bits(0x4602f0c4), /* V(0.9 * 9311.3238710010046) */
            f32::from_bits(0xbe9bbf8e), /* V(-0.3041958212306401) */
            f32::from_bits(0xbeba02ef), /* V(-0.3633036457487539) */
            f32::from_bits(0xbeb694c6), /* V(-0.35660379990111464) */
            f32::from_bits(0xbeb04911), /* V(-0.3443074455424403) */
            f32::from_bits(0xbeac8abb), /* V(-0.33699592683512467) */
            f32::from_bits(0xbe9a86aa), /* V(-0.30180866526242109) */
            f32::from_bits(0xbe8be314), /* V(-0.27321683125358037) */
        ],
        [
            f32::from_bits(0x458c6831), /* V(0.9 * 4992.2486445538634) */
            f32::from_bits(0xbf99999a), /* V(-1.2) */
            f32::from_bits(0xbf99999a), /* V(-1.2) */
            f32::from_bits(0xbf4ccccd), /* V(-0.8) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbecccccd), /* V(-0.4) */
            f32::from_bits(0xbf000000), /* V(-0.5) */
        ],
    ]))
}

fn dct32x64() -> QuantEncoding {
    QuantEncoding::dct(&DctQuantWeightParams::from_bands(&[
        [
            f32::from_bits(0x466ffb98), /* V(0.65 * 23629.073922049845) */
            f32::from_bits(0xbf833333), /* V(-1.025) */
            f32::from_bits(0xbf47ae14), /* V(-0.78) */
            f32::from_bits(0xbf266e44), /* V(-0.65012) */
            f32::from_bits(0xbe42fc58), /* V(-0.19041574084286472) */
            f32::from_bits(0xbe5530cc), /* V(-0.20819395464) */
            f32::from_bits(0xbed795b3), /* V(-0.421064) */
            f32::from_bits(0xbea798e8), /* V(-0.32733845535848671) */
        ],
        [
            f32::from_bits(0x45aeeae2), /* V(0.65 * 8611.3238710010046) */
            f32::from_bits(0xbe9bbf8e), /* V(-0.3041958212306401) */
            f32::from_bits(0xbeba02ef), /* V(-0.3633036457487539) */
            f32::from_bits(0xbeb694c6), /* V(-0.35660379990111464) */
            f32::from_bits(0xbeb04911), /* V(-0.3443074455424403) */
            f32::from_bits(0xbeac8abb), /* V(-0.33699592683512467) */
            f32::from_bits(0xbe9a86aa), /* V(-0.30180866526242109) */
            f32::from_bits(0xbe8be314), /* V(-0.27321683125358037) */
        ],
        [
            f32::from_bits(0x45367f63), /* V(0.65 * 4492.2486445538634) */
            f32::from_bits(0xbf99999a), /* V(-1.2) */
            f32::from_bits(0xbf99999a), /* V(-1.2) */
            f32::from_bits(0xbf4ccccd), /* V(-0.8) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbecccccd), /* V(-0.4) */
            f32::from_bits(0xbf000000), /* V(-0.5) */
        ],
    ]))
}

fn dct128x128() -> QuantEncoding {
    QuantEncoding::dct(&DctQuantWeightParams::from_bands(&[
        [
            f32::from_bits(0x473b3c55), /* V(1.8 * 26629.073922049845) */
            f32::from_bits(0xbf833333), /* V(-1.025) */
            f32::from_bits(0xbf47ae14), /* V(-0.78) */
            f32::from_bits(0xbf266e44), /* V(-0.65012) */
            f32::from_bits(0xbe42fc58), /* V(-0.19041574084286472) */
            f32::from_bits(0xbe5530cc), /* V(-0.20819395464) */
            f32::from_bits(0xbed795b3), /* V(-0.421064) */
            f32::from_bits(0xbea798e8), /* V(-0.32733845535848671) */
        ],
        [
            f32::from_bits(0x4682f0c4), /* V(1.8 * 9311.3238710010046) */
            f32::from_bits(0xbe9bbf8e), /* V(-0.3041958212306401) */
            f32::from_bits(0xbeba02ef), /* V(-0.3633036457487539) */
            f32::from_bits(0xbeb694c6), /* V(-0.35660379990111464) */
            f32::from_bits(0xbeb04911), /* V(-0.3443074455424403) */
            f32::from_bits(0xbeac8abb), /* V(-0.33699592683512467) */
            f32::from_bits(0xbe9a86aa), /* V(-0.30180866526242109) */
            f32::from_bits(0xbe8be314), /* V(-0.27321683125358037) */
        ],
        [
            f32::from_bits(0x460c6831), /* V(1.8 * 4992.2486445538634) */
            f32::from_bits(0xbf99999a), /* V(-1.2) */
            f32::from_bits(0xbf99999a), /* V(-1.2) */
            f32::from_bits(0xbf4ccccd), /* V(-0.8) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbecccccd), /* V(-0.4) */
            f32::from_bits(0xbf000000), /* V(-0.5) */
        ],
    ]))
}

fn dct64x128() -> QuantEncoding {
    QuantEncoding::dct(&DctQuantWeightParams::from_bands(&[
        [
            f32::from_bits(0x46effb98), /* V(1.3 * 23629.073922049845) */
            f32::from_bits(0xbf833333), /* V(-1.025) */
            f32::from_bits(0xbf47ae14), /* V(-0.78) */
            f32::from_bits(0xbf266e44), /* V(-0.65012) */
            f32::from_bits(0xbe42fc58), /* V(-0.19041574084286472) */
            f32::from_bits(0xbe5530cc), /* V(-0.20819395464) */
            f32::from_bits(0xbed795b3), /* V(-0.421064) */
            f32::from_bits(0xbea798e8), /* V(-0.32733845535848671) */
        ],
        [
            f32::from_bits(0x462eeae2), /* V(1.3 * 8611.3238710010046) */
            f32::from_bits(0xbe9bbf8e), /* V(-0.3041958212306401) */
            f32::from_bits(0xbeba02ef), /* V(-0.3633036457487539) */
            f32::from_bits(0xbeb694c6), /* V(-0.35660379990111464) */
            f32::from_bits(0xbeb04911), /* V(-0.3443074455424403) */
            f32::from_bits(0xbeac8abb), /* V(-0.33699592683512467) */
            f32::from_bits(0xbe9a86aa), /* V(-0.30180866526242109) */
            f32::from_bits(0xbe8be314), /* V(-0.27321683125358037) */
        ],
        [
            f32::from_bits(0x45b67f63), /* V(1.3 * 4492.2486445538634) */
            f32::from_bits(0xbf99999a), /* V(-1.2) */
            f32::from_bits(0xbf99999a), /* V(-1.2) */
            f32::from_bits(0xbf4ccccd), /* V(-0.8) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbecccccd), /* V(-0.4) */
            f32::from_bits(0xbf000000), /* V(-0.5) */
        ],
    ]))
}

fn dct256x256() -> QuantEncoding {
    QuantEncoding::dct(&DctQuantWeightParams::from_bands(&[
        [
            f32::from_bits(0x47bb3c55), /* V(3.6 * 26629.073922049845) */
            f32::from_bits(0xbf833333), /* V(-1.025) */
            f32::from_bits(0xbf47ae14), /* V(-0.78) */
            f32::from_bits(0xbf266e44), /* V(-0.65012) */
            f32::from_bits(0xbe42fc58), /* V(-0.19041574084286472) */
            f32::from_bits(0xbe5530cc), /* V(-0.20819395464) */
            f32::from_bits(0xbed795b3), /* V(-0.421064) */
            f32::from_bits(0xbea798e8), /* V(-0.32733845535848671) */
        ],
        [
            f32::from_bits(0x4702f0c4), /* V(3.6 * 9311.3238710010046) */
            f32::from_bits(0xbe9bbf8e), /* V(-0.3041958212306401) */
            f32::from_bits(0xbeba02ef), /* V(-0.3633036457487539) */
            f32::from_bits(0xbeb694c6), /* V(-0.35660379990111464) */
            f32::from_bits(0xbeb04911), /* V(-0.3443074455424403) */
            f32::from_bits(0xbeac8abb), /* V(-0.33699592683512467) */
            f32::from_bits(0xbe9a86aa), /* V(-0.30180866526242109) */
            f32::from_bits(0xbe8be314), /* V(-0.27321683125358037) */
        ],
        [
            f32::from_bits(0x468c6831), /* V(3.6 * 4992.2486445538634) */
            f32::from_bits(0xbf99999a), /* V(-1.2) */
            f32::from_bits(0xbf99999a), /* V(-1.2) */
            f32::from_bits(0xbf4ccccd), /* V(-0.8) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbecccccd), /* V(-0.4) */
            f32::from_bits(0xbf000000), /* V(-0.5) */
        ],
    ]))
}

fn dct128x256() -> QuantEncoding {
    QuantEncoding::dct(&DctQuantWeightParams::from_bands(&[
        [
            f32::from_bits(0x476ffb98), /* V(2.6 * 23629.073922049845) */
            f32::from_bits(0xbf833333), /* V(-1.025) */
            f32::from_bits(0xbf47ae14), /* V(-0.78) */
            f32::from_bits(0xbf266e44), /* V(-0.65012) */
            f32::from_bits(0xbe42fc58), /* V(-0.19041574084286472) */
            f32::from_bits(0xbe5530cc), /* V(-0.20819395464) */
            f32::from_bits(0xbed795b3), /* V(-0.421064) */
            f32::from_bits(0xbea798e8), /* V(-0.32733845535848671) */
        ],
        [
            f32::from_bits(0x46aeeae2), /* V(2.6 * 8611.3238710010046) */
            f32::from_bits(0xbe9bbf8e), /* V(-0.3041958212306401) */
            f32::from_bits(0xbeba02ef), /* V(-0.3633036457487539) */
            f32::from_bits(0xbeb694c6), /* V(-0.35660379990111464) */
            f32::from_bits(0xbeb04911), /* V(-0.3443074455424403) */
            f32::from_bits(0xbeac8abb), /* V(-0.33699592683512467) */
            f32::from_bits(0xbe9a86aa), /* V(-0.30180866526242109) */
            f32::from_bits(0xbe8be314), /* V(-0.27321683125358037) */
        ],
        [
            f32::from_bits(0x46367f63), /* V(2.6 * 4492.2486445538634) */
            f32::from_bits(0xbf99999a), /* V(-1.2) */
            f32::from_bits(0xbf99999a), /* V(-1.2) */
            f32::from_bits(0xbf4ccccd), /* V(-0.8) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbf333333), /* V(-0.7) */
            f32::from_bits(0xbecccccd), /* V(-0.4) */
            f32::from_bits(0xbf000000), /* V(-0.5) */
        ],
    ]))
}

pub(super) fn library_init() -> [QuantEncoding; 17] {
    [
        dct(),
        identity(),
        dct2x2(),
        dct4x4(),
        dct16x16(),
        dct32x32(),
        dct8x16(),
        dct8x32(),
        dct16x32(),
        dct4x8(),
        afv0(),
        dct64x64(),
        dct32x64(),
        dct128x128(),
        dct64x128(),
        dct256x256(),
        dct128x256(),
    ]
}
