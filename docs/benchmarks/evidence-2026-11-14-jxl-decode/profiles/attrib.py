# Attribute self samples of allocator / memmove / timer frames to the nearest
# ancestor frame of the decoder (port: `3jxl` in the mangled name; C: `jxl::`).
import re,sys,collections
f,rep,mark=sys.argv[1],float(sys.argv[2]),sys.argv[3]
lines=open(f).read().split("Call graph:")[1].split("Total number in stack")[0].splitlines()
stack=[];res=collections.Counter()
leafrx=re.compile(r"malloc|free|xzm_|memmove|memset|bzero|mach_absolute_time|rust_alloc|rust_dealloc|memcpy")
for ln in lines:
    m=re.match(r"^([ +!:|]*)(\d+) (.*?)  \(in ",ln)
    if not m: continue
    depth=len(m.group(1)); n=int(m.group(2)); name=m.group(3)
    while stack and stack[-1][0]>=depth: stack.pop()
    stack.append((depth,name,n))
for i,ln in enumerate(lines): pass
# second pass: self samples = n - sum(children n)
nodes=[]
for ln in lines:
    m=re.match(r"^([ +!:|]*)(\d+) (.*?)  \(in ",ln)
    if m: nodes.append((len(m.group(1)),int(m.group(2)),m.group(3)))
for i,(d,n,name) in enumerate(nodes):
    child=0;j=i+1
    while j<len(nodes) and nodes[j][0]>d:
        if nodes[j][0]==min(x[0] for x in nodes[i+1:j+1]) : pass
        j+=1
    kids=[x for x in nodes[i+1:j] if x[0]==min([y[0] for y in nodes[i+1:j]] or [0])]
    self_n=n-sum(k[1] for k in kids)
    if self_n<=0 or not leafrx.search(name): continue
    # ancestor
    k=i-1;dd=d;anc="?"
    while k>=0:
        if nodes[k][0]<dd:
            dd=nodes[k][0]
            if mark in nodes[k][2] and not leafrx.search(nodes[k][2]): anc=nodes[k][2];break
        k-=1
    res[(re.sub(r"\(.*","",anc)[:140], "alloc" if re.search("malloc|free|xzm|alloc|mach",name) else "mem")]+=self_n
for (a,kind),n in res.most_common(14): print(f"{n/rep:7.1f} ms/decode  {kind:5s} {a}")
