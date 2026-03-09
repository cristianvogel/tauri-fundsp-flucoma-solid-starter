import type { JSXElement } from "solid-js";

type ProjectShellProps = {
  children: JSXElement;
};

export function ProjectShell(props: ProjectShellProps) {
  return (
    <main class="app-shell">
      <div class="background-orb background-orb-a" />
      <div class="background-orb background-orb-b" />
      <div class="content-frame">{props.children}</div>
    </main>
  );
}
