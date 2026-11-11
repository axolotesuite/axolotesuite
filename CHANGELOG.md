# Novidades

Todas as mudanças importantes do Axolote Suite ficam aqui.
O formato segue o [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/) e as versões seguem o [SemVer](https://semver.org/lang/pt-BR/).

## [1.4.0] - 2026-11-11

### Adicionado
- **Barra de Pesquisa Dinâmica em Tempo Real**:
  - Filtro em tempo real de ferramentas com normalização inteligente de acentuação (`á/à/ã` -> `a`, `é/ê` -> `e`, etc.).
  - Atalho de teclado: pressione **Enter** após digitar para navegar instantaneamente para a primeira ferramenta correspondente.
  - Botão dedicado `✕ Limpar Busca` e sugestões interativas com 1 clique.
- **Instalador e Desinstalador Nativo Multi-idioma (Inno Setup)**:
  - 19 idiomas principais integrados: Português do Brasil, Português (PT), Inglês, Espanhol, Francês, Alemão, Italiano, Japonês, Coreano, Russo, Holandês, Polonês, Turco, Ucraniano, Tcheco, Dinamarquês, Finlandês, Húngaro e Norueguês.
  - Caixa de diálogo de seleção de idioma na inicialização (`ShowLanguageDialog=yes`).
  - Desinstalador dedicado registrado no Menu Iniciar (`{uninstallexe}`) e no Painel de Controle do Windows ("Aplicativos e Recursos").
  - Criação opcional de atalhos na Área de Trabalho e no Menu Iniciar.
- **Release Automatizado no GitHub Actions**:
  - Compilação otimizada em modo Release e empacotamento Inno Setup automático com upload de `Axolote_Setup.exe` e `axolote_suite.exe`.

### Alterado
- **Layout Bento Grid (1440×900) Aperfeiçoado**:
  - Remoção do painel de telemetria / status do motor da tela Home para foco total nas ferramentas.
  - Remoção dos botões de instalador interno da barra superior e do menu lateral (instalação e desinstalação agora 100% nativas).
  - Abertura imediata do aplicativo na Central de Ferramentas sem telas de bloqueio.

## [1.3.0] - 2026-11-07

### Adicionado
- **Conversor Fácil Universal** com destinos inteligentes por tipo de arquivo (imagem, documento, vídeo, áudio e ZIP).
- Sufixos automáticos `(1)`, `(2)` e gravação atômica via `.axolote_tmp`.
- Atalho global `Ctrl + Alt + N` e arrastar e soltar do Explorer.

### Alterado
- **Cores & Senhas** simplificado: ficaram só *Cores & Paleta de Foto* e *Senhas Fortes & UUID*.

### Removido
- Efeitos sonoros do sistema.
- Abas de Hash e Base64/JSON/Texto.

[1.4.0]: https://github.com/axolotesuite/axolotesuite/releases/tag/v1.4.0
[1.3.0]: https://github.com/axolotesuite/axolotesuite/releases/tag/v1.3.0
