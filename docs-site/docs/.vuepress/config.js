import { viteBundler } from "@vuepress/bundler-vite";
import { defaultTheme } from "@vuepress/theme-default";

export default {
  lang: "en-US",
  title: "AktSQL",
  description: "AktSQL Database Management official site.",
  head: [
    ["link", { rel: "icon", href: "/aktsql-logo.svg" }],
    ["meta", { name: "theme-color", content: "#090909" }],
    ["meta", { property: "og:title", content: "AktSQL Database Management" }],
    [
      "meta",
      {
        property: "og:description",
        content:
          "A focused SQL GUI for database browsing, query execution, and schema editing.",
      },
    ],
  ],
  bundler: viteBundler(),
  theme: defaultTheme({
    logo: "/aktsql-logo.svg",
    navbar: [
      { text: "Guide", link: "/guide/getting-started.html" },
      { text: "Downloads", link: "/downloads.html" },
      { text: "Deploy", link: "/cloudflare-pages.html" },
      { text: "GitHub", link: "https://github.com/AktSQL/aktsql" },
    ],
    sidebar: [
      { text: "Overview", link: "/" },
      { text: "Getting Started", link: "/guide/getting-started.html" },
      { text: "Downloads", link: "/downloads.html" },
      { text: "Cloudflare Pages", link: "/cloudflare-pages.html" },
    ],
    repo: "AktSQL/aktsql",
    docsDir: "docs-site/docs",
    editLink: false,
    lastUpdated: false,
    contributors: false,
  }),
};
