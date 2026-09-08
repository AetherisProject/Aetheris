document.addEventListener('DOMContentLoaded',function(){
  var s=document.getElementById('search');
  if(s)s.addEventListener('input',function(e){
    var q=e.target.value.toLowerCase();
    document.querySelectorAll('.sidebar a').forEach(function(a){
      a.style.display=a.textContent.toLowerCase().includes(q)?'':'none';
    });
  });
  document.querySelectorAll('a[href^="#"]').forEach(function(a){
    a.addEventListener('click',function(e){
      e.preventDefault();
      var t=document.querySelector(this.getAttribute('href'));
      if(t)t.scrollIntoView({behavior:'smooth'});
    });
  });
  document.querySelectorAll('pre').forEach(function(pre){
    var b=document.createElement('button');
    b.textContent='Copy';
    b.style.cssText='position:absolute;top:.5rem;right:.5rem;padding:.25rem .5rem;background:#334155;border:1px solid #334155;border-radius:4px;color:#cbd5e1;font-size:.75rem;cursor:pointer';
    b.addEventListener('click',function(){
      navigator.clipboard.writeText(pre.textContent);
      b.textContent='Copied!';
      setTimeout(function(){b.textContent='Copy'},2000);
    });
    pre.style.position='relative';
    pre.appendChild(b);
  });
  
  // Initialize design demo
  if(document.getElementById('deviceFrame')){
    switchDevice('mobile');
  }
});

// Design System Demo Functions
function switchDevice(device) {
  const frame = document.getElementById('deviceFrame');
  const viewport = document.getElementById('demoViewport');
  const nav = document.getElementById('demoNav');
  const grid = document.getElementById('demoGrid');
  
  // Remove all device classes
  frame.classList.remove('mobile', 'tablet', 'desktop');
  nav.classList.remove('demo-nav-bottom');
  
  // Add selected device class
  frame.classList.add(device);
  
  // Update navigation position based on device
  if (device === 'mobile') {
    nav.classList.add('demo-nav-bottom');
  }
  
  // Update button states
  document.querySelectorAll('.device-btn').forEach(btn => {
    btn.classList.remove('active');
    if (btn.dataset.device === device) {
      btn.classList.add('active');
    }
  });
}

function switchTheme(theme) {
  const frame = document.getElementById('deviceFrame');
  
  // Remove all theme classes
  frame.classList.remove('light', 'high-contrast');
  
  // Add selected theme class
  if (theme !== 'dark') {
    frame.classList.add(theme);
  }
  
  // Update button states
  document.querySelectorAll('.theme-btn').forEach(btn => {
    btn.classList.remove('active');
    if (btn.dataset.theme === theme) {
      btn.classList.add('active');
    }
  });
}

function toggleModal() {
  const modal = document.getElementById('demoModal');
  modal.classList.toggle('active');
}

function switchPlatform(platform) {
  const content = document.getElementById('demoContent');
  const title = document.getElementById('platform-title');
  const desc = document.getElementById('platform-desc');
  const layout = document.getElementById('platformLayout');

  // Reset layout classes
  layout.className = 'platform-layout';
  layout.classList.add(platform + '-layout');

  // Update content based on platform
  switch (platform) {
    case 'desktop':
      title.innerText = 'Desktop Dashboard (Win/Lin)';
      desc.innerText = 'High-density, sidebar-navigation, keyboard-optimized layout.';
      break;
    case 'mobile':
      title.innerText = 'Mobile App (Android)';
      desc.innerText = 'Touch-optimized bottom nav, swipeable cards, list-heavy layout.';
      break;
    case 'browser':
      title.innerText = 'Browser Extension';
      desc.innerText = 'Compact, ephemeral popup for quick secret access and autofill.';
      break;
    case 'tablet':
      title.innerText = 'Tablet Interface';
      desc.innerText = 'Master-detail split-view layout for larger touch surfaces.';
      break;
  }
  
  // Update button states
  document.querySelectorAll('.platform-btn').forEach(btn => {
    btn.classList.remove('active');
    if (btn.dataset.platform === platform) {
      btn.classList.add('active');
    }
  });
}
