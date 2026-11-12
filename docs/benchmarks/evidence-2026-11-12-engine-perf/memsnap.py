"""lldb helper: stop the CLI at a symbol and dump live malloc stacks.

Usage: lldb --batch -o "command script import memsnap.py" -o "memsnap <symbol> <outfile> -- <cmd...>"
Requires nothing beyond Xcode's lldb and /usr/bin/malloc_history.
"""
import os
import subprocess

import lldb


def memsnap(debugger, command, result, internal_dict):
    head, _, rest = command.partition(" -- ")
    symbol, outfile = head.split()
    args = rest.split()
    target = debugger.CreateTarget(args[0])
    target.BreakpointCreateByName(symbol)
    launch = lldb.SBLaunchInfo(args[1:])
    launch.SetEnvironmentEntries(["MallocStackLogging=1"], True)
    error = lldb.SBError()
    process = target.Launch(launch, error)
    if not error.Success() or process.GetState() != lldb.eStateStopped:
        print("launch failed", error, process.GetState())
        return
    pid = process.GetProcessID()
    with open(outfile, "w") as f:
        subprocess.run(["/usr/bin/malloc_history", str(pid), "-allBySize"], stdout=f, stderr=subprocess.STDOUT)
    with open(outfile + ".heap", "w") as f:
        subprocess.run(["/usr/bin/heap", str(pid), "-s"], stdout=f, stderr=subprocess.STDOUT)
    process.Kill()
    print("pid", pid, "dumped to", outfile)


def __lldb_init_module(debugger, internal_dict):
    debugger.HandleCommand("command script add -f memsnap.memsnap memsnap")
