// Dicionário bilíngue para o site oficial (Português e Inglês)
const i18nData = {
  pt: {
    pageTitle: "Axolote Suite v1.3.0 — Central 100% Offline & Nativa para Windows",
    navConvert: "Conversor Fácil",
    navTools: "12 Ferramentas",
    navInstall: "Instalação",
    navSecurity: "Segurança",
    navDownload: "Baixar v1.3.0",
    heroBadge: "Versão 1.3.0 Oficial • Windows 10 & 11 (64-bit) • 100% Offline",
    heroH1: "Sua central completa de ferramentas em <span class=\"gradient-text\">Rust Nativo</span>.",
    heroP: "Converta imagens, documentos (<strong>DOCX, PDF, HTML, MD, TXT</strong>), vídeos, áudios e <strong>ZIP</strong> em 1 clique. Inclui Revelador de Fotos ao vivo com histograma RGB, sincronização em lote, Editor de Vídeo, Estúdio de Cores & Senhas, Calculadora e instalador com <strong>FFmpeg automático</strong>.",
    btnDownloadZip: "Baixar Pacote Portátil (.zip) — Sem Avisos",
    btnDownloadExe: "Baixar Instalador Direto (.exe)",
    btnSource: "Código Aberto no GitHub",
    featurePill1: "100% Offline & Sem Telemetria",
    featurePill2: "Nativo em Rust + Iced (Zero Electron)",
    featurePill3: "Preservação Total do Original (1) (2)",
    simTag: "Simulador Interativo",
    simTitle: "Teste a velocidade do <span class=\"gradient-text\">Conversor Fácil</span>",
    simDesc: "Escolha qualquer formato abaixo para ver como o Axolote Suite processa localmente sem sobrescrever nada:",
    simTabImg: "🖼 Imagens (PNG, JPG, WebP…)",
    simTabDoc: "📄 Documentos (PDF, DOCX, TXT…)",
    simTabVid: "🎬 Vídeo & Áudio (MP4, MP3…)",
    simPrompt: "Aperte no formato que você quer gerar:",
    toolsTag: "Suite Completa",
    toolsTitle: "Tudo o que você precisa num <span class=\"gradient-text\">único executável leve</span>",
    toolsDesc: "Chega de sites cheios de propagandas, limites diários e uploads para a nuvem. O Axolote Suite roda 100% no seu computador.",
    cardQuickTitle: "Conversor Fácil Universal",
    cardQuickDesc: "Botões diretos para converter imagens, textos, DOCX, PDF, HTML, Markdown, vídeos, GIF, áudios e pacotes ZIP preservando o original.",
    cardDevTitle: "Estúdio de Cores & Senhas",
    cardDevDesc: "Conversor HEX/RGB/HSL/CMYK, extrator de paleta dominante de imagens e gerador de senhas fortes com medidor de entropia e UUID.",
    cardPhotoTitle: "Revelador & Estúdio de Fotos",
    cardPhotoDesc: "Exposição, realces, sombras, temperatura, vibração, nitidez, vinheta, histograma RGB ao vivo, presets cinematográficos e sincronização em lote de pastas.",
    cardVideoTitle: "Editor & Compressor de Vídeo",
    cardVideoDesc: "Corte na linha do tempo, controle de velocidade (0.5× a 2.0×), formatos 9:16 / 16:9 / 1:1, filtros de cinema e compressão CRF.",
    cardPdfTitle: "Central Completa de PDF",
    cardPdfDesc: "Junte múltiplos arquivos PDF na ordem que desejar, divida páginas ou intervalos e inspecione metadados técnicos em segundos.",
    cardAudioTitle: "Estúdio de Áudio",
    cardAudioDesc: "Converta entre MP3, WAV, FLAC, OGG, AAC e M4A, extraia a trilha sonora de vídeos e comprima áudios com controle de bitrate.",
    cardZipTitle: "Compactador ZIP Seguro",
    cardZipDesc: "Compacte arquivos e pastas completas ou extraia pacotes ZIP com blindagem automática contra ataques Zip-Slip.",
    cardCalcTitle: "Calculadora & Produtividade",
    cardCalcDesc: "Calculadora científica expressa, bloco de notas (.txt), lista de tarefas com filtros, organizador de pastas e renomeação em lote.",
    installTag: "Setup Automático",
    installTitle: "Pronto para usar em <span class=\"gradient-text\">3 passos simples</span>",
    installDesc: "O instalador integrado configura tudo no seu perfil de usuário sem exigir permissões complicadas.",
    step1Title: "Baixe o Pacote Oficial",
    step1Desc: "Baixe o executável direto ou o arquivo .zip portátil do GitHub Releases oficial.",
    step2Title: "Setup + FFmpeg Automático",
    step2Desc: "Ao abrir pela primeira vez, o assistente cria os atalhos e baixa o motor do FFmpeg sozinho.",
    step3Title: "Use 100% Offline",
    step3Desc: "Pronto! Use todas as 12 ferramentas localmente com velocidade máxima, tema escuro/claro e privacidade total.",
    secTag: "Código 100% Seguro & Auditado",
    secTitle: "Por que aparece aviso ao baixar? <span class=\"gradient-text\">Explicação Transparente</span>",
    secDesc: "O Axolote Suite é 100% gratuito, sem anúncios, 100% offline e de Código Aberto (Open Source) em Rust. Entenda por que o Windows ou o navegador exibe um alerta no primeiro download e como abrir com tranquilidade:",
    secWhyTitle: "Por que o aviso aparece?",
    secWhyDesc: "A Microsoft cobra mais de US$ 400 por ano por um certificado corporativo (EV Code Signing). Como o Axolote Suite é um projeto independente e gratuito, o Windows mostra o aviso padrão de \"Editor desconhecido / Arquivo pouco baixado\".",
    secNavTitle: "No Navegador (Chrome / Edge)",
    secNavDesc: "Se o navegador avisar que o arquivo axolote_suite.exe não é baixado com frequência, clique nos três pontinhos (⋯) ao lado do download e escolha Manter → Manter assim mesmo. (Dica: o arquivo .zip não tem esse aviso!)",
    secWinTitle: "No Windows SmartScreen",
    secWinDesc: "Ao abrir o instalador pela 1ª vez, se aparecer a janela \"O Windows protegeu o computador\", clique em Mais informações e em seguida no botão Executar assim mesmo.",
    secAuditTitle: "🔍 Verifique Você Mesmo no VirusTotal (Hash SHA-256 Oficial)",
    secAuditDesc: "Qualquer pessoa pode auditar o código-fonte completo em Rust no GitHub ou conferir a assinatura criptográfica SHA-256 do executável oficial diretamente no VirusTotal:",
    ctaTitle: "Baixe o <span class=\"gradient-text\">Axolote Suite v1.3.0</span> agora mesmo",
    ctaDesc: "Gratuito, código aberto (MIT / Apache-2.0), leve e construído nativamente em Rust para Windows 10 e 11.",
    footerText: "<strong>Axolote Suite v1.3.0</strong> • Desenvolvido em Rust + Iced • <a href=\"https://github.com/axolotesuite/axolotesuite\" target=\"_blank\" rel=\"noopener\">Repositório Oficial no GitHub</a>"
  },
  en: {
    pageTitle: "Axolote Suite v1.3.0 — 100% Offline & Native Workstation for Windows",
    navConvert: "Quick Convert",
    navTools: "12 Tools",
    navInstall: "Installation",
    navSecurity: "Security",
    navDownload: "Download v1.3.0",
    heroBadge: "Official v1.3.0 Release • Windows 10 & 11 (64-bit) • 100% Offline",
    heroH1: "Your complete local creative & utility suite in <span class=\"gradient-text\">Native Rust</span>.",
    heroP: "Convert images, documents (<strong>DOCX, PDF, HTML, MD, TXT</strong>), videos, audio, and <strong>ZIP</strong> in 1 click. Includes Lightroom-style live Photo Developer with RGB histogram, batch sync, timeline Video Editor, Colors & Passwords, Calculator, and <strong>auto FFmpeg setup</strong>.",
    btnDownloadZip: "Download Portable ZIP (.zip) — No Warnings",
    btnDownloadExe: "Download Direct Executable (.exe)",
    btnSource: "Open Source on GitHub",
    featurePill1: "100% Offline & Zero Telemetry",
    featurePill2: "Native Rust + Iced (Zero Electron)",
    featurePill3: "Preserves Original Files (1) (2)",
    simTag: "Interactive Simulator",
    simTitle: "Experience the speed of <span class=\"gradient-text\">Quick Convert</span>",
    simDesc: "Click any target format below to see how Axolote Suite safely processes files locally with zero overwriting:",
    simTabImg: "🖼 Images (PNG, JPG, WebP…)",
    simTabDoc: "📄 Documents (PDF, DOCX, TXT…)",
    simTabVid: "🎬 Video & Audio (MP4, MP3…)",
    simPrompt: "Click your desired target format:",
    toolsTag: "Complete Suite",
    toolsTitle: "Everything you need in a <span class=\"gradient-text\">single lightweight app</span>",
    toolsDesc: "Say goodbye to ad-filled web converters, upload limits, and cloud privacy issues. Axolote Suite runs 100% locally on your machine.",
    cardQuickTitle: "Universal Quick Convert",
    cardQuickDesc: "One-click conversion for photos, text, DOCX, PDF, HTML, Markdown, video, GIF, audio, and ZIP without touching the original.",
    cardDevTitle: "Colors & Passwords Studio",
    cardDevDesc: "HEX/RGB/HSL/CMYK converter, photo dominant palette extractor, and cryptographically secure password & UUID v4 generator.",
    cardPhotoTitle: "Photo Developer & Studio",
    cardPhotoDesc: "Exposure stops, highlights, shadows, white balance, vibrance, sharpness, film grain, live RGB histogram, cinema presets, and batch folder sync.",
    cardVideoTitle: "Video Editor & Compressor",
    cardVideoDesc: "Timeline trimming, 0.5× to 2.0× speed control, 9:16 / 16:9 / 1:1 canvas formats, cinema filters, and CRF compression.",
    cardPdfTitle: "Complete PDF Hub",
    cardPdfDesc: "Merge multiple PDF files in any order, split pages or custom ranges, and inspect technical metadata in seconds.",
    cardAudioTitle: "Audio Studio",
    cardAudioDesc: "Convert between MP3, WAV, FLAC, OGG, AAC, and M4A, extract audio tracks from video, and compress with bitrate control.",
    cardZipTitle: "Secure ZIP Archiver",
    cardZipDesc: "Compress files and entire folders, or extract archives with built-in defense against Zip-Slip traversal attacks.",
    cardCalcTitle: "Calculator & Productivity",
    cardCalcDesc: "Scientific expression calculator, plain text notepad (.txt), filtered task checklist, folder organizer, and batch renamer.",
    installTag: "Automatic Setup",
    installTitle: "Ready to use in <span class=\"gradient-text\">3 simple steps</span>",
    installDesc: "The integrated setup helper configures everything inside your user profile without administrative hassles.",
    step1Title: "Download the Official Package",
    step1Desc: "Download the direct executable or the portable .zip package from the official GitHub Releases page.",
    step2Title: "Setup + Auto FFmpeg",
    step2Desc: "On first launch, the wizard creates Desktop/Start Menu shortcuts and downloads the FFmpeg engine automatically.",
    step3Title: "Enjoy 100% Offline",
    step3Desc: "You're all set! Use all 12 tools locally with maximum native speed, dark/light theme, and total privacy.",
    secTag: "100% Safe & Audited Code",
    secTitle: "Why does a browser warning appear? <span class=\"gradient-text\">Clear Explanation</span>",
    secDesc: "Axolote Suite is 100% free, ad-free, 100% offline, and open-source in Rust. Here is why Windows or web browsers show a temporary caution dialog on first download and how to proceed:",
    secWhyTitle: "Why does the warning appear?",
    secWhyDesc: "Microsoft charges over $400/year for an enterprise EV Code Signing certificate. Because Axolote Suite is a free, community-driven project, Windows defaults to an 'Unknown Publisher / Low Download Count' dialog.",
    secNavTitle: "In your Browser (Chrome / Edge)",
    secNavDesc: "If your browser flags axolote_suite.exe as uncommon, click the three dots (⋯) next to the download and select Keep → Keep anyway. (Pro Tip: Downloading the .zip file skips this warning completely!)",
    secWinTitle: "In Windows SmartScreen",
    secWinDesc: "When opening the executable for the first time, if you see 'Windows protected your PC', click 'More info' and then 'Run anyway'.",
    secAuditTitle: "🔍 Verify It Yourself on VirusTotal (Official SHA-256)",
    secAuditDesc: "Anyone can audit the entire Rust source code on GitHub or verify the official cryptographic SHA-256 hash directly on VirusTotal:",
    ctaTitle: "Download <span class=\"gradient-text\">Axolote Suite v1.3.0</span> right now",
    ctaDesc: "Free, open-source (MIT / Apache-2.0), lightweight, and natively built in Rust for Windows 10 and 11.",
    footerText: "<strong>Axolote Suite v1.3.0</strong> • Built in Rust + Iced • <a href=\"https://github.com/axolotesuite/axolotesuite\" target=\"_blank\" rel=\"noopener\">Official GitHub Repository</a>"
  }
};

