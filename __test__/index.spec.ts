import { expect, test } from 'vitest'
import { optimize } from '../index'

test('sync function from native code', () => {
  const inputXml = `
<?xml version="1.0" encoding="UTF-8"?>
<svg width="100px" height="100px" viewBox="0 0 100 100"
     xmlns="http://www.w3.org/2000/svg"
     xmlns:xlink="http://www.w3.org/1999/xlink"
     xmlns:myeditor="http://www.myeditor.com/ns"
     version="1.1">
    <myeditor:metadata>
        <myeditor:source>AwesomeIcon Design</myeditor:source>
        <myeditor:version>1.2</myeditor:version>
        <myeditor:exporter>MyEditor Pro Exporter v3.0</myeditor:exporter>
    </myeditor:metadata>
    <title>My Awesome Icon</title>
    <desc>A blue circle with a red star inside.</desc>
    <g id="BackgroundLayer" myeditor:layerName="Background">
        <circle cx="50" cy="50" r="45" fill="blue" id="blue_circle_bg" myeditor:objectID="obj123"/>
    </g>
    <g id="ForegroundLayer" myeditor:layerName="Foreground" myeditor:isDecorative="false">
        <polygon points="50,15 61,35 85,35 67,50 73,70 50,60 27,70 33,50 15,35 39,35"
                 fill="red"
                 id="red_star_shape"
                 myeditor:objectID="obj456"
                 myeditor:customAttribute="important_shape"/>
    </g>
</svg>
`

  const res = optimize(inputXml)
  // biome-ignore lint/suspicious/noConsole: <explanation>
  console.log(res)
  expect(1).toBe(1)
})
