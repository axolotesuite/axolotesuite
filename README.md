# 🦎 Axolote Suite v1.2.0

![Logo Axolote Suite](assets/logo_full.jpg)

Sua central **100% offline, nativa e modular** para Windows 10 e Windows 11, desenvolvida em **Rust** com **Iced**.

Projetada com a identidade visual oficial do **Axolote Suite** (tema escuro profundo, roxo `#7c3aed` e azul neon `#3b82f6`), **design totalmente responsivo** e **guias passo a passo ("💡 Como usar")** em todas as ferramentas.

---

## ✨ Principais Funcionalidades

### 🎨 1. Estúdio & Editor de Fotos (com Preview ao Vivo)
- **Monitor de Pré-visualização em Tempo Real**: ajuste controles e veja o resultado imediatamente na foto antes de exportar.
- **Luz e Cor**: controles deslizantes de **Brilho**, **Contraste** e **Saturação** (`-100` a `+100`).
- **Filtros Instantâneos**: *Original*, *Preto & Branco*, *Sépia*, *Quente*, *Frio*, *Neon/Axolote* e *Negativo*.
- **Enquadramento e Recorte Automático**: `Livre`, `1:1 (Instagram)`, `16:9 (YouTube)`, `9:16 (Reels/TikTok/Shorts)` e `4:3`.
- **Transformações**: girar `90° Esq.`, `90° Dir.`, espelhar horizontalmente e verticalmente.
- **Redimensionamento, Conversão e Compressão**: filtro Lanczos3 com escalas rápidas (`25%`, `50%`, `75%`, `100%`), conversão entre **PNG, JPG, WebP, BMP e ICO** e compressão JPEG (`10%–100%`).

### 🎬 2. Estúdio & Editor de Vídeo
- **Monitor de Quadro + Linha do Tempo**: defina o ponto de corte inicial (`De`) e final (`Até`) do clipe.
- **Controle de Velocidade**: `0.5× (Câmera Lenta)`, `1.0×`, `1.25×`, `1.5×` e `2.0× (Time-lapse)` com ajuste automático de áudio (`atempo`).
- **Formato de Tela (Canvas)**: `Original`, `16:9 YouTube (1920×1080)`, `9:16 TikTok/Reels (1080×1920)` e `1:1 Feed (1080×1080)`.
- **Filtros de Cinema & Áudio**: estilos *Vibrante*, *P&B*, *Retrô* e *Cinema*, rotação/espelhamento, modo mudo (`Mute`) e amplificação de volume até `200%`.
- **Conversor & Compressor CRF**: converta entre **MP4, MKV, AVI, MOV e WebM** ou comprima com controle **CRF (`18–36`)**.

### 📄 3. Central de Documentos PDF
- **Unir PDFs**: combine vários documentos na ordem que quiser (`↑ / ↓`).
- **Dividir PDF**: extraia todas as páginas ou um intervalo específico (`Da página .. Até a página`) em arquivos individuais.
- **Inspeção Técnica**: consulte número de páginas, versão do PDF, tamanho e status de criptografia.

### 📋 4. Produtividade, Arquivos e ZIP
- **Bloco de Notas (`.txt`)**: editor multi-linha com salvamento local e indicador de alterações.
- **Lista de Tarefas**: filtros (*Todas*, *Pendentes*, *Concluídas*) e salvamento automático.
- **Calculadora Científica Expressa**: suporta parênteses, potência (`^`), resto (`%`), decimais com vírgula e histórico clicável.
- **Organizador Inteligente de Pastas**: separa arquivos soltos em subpastas (*Imagens, PDFs, Documentos, Áudios, Vídeos, Compactados*) com simulação prévia.
- **Renomeação em Lote**: substituir texto, prefixo, sufixo, numeração sequencial e maiúsculas/minúsculas com pré-visualização instantânea.
- **Compactador e Extrator ZIP**: proteção contra *Zip-Slip* e compactação recursiva de pastas completas.

### 💿 5. Instalador Principal (Setup) com Download Automático do FFmpeg & Desinstalador
- **Tela Principal de Setup no Primeiro Uso**:
  - Instala apenas o essencial no PC, **baixa e configura automaticamente o FFmpeg** (sem o usuário precisar instalar nada por fora), cria atalhos na **Área de Trabalho** e no **Menu Iniciar** com o ícone oficial e registra no Windows sem exigir Administrador.
- **Tela de Desinstalação** integrada (ou via `axolote_suite.exe --uninstall`):
  - Remove os arquivos instalados, o FFmpeg local, atalhos e registro do Windows, com opção de manter ou apagar suas notas e configurações pessoais.

---

## 📂 Estrutura do Projeto

```text
Axolote_Suite/
├── assets/                  # Logos e ícones oficiais do Axolote Suite (.ico, .png, .jpg)
├── src/
│   ├── main.rs              # Ponto de entrada com ícone nativo da janela
│   ├── lib.rs               # Biblioteca raiz
│   ├── app.rs               # Shell principal, barra lateral animada, busca e painel Início
│   ├── core/                # Configurações, histórico, fila assíncrona, caminhos e download FFmpeg
│   ├── shared/              # Tema visual (escuro/claro), componentes responsivos e logos
│   └── modules/             # Módulos (productivity, images, video, audio, pdf, zip,
│                            #          batch_rename, installer, queue, history, settings)
├── Cargo.toml
├── LICENSE
└── README.md
```

---

## 🚀 Como Compilar e Publicar

```powershell
# Gerar o executável final de produção (otimizado e portátil)
cargo build --release
```

O executável final pronto para distribuição fica em:
```text
target\release\axolote_suite.exe
```

## 📄 Licença

Distribuído sob licença **MIT OR Apache-2.0** (veja [LICENSE](LICENSE)).
