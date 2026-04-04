import clsx from 'clsx';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';

import styles from './index.module.css';

const guideCards = [
  {
    title: 'Start Fast',
    to: '/docs/getting-started',
    body: 'Build a client, choose a resource family, and understand the mirror versus convenience split.',
  },
  {
    title: 'Inspect Coverage',
    to: '/docs/api-coverage',
    body: 'Track exact endpoint parity against Alpaca Market Data and inspect current implemented gaps.',
  },
  {
    title: 'Use the API Reference',
    to: '/docs/reference',
    body: 'Browse generated module pages for clients, request structs, response wrappers, models, and enums.',
  },
  {
    title: 'Explore the Repository',
    to: '/docs/project-structure',
    body: 'See the source tree, examples, tests, benchmarks, and the documentation generation pipeline.',
  },
];

const resourceCards = [
  {title: 'Stocks', to: '/docs/reference/stocks', body: 'Historical, latest, snapshot, and metadata endpoints with batch and single-symbol coverage.'},
  {title: 'Options', to: '/docs/reference/options', body: 'Historical, latest, snapshots, chain lookups, and exchange metadata.'},
  {title: 'Crypto', to: '/docs/reference/crypto', body: 'Historical, latest, orderbook, and snapshot endpoints for the implemented public data routes.'},
  {title: 'News', to: '/docs/reference/news', body: 'News listing with strict mirror semantics and pagination helpers.'},
  {title: 'Corporate Actions', to: '/docs/reference/corporate-actions', body: 'Corporate action listing with paginated collection helpers.'},
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
                <h1 className={styles.title}>A documentation site that stays aligned with the shipped crate.</h1>
                <p className={styles.subtitle}>
                  The site combines hand-written guides, generated project structure pages, generated API reference pages,
                  and GitHub Pages-hosted rustdoc deep links.
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
                <div className={styles.panelLabel}>Public contract</div>
                <ul className={styles.panelList}>
                  <li>Official Alpaca Market Data HTTP API is the semantic source of truth.</li>
                  <li>Request and response field words stay aligned with the official API.</li>
                  <li>Rust public naming stays idiomatic and async-only.</li>
                  <li>The docs site ships with both narrative docs and rustdoc deep links.</li>
                </ul>
              </div>
            </div>
          </div>
        </section>

        <section className={styles.section}>
          <div className="container">
            <div className={styles.sectionHeader}>
              <h2>Guide paths</h2>
              <p>Use the narrative docs to understand the crate contract before diving into generated type pages.</p>
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
              <h2>Reference modules</h2>
              <p>Each module page lists public client methods, request structs, response wrappers, models, enums, and related tests.</p>
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
