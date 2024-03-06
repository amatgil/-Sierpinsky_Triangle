# Sierpinski Triangle Drawing Program

This C++/Rust program allows you to generate and visualize the Sierpinski Triangle.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Usage](#usage)
- [Internals](#internals)
- [License](#license)

## Introduction

The Sierpinski Triangle is a fractal named after the Polish mathematician Wacław Sierpiński. It
is formed by recursively subdividing an equilateral triangle into smaller equilateral triangles.
This program provides a simple and customizable way to draw the Sierpinski Triangle.

This program does not generate it using this definition: instead, it draws itself in recursively
from nothing using the simple base pattern "`XX`". This _would_ make it twice as wide as it is
tall, but by simply drawing each row twice, pixels become 2x2 squares, making the whole image
a square itself.

This project/software/repo was created to explore using ffi from C++ to Rust to interact with
other's already written code, and it was both a success and a great learning experience that I'd
rather not repeat

## Usage

1. **Clone:**

```bash
git clone https://github.com/amatgil/Sierpinsky_Triangle
```

2. **Compile and run:**

```bash
just run 6
```
`6` is be any non-negative integer: the number of recursive iterations that will be done to generate
the fractal. The images get noticeably heavy at the resolutions around which levels like 12 or 13 
become noticeable.

## Internals

The generation is done at `SierpinskyTriangle.cc`, the drawing and saving of the image
is done in at `src/main.rs` file.

## License

This project and all files contained therein and licensed under the
[GNU GPLv3](https://www.gnu.org/licenses/gpl-3.0.txt) license (see COPYING file).
