# 🦎 Relatório Completo de Funcionalidades — Axolote Suite v1.3.0
> **Objetivo deste documento:** Este relatório foi estruturado para ser copiado e colado diretamente no **Google Gemini** (ou em qualquer IA de Design / UI/UX), permitindo gerar conceitos visuais, layouts de telas modernos, paletas de cores, mockups e componentes elegantes para a interface do aplicativo.

---

## 📋 Prompt para Enviar ao Gemini (Copie e Cole com o Relatório Abaixo)

```text
Você é um Designer Sênior Especialista em UI/UX para Aplicativos Desktop Nativos (com foco em Rust, Iced GUI, Fluent Design do Windows 11 e estética Minimalista Cyberpunk / Glassmorphism Neon).

Analise o relatório completo de funcionalidades do Axolote Suite abaixo e crie:
1. Um Guia Completo de Redesign de UI/UX (Layout, navegação lateral, topo, cards e microinterações).
2. Uma Paleta de Cores e Tipografia Moderna (Temas Dark Neon e Light Clean).
3. Wireframes / Especificações Visuais Detalhadas para as telas principais:
   - Dashboard Inicial (Home)
   - Conversor Fácil (Drag & Drop)
   - Revelador de Fotos (com Histograma RGB ao vivo e controles estilo Lightroom)
   - Editor de Vídeo (Linha do tempo e formatos 9:16 / 16:9)
4. Ideias de Microinterações, animações suaves e ícones para tornar o app irresistível, moderno e com cara de software profissional de ponta.

--- SEGUE ABAIXO O RELATÓRIO DO APLICATIVO ---
```

---

## 1. 🎯 Visão Geral do Aplicativo
* **Nome:** Axolote Suite (v1.3.0)
* **Tecnologia:** Escrito do zero em **Rust puro** com **Iced 0.12** (framework nativo com aceleração por GPU via WGPU).
* **Plataforma:** Windows 10 e Windows 11 (64-bit).
* **Proposta de Valor:** Central completa de utilitários e ferramentas criativas, **100% offline, gratuita, sem anúncios, sem rastreamento/telemetria e com boot em menos de 0.2 segundos**. Consome menos de 45 MB de RAM (substitui dezenas de web converters e apps pesados em Electron).
* **Segurança de Arquivos:** Blindagem de escrita atômica. O app nunca sobrescreve o arquivo original; ele escreve em `.tmp` e salva com sufixo `(1)`, `(2)` automaticamente.
* **Internacionalização:** Detecção automática do idioma do computador (`sys_locale`). Inicia em Inglês para PCs internacionais e em Português para PCs do Brasil, com seletor manual nas Configurações.

---

## 2. 🗺️ Mapa de Telas e Módulos Detalhados

### 🏠 Módulo 1: Início (Home Dashboard)
* **Barra Superior (Top Bar):**
  - Campo de busca instantâneo (`🔍 Pesquisar ferramenta...`) que filtra todos os módulos em tempo real.
  - Botão de acesso rápido ao instalador/desinstalador.
* **Hero Banner do Mascote:**
  - Badge de status (`✨ v1.3.0 • 100% Offline & Nativo`).
  - Título de boas-vindas com gradiente neon.
  - Logo/mascote vetorial do Axolote.
* **Central de Acesso Rápido (Bento Grid):**
  - Cards clicáveis para abrir diretamente qualquer ferramenta com 1 clique.
* **Painel de Status do Sistema:**
  - Contador de notas salvas e tarefas pendentes.
  - Indicador do status do motor do FFmpeg (se está pronto para uso).
* **Painel de Últimas Novidades (Release Notes):**
  - Changelog visual das novidades da versão.

---

### 🔄 Módulo 2: Conversor Fácil Universal (Quick Convert)
* **Área de Drag & Drop Ampla:**
  - O usuário pode arrastar qualquer arquivo ou pasta do Windows Explorer para a janela, ou pressionar o atalho global `Ctrl + Alt + N`.
* **Detecção Inteligente de Formato:**
  - Reconhece automaticamente a extensão do arquivo e exibe **apenas os botões de formatos reais suportados**.
* **Categorias Suportadas:**
  - **Imagens:** PNG, JPG, WebP, BMP, ICO.
  - **Documentos:** TXT ↔ Markdown ↔ HTML ↔ DOCX ↔ PDF.
  - **Vídeo & Áudio:** MP4, WebM, MKV, GIF, MP3, WAV, FLAC, OGG.
  - **Arquivos:** ZIP (descompactação rápida).
