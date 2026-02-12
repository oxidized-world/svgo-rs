# svgo-rs

`svgo-rs` is a high-performance SVG optimizer written in Rust, designed to process and optimize SVG files efficiently. It provides bindings for Node.js through NAPI-RS, making it accessible for JavaScript and TypeScript developers.

## Features

- Written in Rust for high performance.
- Provides plugins for various SVG optimizations, such as removing metadata, comments, and unnecessary elements.
- Supports multiple platforms with precompiled binaries.
- Easy integration with Node.js projects.

## Installation

Install the package via npm:

```bash
npm install @svg-rs/svgo
```

- [ ] cleanupIds
- [ ] removeUselessDefs
- [ ] cleanupNumericValues
- [ ] convertColors
- [ ] removeUnknownsAndDefaults
- [x] convertPathData
- [x] convertTransform
- [ ] removeEmptyAttrs

- [ ] addClassesToSVGElement
- [ ] cleanupListOfValues
- [x] convertOneStopGradients
- [x] convertStyleToAttrs
- [ ] prefixIds
- [ ] removeAttributesBySelector
- [ ] removeAttrs
- [ ] removeDimensions
- [ ] removeElementsByAttr
- [ ] removeOffCanvasPaths
- [ ] removeRasterImages
- [ ] removeScripts
- [ ] removeStyleElement
- [ ] removeViewBox
- [ ] removeXMLNS
- [ ] removeXlink
- [ ] reusePaths

- [x] convertEllipseToCircle
- [x] convertShapeToPath
- [x] inlineStyles
- [x] mergePaths
- [x] mergeStyles
- [x] minifyStyles

## Benchmarks

`svgo-rs` provides significant performance improvements over JavaScript-based SVG optimizers. Run the benchmarks in the `benchmark/` directory to compare:

```bash
yarn bench
```

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on [GitHub](https://github.com/oxidized-world/svgo-rs).
