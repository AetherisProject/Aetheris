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
});
