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

## Usage

### Node.js

```javascript
const { optimize } = require('@svg-rs/svgo')

const inputSvg = `<svg xmlns="http://www.w3.org/2000/svg">
  <g attr1="val1">
    <desc>Example</desc>
    <circle cx="50" cy="50" r="40"/>
  </g>
</svg>`

const optimizedSvg = optimize(inputSvg)
console.log(optimizedSvg)
```

## Plugins

- [ ] addAttributesToSVGElement
- [ ] addClassesToSVGElement
- [ ] cleanupAttrs
- [ ] cleanupEnableBackground
- [ ] cleanupIds
- [ ] cleanupListOfValues
- [ ] cleanupNumericValues
- [ ] collapseGroups
- [ ] convertColors
- [ ] convertEllipseToCircle
- [ ] convertOneStopGradients
- [ ] convertPathData
- [ ] convertShapeToPath
- [ ] convertStyleToAttrs
- [ ] convertTransform
- [ ] inlineStyles
- [ ] mergePaths
- [ ] mergeStyles
- [ ] minifyStyles
- [x] moveElemsAttrsToGroup
- [ ] moveGroupAttrsToElems
- [ ] prefixIds
- [ ] removeAttributesBySelector
- [ ] removeAttrs
- [x] removeComments
- [ ] removeDeprecatedAttrs
- [x] removeDesc
- [ ] removeDimensions
- [x] removeDoctype
- [x] removeEditorsNSData
- [ ] removeElementsByAttr
- [ ] removeEmptyAttrs
- [ ] removeEmptyContainers
- [ ] removeEmptyText
- [ ] removeHiddenElems
- [x] removeMetadata
- [ ] removeNonInheritableGroupAttrs
- [ ] removeOffCanvasPaths
- [ ] removeRasterImages
- [ ] removeScripts
- [ ] removeStyleElement
- [x] removeTitle
- [ ] removeUnknownsAndDefaults
- [ ] removeUnusedNS
- [ ] removeUselessDefs
- [ ] removeUselessStrokeAndFill
- [ ] removeViewBox
- [ ] removeXMLNS
- [x] removeXMLProcInst
- [ ] removeXlink
- [ ] reusePaths
- [ ] sortAttrs
- [ ] sortDefsChildren

## Benchmarks

`svgo-rs` provides significant performance improvements over JavaScript-based SVG optimizers. Run the benchmarks in the `benchmark/` directory to compare:

```bash
yarn bench
```

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on [GitHub](https://github.com/oxidized-world/svgo-rs).
