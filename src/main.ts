import "./styles.css";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
// DittoDeb popup: virtualized list + preview pane + pins
type Clip = { id: number; text: string; pinned?: boolean };
type ClipDetail = { id: number; text: string; html: string | null; kind: string };
const ITEM_H = 68;
document.querySelector("#app")!.innerHTML = `
<div class=layout>
  <div class=list-pane>
    <div class=search-wrap>
      <div class=search-box>
        <span class=search-icon><svg width=18 height=18 viewBox="0 0 24 24" fill=none stroke=currentColor stroke-width=2><circle cx=11 cy=11 r=7 /><path d="m20 20-3.5-3.5"/></svg></span>
        <input id=q placeholder="Search clips — type to filter…" autocomplete=off spellcheck=false />
        <span class=kbd-hint><kbd>Enter</kbd> paste <kbd>Esc</kbd> close</span>
      </div>
      <div class=meta-row>
        <span class=meta-count><span class=dot></span><strong id=count>0</strong>&nbsp;clips</span>
        <span id=status>Ctrl+\` to toggle</span>
      </div>
    </div>
    <div id=list class=list-virt></div>
  </div>
  <aside id=preview class=preview-pane>
    <div class=preview-title>Preview</div>
    <div id=preview-body class=preview-body><div class=empty-sub>Select a clip.</div></div>
  </aside>
</div>
<div class=footer><span><kbd>↑↓</kbd> navigate</span><span><kbd>Enter</kbd> paste selected</span><span><kbd>Esc</kbd> hide</span></div>`;
const q = document.querySelector("#q") as HTMLInputElement;
const list = document.querySelector("#list") as HTMLDivElement;
const previewBody = document.querySelector("#preview-body") as HTMLDivElement;
let items: Clip[] = [];
let sel = 0;
let scrollTop = 0;
const countEl = document.querySelector("#count")!;
const statusEl = document.querySelector("#status")!;
async function paste(id: number) {
  statusEl.textContent = "Pasting…";
  try { await invoke("paste_clip_cmd", { id }); } catch (e) { statusEl.textContent = String(e); }
}
async function togglePin(id: number) {
  try { await invoke("toggle_pin_cmd", { id }); await refresh(); } catch (e) { statusEl.textContent = String(e); }
}
function esc(s: string) { return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;"); }
function sanitizeHtml(s: string) { return s.replace(/<script[\s\S]*?<\/script>/gi, "").replace(/<[^>]+>/g, "").slice(0, 4000); }
async function updatePreview() {
  if (!items.length) { previewBody.innerHTML = "<div class=empty-sub>No selection.</div>"; return; }
  const c = items[sel];
  try {
    const d = await invoke<ClipDetail | null>("get_clip_detail_cmd", { id: c.id });
    if (!d) { previewBody.innerHTML = "<div class=empty-sub>Not found.</div>"; return; }
    let html = "";
    if (d.kind === "image") {
      html += `<div class=preview-kind>Image</div>`;
      if (d.html) html += `<img class=preview-img src="${convertFileSrc(d.html)}" alt="clip image" />`;
    } else {
      html += `<div class=preview-kind>${esc(d.kind || "text")}</div>`;
      html += `<pre class=preview-text>${esc(d.text)}</pre>`;
      if (d.html) html += `<div class=preview-html>${sanitizeHtml(d.html)}</div>`;
    }
    previewBody.innerHTML = html;
  } catch (e) {
    previewBody.innerHTML = `<div class=empty-sub>${esc(String(e))}</div>`;
  }
}
function visibleRange() {
  const h = list.clientHeight || 400;
  const start = Math.max(0, Math.floor(scrollTop / ITEM_H) - 4);
  const count = Math.ceil(h / ITEM_H) + 8;
  const end = Math.min(items.length, start + count);
  return { start, end };
}
function renderList() {
  list.innerHTML = "";
  if (!items.length) {
    list.innerHTML = `<div class=empty><div class=empty-title>No clips found</div><div class=empty-sub>Copy something and it will appear here.</div></div>`;
    return;
  }
  const { start, end } = visibleRange();
  const top = document.createElement("div"); top.style.height = (start * ITEM_H) + "px"; list.appendChild(top);
  for (let i = start; i < end; i++) {
    const c = items[i];
    const li = document.createElement("div");
    li.className = "vrow" + (i === sel ? " selected" : "") + (c.pinned ? " pinned" : "");
    li.style.height = ITEM_H + "px";
    li.dataset.id = String(c.id);
    const pinLabel = c.pinned ? "★" : "☆";
    li.innerHTML = `<span class=clip-id>${i + 1}</span><span class=clip-text>${esc(c.text.slice(0, 160))}</span><span class=clip-side><button class="pin-btn${c.pinned ? " is-pinned" : ""}" title="Pin/unpin">${pinLabel}</button></span>`;
    li.onclick = () => { sel = i; paintSel(); paste(c.id); };
    li.onmouseenter = () => { sel = i; paintSel(); };
    const btn = li.querySelector(".pin-btn") as HTMLButtonElement;
    btn.onclick = (e) => { e.stopPropagation(); togglePin(c.id); };
    list.appendChild(li);
  }
  const bottom = document.createElement("div"); bottom.style.height = ((items.length - end) * ITEM_H) + "px"; list.appendChild(bottom);
}
function paintSel() {
  // keep selected row visible
  const viewH = list.clientHeight || 400;
  const top = sel * ITEM_H;
  if (top < scrollTop) scrollTop = top;
  else if (top + ITEM_H > scrollTop + viewH) scrollTop = top + ITEM_H - viewH;
  list.scrollTop = scrollTop;
  renderList();
  updatePreview();
}
async function refresh() {
  const term = q.value.trim();
  statusEl.textContent = term ? `Filter: “${term.slice(0, 28)}”` : "Recent clips";
  const raw = await invoke<Clip[]>("search_clips_cmd", { q: term || "a", limit: 200 });
  let pinned: Clip[] = [];
  try { pinned = await invoke<Clip[]>("list_pinned_cmd"); } catch { pinned = []; }
  const pinnedIds = new Set(pinned.map((p) => p.id));
  items = raw.map((c) => ({ ...c, pinned: pinnedIds.has(c.id) }));
  countEl.textContent = String(items.length);
  sel = 0; scrollTop = 0; list.scrollTop = 0;
  renderList();
  updatePreview();
}
list.addEventListener("scroll", () => { scrollTop = list.scrollTop; renderList(); });
q.addEventListener("input", refresh);
q.addEventListener("keydown", async (e) => {
  if (e.key === "Escape") await getCurrentWindow().hide();
  else if (e.key === "ArrowDown") { e.preventDefault(); if (items.length) { sel = (sel + 1) % items.length; paintSel(); } }
  else if (e.key === "ArrowUp") { e.preventDefault(); if (items.length) { sel = (sel - 1 + items.length) % items.length; paintSel(); } }
  else if (e.key === "Enter" && items.length > 0) await paste(items[sel].id);
});
// hotkey customization UI
const footer = document.querySelector('.footer') as HTMLDivElement;
const hkBtn = document.createElement('button');
hkBtn.textContent = 'Hotkey';
hkBtn.className = 'pin-btn';
hkBtn.style.marginLeft = '8px';
footer.appendChild(hkBtn);
hkBtn.onclick = async () => {
  try {
    const cur = await invoke<string>('get_hotkey_cmd');
    const val = window.prompt('Hotkey (e.g. ctrl+shift+v, ctrl+backquote):', cur);
    if (val) {
      await invoke('set_hotkey_cmd', { hotkey: val });
      statusEl.textContent = `Hotkey set: ${val}`;
    }
  } catch (e) { statusEl.textContent = String(e); }
};
invoke<string>('get_hotkey_cmd').then((h) => { statusEl.textContent = `Toggle: ${h}`; }).catch(() => {});
refresh();
// settings appended
// Advanced settings UI
const settingsBtn = document.createElement('button');
settingsBtn.textContent = 'Settings';
settingsBtn.className = 'pin-btn';
settingsBtn.style.marginLeft = '8px';
footer.appendChild(settingsBtn);

const modal = document.createElement('div');
modal.style.cssText = 'position:fixed;inset:0;background:rgba(0,0,0,.6);display:none;align-items:center;justify-content:center;z-index:999;';
const box = document.createElement('div');
box.style.cssText = 'background:#1a2230;padding:18px;border-radius:12px;min-width:320px;color:#e6f0ff;';
box.innerHTML = `
  <div style="font-weight:600;margin-bottom:10px;">Advanced settings</div>
  <label>Max clips <input id=cfg-maxclips type=number></label><br>
  <label>Max text bytes <input id=cfg-maxtext type=number></label><br>
  <label>Max image MB <input id=cfg-maximg type=number></label><br>
  <label>Paste delay ms <input id=cfg-pastedelay type=number></label><br>
  <label><input id=cfg-case type=checkbox> Case-sensitive search</label><br>
  <label><input id=cfg-autostart type=checkbox> Autostart</label><br>
  <div style="margin-top:12px;display:flex;gap:8px;">
    <button id=cfg-save class=pin-btn>Save</button>
    <button id=cfg-close class=pin-btn>Close</button>
  </div>`;
modal.appendChild(box);
document.body.appendChild(modal);

type AppConfig = { max_clips:number; max_text_bytes:number; max_image_mb:number; poll_interval_ms:number; paste_delay_ms:number; search_case_sensitive:boolean; autostart:boolean; };

async function openSettings() {
  try {
    const c = await invoke<AppConfig>('get_config_cmd');
    (document.querySelector('#cfg-maxclips') as HTMLInputElement).value = String(c.max_clips);
    (document.querySelector('#cfg-maxtext') as HTMLInputElement).value = String(c.max_text_bytes);
    (document.querySelector('#cfg-maximg') as HTMLInputElement).value = String(c.max_image_mb);
    (document.querySelector('#cfg-pastedelay') as HTMLInputElement).value = String(c.paste_delay_ms);
    (document.querySelector('#cfg-case') as HTMLInputElement).checked = c.search_case_sensitive;
    (document.querySelector('#cfg-autostart') as HTMLInputElement).checked = c.autostart;
  } catch (e) { statusEl.textContent = String(e); }
  modal.style.display = 'flex';
}
settingsBtn.onclick = openSettings;
(document.querySelector('#cfg-close') as HTMLButtonElement).onclick = () => { modal.style.display = 'none'; };
(document.querySelector('#cfg-save') as HTMLButtonElement).onclick = async () => {
  const cfg: AppConfig = {
    max_clips: Number((document.querySelector('#cfg-maxclips') as HTMLInputElement).value),
    max_text_bytes: Number((document.querySelector('#cfg-maxtext') as HTMLInputElement).value),
    max_image_mb: Number((document.querySelector('#cfg-maximg') as HTMLInputElement).value),
    poll_interval_ms: 500,
    paste_delay_ms: Number((document.querySelector('#cfg-pastedelay') as HTMLInputElement).value),
    search_case_sensitive: (document.querySelector('#cfg-case') as HTMLInputElement).checked,
    autostart: (document.querySelector('#cfg-autostart') as HTMLInputElement).checked,
  };
  try { await invoke('set_config_cmd', { cfg }); statusEl.textContent = 'Settings saved'; modal.style.display = 'none'; }
  catch (e) { statusEl.textContent = String(e); }
};
modal.addEventListener('click', (e) => { if (e.target === modal) modal.style.display = 'none'; });
// Error log viewer
const logBtn = document.createElement('button');
logBtn.textContent = 'Log';
logBtn.className = 'pin-btn';
logBtn.style.marginLeft = '8px';
footer.appendChild(logBtn);
logBtn.onclick = async () => {
  try {
    const log = await invoke<string>('get_error_log_cmd');
    window.alert(log.trim() ? log.slice(-4000) : 'No errors logged.');
  } catch (e) { statusEl.textContent = String(e); }
};