// Detecção automática de idioma do navegador/computador
function detectInitialLanguage() {
  const saved = localStorage.getItem('axolote_lang');
  if (saved && (saved === 'pt' || saved === 'en')) {
    return saved;
  }
  const browserLang = (navigator.language || navigator.userLanguage || 'en').toLowerCase();
  return browserLang.startsWith('pt') ? 'pt' : 'en';
}

let currentLang = detectInitialLanguage();

function applyLanguage(lang) {
  currentLang = lang;
  localStorage.setItem('axolote_lang', lang);
  document.documentElement.lang = lang === 'pt' ? 'pt-BR' : 'en';

  const data = i18nData[lang] || i18nData.en;

  // Atualizar elementos com data-i18n
  document.querySelectorAll('[data-i18n]').forEach((el) => {
    const key = el.dataset.i18n;
    if (data[key]) {
      el.innerHTML = data[key];
    }
  });

  // Atualizar título da página
  if (data.pageTitle) {
    document.title = data.pageTitle;
  }

  // Atualizar botões de idioma
  document.querySelectorAll('.lang-btn').forEach((btn) => {
    if (btn.dataset.lang === lang) {
      btn.classList.add('active');
    } else {
      btn.classList.remove('active');
    }
  });

  // Atualiza simulador ativo se houver botão selecionado
  const activeFmt = document.querySelector('.fmt-btn.selected');
  if (activeFmt) {
    activeFmt.click();
  }
}

// Inicializa no carregamento do DOM
document.addEventListener('DOMContentLoaded', () => {
  applyLanguage(currentLang);

  document.querySelectorAll('.lang-btn').forEach((btn) => {
    btn.addEventListener('click', () => {
      const targetLang = btn.dataset.lang;
      applyLanguage(targetLang);
    });
  });
});
