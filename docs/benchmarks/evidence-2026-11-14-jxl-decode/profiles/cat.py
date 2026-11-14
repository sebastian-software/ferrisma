import re,sys
cats=[("entropy (ANS/AC tokens, load_block, bit reader)",r"DecodeACVarBlock|ReadHybridUint|read_symbol_ans|read_hybrid_uint|load_block|LoadBlock|BitReader|refill|GetBlockFromBitstream"),
("modular (DC/AC metadata, MA tree)",r"Modular|modular|PredictTree|weighted|MATree|context_predict|decode_ac_metadata|DecodeAcMetadata"),
("pipeline row plumbing (simd_rows::read, prepare_io_rows, RenderRect, process_buffers, borders)",r"simd_rows4read|prepare_io_rows|RenderRect|process_buffers|LoadBorders|SaveBorders|get_input_row|array5drain|try_map|spec_from_iter"),
("memmove/memset",r"memmove|memset|bzero|memcpy"),
("malloc/free",r"malloc|free|xzm_|alloc_shim"),
("EPF",r"EPF|epf"),("Gaborish",r"Gaborish|gaborish"),("XYB",r"XYB|xyb"),("FromLinear",r"FromLinear|from_linear"),
("write to output",r"WriteToOutput|stage_write|output_buffers"),
("dequant + IDCT",r"Dequant|IDCT|idct|DCT|dct|TransformToPixels|transform_to_pixels|LowestFrequencies|Transpose"),
("timer (mach_absolute_time)",r"mach_absolute_time")]
f,rep=sys.argv[1],float(sys.argv[2])
t=open(f).read().split("Sort by top of stack")[1].split("Binary Images")[0]
tot={};S=0
for line in t.splitlines()[1:]:
    m=re.match(r"\s+(.*?)\s+(\d+)$",line)
    if not m: continue
    name,n=m.group(1),int(m.group(2)); S+=n
    for c,rx in cats:
        if re.search(rx,name): tot[c]=tot.get(c,0)+n; break
    else: tot["other"]=tot.get("other",0)+n
print(f"total samples {S}, per decode {S/rep:.0f} (1 ms interval)")
for c,n in sorted(tot.items(),key=lambda x:-x[1]): print(f"{c:95s} {n/rep:7.1f} ms/decode {100*n/S:5.1f}%")
