# Product Overview

svgo-rs is a high-performance SVG optimizer written in Rust with Node.js bindings via NAPI-RS. It processes and optimizes SVG files by removing unnecessary elements, attributes, and metadata while preserving visual integrity.

## Key Features
- High-performance Rust implementation
- Plugin-based architecture for modular SVG optimizations
- Cross-platform support with precompiled binaries
- Node.js integration through NAPI bindings
- Significant performance improvements over JavaScript-based optimizers

## Target Use Cases
- SVG file optimization in build pipelines
- Reducing SVG file sizes for web applications
- Batch processing of SVG assets
- Integration into Node.js/JavaScript toolchains

## Current Plugin Status
The project implements a subset of SVGO plugins with focus on:
- Metadata removal (comments, descriptions, titles, doctype)
- Namespace cleanup (editor data, XML processing instructions)
- Attribute optimization (moving common attributes to parent groups)
