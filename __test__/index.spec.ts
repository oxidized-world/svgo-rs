import { expect, test } from 'vitest'
import { main } from '../index'

test('sync function from native code', () => {
  const res = main()
  console.log("[debug log] - fromnativecode', - res:", res)

  expect(1).toBe(1)
})
