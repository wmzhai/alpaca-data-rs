import type {SidebarsConfig} from "@docusaurus/plugin-content-docs";

const sidebars: SidebarsConfig = {
  docsSidebar: [
    {
      type: "category",
      label: "Overview",
      collapsed: false,
      items: [
        "index"
      ]
    },
    {
      type: "category",
      label: "Guide",
      collapsed: false,
      items: [
        "getting-started",
        "authentication",
        "layers"
      ]
    },
    {
      type: "category",
      label: "Architecture",
      collapsed: false,
      items: [
        "project-structure"
      ]
    },
    {
      type: "category",
      label: "API Reference",
      collapsed: false,
      items: [
        "reference/index",
        "reference/client",
        "reference/stocks",
        "reference/options",
        "reference/crypto",
        "reference/news",
        "reference/corporate-actions",
        "reference/common",
        "reference/transport"
      ]
    },
    {
      type: "category",
      label: "Coverage",
      collapsed: false,
      items: [
        "api-coverage"
      ]
    },
    {
      type: "category",
      label: "Examples",
      collapsed: false,
      items: [
        "examples"
      ]
    },
    {
      type: "category",
      label: "Release",
      collapsed: false,
      items: [
        "release-checklist"
      ]
    }
  ]
};

export default sidebars;
