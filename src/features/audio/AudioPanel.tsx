import { SectionCard } from "../../shared/ui/SectionCard";

type AudioTransportStatus = {
  running: boolean;
  outputAvailable: boolean;
  deviceName: string | null;
  sampleRateHz: number | null;
  channelCount: number | null;
  transportMode: string;
  decodeStatus: string;
  lastError: string | null;
  framesRendered: number;
  playheadSeconds: number;
};

type AudioPanelProps = {
  status: string;
  transport: AudioTransportStatus | undefined;
  transportMessage: string | null;
  transportPending: boolean;
  onToggleTransport: () => Promise<void>;
};

export function AudioPanel(props: AudioPanelProps) {
  return (
    <SectionCard title="funDSP status" eyebrow="DSP">
      <p>{props.status}</p>
      <div class="transport-meta">
        <p>Output device: {props.transport?.deviceName ?? "No default device detected"}</p>
        <p>
          Stream:{" "}
          {props.transport?.running
            ? `${props.transport.transportMode} at ${props.transport.sampleRateHz ?? "?"} Hz / ${
                props.transport.channelCount ?? "?"
              } ch`
            : "stopped"}
        </p>
        <p>
          Playhead:{" "}
          {props.transport
            ? `${props.transport.playheadSeconds.toFixed(2)} s (${props.transport.framesRendered} frames)`
            : "0.00 s"}
        </p>
        <p>{props.transport?.decodeStatus ?? "Checking Symphonia decode path..."}</p>
        {props.transport?.lastError ? <p>Last error: {props.transport.lastError}</p> : null}
        {props.transportMessage ? <p>Event: {props.transportMessage}</p> : null}
      </div>
      <button
        class="transport-button"
        disabled={props.transportPending || !props.transport?.outputAvailable}
        onClick={() => {
          void props.onToggleTransport();
        }}
      >
        {props.transportPending
          ? "Working..."
          : props.transport?.running
            ? "Stop realtime output"
            : "Start realtime output"}
      </button>
      <p>
        Extend this area with file-backed transport, modulation, routing, and event streaming.
      </p>
    </SectionCard>
  );
}
