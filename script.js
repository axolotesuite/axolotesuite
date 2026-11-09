// 1. Fundo de Partículas e Constelação Neon (60 FPS)
const canvas = document.getElementById('bg-canvas');
const ctx = canvas.getContext('2d');
let particles = [];
let mouse = { x: -1000, y: -1000 };

function resizeCanvas() {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
}
window.addEventListener('resize', resizeCanvas);
resizeCanvas();

window.addEventListener('mousemove', (e) => {
  mouse.x = e.clientX;
  mouse.y = e.clientY;
});

class Particle {
  constructor() {
    this.reset();
  }
  reset() {
    this.x = Math.random() * canvas.width;
    this.y = Math.random() * canvas.height;
    this.vx = (Math.random() - 0.5) * 0.55;
    this.vy = (Math.random() - 0.5) * 0.55;
    this.r = Math.random() * 2.1 + 0.8;
    this.color = Math.random() > 0.5 ? 'rgba(124, 58, 237, 0.65)' : 'rgba(6, 182, 212, 0.65)';
  }
  update() {
    this.x += this.vx;
    this.y += this.vy;
    if (this.x < 0 || this.x > canvas.width) this.vx *= -1;
    if (this.y < 0 || this.y > canvas.height) this.vy *= -1;
  }
  draw() {
    ctx.beginPath();
    ctx.arc(this.x, this.y, this.r, 0, Math.PI * 2);
    ctx.fillStyle = this.color;
    ctx.fill();
  }
}

for (let i = 0; i < 58; i++) {
  particles.push(new Particle());
}

function animateBg() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  for (let i = 0; i < particles.length; i++) {
    const p = particles[i];
    p.update();
    p.draw();

    for (let j = i + 1; j < particles.length; j++) {
      const q = particles[j];
      const dx = p.x - q.x;
      const dy = p.y - q.y;
      const dist = Math.hypot(dx, dy);
      if (dist < 130) {
        ctx.beginPath();
        ctx.moveTo(p.x, p.y);
        ctx.lineTo(q.x, q.y);
        ctx.strokeStyle = `rgba(124, 58, 237, ${(1 - dist / 130) * 0.18})`;
        ctx.lineWidth = 1;
        ctx.stroke();
      }
    }

    const md = Math.hypot(p.x - mouse.x, p.y - mouse.y);
    if (md < 160) {
      ctx.beginPath();
      ctx.moveTo(p.x, p.y);
      ctx.lineTo(mouse.x, mouse.y);
      ctx.strokeStyle = `rgba(6, 182, 212, ${(1 - md / 160) * 0.35})`;
      ctx.lineWidth = 1.2;
      ctx.stroke();
    }
  }
  requestAnimationFrame(animateBg);
}
animateBg();

// 2. Efeito 3D Tilt no Card Principal da Hero
const heroCard = document.getElementById('hero-tilt-card');
if (heroCard) {
  heroCard.addEventListener('mousemove', (e) => {
    const rect = heroCard.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    const cx = rect.width / 2;
    const cy = rect.height / 2;
    const rotX = ((y - cy) / cy) * -6;
    const rotY = ((x - cx) / cx) * 6;
    heroCard.style.transform = `rotateX(${rotX}deg) rotateY(${rotY}deg)`;
    heroCard.style.setProperty('--mouse-x', `${x}px`);
    heroCard.style.setProperty('--mouse-y', `${y}px`);
  });

  heroCard.addEventListener('mouseleave', () => {
    heroCard.style.transform = 'rotateX(0deg) rotateY(0deg)';
  });
}

// 3. Abas do Simulador Interativo
const demoTabs = document.querySelectorAll('.demo-tab');
const demoPanels = document.querySelectorAll('.demo-panel');

demoTabs.forEach((tab) => {
  tab.addEventListener('click', () => {
    const target = tab.dataset.target;
    demoTabs.forEach((t) => t.classList.remove('active'));
    demoPanels.forEach((p) => p.classList.remove('active'));
    tab.classList.add('active');
    const panel = document.getElementById(target);
    if (panel) panel.classList.add('active');
  });
});

// 4. Simulador de cliques nos botões do Conversor Fácil
const fmtBtns = document.querySelectorAll('.fmt-btn');
const demoOutputLog = document.getElementById('demo-convert-log');

fmtBtns.forEach((btn) => {
  btn.addEventListener('click', () => {
    fmtBtns.forEach((b) => b.classList.remove('selected'));
    btn.classList.add('selected');
    const fmt = btn.dataset.fmt;
    const ext = fmt.toLowerCase();
    const isEn = typeof currentLang !== 'undefined' && currentLang === 'en';
    if (demoOutputLog) {
      if (isEn) {
        demoOutputLog.innerHTML = `
          <div>✔ <strong>Source (Read-Only):</strong> annual_report.png</div>
          <div>⚡ <strong>Atomic Write:</strong> .annual_report.${ext}.axolote_tmp</div>
          <div>🛡 <strong>Zero Overwrite:</strong> annual_report (1).${ext}</div>
          <div style="color:#10b981;margin-top:6px;font-weight:700;">🚀 Converted to ${fmt} successfully (100% offline)!</div>
        `;
      } else {
        demoOutputLog.innerHTML = `
          <div>✔ <strong>Origem (Somente Leitura):</strong> relatorio_anual.png</div>
          <div>⚡ <strong>Escrita Atômica:</strong> .relatorio_anual.${ext}.axolote_tmp</div>
          <div>🛡 <strong>Sem Sobrescrita:</strong> relatorio_anual (1).${ext}</div>
          <div style="color:#10b981;margin-top:6px;font-weight:700;">🚀 Convertido para ${fmt} com sucesso (100% offline)!</div>
        `;
      }
    }
  });
});

// 5. Revelação suave dos cards ao rolar a página
const observer = new IntersectionObserver(
  (entries) => {
    entries.forEach((entry, idx) => {
      if (entry.isIntersecting) {
        setTimeout(() => {
          entry.target.classList.add('visible');
        }, idx * 65);
      }
    });
  },
  { threshold: 0.12 }
);

document.querySelectorAll('.bento-card').forEach((card) => observer.observe(card));
