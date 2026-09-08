// Browser plugin UI example — message passing to background/service worker
function send(action) {
  if (chrome.runtime && chrome.runtime.sendMessage) {
    chrome.runtime.sendMessage({ action: action, source: 'popup' }, (res) => {
      console.log('Popup response:', res);
    });
  }
  // UI feedback
  const btn = event && event.target ? event.target : null;
  if (btn && btn.tagName === 'BUTTON') {
    btn.textContent = 'Sent ✓';
    setTimeout(() => btn.textContent = btn.textContent.replace('Sent ✓', btn.dataset.original || 'Action'), 1200);
  }
}

// Initialize: show vault status from chrome.storage
if (chrome.storage && chrome.storage.local) {
  chrome.storage.local.get(['vaultStatus'], (data) => {
    const el = document.querySelector('.status');
    if (el && data.vaultStatus) el.textContent = '✓ ' + data.vaultStatus;
  });
}

document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('button').forEach(b => { b.dataset.original = b.textContent; });
});
