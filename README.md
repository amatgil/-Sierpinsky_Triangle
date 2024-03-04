# Sierpinski Triangle Drawing Program

This C++/Rust program allows you to generate and visualize the Sierpinski Triangle.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Usage](#usage)
- [Getting Started](#getting-started)
- [License](#license)

## Introduction

The Sierpinski Triangle is a fractal named after the Polish mathematician Wacław Sierpiński. It is formed by recursively subdividing an equilateral triangle into smaller equilateral triangles. This program provides a simple and customizable way to draw the Sierpinski Triangle.

This program does not generate it using this definition: instead, it draws itself in recursively from nothing using the simple base pattern "`XX`".

## Usage

1. **Clone:**

```bash
git clone https://git.ilsrv.com/Gerysto/Sierpinsky_Triangle.git
```

3. **Compile and run:**

```bash
just authenticate # If you're using ssh-agent but it's not running, only
just run 
```
(this does require having [just](https://github.com/casey/just) installed on the system).

## Getting Started

To get started with this project, follow the [Usage](#usage) instructions. The generation is done at `SierpinskyTriangle.cc` while the drawing in of the image is done in the `main.rs` file.

## License

This project and all files contained therein and licensed under the [GNU GPLv3](https://www.gnu.org/licenses/gpl-3.0.txt) license (see COPYING file).