* **Feedback Visual:**
  - Lista de arquivos na fila com barra de progresso, tempo decorrido, tamanho original vs convertido e botão direto para abrir a pasta de saída.

---

### 🎨 Módulo 3: Estúdio de Cores & Senhas (Colors & Passwords)
* **Conversor Cromático Universal:**
  - Conversão em tempo real entre `HEX`, `RGB (0-255)`, `HSL (Graus e %)` e `CMYK (%)`.
  - Preview visual da cor em caixa grande com efeito de brilho e botão de copiar para a área de transferência.
* **Extrator de Paleta de Imagens (Photo Palette Extractor):**
  - O usuário solta qualquer foto e o app extrai automaticamente as **6 cores dominantes** com códigos HEX copiáveis.
* **Gerador de Senhas Criptográficas:**
  - Seletor de tamanho (8 a 64 caracteres).
  - Toggles para Maiúsculas, Minúsculas, Números e Símbolos especiais.
  - **Medidor de Entropia em Bits** com barra colorida (Fraca, Média, Forte, Nível Militar).
  - Gerador instantâneo de **UUID v4**.

---

### 🖼️ Módulo 4: Revelador & Estúdio de Fotos (Live Photo Studio)
*Inspirado em softwares profissionais de revelação como Adobe Lightroom e Darktable, rodando nativo em Rust.*
* **Abas de Trabalho:**
  1. **Revelador Profissional (Ao Vivo):**
     - Preview de imagem central com zoom e pan.
     - **Histograma RGB de 256 canais ao vivo** (mostra canais Vermelho, Verde, Azul e Luminância se atualizando em tempo real com 60 FPS conforme o usuário mexe nos sliders).
     - **Controles de Luz:** Exposição em passos de EV (-3.0 a +3.0), Realces (Highlights), Sombras (Shadows), Brilho e Contraste.
     - **Controles de Cor:** Balanço de Branco / Temperatura (Frio a Quente), Matiz (Tint), Saturação e Vibração.
     - **Efeitos & Detalhe:** Nitidez (Sharpen), Vinheta circular com controle de difusão e Granulação de Filme (Film Grain).
  2. **Presets de Cinema (1-Clique):**
     - Botões rápidos com visuais pré-definidos: *Film Noir (P&B Dramático)*, *Cyber Neon*, *Golden Hour*, *Vintage Warm*, *Frio Polar*.
  3. **Sincronização em Lote (Batch Sync):**
     - Permite selecionar uma pasta com dezenas de fotos e aplicar os mesmos ajustes e presets para todas de uma vez, salvando numa subpasta `_reveladas` sem tocar nos originais.
  4. **Redimensionamento & Conversão:**
     - Proporções travadas, ajuste por porcentagem ou pixels exatos e conversão entre WebP, PNG e JPG com controle de qualidade.

---

### 🎬 Módulo 5: Editor & Compressor de Vídeo (Video Studio)
* **Linha do Tempo Interativa (Timeline Trimming):**
  - Seletores de corte de início (`Ponto A`) e fim (`Ponto B`) com precisão de milissegundos.
* **Controle de Velocidade:**
  - Acelerar ou diminuir a velocidade do vídeo (`0.5×`, `0.75×`, `1.0×`, `1.25×`, `1.5×`, `2.0×`).
* **Formatos de Tela (Canvas Aspect Ratio):**
  - `16:9` (Widescreen para YouTube / TV).
  - `9:16` (Vertical para Instagram Reels, TikTok e YouTube Shorts).
  - `1:1` (Quadrado para posts de redes sociais).
* **Compressor CRF Inteligente:**
  - Controle de taxa de compressão constante (CRF de 18 a 32) para reduzir drasticamente o tamanho do vídeo (ex: de 150 MB para 18 MB) sem perda visível de qualidade.
* **Motor FFmpeg Automático:**
  - O app gerencia o download e a execução do FFmpeg de forma invisível para o usuário.

---

### 📄 Módulo 6: Central Completa de PDF (PDF Hub)
* **Unir PDFs (Merge):**
  - Adicione múltiplos arquivos PDF, reordene a sequência para cima/para baixo com botões dedicados e gere um único documento unificado.
* **Dividir PDFs (Split):**
  - Modo por páginas individuais (cria um arquivo para cada página).
  - Modo por intervalo customizado (ex: extrair páginas 3 a 7 ou intervalos específicos).
* **Inspecionar Metadados:**
  - Número total de páginas, versão do formato PDF, título, autor e permissões de segurança.

---

