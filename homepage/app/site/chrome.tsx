/*
 * Ferrisma's header and footer, in Monografie's own markup.
 *
 * `MonoHeader`, `MonoFooter` and `MonoArdoSite` from `ferramenta-family` take a
 * registered identity (`product`), and Ferrisma is not in the family registry
 * at the pinned package commit. Rather than pass a name the registry does not
 * know, this file renders the same classes the package renders, with the
 * site-local mark (`.fx-mark` in site.css) in place of the registry's
 * `.ferra-mark[data-product]`. The menu behavior is the package's, copied
 * from `MonoChrome.tsx` (MIT, same workshop).
 *
 * When the registry has a `ferrisma` entry with its mark: delete this file and
 * render `MonoArdoSite product="ferrisma"` in root.tsx (see DESIGN-NOTES.md).
 */
import { type ReactElement, type ReactNode, useEffect, useId, useRef, useState } from "ardo/react";
import { WORKSHOP } from "ferramenta-family/registry";

import { FAMILY_SITE, NAME, REPOSITORY } from "../ferrisma";

/** The site-local mark; decorative beside the name. */
function SiteMark() {
  return <span className="ferra-mark fx-mark" aria-hidden="true" />;
}

/** Menu state that closes on Escape, on a click outside or on a link, and when the screen widens. */
function useMenu() {
  const [open, setOpen] = useState(false);
  const headerRef = useRef<HTMLElement>(null);
  const buttonRef = useRef<HTMLButtonElement>(null);
  useEffect(() => {
    if (!open) return undefined;
    const close = () => setOpen(false);
    const onKey = (event: KeyboardEvent) => {
      if (event.key !== "Escape") return;
      close();
      buttonRef.current?.focus();
    };
    const onClick = (event: MouseEvent) => {
      const target = event.target;
      if (!(target instanceof Element)) return;
      if (headerRef.current?.contains(target) !== true || target.closest(".mono-menu a")) close();
    };
    const wide = matchMedia("(min-width: 961px)");
    const onWide = () => {
      if (wide.matches) close();
    };
    document.addEventListener("keydown", onKey);
    document.addEventListener("click", onClick);
    wide.addEventListener("change", onWide);
    return () => {
      document.removeEventListener("keydown", onKey);
      document.removeEventListener("click", onClick);
      wide.removeEventListener("change", onWide);
    };
  }, [open]);
  return { open, setOpen, headerRef, buttonRef };
}

export type SiteHeaderProps = {
  home: string;
  links: ReactElement[];
  /** Search and, on documentation pages, Ardo's menu. */
  actions?: ReactNode;
};

/** The steel navigation bar, as `MonoHeader` draws it; links fold into a menu below 961px. */
export function SiteHeader({ home, links, actions }: SiteHeaderProps) {
  const { open, setOpen, headerRef, buttonRef } = useMenu();
  const menuId = useId();
  return (
    <header className="mono-header" ref={headerRef} data-open={open}>
      <div className="mono-header-in">
        <a className="mono-brand" href={home}>
          <SiteMark />
          {NAME}
        </a>
        <button
          ref={buttonRef}
          className="mono-menu-button"
          type="button"
          aria-expanded={open}
          aria-controls={menuId}
          onClick={() => setOpen(!open)}
        >
          <i aria-hidden="true" />
          <span className="mono-menu-label">Menu</span>
        </button>
        <div className="mono-menu" id={menuId}>
          <nav aria-label="Main">
            <ul>
              {links.map((link) => (
                <li key={link.key}>{link}</li>
              ))}
            </ul>
          </nav>
          <div className="mono-header-end">
            {/* No release yet: the tag says what the project is instead of a version. */}
            <span className="mono-release" title="No functional release yet">
              experimental
            </span>
            <a className="mono-family" href={FAMILY_SITE}>
              <span className="ferra-mark" data-product="ferramenta" aria-hidden="true" />
              Ferramenta
            </a>
          </div>
        </div>
        <div className="mono-header-actions">
          {actions}
          <a className="mono-repository" href={REPOSITORY} aria-label="Source code on GitHub">
            <svg className="icon" width="20" height="20" aria-hidden="true">
              <use href="#i-github" />
            </svg>
          </a>
        </div>
      </div>
    </header>
  );
}

/** The footer, as `MonoFooter` draws it for a member. */
export function SiteFooter({ home }: { home: string }) {
  return (
    <footer className="mono-footer">
      <div className="mono-footer-in">
        <SiteMark />
        <p>
          <a href={home}>{NAME}</a>, part of <a href={FAMILY_SITE}>Ferramenta</a>
        </p>
        <a href={REPOSITORY}>GitHub</a>
        <a href={WORKSHOP.imprint}>Imprint</a>
        <a href={WORKSHOP.privacy}>Privacy</a>
        <p className="mono-footer-copy">
          © {new Date().getFullYear()} {WORKSHOP.company}
        </p>
      </div>
    </footer>
  );
}
