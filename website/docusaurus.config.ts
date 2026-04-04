import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'alpaca-data',
  tagline: 'High-Performance Rust Client for Alpaca Market Data API',
  favicon: 'img/logo.svg',
  url: 'https://wmzhai.github.io',
  baseUrl: '/alpaca-data-rs/',
  organizationName: 'wmzhai',
  projectName: 'alpaca-data-rs',
  trailingSlash: false,
  onBrokenLinks: 'throw',
  markdown: {
    hooks: {
      onBrokenMarkdownLinks: 'throw',
    },
  },
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },
  presets: [
    [
      'classic',
      {
        docs: {
          path: '../docs',
          routeBasePath: 'docs',
          sidebarPath: './sidebars.ts',
          editUrl: 'https://github.com/wmzhai/alpaca-data-rs/tree/main/',
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],
  themeConfig: {
    image: 'img/social-card.svg',
    navbar: {
      title: 'alpaca-data',
      logo: {
        alt: 'alpaca-data logo',
        src: 'img/logo.svg',
      },
      items: [
        {to: '/docs', label: 'Docs', position: 'left'},
        {to: '/docs/reference', label: 'API Reference', position: 'left'},
        {to: '/docs/examples', label: 'Examples', position: 'left'},
        {href: 'https://docs.rs/alpaca-data', label: 'docs.rs', position: 'right'},
        {href: 'https://github.com/wmzhai/alpaca-data-rs', label: 'GitHub', position: 'right'},
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Docs',
          items: [
            {label: 'Getting Started', to: '/docs/getting-started'},
            {label: 'Project Structure', to: '/docs/project-structure'},
            {label: 'API Coverage', to: '/docs/api-coverage'},
          ],
        },
        {
          title: 'Reference',
          items: [
            {label: 'Client', to: '/docs/reference/client'},
            {label: 'Stocks', to: '/docs/reference/stocks'},
            {label: 'Options', to: '/docs/reference/options'},
          ],
        },
        {
          title: 'Links',
          items: [
            {label: 'docs.rs', href: 'https://docs.rs/alpaca-data'},
            {label: 'Repository', href: 'https://github.com/wmzhai/alpaca-data-rs'},
            {label: 'GitHub Pages', href: 'https://wmzhai.github.io/alpaca-data-rs/'},
          ],
        },
      ],
      copyright: `Copyright ${new Date().getFullYear()} alpaca-data contributors.`,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
