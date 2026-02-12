import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("cleanupIds removes unused ids", () => {
  const input = '<svg><g id="a"/><g id="b"/></svg>';
  const out = runWithPlugins(input, ["cleanupIds"], {
    cleanupIdsRemove: true,
    cleanupIdsMinify: false,
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "cleanupIds", params: { remove: true, minify: false } }],
  }).data;
  expect(out).toBe(expected);
});

test("cleanupIds minifies referenced ids", () => {
  const input = '<svg><defs><clipPath id="myClip"><rect width="10" height="10"/></clipPath></defs><rect clip-path="url(#myClip)"/></svg>';
  const out = runWithPlugins(input, ["cleanupIds"], {
    cleanupIdsRemove: true,
    cleanupIdsMinify: true,
    cleanupIdsForce: true,
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "cleanupIds", params: { remove: true, minify: true, force: true } }],
  }).data;
  expect(out).toBe(expected);
});

test("cleanupIds handles malformed reference and avoids conflicting generated id", () => {
  const input =
    '<svg xmlns="http://www.w3.org/2000/svg"><defs><path id="uwu" d="M0 0"/></defs><use href="#a"/><use href="#uwu"/></svg>';
  const out = runWithPlugins(input, ["cleanupIds"], {
    cleanupIdsRemove: true,
    cleanupIdsMinify: true,
    cleanupIdsForce: true,
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "cleanupIds", params: { remove: true, minify: true, force: true } }],
  }).data;
  expect(out).toBe(expected);
});

test("cleanupIds handles non-ascii ids and url references", () => {
  const input =
    '<svg xmlns="http://www.w3.org/2000/svg"><defs><linearGradient id="渐变_1"><stop stop-color="#5a2100"/></linearGradient></defs><rect fill="url(#渐变_1)"/></svg>';
  const out = runWithPlugins(input, ["cleanupIds"], {
    cleanupIdsRemove: true,
    cleanupIdsMinify: true,
    cleanupIdsForce: true,
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "cleanupIds", params: { remove: true, minify: true, force: true } }],
  }).data;
  expect(out).toBe(expected);
});
