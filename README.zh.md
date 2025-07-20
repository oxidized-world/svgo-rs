# svgo-rs

`svgo-rs` 是一个用 Rust 编写的高性能 SVG 优化器，旨在高效地处理和优化 SVG 文件。它通过 NAPI-RS 提供 Node.js 的绑定，使 JavaScript 和 TypeScript 开发者可以轻松使用。

## 特性

- 使用 Rust 编写，性能卓越。
- 提供多种 SVG 优化插件，例如移除元数据、注释和不必要的元素。
- 支持多个平台，并提供预编译的二进制文件。
- 可轻松集成到 Node.js 项目中。

## 安装

通过 npm 安装此包：

```bash
npm install @svg-rs/svgo
```

## 使用方法

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


## 插件

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

## 基准测试

`svgo-rs` 相较于基于 JavaScript 的 SVG 优化器提供了显著的性能提升。可以在 `benchmark/` 目录中运行基准测试进行比较：

```bash
yarn bench
```

## 许可证

此项目基于 MIT 许可证授权。详情请参阅 [LICENSE](./LICENSE) 文件。

## 贡献

欢迎贡献！请在 [GitHub](https://github.com/oxidized-world/svgo-rs) 上提交 issue 或 pull request。
