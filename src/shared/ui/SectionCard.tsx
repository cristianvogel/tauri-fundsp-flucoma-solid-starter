import type { JSXElement } from "solid-js";

type SectionCardProps = {
  eyebrow: string;
  title: string;
  children: JSXElement;
};

export function SectionCard(props: SectionCardProps) {
  return (
    <section class="status-card">
      <p class="eyebrow">{props.eyebrow}</p>
      <h2>{props.title}</h2>
      {props.children}
    </section>
  );
}
