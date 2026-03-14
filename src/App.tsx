import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import {
  createResource,
  createSignal,
  For,
  Match,
  onCleanup,
  onMount,
  Show,
  Switch,
} from "solid-js";
import { AnalysisPanel } from "./features/analysis/AnalysisPanel";
import { AudioPanel } from "./features/audio/AudioPanel";
import { ProjectShell } from "./features/project/ProjectShell";

type StarterOverview = {
  starterName: string;
  frontendStack: string[];
  backendStack: string[];
  architecture: Array<{ area: string; purpose: string }>;
  flucomaStatus: string;
  fundspStatus: string;
  nextSteps: string[];
};

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

type AudioTransportEvent = {
  kind: string;
  status: AudioTransportStatus;
  message: string | null;
};

async function loadOverview() {
  return invoke<StarterOverview>("starter_overview");
}

async function loadTransport() {
  return invoke<AudioTransportStatus>("audio_transport_status");
}

export default function App() {
  const [overview, { refetch: refetchOverview }] = createResource(loadOverview);
  const [transport, setTransport] = createSignal<AudioTransportStatus>();
  const [transportMessage, setTransportMessage] = createSignal<string | null>(null);
  const [transportPending, setTransportPending] = createSignal(false);

  onMount(() => {
    let active = true;
    void loadTransport().then((status) => {
      if (active) {
        setTransport(status);
      }
    });

    const unlisten = listen<AudioTransportEvent>("audio://transport", (event) => {
      if (!active) {
        return;
      }

      setTransport(event.payload.status);
      setTransportMessage(event.payload.message);
      setTransportPending(false);
    });

    onCleanup(() => {
      active = false;
      void unlisten.then((dispose) => dispose());
    });
  });

  async function toggleTransport() {
    const currentTransport = transport();
    if (!currentTransport) {
      return;
    }

    setTransportPending(true);
    try {
      if (currentTransport.running) {
        await invoke("audio_transport_stop");
      } else {
        await invoke("audio_transport_start");
      }
      await refetchOverview();
    } catch (error) {
      setTransportPending(false);
      setTransportMessage(String(error));
      throw error;
    } finally {
      setTransportPending(false);
    }
  }

  return (
    <ProjectShell>
      <Switch>
        <Match when={overview.error}>
          <section class="status-card">
            <p class="eyebrow">Backend status</p>
            <h2>Starter command failed</h2>
            <p>{String(overview.error)}</p>
          </section>
        </Match>
        <Match when={overview.loading}>
          <section class="status-card">
            <p class="eyebrow">Boot</p>
            <h2>Loading starter overview</h2>
            <p>The Solid frontend is waiting on the Tauri backend.</p>
          </section>
        </Match>
        <Match when={overview()}>
          {(data) => (
            <>
              <section class="hero">
                <p class="eyebrow">Template repository</p>
                <h1>{data().starterName}</h1>
                <p class="hero-copy">
                  An offline-first desktop audio sketch for uploading a source file, decomposing it
                  into NMF bases, and preparing those learned spectra for a later realtime filter
                  pass.
                </p>
                <div class="stack-grid">
                  <div class="stack-card">
                    <span class="stack-label">Frontend</span>
                    <For each={data().frontendStack}>{(item) => <p>{item}</p>}</For>
                  </div>
                  <div class="stack-card">
                    <span class="stack-label">Backend</span>
                    <For each={data().backendStack}>{(item) => <p>{item}</p>}</For>
                  </div>
                </div>
              </section>
              <div class="panel-grid">
                <AnalysisPanel status={data().flucomaStatus} />
                <AudioPanel
                  status={data().fundspStatus}
                  transport={transport()}
                  transportMessage={transportMessage()}
                  transportPending={transportPending()}
                  onToggleTransport={toggleTransport}
                />
              </div>
              <section class="architecture-grid">
                <For each={data().architecture}>
                  {(item) => (
                    <article class="architecture-card">
                      <p class="eyebrow">{item.area}</p>
                      <p>{item.purpose}</p>
                    </article>
                  )}
                </For>
              </section>
              <Show when={data().nextSteps.length > 0}>
                <section class="status-card">
                  <p class="eyebrow">Suggested expansion</p>
                  <For each={data().nextSteps}>{(item) => <p>{item}</p>}</For>
                </section>
              </Show>
            </>
          )}
        </Match>
      </Switch>
    </ProjectShell>
  );
}
