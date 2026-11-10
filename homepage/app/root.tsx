import { useState, type ReactNode } from "ardo/react";
import { ArdoActiveLink, useArdoPageData, useArdoPath } from "ardo/runtime";
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
import { WORKSHOP } from "ferramenta-family/registry";
import "ferramenta-family/monografie.css";
import "ferramenta-family/monografie-docs.css";
import "ferramenta-family/monografie-ardo.css";
import "./site.css";
import icon from "./icon.svg?url";

const sections = [
  {
    id: "guide",
    label: "Guide",
    pages: [
      ["Getting started", "/guide/getting-started"],
      ["Status", "/guide/status"],
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
function Article({ children }: { children: ReactNode }) {
  const page = useArdoPageData();
  return (
    <ArdoDocContent
      tocSource="dom"
      header={
        <header className="mono-prose">
          <h1>{page?.title ?? "Documentation"}</h1>
        </header>
      }
    >
      <div className="mono-prose">{children}</div>
    </ArdoDocContent>
  );
}
export function Layout({ children }: { children: ReactNode }) {
  return (
    <ArdoRootLayout colorScheme="light" className="mono-ardo" icon={icon} iconBasePath={false}>
      {children}
    </ArdoRootLayout>
  );
}
export const ErrorBoundary = ArdoErrorBoundary;
export default function Root() {
  const path = useArdoPath();
  const home = path === "/" || path === "";
  const [open, setOpen] = useState(false);
  return (
    <MonoPage className={home ? "fx-site" : "mono-docs fx-docs"}>
      <MarkDefs />
      <ArdoRoot
        className="mono-ardo-shell"
        scrollMode="document"
        header={
          <header className="mono-header fx-header">
            <div className="mono-header-in">
              <ArdoActiveLink className="mono-brand" to="/">
                Ferrixel <span className="fx-status">Experimental</span>
              </ArdoActiveLink>
              <nav className="fx-nav" aria-label="Main">
                <ArdoActiveLink to="/guide/getting-started">Guide</ArdoActiveLink>
                <ArdoActiveLink to="/codecs/jpeg">Codecs</ArdoActiveLink>
                <a href="https://ferramenta.dev">Ferramenta</a>
              </nav>
              <div className="mono-header-actions">
                <ArdoMobileNavigation
                  title="Ferrixel documentation"
                  open={open}
                  onOpenChange={setOpen}
                >
                  <Sidebar />
                </ArdoMobileNavigation>
                <ArdoHeaderSearch variant="icon" onActivate={() => setOpen(false)} />
                <a
                  className="mono-repository"
                  href="https://github.com/sebastian-software/ferrixel"
                  aria-label="Source code on GitHub"
                >
                  <svg className="icon" width="20" height="20" aria-hidden="true">
                    <use href="#i-github" />
                  </svg>
                </a>
              </div>
            </div>
          </header>
        }
        sidebar={home ? false : <Sidebar />}
        footer={false}
        components={{ table: MonoReadingTable, wrapper: Article }}
      />
      <footer className="mono-footer">
        <div className="mono-footer-in">
          <p>
            Ferrixel, an experimental image pipeline from{" "}
            <a href="https://ferramenta.dev">Ferramenta</a>.
          </p>
          <a href="https://github.com/sebastian-software/ferrixel">GitHub</a>
          <a href={WORKSHOP.imprint}>Imprint</a>
          <a href={WORKSHOP.privacy}>Privacy</a>
          <p className="mono-footer-copy">
            © {new Date().getFullYear()} {WORKSHOP.company}
          </p>
        </div>
      </footer>
    </MonoPage>
  );
}
