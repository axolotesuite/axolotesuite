# Novidades

Todas as mudanças importantes do Axolote Suite ficam aqui.
O formato segue o [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/) e as versões seguem o [SemVer](https://semver.org/lang/pt-BR/).

## [1.4.1] - 2026-11-11

### Adicionado
- **Ícone Oficial no Executável e nos Atalhos do Windows**:
  - Embedding nativo do `assets/icon.ico` no executável `axolote_suite.exe` via `build.rs`.
  - Atalhos do Menu Iniciar, Área de Trabalho e Desinstalador no `installer.iss` configurados explicitamente com `IconFilename: "{app}\assets\icon.ico"`.
- **Testes Automatizados da Tela Inicial (`app.rs`)**:
  - Cobertura de responsividade (`560×480` até `1920×1080`), temas claro/escuro, idiomas PT/EN, busca por relevância e navegação direta por abas.

### Alterado
- **Tela Inicial (Home/Dashboard) Mais Equilibrada e Funcional**:
  - Ajuste do breakpoint de desktop amplo (`>= 1040 px`) para exibir o Bento Grid completo já na janela padrão (`1380×880`), com alturas alinhadas entre os cartões de cada linha e inclusão de acesso direto a **Vídeo & Áudio**, **ZIP & Arquivos em Lote** e **Produtividade & Calculadora**.
  - Remoção de botões decorativos sem ação (`Modo local`), remoção de botões aninhados na área de arrastar e soltar e substituição de rótulos não implementados (`Comprimir H.265`, `Converter WebP/AVIF`, `Remover Chiado`, `Segurança & Hash`).
  - Cartões com navegação real para abas/modos específicos (`Cores & Paleta` / `Senhas Fortes & UUID`, `Unir PDFs` / `Dividir PDF`, `Revelador` / `Lote` / `Redimensionar`) e exibição real da fila da sessão e histórico recente.
  - Busca rápida corrigida com correspondência por palavras, ranking por relevância de título/tags e remoção de palavras-chave de recursos inexistentes.

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

[1.4.1]: https://github.com/axolotesuite/axolotesuite/releases/tag/v1.4.1
[1.4.0]: https://github.com/axolotesuite/axolotesuite/releases/tag/v1.4.0
[1.3.0]: https://github.com/axolotesuite/axolotesuite/releases/tag/v1.3.0
