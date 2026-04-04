import clsx from 'clsx';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';

import styles from './index.module.css';

const guideCards = [
  {
    title: 'Get Started',
    to: '/docs/getting-started',
    body: 'Create a client, pick a resource family, and understand mirror versus convenience methods.',
  },
  {
    title: 'Check Coverage',
    to: '/docs/api-coverage',
    body: 'See which official endpoints are implemented and which gaps are still open.',
  },
  {
    title: 'Use the API Reference',
    to: '/docs/reference',
    body: 'Open generated pages for clients, requests, responses, models, enums, and rustdoc links.',
  },
  {
    title: 'Project Layout',
    to: '/docs/project-structure',
    body: 'See the source tree, examples, tests, benchmarks, and docs generation entry points.',
  },
];

const resourceCards = [
  {title: 'Stocks', to: '/docs/reference/stocks', body: 'Historical data, latest data, snapshots, and metadata endpoints with batch and single-symbol variants.'},
  {title: 'Options', to: '/docs/reference/options', body: 'Historical data, latest data, snapshots, chain lookups, and exchange metadata.'},
  {title: 'Crypto', to: '/docs/reference/crypto', body: 'Historical data, latest data, orderbooks, and snapshots.'},
  {title: 'News', to: '/docs/reference/news', body: 'News listing with `list`, `list_all`, and `list_stream`.'},
  {title: 'Corporate Actions', to: '/docs/reference/corporate-actions', body: 'Corporate action listing with `list`, `list_all`, and `list_stream`.'},
];

export default function Home(): JSX.Element {
  return (
    <Layout
      title="Documentation"
      description="Public documentation and generated API reference for alpaca-data."
    >
      <main>
        <section className={styles.hero}>
          <div className="container">
            <div className={styles.heroGrid}>
              <div>
                <p className={styles.kicker}>Rust + Alpaca Market Data</p>
                <h1 className={styles.title}>High-Performance Rust Client for Alpaca Market Data API</h1>
                <p className={styles.subtitle}>
                  Typed async access to the official HTTP endpoints, plus pagination helpers for paginated routes.
                  The docs cover usage, endpoint coverage, examples, and rustdoc links.
                </p>
                <div className={styles.actions}>
                  <Link className="button button--primary button--lg" to="/docs/getting-started">
                    Read the docs
                  </Link>
                  <Link className="button button--secondary button--lg" to="/docs/reference">
                    Browse API reference
                  </Link>
                </div>
              </div>
              <div className={styles.heroPanel}>
                <div className={styles.panelLabel}>Crate contract</div>
                <ul className={styles.panelList}>
                  <li>The official Alpaca Market Data HTTP API defines endpoint semantics.</li>
                  <li>Request and response field words use Alpaca&apos;s original API terms.</li>
                  <li>Public Rust names stay idiomatic and async-only.</li>
                  <li>The docs include guides, coverage tables, examples, and rustdoc links.</li>
                </ul>
              </div>
            </div>
          </div>
        </section>

        <section className={styles.section}>
          <div className="container">
            <div className={styles.sectionHeader}>
              <h2>Core Guides</h2>
              <p>Start with crate behavior, auth rules, and repository layout before jumping into generated type pages.</p>
            </div>
            <div className={styles.cardGrid}>
              {guideCards.map((card) => (
                <Link key={card.title} className={clsx(styles.card, styles.guideCard)} to={card.to}>
                  <h3>{card.title}</h3>
                  <p>{card.body}</p>
                </Link>
              ))}
            </div>
          </div>
        </section>

        <section className={clsx(styles.section, styles.resourceSection)}>
          <div className="container">
            <div className={styles.sectionHeader}>
              <h2>API Modules</h2>
              <p>Each module page lists client methods, request structs, response wrappers, models, enums, tests, and rustdoc links.</p>
            </div>
            <div className={styles.cardGrid}>
              {resourceCards.map((card) => (
                <Link key={card.title} className={clsx(styles.card, styles.resourceCard)} to={card.to}>
                  <h3>{card.title}</h3>
                  <p>{card.body}</p>
                </Link>
              ))}
            </div>
          </div>
        </section>
      </main>
    </Layout>
  );
}
