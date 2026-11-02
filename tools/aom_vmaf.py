"""Link the retained native VMAF engine from selected C/Rust consumers."""
import shlex
import subprocess
import sys


def link_flags():
    flags = shlex.split(subprocess.check_output(['pkg-config', '--libs', 'libvmaf'], text=True))
    # VMAF's static archive includes svm.cpp, but its generated .pc file omits
    # the C++ runtime that the original C++ linker driver supplies implicitly.
    flags.append('-lc++' if sys.platform == 'darwin' else '-lstdc++')
    return flags
