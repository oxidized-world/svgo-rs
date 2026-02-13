import { expect, test } from "vitest";
import { optimize as optimizeSvgo } from "svgo";
import { runWithPlugins } from "./test-utils";

test("prefixIds prefixes id and references", () => {
  const input = '<svg><defs><clipPath id="a"><rect width="10" height="10"/></clipPath></defs><rect clip-path="url(#a)"/></svg>';
  const out = runWithPlugins(input, ["prefixIds"], {
    prefixIdsPrefix: "p",
    prefixIdsDelim: "__",
    prefixIdsPrefixIds: true,
    prefixIdsPrefixClassNames: true,
  });
  const expected = optimizeSvgo(input, {
    plugins: [{ name: "prefixIds", params: { prefix: "p", delim: "__", prefixIds: true, prefixClassNames: true } }],
  }).data;
  expect(out).toBe(expected);
});

test("prefixIds rewrites class names and style selectors", () => {
  const input = '<svg><style>.a{fill:url(#g)}</style><defs><linearGradient id="g"/></defs><rect class="a"/></svg>';
  const out = runWithPlugins(input, ["prefixIds"], {
    prefixIdsPrefix: "x",
    prefixIdsDelim: "-",
    prefixIdsPrefixIds: true,
    prefixIdsPrefixClassNames: true,
  });
  const expected = optimizeSvgo(input, {
    plugins: [
      { name: "prefixIds", params: { prefix: "x", delim: "-", prefixIds: true, prefixClassNames: true } },
    ],
  }).data;
  expect(out).toBe(expected);
});

test("prefixIds can disable class prefixing", () => {
  const input = '<svg><style>.a{fill:red}</style><rect class="a" id="r"/></svg>';
  const out = runWithPlugins(input, ["prefixIds"], {
    prefixIdsPrefix: "p",
    prefixIdsDelim: "__",
    prefixIdsPrefixIds: true,
    prefixIdsPrefixClassNames: false,
  });
  const expected = optimizeSvgo(input, {
    plugins: [
      {
        name: "prefixIds",
        params: { prefix: "p", delim: "__", prefixIds: true, prefixClassNames: false },
      },
    ],
  }).data;
  expect(out).toBe(expected);
});
