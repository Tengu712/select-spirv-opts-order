@echo off

set tmpdir=%cd%

if not exist ".\external" (
  mkdir external
  git clone https://github.com/rust-random/rand.git .\external\rand
  cargo build --release --manifest-path .\external\rand\Cargo.toml
)

if not exist ".\bin" (
  mkdir bin
)

cd bin

set clwith=cl ^
  /c ^
  ^
  /O2 /Oi ^
  /EHsc /Gd /GL /Gy ^
  /DNDEBUG /D_CONSOLE /D_UNICODE /DUNICODE /DRELEASE_BUILD ^
  /permissive- /Zc:inline ^
  /MD ^
  /FC /nologo /utf-8 ^
  /sdl /W4 /WX ^
  ^
  /I%VulkanInclude%
  
%clwith% /Fo:buffer.obj "%tmpdir%\src\c\util\memory\buffer.c"
%clwith% /Fo:image.obj  "%tmpdir%\src\c\util\memory\image.c"
%clwith% /Fo:memory.obj "%tmpdir%\src\c\util\memory\memory.c"
%clwith% /Fo:shader.obj "%tmpdir%\src\c\util\shader.c"
%clwith% /Fo:app.obj "%tmpdir%\src\c\app.c"
%clwith% /Fo:main.obj "%tmpdir%\src\c\main.c"

lib ^
  /OUT:vulkan-wrapper.lib ^
  /LIBPATH:%VulkanLib% ^
  vulkan-1.lib ^
  *.obj

del *.obj

rustc ^
  -o ga.exe ^
  --edition=2021 ^
  -L "%tmpdir%\external\rand\target\release\deps" ^
  --extern rand="%tmpdir%\external\rand\target\release\librand.rlib" ^
  "%tmpdir%\src\rust\main.rs"

glslc -o .\shader.vert.spv ..\src\shader\shader.vert
glslc -o .\shader.org.frag.spv ..\src\shader\shader.frag

cd %tmpdir%
