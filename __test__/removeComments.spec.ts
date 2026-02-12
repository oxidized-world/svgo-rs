import { expect, test } from "vitest";
import { runWithPlugins } from "./test-utils";

test("removeComments removes comments except preserved patterns by default", () => {
  const input = "<svg><!--! keep --><!-- remove --><g/></svg>";
  const out = runWithPlugins(input, ["removeComments"]);
  expect(out).toContain("<!--! keep -->");
  expect(out).not.toContain("<!-- remove -->");
});

test("removeComments can disable preservePatterns (remove all comments)", () => {
  const input = "<svg><!--! keep --><!-- remove --><g/></svg>";
  const out = runWithPlugins(input, ["removeComments"], {
    removeCommentsPreservePatternsDisabled: true,
  });
  expect(out).not.toContain("<!--! keep -->");
  expect(out).not.toContain("<!-- remove -->");
});
