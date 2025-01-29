import { expect, test } from 'vitest'
import { optimize } from '../index'

test('sync function from native code', () => {
  const inputXml = `
<?xml version="1.0"?>
<g name="lcs" id="main" class="container">
    <p name="lcs" id="paragraph">Example</p>
    <text name="lcs" id="text"></text>
</g>
`

  const res = optimize(inputXml)
  console.log(res)
  expect(1).toBe(1)
})
