# Raytracing in One Weekend in Rust
Learning the basics of raytracing method and Rust programming language
by rewriting every C++ source code in the book "Raytracing in One Weekend" written
by Peter Shirley

## Contents
- [UPDATES](#UPDATES)
- [TODO](#TODO)
- [References](#References)

## So, why another Raytracing in One Weekend in Rust?
I re-read Raytracing in One Weekend recently and the code structure changed and,
my previous code was terrible. \
I wanted to rewrite to code with a better structure and base it off of the new
code base in the book. \
What differentiates this from other implementations are the following features.
1. Not using stdout for writing PPM.
    - Created a dedicated PPM writer to get rid of the bottleneck from using stdout.
2. Added PNG image writer by using the `image` crate for handling PNG file writer.
3. Parallel renderer. (Because multithread = fast)


## UPDATES
2022/5/31
- Erased every past code in `rust` branch due to fatal flaw in code
    - Backed previous codes to `rust_bak` branch
- Completed [2.2. Creating an Image File](https://raytracing.github.io/books/RayTracingInOneWeekend.html#outputanimage/creatinganimagefile)

2022/6/5
- Completed [12 Defocus Blur](https://raytracing.github.io/books/RayTracingInOneWeekend.html#defocusblur)

2022/6/8
- Completed [5.5. Using Random Vectors in the Lattice Points](https://raytracing.github.io/books/RayTracingTheNextWeek.html#perlinnoise/usingrandomvectorsonthelatticepoints)

2022/6/11
- Completed [7 Rectangles and Lights](https://raytracing.github.io/books/RayTracingTheNextWeek.html#rectanglesandlights)

2022/6/12
- Completed [8 Instances](https://raytracing.github.io/books/RayTracingTheNextWeek.html#instances)

2022/6/16
- Completed [9 Volumes](https://raytracing.github.io/books/RayTracingTheNextWeek.html#volumes)

2024/9/25
- Rewrite in rust with better code base

2024/10/12
- Moved rust-rewrite to rust
- Completed Raytracing in One Weekend

## References
1. [Peter Shirley, Raytracing in One Weekend, 2020-12-07](https://github.com/RayTracing/raytracing.github.io)
2. [render.rs from ebkalderon for multi-threaded rendering](https://github.com/ebkalderon/ray-tracing-in-one-weekend/blob/master/src/render.rs)
