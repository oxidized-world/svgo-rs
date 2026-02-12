import { expect, test } from "vitest";
import { runWithPlugins } from "./test-utils";

test("minifyStyles minifies style element css", () => {
  const input = '<svg><style> .a { fill : red ; } </style><rect class="a"/></svg>';
  const out = runWithPlugins(input, ["minifyStyles"]);
  expect(out).toContain("<style>.a{fill:red;}</style>");
});

test("minifyStyles minifies style attribute", () => {
  const input = '<svg><rect style=" fill : red ; stroke : blue ; "/></svg>';
  const out = runWithPlugins(input, ["minifyStyles"]);
  expect(out).toContain('style="fill:red;stroke:blue;"');
});

test("minifyStyles removes css comments", () => {
  const input = '<svg><style>/*x*/ .a { fill : red ; } </style><rect style="/*a*/ fill : blue ;"/></svg>';
  const out = runWithPlugins(input, ["minifyStyles"]);
  expect(out).not.toContain("/*");
});

test("minifyStyles removes empty style element", () => {
  const input = '<svg><style>   </style><rect/></svg>';
  const out = runWithPlugins(input, ["minifyStyles"]);
  expect(out).toBe('<svg><rect/></svg>');
});
