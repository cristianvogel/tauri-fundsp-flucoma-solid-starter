import { SectionCard } from "../../shared/ui/SectionCard";

type AnalysisPanelProps = {
  status: string;
};

export function AnalysisPanel(props: AnalysisPanelProps) {
  return (
    <SectionCard title="flucoma-rs status" eyebrow="Analysis">
      <p>{props.status}</p>
      <p>
        Extend this area with corpus import, segmentation, feature extraction, embedding, and
        search workflows.
      </p>
    </SectionCard>
  );
}
