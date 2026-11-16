/*
 * The shell of every page, built like `MonoArdoSite` from ferramenta-family/ardo:
 * the steel header in Ardo's header slot, Ardo's sidebar and menu on the
 * documentation pages, Monografie's prose as the MDX wrapper, and the footer
 * outside Ardo's main landmark. The header and footer are site-local until
 * Ferrisma is in the family registry (./site/chrome.tsx).
 */
import { useState, type ReactNode } from "ardo/react";
import { ArdoActiveLink, useArdoConfig, useArdoPageData, useArdoPath } from "ardo/runtime";
import {
  ArdoDocContent,
  ArdoErrorBoundary,
  ArdoHeaderSearch,
  ArdoMobileNavigation,
  ArdoRoot,
  ArdoRootLayout,
  ArdoSidebar,
  ArdoSidebarLink,
  ArdoSidebarSection,
} from "ardo/ui";
import { MarkDefs, MonoPage, MonoReadingTable } from "ferramenta-family/monografie";
import "ferramenta-family/monografie.css";
import "ferramenta-family/monografie-docs.css";
import "ferramenta-family/monografie-ardo.css";
import "./site.css";
import "./drawings/drawings.css";
import appleTouchIcon from "./assets/ferrisma-apple-touch-icon.png?url";
import icon from "./assets/ferrisma-favicon.png?url";
import { NAME } from "./ferrisma";
import { SiteFooter, SiteHeader } from "./site/chrome";

const sections = [
  {
    id: "guide",
    label: "Guide",
    pages: [
      ["Getting started", "/guide/getting-started"],
      ["How a port is made", "/how-a-port-is-made"],
      ["Status", "/guide/status"],
      ["Quality gate", "/guide/quality-gate"],
      ["Supported inputs and outputs", "/guide/supported-matrix"],
      ["Architecture", "/guide/architecture"],
      ["Color", "/guide/color"],
      ["Codec development", "/guide/codec-development"],
      ["Release gates", "/guide/release-gates"],
      ["Development", "/guide/development"],
      ["Validation", "/guide/validation"],
    ],
  },
  {
    id: "codecs",
    label: "Codecs",
    pages: [
      ["JPEG", "/codecs/jpeg"],
      ["WebP", "/codecs/webp"],
      ["JPEG XL", "/codecs/jpeg-xl"],
      ["AVIF", "/codecs/avif"],
    ],
  },
];

/** The header's links: pages, never anchors on the homepage. */
const links = [
  <ArdoActiveLink key="guide" to="/guide/getting-started">
    Guide
  </ArdoActiveLink>,
  <ArdoActiveLink key="gate" to="/guide/quality-gate">
    Quality gate
  </ArdoActiveLink>,
  <ArdoActiveLink key="codecs" to="/codecs/jpeg">
    Codecs
  </ArdoActiveLink>,
  <ArdoActiveLink key="status" to="/guide/status">
    Status
  </ArdoActiveLink>,
];

function Sidebar() {
  return (
    <ArdoSidebar>
      {sections.map((section) => (
        <ArdoSidebarSection
          key={section.id}
          id={section.id}
          label={section.label}
          to={section.pages[0][1]}
          match={(path: string) => path === `/${section.id}` || path.startsWith(`/${section.id}/`)}
        >
          {section.pages.map(([label, to]) => (
            <ArdoSidebarLink key={to} to={to}>
              {label}
            </ArdoSidebarLink>
          ))}
        </ArdoSidebarSection>
      ))}
    </ArdoSidebar>
  );
}

/** Ardo's menu and search on documentation pages; the menu carries the header's links too. */
function DocsActions() {
  const [open, setOpen] = useState(false);
  return (
    <>
      <ArdoMobileNavigation
        title={`${NAME} documentation`}
        open={open}
        onOpenChange={setOpen}
        nav={
          <nav className="mono-docs-links" aria-label={`${NAME} site`}>
            <ul>
              {links.map((link) => (
                <li key={link.key}>{link}</li>
              ))}
            </ul>
          </nav>
        }
      >
        <Sidebar />
      </ArdoMobileNavigation>
      <ArdoHeaderSearch variant="icon" onActivate={() => setOpen(false)} />
    </>
  );
}

function Article({ children }: { children: ReactNode }) {
  const page = useArdoPageData();
  return (
    <ArdoDocContent
      tocSource="dom"
      header={
        <header className="mono-prose">
          <h1>{page?.title.replaceAll("`", "") ?? "Documentation"}</h1>
        </header>
      }
    >
      <div className="mono-prose">{children}</div>
    </ArdoDocContent>
  );
}

export function Layout({ children }: { children: ReactNode }) {
  return (
    <ArdoRootLayout
      colorScheme="light"
      className="mono-ardo"
      icon={icon}
      appleTouchIcon={appleTouchIcon}
      iconBasePath={false}
    >
      {children}
    </ArdoRootLayout>
  );
}
export const ErrorBoundary = ArdoErrorBoundary;

export default function Root() {
  const path = useArdoPath();
  const config = useArdoConfig();
  const canonical = config.siteUrl ? new URL(path || "/", config.siteUrl).href : undefined;
  // Pages on the homepage's layout: no documentation sidebar, no MDX wrapper.
  const home = ["", "/", "/how-a-port-is-made", "/how-a-port-is-made/"].includes(path);
  return (
    <MonoPage className={home ? "fx-site" : "mono-docs fx-docs"}>
      {home && canonical && <link rel="canonical" href={canonical} />}
      <MarkDefs />
      <ArdoRoot
        className="mono-ardo-shell"
        scrollMode="document"
        header={
          <SiteHeader
            home="/"
            links={links}
            actions={home ? <ArdoHeaderSearch variant="icon" /> : <DocsActions />}
          />
        }
        sidebar={home ? false : <Sidebar />}
        footer={false}
        components={{ table: MonoReadingTable, wrapper: Article }}
      />
      <SiteFooter home="/" />
    </MonoPage>
  );
}
