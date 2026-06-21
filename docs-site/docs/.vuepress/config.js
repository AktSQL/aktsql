import { viteBundler } from "@vuepress/bundler-vite";
import { defaultTheme } from "@vuepress/theme-default";

export default {
  lang: "zh-CN",
  title: "AktSQL",
  description: "AktSQL Database Management 官方网站。",
  head: [
    ["link", { rel: "icon", href: "/aktsql-logo.svg" }],
    ["meta", { name: "theme-color", content: "#090909" }],
    ["meta", { property: "og:title", content: "AktSQL Database Management" }],
    [
      "meta",
      {
        property: "og:description",
        content:
          "面向数据库浏览、SQL 查询和表结构编辑的桌面 SQL GUI。",
      },
    ],
  ],
  bundler: viteBundler(),
  theme: defaultTheme({
    logo: "/aktsql-logo.svg",
    navbar: [
      { text: "指南", link: "/guide/getting-started.html" },
      { text: "下载", link: "/downloads.html" },
      { text: "部署", link: "/cloudflare-pages.html" },
    ],
    sidebar: [
      { text: "概览", link: "/" },
      { text: "快速开始", link: "/guide/getting-started.html" },
      { text: "下载安装", link: "/downloads.html" },
      { text: "Cloudflare Pages", link: "/cloudflare-pages.html" },
    ],
    repo: "AktSQL/aktsql",
    docsDir: "docs-site/docs",
    editLink: false,
    lastUpdated: false,
    contributors: false,
  }),
};
