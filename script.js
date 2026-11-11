// ==========================================================================
// Axolote Suite v1.3.0 — Interactive Motion System (/figma-use-motion)
// 60 FPS Physics, Specular Glare, Interactive Simulator, Scroll Reveals
// ==========================================================================

document.addEventListener('DOMContentLoaded', () => {
  // -------------------------------------------------------------------------
  // 1. Theme Toggle (Dark / Light) with SVG Spin
  // -------------------------------------------------------------------------
  const themeToggleBtn = document.getElementById('theme-toggle');
  const savedTheme = localStorage.getItem('axolote_theme') || 'dark';
  document.documentElement.setAttribute('data-theme', savedTheme);

  if (themeToggleBtn) {
    themeToggleBtn.addEventListener('click', () => {
      const current = document.documentElement.getAttribute('data-theme') === 'light' ? 'light' : 'dark';
      const next = current === 'dark' ? 'light' : 'dark';
      document.documentElement.setAttribute('data-theme', next);
      localStorage.setItem('axolote_theme', next);

      // Micro-spin animation
      const icon = themeToggleBtn.querySelector('svg');
      if (icon) {
        icon.style.transform = 'rotate(360deg)';
        setTimeout(() => {
          icon.style.transform = '';
        }, 450);
      }
    });
  }

  // -------------------------------------------------------------------------
  // 2. Navbar Glass Elevation on Scroll
  // -------------------------------------------------------------------------
  const navbarWrapper = document.querySelector('.navbar-wrapper');
  if (navbarWrapper) {
    window.addEventListener('scroll', () => {
      if (window.scrollY > 28) {
        navbarWrapper.classList.add('scrolled');
      } else {
        navbarWrapper.classList.remove('scrolled');
      }
    }, { passive: true });
  }

  // -------------------------------------------------------------------------
  // 3. 3D Tilt with Specular Mouse Glare on Hero Desktop Preview
  // -------------------------------------------------------------------------
  const heroTiltCard = document.getElementById('hero-tilt-card');
  if (heroTiltCard) {
    let ticking = false;

    heroTiltCard.addEventListener('mousemove', (e) => {
      if (!ticking) {
        requestAnimationFrame(() => {
          const rect = heroTiltCard.getBoundingClientRect();
          const x = e.clientX - rect.left;
          const y = e.clientY - rect.top;
          const cx = rect.width / 2;
          const cy = rect.height / 2;

          const rotX = ((y - cy) / cy) * -5.5;
          const rotY = ((x - cx) / cx) * 5.5;

          heroTiltCard.style.transform = `perspective(1200px) rotateX(${rotX.toFixed(2)}deg) rotateY(${rotY.toFixed(2)}deg)`;
          heroTiltCard.style.setProperty('--mouse-x', `${x}px`);
          heroTiltCard.style.setProperty('--mouse-y', `${y}px`);

          ticking = false;
        });
        ticking = true;
      }
    });

    heroTiltCard.addEventListener('mouseleave', () => {
      heroTiltCard.style.transform = 'perspective(1200px) rotateX(0deg) rotateY(0deg)';
      heroTiltCard.style.setProperty('--mouse-x', `50%`);
      heroTiltCard.style.setProperty('--mouse-y', `50%`);
    });
  }

  // -------------------------------------------------------------------------
  // 4. Quick Convert Output Format Chips Selection
  // -------------------------------------------------------------------------
  const fmtChips = document.querySelectorAll('.fmt-chip');
  const fileConversionTag = document.querySelector('.file-conversion-tag');

  fmtChips.forEach((chip) => {
    chip.addEventListener('click', () => {
      fmtChips.forEach((c) => c.classList.remove('selected'));
      chip.classList.add('selected');

      const chosenFmt = chip.dataset.fmt || chip.textContent.trim();
      if (fileConversionTag) {
        fileConversionTag.textContent = `DOCX → ${chosenFmt}`;
        fileConversionTag.style.color = '#66d9ff';
      }
    });
  });

  // -------------------------------------------------------------------------
  // 5. Interactive Demo Convert Feedback (60 FPS Safe Write Simulation)
  // -------------------------------------------------------------------------
  const btnConvertTrigger = document.getElementById('btn-convert-trigger');
  const recentCard = document.querySelector('.recent-conversion-card');
  const conversionStatusDone = document.querySelector('.conversion-status-done');

  if (btnConvertTrigger) {
    btnConvertTrigger.addEventListener('click', () => {
      const activeChip = document.querySelector('.fmt-chip.selected');
      const fmt = activeChip ? activeChip.textContent.trim() : 'WEBP';
      const isPt = document.documentElement.lang === 'pt-BR';

      btnConvertTrigger.classList.add('is-converting');
      btnConvertTrigger.innerHTML = `<span>⚡ ${isPt ? 'Convertendo atômico...' : 'Converting atomic...'}</span>`;

      if (conversionStatusDone) {
        conversionStatusDone.innerHTML = `<span>⏳</span><span>...</span>`;
      }

      setTimeout(() => {
        btnConvertTrigger.classList.remove('is-converting');
        btnConvertTrigger.style.background = '#30d158';
        btnConvertTrigger.innerHTML = `<span>✔ ${isPt ? `Convertido para ${fmt} em 32ms!` : `Converted to ${fmt} in 32ms!`}</span>`;

        if (conversionStatusDone) {
          conversionStatusDone.innerHTML = `<svg class="icon-svg-xs" viewBox="0 0 24 24" fill="none" stroke="#75de91" stroke-width="2.5"><polyline points="20 6 9 17 4 12"/></svg><span>${isPt ? 'Concluído' : 'Done'}</span>`;
          conversionStatusDone.style.transform = 'scale(1.15)';
          setTimeout(() => {
            conversionStatusDone.style.transform = '';
          }, 300);
        }

        setTimeout(() => {
          btnConvertTrigger.style.background = '';
          btnConvertTrigger.innerHTML = `<span>${isPt ? 'Converter localmente' : 'Convert locally'}</span><svg class="icon-svg-xs" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><polyline points="9 18 15 12 9 6"/></svg>`;
        }, 2800);
      }, 850);
    });
  }

  // -------------------------------------------------------------------------
  // 6. Interactive Drag & Drop on Demo Dropzone
  // -------------------------------------------------------------------------
  const demoDropzone = document.getElementById('demo-dropzone');
  const selectedFileName = document.querySelector('.selected-file-tile .file-details strong');

  if (demoDropzone) {
    ['dragenter', 'dragover'].forEach((eventName) => {
      demoDropzone.addEventListener(eventName, (e) => {
        e.preventDefault();
        e.stopPropagation();
        demoDropzone.classList.add('drag-active');
      });
    });

    ['dragleave', 'drop'].forEach((eventName) => {
      demoDropzone.addEventListener(eventName, (e) => {
        e.preventDefault();
        e.stopPropagation();
        demoDropzone.classList.remove('drag-active');
      });
    });

    demoDropzone.addEventListener('drop', (e) => {
      const files = e.dataTransfer?.files;
      if (files && files.length > 0) {
        const file = files[0];
        if (selectedFileName) {
          selectedFileName.textContent = file.name;
        }
        const isPt = document.documentElement.lang === 'pt-BR';
        const fileSub = document.querySelector('.selected-file-tile .file-details span');
        if (fileSub) {
          fileSub.textContent = `${(file.size / 1024 / 1024).toFixed(1)} MB · ${isPt ? 'Pronto para converter' : 'Ready to convert'}`;
        }
      }
    });
  }

  // -------------------------------------------------------------------------
  // 7. Category Filter Tabs for 12 Tools with Stagger
  // -------------------------------------------------------------------------
  const categoryTabs = document.querySelectorAll('.category-tab');
  const toolCards = document.querySelectorAll('.tool-card');

  categoryTabs.forEach((tab) => {
    tab.addEventListener('click', () => {
      categoryTabs.forEach((t) => t.classList.remove('active'));
      tab.classList.add('active');

      const filter = tab.dataset.filter;
      let delay = 0;

      toolCards.forEach((card) => {
        const match = filter === 'all' || card.dataset.category === filter;
        if (match) {
          card.style.display = 'block';
          card.style.opacity = '0';
          card.style.transform = 'translateY(12px)';
          setTimeout(() => {
            card.style.transition = 'opacity 0.35s ease, transform 0.35s cubic-bezier(0.16, 1, 0.3, 1)';
            card.style.opacity = '1';
            card.style.transform = 'translateY(0)';
          }, delay);
          delay += 40;
        } else {
          card.style.display = 'none';
        }
      });
    });
  });

  // -------------------------------------------------------------------------
  // 8. Scroll-Triggered Reveal Animation (IntersectionObserver)
  // -------------------------------------------------------------------------
  const revealElements = document.querySelectorAll('.reveal-item, .tool-card, .pillar-item, .install-choice-card, .promise-point');

  if ('IntersectionObserver' in window) {
    const revealObserver = new IntersectionObserver(
      (entries, observer) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            entry.target.classList.add('is-visible');
            observer.unobserve(entry.target);
          }
        });
      },
      {
        threshold: 0.12,
        rootMargin: '0px 0px -40px 0px'
      }
    );

    revealElements.forEach((el, index) => {
      el.classList.add('reveal-item');
      el.style.transitionDelay = `${(index % 4) * 60}ms`;
      revealObserver.observe(el);
    });
  } else {
    revealElements.forEach((el) => el.classList.add('is-visible'));
  }
});
