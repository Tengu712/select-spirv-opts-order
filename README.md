# Select SPIRV Optimizations Order

## What is this?

An experiment aimed at speeding up SPIR-V by heuristically selecting the order of optimization application using a GA (Genetic Algorithm).

## Result

We did not obtain results that exceed the measurement error of GPU execution time.
This holds true not only for the current GA system but also for the `-O` option of spirv-opt.
Perhaps in graphics-related tasks, the content of a single shader is trivial compared to the overhead of handling multiple shaders or the sheer number of draw calls.

## Build

### Windows

1. install MSVC
2. install Rust
3. install Vulkan SDK
4. set a environment variable `VulkanInclude` to `VulkanSDK/<version>/Include`
5. set a environment variable `VulkanLib` to `VulkanSDK/<version>/Lib`
6. run `build.bat`

### Docker

1. install Docker
2. run `docker build -t Tengu712/select-spirv-opts-order .`
3. run `docker run Tengu712/select-spirv-opts-order`
