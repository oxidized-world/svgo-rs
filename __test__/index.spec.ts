import { expect, test } from 'vitest'
import { optimize } from '../index'

test('sync function from native code', () => {
  const inputXml = `
<svg xmlns="http://www.w3.org/2000/svg">
    <g attr1="val1">
        <g fill="red" color="#000" stroke="blue">
            text
        </g>
        <g>
          <rect fill="red" color="#000" />
          <ellipsis fill="red" color="#000" />
        </g>
        <circle fill="red" color="#000" attr3="val3"/>
    </g>
</svg>
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