### 🎵 Módulo 7: Estúdio de Áudio
* **Conversor de Áudio:**
  - Conversões entre MP3, WAV, FLAC, OGG, AAC e M4A.
* **Extrator de Trilha Sonora:**
  - Arraste qualquer arquivo de vídeo (MP4, MKV, MOV) e extraia o áudio original direto para MP3 ou WAV.
* **Compressão & Bitrate:**
  - Ajuste de qualidade de 64 kbps (podcast leve) até 320 kbps (alta fidelidade).

---

### 🗜 Módulo 8: Compactador ZIP Seguro (Anti Zip-Slip)
* **Compactação de Arquivos e Pastas:**
  - Compacte arquivos soltos ou árvores de diretórios completas preservando hierarquia.
* **Extração Protegida:**
  - Blindagem automática em Rust contra ataques de travessia de caminho (*Zip-Slip*). O descompactador impede que arquivos maliciosos tentem gravar fora da pasta de destino.

---

### 📋 Módulo 9: Produtividade (Notepad, Tasks & Organizer)
* **Bloco de Notas (.txt):**
  - Editor de texto leve e rápido com salvamento automático local em `%APPDATA%\AxoloteSuite\notas`.
* **Lista de Tarefas (Checklist):**
  - Criação rápida de afazeres, checkboxes interativos, filtros por *Todas / Pendentes / Concluídas* e contador no menu lateral.
* **Organizador de Pastas:**
  - Agrupa automaticamente arquivos soltos em pastas por categoria (Imagens, Vídeos, Documentos, Instaladores).

---

### 🖩 Módulo 10: Calculadora Científica Expressa
* **Suporte a Expressões:**
  - Expressões livres com suporte a parênteses, prioridade matemática real, potências (`^`) e porcentagem (`%`).
* **Histórico Lateral Clicável:**
  - Todas as contas anteriores ficam listadas na barra lateral; ao clicar no resultado antigo, ele volta para a tela de cálculo.
* **Formatação Numérica Brasileira e Internacional:**
  - Separadores de milhar e decimais corretos (`1.250,50` ou `1,250.50`).

---

### 📁 Módulo 11: Arquivos em Lote (Batch Rename)
* **Regras de Renomeação em Massa:**
  - Adicionar Prefixo ou Sufixo.
  - Localizar e Substituir texto no nome.
  - Numeração sequencial automática (`foto_001.jpg`, `foto_002.jpg`).
* **Tabela de Pré-visualização ao Vivo:**
  - Mostra lado a lado o *Nome Antigo* e o *Novo Nome* antes de aplicar a alteração no disco.

---

### 📦 Módulo 12: Fila de Tarefas & Histórico
* **Fila de Operações em Segundo Plano (Background Queue):**
  - Monitoramento de conversões pesadas de vídeo e lotes de fotos sem nunca congelar a interface.
* **Histórico Completo:**
  - Log auditável das operações feitas no aplicativo com status de sucesso/erro e timestamp.

---

### ⚙️ Módulo 13: Configurações, Setup & Desinstalação Limpa
* **Seletor de Idioma:**
  - `🌐 Auto (Detectar do Windows)` | `🇧🇷 Português` | `🇺🇸 English`.
* **Alternador de Tema:**
  - Tema Escuro (Dark Neon) e Tema Claro (Light Clean).
* **Comportamento:**
  - Abrir ou não automaticamente a pasta de destino após concluir conversões.
  - Controle de retenção máxima do histórico (20 a 500 itens).
* **Setup Automático & Botão "Apagar Tudo do Computador":**
  - Se não estiver instalado, mostra assistente de 3 passos com download automático do FFmpeg.
  - Depois de instalado, o assistente redundante some e vira o painel **Limpeza do Sistema** com o botão vermelho:
    `💥 APAGAR TUDO DO COMPUTADOR` (remove atalhos, chaves do registro do Windows, pasta `%APPDATA%\AxoloteSuite` e arquivos de instalação).

---

## 3. 🎨 Diretrizes de Estilo Desejadas para o Redesign
* **Identidade:** Moderna, limpa, tecnológica e amigável (inspirada no mascote Axolote).
* **Cores Base Atuais:** Fundo escuro profundo (`#0d0d14`), superfície de cards (`#151622`), bordas suaves translúcidas (`rgba(255,255,255,0.08)`), roxo primário (`#7c3aed`), azul elétrico (`#3b82f6`) e ciano neon (`#06b6d4`).
* **Sensação:** Aplicativo de workstation nativa rápida (como Arc Browser, Linear, Obsidian, Blender), fugindo de interfaces cinzas antigas do Windows 98/XP.
