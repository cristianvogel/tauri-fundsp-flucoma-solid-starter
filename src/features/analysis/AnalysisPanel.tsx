import { createSignal, For, onCleanup, Show } from "solid-js";
import { SectionCard } from "../../shared/ui/SectionCard";

type AnalysisPanelProps = {
  status: string;
};

type AudioPreview = {
  durationSeconds: number | null;
};

type BufNmfConfig = {
  components: number;
  fftSize: number;
  hopSize: number;
  iterations: number;
  basesMode: 0 | 1 | 2;
  actMode: 0 | 1 | 2;
};

type NmfFilterConfig = {
  iterations: number;
  filterSource: "same-file" | "alternate-file" | "realtime-input";
  outputMode: "components" | "mix" | "both";
};

const MODE_LABELS = {
  0: "Random init",
  1: "Seed and update",
  2: "Seed and freeze",
} as const;

const OFFLINE_DELIVERABLES = [
  "Learn bases from the uploaded file with BufNMF.",
  "Capture activations for each component over time.",
  "Resynthesise isolated component stems offline.",
  "Pass learned bases forward as NMFFilter templates.",
] as const;

export function AnalysisPanel(props: AnalysisPanelProps) {
  const [selectedFile, setSelectedFile] = createSignal<File | null>(null);
  const [previewUrl, setPreviewUrl] = createSignal<string | null>(null);
  const [preview, setPreview] = createSignal<AudioPreview>({ durationSeconds: null });
  const [bufNmf, setBufNmf] = createSignal<BufNmfConfig>({
    components: 4,
    fftSize: 2048,
    hopSize: 512,
    iterations: 80,
    basesMode: 0,
    actMode: 0,
  });
  const [nmfFilter, setNmfFilter] = createSignal<NmfFilterConfig>({
    iterations: 12,
    filterSource: "same-file",
    outputMode: "components",
  });
  const [workflowPrepared, setWorkflowPrepared] = createSignal(false);

  function updatePreviewUrl(file: File | null) {
    const currentUrl = previewUrl();
    if (currentUrl) {
      URL.revokeObjectURL(currentUrl);
    }

    if (!file) {
      setPreviewUrl(null);
      setPreview({ durationSeconds: null });
      return;
    }

    setPreviewUrl(URL.createObjectURL(file));
  }

  function handleFileChange(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const file = input.files?.[0] ?? null;
    setSelectedFile(file);
    updatePreviewUrl(file);
    setWorkflowPrepared(false);
  }

  function setBufNmfField<K extends keyof BufNmfConfig>(key: K, value: BufNmfConfig[K]) {
    setBufNmf((current) => ({ ...current, [key]: value }));
    setWorkflowPrepared(false);
  }

  function setNmfFilterField<K extends keyof NmfFilterConfig>(key: K, value: NmfFilterConfig[K]) {
    setNmfFilter((current) => ({ ...current, [key]: value }));
    setWorkflowPrepared(false);
  }

  function formatDuration(durationSeconds: number | null) {
    if (durationSeconds === null || !Number.isFinite(durationSeconds)) {
      return "Waiting for preview metadata";
    }

    const minutes = Math.floor(durationSeconds / 60);
    const seconds = durationSeconds % 60;
    return `${minutes}:${seconds.toFixed(1).padStart(4, "0")}`;
  }

  onCleanup(() => {
    const currentUrl = previewUrl();
    if (currentUrl) {
      URL.revokeObjectURL(currentUrl);
    }
  });

  return (
    <SectionCard title="Offline NMF workflow" eyebrow="Analysis">
      <p>{props.status}</p>
      <p class="panel-intro">
        This first pass is intentionally offline: upload one audio file, derive spectral bases with
        <code>BufNMF</code>, then hand those bases to an <code>NMFFilter</code> stage that is
        ready to become realtime later.
      </p>

      <div class="workflow-stage-grid">
        <article class="workflow-stage">
          <div class="stage-head">
            <p class="stage-step">Stage 1</p>
            <h3>Source file</h3>
          </div>
          <label class="upload-dropzone">
            <span class="upload-title">Upload analysis source</span>
            <span class="upload-copy">
              Choose a mono or stereo file to preview, decompose, and reuse as the first seeded
              filter template set.
            </span>
            <input accept="audio/*" class="file-input" type="file" onChange={handleFileChange} />
          </label>

          <Show when={selectedFile()} fallback={<p class="muted-note">No audio selected yet.</p>}>
            {(file) => (
              <div class="file-summary">
                <div>
                  <span class="summary-label">File</span>
                  <p>{file().name}</p>
                </div>
                <div>
                  <span class="summary-label">Size</span>
                  <p>{(file().size / (1024 * 1024)).toFixed(2)} MB</p>
                </div>
                <div>
                  <span class="summary-label">Duration</span>
                  <p>{formatDuration(preview().durationSeconds)}</p>
                </div>
              </div>
            )}
          </Show>

          <Show when={previewUrl()}>
            {(url) => (
              <audio
                class="audio-preview"
                controls
                preload="metadata"
                src={url()}
                onLoadedMetadata={(event) => {
                  const audio = event.currentTarget as HTMLAudioElement;
                  setPreview({ durationSeconds: audio.duration });
                }}
              />
            )}
          </Show>
        </article>

        <article class="workflow-stage">
          <div class="stage-head">
            <p class="stage-step">Stage 2</p>
            <h3>BufNMF decomposition</h3>
          </div>

          <div class="control-grid">
            <label class="field">
              <span>Components</span>
              <input
                type="range"
                min="2"
                max="12"
                value={bufNmf().components}
                onInput={(event) =>
                  setBufNmfField("components", Number(event.currentTarget.value))
                }
              />
              <strong>{bufNmf().components}</strong>
            </label>

            <label class="field">
              <span>FFT size</span>
              <select
                value={bufNmf().fftSize}
                onChange={(event) => setBufNmfField("fftSize", Number(event.currentTarget.value))}
              >
                <For each={[1024, 2048, 4096, 8192]}>{(size) => <option value={size}>{size}</option>}</For>
              </select>
            </label>

            <label class="field">
              <span>Hop size</span>
              <select
                value={bufNmf().hopSize}
                onChange={(event) => setBufNmfField("hopSize", Number(event.currentTarget.value))}
              >
                <For each={[256, 512, 1024, 2048]}>{(size) => <option value={size}>{size}</option>}</For>
              </select>
            </label>

            <label class="field">
              <span>Iterations</span>
              <input
                type="number"
                min="1"
                max="400"
                value={bufNmf().iterations}
                onInput={(event) =>
                  setBufNmfField("iterations", Number(event.currentTarget.value))
                }
              />
            </label>

            <label class="field">
              <span>Bases mode</span>
              <select
                value={bufNmf().basesMode}
                onChange={(event) =>
                  setBufNmfField("basesMode", Number(event.currentTarget.value) as 0 | 1 | 2)
                }
              >
                <option value="0">{MODE_LABELS[0]}</option>
                <option value="1">{MODE_LABELS[1]}</option>
                <option value="2">{MODE_LABELS[2]}</option>
              </select>
            </label>

            <label class="field">
              <span>Activation mode</span>
              <select
                value={bufNmf().actMode}
                onChange={(event) =>
                  setBufNmfField("actMode", Number(event.currentTarget.value) as 0 | 1 | 2)
                }
              >
                <option value="0">{MODE_LABELS[0]}</option>
                <option value="1">{MODE_LABELS[1]}</option>
                <option value="2">{MODE_LABELS[2]}</option>
              </select>
            </label>
          </div>

          <div class="deliverable-list">
            <For each={OFFLINE_DELIVERABLES}>
              {(item) => (
                <p>
                  <span class="deliverable-dot" />
                  {item}
                </p>
              )}
            </For>
          </div>
        </article>

        <article class="workflow-stage workflow-stage-accent">
          <div class="stage-head">
            <p class="stage-step">Stage 3</p>
            <h3>NMFFilter handoff</h3>
          </div>

          <div class="control-grid">
            <label class="field">
              <span>Filter source</span>
              <select
                value={nmfFilter().filterSource}
                onChange={(event) =>
                  setNmfFilterField(
                    "filterSource",
                    event.currentTarget.value as NmfFilterConfig["filterSource"],
                  )
                }
              >
                <option value="same-file">Use the uploaded file</option>
                <option value="alternate-file">Swap to a second file later</option>
                <option value="realtime-input">Reserve for live input</option>
              </select>
            </label>

            <label class="field">
              <span>Per-frame iterations</span>
              <input
                type="number"
                min="1"
                max="64"
                value={nmfFilter().iterations}
                onInput={(event) =>
                  setNmfFilterField("iterations", Number(event.currentTarget.value))
                }
              />
            </label>

            <label class="field">
              <span>Output routing</span>
              <select
                value={nmfFilter().outputMode}
                onChange={(event) =>
                  setNmfFilterField(
                    "outputMode",
                    event.currentTarget.value as NmfFilterConfig["outputMode"],
                  )
                }
              >
                <option value="components">Separate component streams</option>
                <option value="mix">Summed filtered mix</option>
                <option value="both">Both component streams and mix</option>
              </select>
            </label>
          </div>

          <p class="panel-intro">
            <code>NMFFilter</code> works from trained bases. In this flow, the uploaded file is the
            training source today; the next iteration can keep the same learned bases while the
            input becomes a live stream.
          </p>

          <button
            class="transport-button"
            disabled={!selectedFile()}
            onClick={() => setWorkflowPrepared(true)}
          >
            {workflowPrepared() ? "Offline path prepared" : "Prepare offline analysis path"}
          </button>

          <Show when={workflowPrepared()}>
            <div class="pipeline-summary">
              <p class="summary-title">Prepared pipeline</p>
              <p>
                Run <code>BufNMF</code> on <strong>{selectedFile()?.name}</strong> with{" "}
                <strong>{bufNmf().components}</strong> components, <strong>{bufNmf().fftSize}</strong>{" "}
                FFT, <strong>{bufNmf().hopSize}</strong> hop, and{" "}
                <strong>{bufNmf().iterations}</strong> iterations.
              </p>
              <p>
                Reuse the learned bases in <code>NMFFilter</code> with{" "}
                <strong>{nmfFilter().iterations}</strong> per-frame iterations, targeting{" "}
                <strong>{nmfFilter().filterSource.replace("-", " ")}</strong>, and emit{" "}
                <strong>{nmfFilter().outputMode.replace("-", " ")}</strong>.
              </p>
              <p>
                Seed modes: bases <strong>{MODE_LABELS[bufNmf().basesMode]}</strong>, activations{" "}
                <strong>{MODE_LABELS[bufNmf().actMode]}</strong>.
              </p>
            </div>
          </Show>
        </article>
      </div>
    </SectionCard>
  );
}
