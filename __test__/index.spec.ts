import { expect, test } from 'vitest'
import { optimize } from '../index'

test('sync function from native code', () => {
  const inputXml = `
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<?xml version="1.0"?>
<g name="lcs" id="main" class="container">
    <p name="lcs" id="paragraph">Example1</p>
    <p name="lcs" id="paragraph">Example2</p>
    <text name="lcs" id="text">
      <desc>111</desc>
    </text>
</g>
`

  const res = optimize(inputXml, {
    plugins: {
      removeDesc: {
        removeAny: true,
      },
    },
  })
  console.log(res)
  expect(1).toBe(1)
})
