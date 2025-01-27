import { expect, test } from 'vitest'
import { optimize } from '../index'

test('sync function from native code', () => {
  const inputXml = `
<?xml version="1.0"?>
<div id="main">
    <p id="paragraph">Example</p>
</div>
`

  const res = optimize(inputXml)
  console.log(res)
  expect(1).toBe(1)
})
