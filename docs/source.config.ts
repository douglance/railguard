import { defineDocs, defineConfig } from "fumadocs-mdx/config";
import { rehypeCodeDefaultOptions } from "fumadocs-core/mdx-plugins";
import { orangeDarkTheme, orangeLightTheme } from "./lib/orange-dark-theme";

export const docs = defineDocs({
  dir: "content/docs",
});

export default defineConfig({
  mdxOptions: {
    rehypeCodeOptions: {
      themes: {
        // Custom orange themes based on Atom One Dark aesthetic
        light: orangeLightTheme,
        dark: orangeDarkTheme,
      },
      transformers: [...(rehypeCodeDefaultOptions.transformers ?? [])],
    },
  },
});
