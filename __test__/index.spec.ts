import { readdirSync, readFileSync } from 'node:fs'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

import { describe, expect, test } from 'vitest'

import { optimize } from '../index'

const fixturesDir = fileURLToPath(new URL('./fixtures', import.meta.url))

const readFixture = (name: string) => readFileSync(join(fixturesDir, name), 'utf8')

const fixtures = readdirSync(fixturesDir)
  .filter((name) => name.endsWith('.svg'))
  .sort()

describe('optimize', () => {
  test('there are fixtures to run', () => {
    expect(fixtures.length).toBeGreaterThan(0)
  })

  // Every fixture output is snapshotted so that any change in behaviour (for
  // example while upgrading dependencies) shows up as an explicit diff.
  test.each(fixtures)('%s', (name) => {
    const output = optimize(readFixture(name))

    expect(typeof output).toBe('string')
    expect(output).toContain('<svg')
    expect(output).toMatchSnapshot()
  })

  // Snapshot serialisation rewrites `\r` to `\n`, so raw whitespace inside
  // attribute values needs an explicit byte level assertion. XML attribute
  // value normalisation (tab/CR/LF -> space) is deliberately NOT applied, to
  // keep the value exactly as it was written in the source document.
  test('keeps raw whitespace inside attribute values', () => {
    const output = optimize(readFixture('multiline-attr.svg'))

    expect(output).toContain('data-tab="a\tb"')
    expect(output).toContain('data-cr="x\ry"')
    expect(output).toContain('points="0,0\n1,1\n2,0"')
    expect(output).toContain('data-mixed=" lead\n  and\ttab  trail "')
  })
})
