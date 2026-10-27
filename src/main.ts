import "./styles.css";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
// DittoDeb popup: virtualized list + preview pane + pins
type Clip = { id: number; text: string; pinned?: boolean };
type ClipDetail = { id: number; text: string; html: string | null; kind: string };
const ROW_H = 68;
const STRIDE = ROW_H;
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
<div class=footer><span><kbd>↑↓</kbd> navigate</span><button class=footer-btn type=button data-action=enter><kbd>Enter</kbd> paste selected</button><button class=footer-btn type=button data-action=hide><kbd>Esc</kbd> hide</button></div>`;
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
async function pastePlain(id: number) {
  statusEl.textContent = "Pasting plain…";
  try { await invoke("paste_clip_plain_cmd", { id }); } catch (e) { statusEl.textContent = String(e); }
}
async function deleteClip(id: number) {
  const target = items.find((c) => c.id === id);
  if (target?.pinned) { statusEl.textContent = "Cannot delete pinned clip"; return; }
  const idx = items.findIndex((c) => c.id === id);
  try {
    await invoke("delete_clip_cmd", { id });
    await refresh();
    if (items.length) {
      let next = idx >= 0 ? idx : 0;
      if (next >= items.length) next = items.length - 1;
      sel = next;
      paintSel();
    } else {
      sel = 0;
      updatePreview();
    }
  } catch (e) { statusEl.textContent = String(e); }
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
  const start = Math.max(0, Math.floor(scrollTop / STRIDE) - 4);
  const count = Math.ceil(h / STRIDE) + 8;
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
  const top = document.createElement("div"); top.style.height = (start * STRIDE) + "px"; list.appendChild(top);
  for (let i = start; i < end; i++) {
    const c = items[i];
    const li = document.createElement("div");
    li.className = "vrow" + (i === sel ? " selected" : "") + (c.pinned ? " pinned" : "");
    li.style.height = ROW_H + "px";
    li.dataset.id = String(c.id);
    const pinLabel = c.pinned ? "★" : "☆";
    li.innerHTML = `<span class=clip-id>${i + 1}</span><span class=clip-text>${esc(c.text.slice(0, 160))}</span><span class=clip-side><button class="pin-btn${c.pinned ? " is-pinned" : ""}" title="Pin/unpin">${pinLabel}</button>${c.pinned ? "" : `<button class="del-btn" title="Delete clip">✕</button>`}</span>`;
    li.onclick = () => { sel = i; paintSel(); paste(c.id); };
    li.onmouseenter = () => {
      document.querySelectorAll('.vrow.selected').forEach((el) => el.classList.remove('selected'));
      li.classList.add('selected');
      sel = i;
      updatePreview();
    };
    const btn = li.querySelector(".pin-btn") as HTMLButtonElement;
    btn.onclick = (e) => { e.stopPropagation(); togglePin(c.id); };
    const delBtn = li.querySelector(".del-btn") as HTMLButtonElement | null;
    if (delBtn) delBtn.onclick = (e) => { e.stopPropagation(); deleteClip(c.id); };
    list.appendChild(li);
  }
  const bottom = document.createElement("div"); bottom.style.height = ((items.length - end) * STRIDE) + "px"; list.appendChild(bottom);
}
function paintSel() {
  // keep selected row visible
  const viewH = list.clientHeight || 400;
  const top = sel * STRIDE;
  if (top < scrollTop) scrollTop = top;
  else if (top + ROW_H > scrollTop + viewH) scrollTop = top + ROW_H - viewH;
  list.scrollTop = scrollTop;
  renderList();
  updatePreview();
}
async function refresh() {
  const term = q.value.trim();
  statusEl.textContent = term ? `Filter: “${term.slice(0, 28)}”` : "Recent clips";
  const raw = await invoke<Clip[]>("search_clips_cmd", { q: term, limit: 200 });
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
q.focus();
getCurrentWindow().onFocusChanged(({ payload }) => { if (payload) q.focus(); });
async function hideWindow() {
  q.value = "";
  try { await invoke("hide_window_cmd"); }
  catch (e1) {
    try { await getCurrentWindow().hide(); }
    catch (e2) { statusEl.textContent = String(e2 || e1); }
  }
}
q.addEventListener("keydown", async (e) => {
  if (e.key === "Escape") { e.preventDefault(); await hideWindow(); }
  else if (e.key === "ArrowDown") { e.preventDefault(); if (items.length) { sel = (sel + 1) % items.length; paintSel(); } }
  else if (e.key === "ArrowUp") { e.preventDefault(); if (items.length) { sel = (sel - 1 + items.length) % items.length; paintSel(); } }
  else if (e.key === "Delete" && items.length > 0 && q.value.trim() === "") { e.preventDefault(); await deleteClip(items[sel].id); }
  else if (e.key === "Enter" && items.length > 0) {
    e.preventDefault();
    if (e.shiftKey) await pastePlain(items[sel].id); else await paste(items[sel].id);
  }
  // Ctrl+Shift+V: paste selected clip as plain text (Ditto-style).
  if (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === "v" && items.length > 0) {
    e.preventDefault();
    await pastePlain(items[sel].id);
  }
});
// Global Delete handling: remove selected clip when not editing search text.
document.addEventListener('keydown', async (e) => {
  if (e.key !== 'Delete') return;
  if (document.activeElement === q && q.value.trim() !== '') return;
  if (!items.length) return;
  e.preventDefault();
  await deleteClip(items[sel].id);
});
// hotkey customization UI
const footer = document.querySelector('.footer') as HTMLDivElement;
const hkBtn = document.createElement('button');
hkBtn.textContent = 'Hotkey';
hkBtn.className = 'footer-btn';
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
getCurrentWindow().onFocusChanged(({ payload }) => { if (payload) refresh().catch(() => {}); }).catch(() => {});
document.addEventListener('visibilitychange', () => { if (document.hidden) q.value = ""; });
refresh();
// settings appended
// Advanced settings UI
const settingsBtn = document.createElement('button');
settingsBtn.textContent = 'Settings';
settingsBtn.className = 'footer-btn';
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
    <button id=cfg-save class=footer-btn>Save</button>
    <button id=cfg-close class=footer-btn>Close</button>
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
logBtn.className = 'footer-btn';
footer.appendChild(logBtn);
logBtn.onclick = async () => {
  try {
    const log = await invoke<string>('get_error_log_cmd');
    window.alert(log.trim() ? log.slice(-4000) : 'No errors logged.');
  } catch (e) { statusEl.textContent = String(e); }
};

// Hide button
const hideBtn = document.createElement('button');
hideBtn.textContent = 'Hide (Esc)';
hideBtn.className = 'footer-btn';
footer.appendChild(hideBtn);
hideBtn.onclick = hideWindow;

// Make footer hint buttons actionable
footer.querySelectorAll('button.footer-btn[data-action="hide"]').forEach((b) => {
  (b as HTMLButtonElement).onclick = hideWindow;
});
footer.querySelectorAll('button.footer-btn[data-action="enter"]').forEach((b) => {
  (b as HTMLButtonElement).onclick = async () => { if (items.length) await paste(items[sel].id); };
});

// Global Escape handling (works even when focus is not in the search box)
document.addEventListener('keydown', (e) => {
  if (e.key !== 'Escape') return;
  if (modal.style.display === 'flex') {
    modal.style.display = 'none';
    e.preventDefault();
    return;
  }
  e.preventDefault();
  void hideWindow();
});
