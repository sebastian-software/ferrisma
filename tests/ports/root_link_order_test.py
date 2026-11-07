#!/usr/bin/env python3
"""Exercise the real root CMake overlay with a tiny static runtime dependency."""
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]


class StaticRuntimeOrder(unittest.TestCase):
    @unittest.skipIf(sys.platform == 'darwin', 'Mach-O archive resolution does not depend on order')
    def test_selected_engine_archive_follows_the_imported_root_archive(self):
        # Link-order fixture only; it supplies no engine or codec evidence.
        with tempfile.TemporaryDirectory() as directory:
            work = Path(directory).resolve()
            sources = {'engine': 'int engine_fixture(void) { return 23; }\n',
                       'root': 'extern int engine_fixture(void); int root_fixture(void) { return engine_fixture(); }\n'}
            for name, source in sources.items():
                (work / f'{name}.c').write_text(source)
                subprocess.run(['cc', '-c', str(work / f'{name}.c'), '-o', str(work / f'{name}.o')], check=True)
                subprocess.run(['ar', 'rcs', str(work / f'lib{name}-fixture.a'), str(work / f'{name}.o')], check=True)
            (work / 'empty.c').write_text('/* Replaced by the real root overlay. */\n')
            (work / 'consumer.c').write_text('extern int root_fixture(void); int main(void) { return root_fixture() != 23; }\n')
            (work / 'CMakeLists.txt').write_text(f'''cmake_minimum_required(VERSION 3.20)
project(root_engine_order C)
set(LIBAVIF_RS_ARCHIVE "{work}/libroot-fixture.a")
set(LIBAVIF_RS_NATIVE_DEPENDENCY_ROOT "{work}")
set(BUILD_SHARED_LIBS OFF)
include("{ROOT}/tools/local-dependencies.cmake")
add_library(yuv::yuv STATIC IMPORTED GLOBAL)
set_target_properties(yuv::yuv PROPERTIES IMPORTED_LOCATION "{work}/libengine-fixture.a")
add_library(avif_obj OBJECT empty.c)
target_link_libraries(avif_obj PUBLIC $<BUILD_INTERFACE:yuv::yuv>)
add_library(avif STATIC $<TARGET_OBJECTS:avif_obj>)
target_link_libraries(avif PUBLIC avif_obj)
add_executable(root-consumer consumer.c)
target_link_libraries(root-consumer PRIVATE avif)
''')
            build = work / 'build'
            subprocess.run(['cmake', '-S', work, '-B', build, '-G', 'Ninja'],
                           check=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
            commands = subprocess.check_output(['ninja', '-C', build, '-t', 'commands', 'root-consumer'], text=True)
            link = commands.splitlines()[-1]
            self.assertLess(link.index(str(work / 'libroot-fixture.a')), link.rindex(str(work / 'libengine-fixture.a')))
            subprocess.run(['cmake', '--build', build, '--parallel', '1'],
                           check=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
            subprocess.run([build / 'root-consumer'], check=True)


if __name__ == '__main__':
    unittest.main()
