function saveOptions() {
  const opts = {
    relayUrl: document.getElementById('relay-url').value,
    syncMode: document.getElementById('sync-mode').value,
    clipboard: document.getElementById('enable-clipboard').checked,
    contextMenu: document.getElementById('enable-contextmenu').checked,
  };
  chrome.storage.sync.set({ aetherisOptions: opts }, () => {
    const s = document.getElementById('status');
    s.textContent = 'Saved at ' + new Date().toISOString();
  });
}

document.addEventListener('DOMContentLoaded', () => {
  chrome.storage.sync.get('aetherisOptions', (res) => {
    if (res.aetherisOptions) {
      document.getElementById('relay-url').value = res.aetherisOptions.relayUrl || 'ws://localhost:8765';
      document.getElementById('sync-mode').value = res.aetherisOptions.syncMode || 'Auto (default)';
      document.getElementById('enable-clipboard').checked = !!res.aetherisOptions.clipboard;
      document.getElementById('enable-contextmenu').checked = res.aetherisOptions.contextMenu !== false;
    }
  });
});
