import { expect, test } from 'vitest'
import { optimize } from '../index'

test('sync function from native code', () => {
  const inputXml = `
<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox=" 0 0  150 100 " width="150">
  <!-- Created with love! -->
  <defs>
    <ellipse cx="50" cy="50.0" rx="50.00" ry="auto" fill="black" id="circle"/>
  </defs>
  <g>
    <use href="#circle" transform="skewX(16)"/>
    <rect id="useless" width="0" height="0" fill="#ff0000"/>
  </g>
</svg>
`

  const res = optimize(inputXml)
  // biome-ignore lint/suspicious/noConsole: <explanation>
  console.log(res)
  expect(1).toBe(1)
})
