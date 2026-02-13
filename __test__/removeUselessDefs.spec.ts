import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("removeUselessDefs removes defs content without id", () => {
  const input = '<svg><defs><path d="M0 0"/></defs><rect width="1" height="1"/></svg>';
  const out = runWithPlugins(input, ["removeUselessDefs"]);
  const expected = optimizeSvgo(input, { plugins: ["removeUselessDefs"] }).data;
  expect(out).toBe(expected);
});

test("removeUselessDefs keeps referenced defs children", () => {
  const input = '<svg><defs><path id="p" d="M0 0"/></defs><use href="#p"/></svg>';
  const out = runWithPlugins(input, ["removeUselessDefs"]);
  const expected = optimizeSvgo(input, { plugins: ["removeUselessDefs"] }).data;
  expect(out).toBe(expected);
});

test("removeUselessDefs keeps style in defs", () => {
  const input = '<svg><defs><style>.a{fill:red}</style></defs><rect class="a"/></svg>';
  const out = runWithPlugins(input, ["removeUselessDefs"]);
  const expected = optimizeSvgo(input, { plugins: ["removeUselessDefs"] }).data;
  expect(out).toBe(expected);
});
