<div align="center">

<img src="assets/branding/banner.svg" alt="Banner do Axolote Suite" width="100%">

# Axolote Suite

### Uma suíte de produtividade e ferramentas locais para Windows.

<p>
  <img src="https://img.shields.io/badge/Status-Em%20desenvolvimento-f59e0b" alt="Em desenvolvimento">
  <img src="https://img.shields.io/badge/Linguagem-TypeScript-3178C6?logo=typescript&logoColor=white" alt="TypeScript">
  <img src="https://img.shields.io/badge/Desktop-Electron-47848F?logo=electron&logoColor=white" alt="Electron">
  <img src="https://img.shields.io/badge/Plataforma-Windows%2010%20%2F%2011-5865F2?logo=windows&logoColor=white" alt="Windows 10 e 11">
</p>

**Produtividade • Conversão de arquivos • Privacidade • Controle local**

</div>

---

> [!IMPORTANT]
> **O Axolote Suite ainda está em desenvolvimento.** O projeto está sendo reorganizado em uma versão inicial menor, chamada provisoriamente de **Axolote Suite Mini**. Ainda não existe uma versão pública pronta para download. As ferramentas descritas nesta página representam o escopo planejado e não devem ser consideradas implementadas ou testadas.

## 🦎 O que é o Axolote Suite?

O **Axolote Suite** é um projeto de aplicativo desktop para Windows 10 e Windows 11. A proposta é reunir, em uma única interface, ferramentas úteis para produtividade, organização e conversão de arquivos.

A ideia é evitar que o usuário precise abrir vários programas diferentes para tarefas simples, como organizar notas, gerenciar tarefas, converter imagens, trabalhar com PDFs ou processar arquivos de áudio e vídeo.

O desenvolvimento está sendo recomeçado com um escopo menor e mais realista. Em vez de tentar criar muitos módulos complexos de uma só vez, a prioridade será construir uma base funcional, testar cada ferramenta e ampliar o programa por etapas.

## 🚧 Estado atual do desenvolvimento

O projeto está em fase de reorganização e desenvolvimento inicial.

| Item | Situação atual |
|---|---|
| Nome e identidade visual | Preparados |
| Objetivo e escopo do produto | Em definição |
| Linguagem principal | TypeScript escolhida |
| Aplicativo desktop | Electron planejado |
| Ferramentas de desenvolvimento | Vite e electron-builder planejados |
| Ferramentas de produtividade | Planejadas |
| Ferramentas de PDF | Planejadas |
| Conversão de imagens | Planejada |
| Conversão de vídeo e áudio | Planejada |
| Instalador do Windows | Ainda não disponível |
| Versão pública para download | Ainda não disponível |

**Importante:** este repositório contém a apresentação e a documentação do projeto. O código do aplicativo, os instaladores e as funcionalidades finais ainda não estão publicados aqui. O estado desta tabela será atualizado conforme houver entregas verificáveis.

## 🎯 O que o projeto pretende resolver?

Tarefas cotidianas podem exigir vários aplicativos: um para converter imagens, outro para juntar PDFs, outro para comprimir vídeos e mais um para anotar compromissos.

O Axolote Suite pretende reunir ferramentas desse tipo em um ambiente organizado, com uma experiência visual consistente e operações realizadas localmente sempre que possível.

O objetivo não é prometer substituir todos os programas profissionais. A prioridade é oferecer ferramentas práticas, fáceis de entender e confiáveis para necessidades comuns.

## 🧰 Ferramentas planejadas

Os módulos serão desenvolvidos por etapas. A ordem poderá mudar conforme os testes, a complexidade técnica e a viabilidade de cada recurso.

### 📝 Produtividade

- **Bloco de notas:** criar, editar, pesquisar e organizar notas.
- **Gerenciador de tarefas:** registrar atividades, marcar como concluídas e filtrar pendências.
- **Calculadora:** operações matemáticas básicas.

### 📄 PDF

- Juntar documentos PDF.
- Dividir um PDF em arquivos menores.
- Extrair páginas específicas.
- Reorganizar, girar ou remover páginas.
- Criar PDF a partir de imagens.
- Avaliar opções de compressão.

### 🖼️ Imagens

- Converter PNG, JPG, JPEG e WebP, conforme suporte implementado.
- Comprimir imagens.
- Redimensionar com proporção preservada.
- Processar várias imagens em lote.
- Consultar dimensões e informações básicas.

### 🎬 Vídeo

- Converter arquivos entre formatos suportados.
- Escolher opções de codificação e qualidade.
- Redimensionar vídeos.
- Cortar trechos.
- Consultar duração, resolução e codecs.
- Acompanhar o progresso e cancelar operações.

### 🎧 Áudio

- Converter formatos de áudio compatíveis.
- Extrair áudio de vídeos.
- Cortar trechos.
- Ajustar parâmetros de qualidade e bitrate, quando suportados.

### 🗂️ Arquivos e utilitários

- Compactar e descompactar arquivos ZIP.
- Renomear vários arquivos em lote.
- Organizar operações em uma fila.
- Consultar o histórico de conversões.

> **Sobre formatos e codecs:** a lista final dependerá das bibliotecas e ferramentas efetivamente integradas, das capacidades da versão utilizada e das respectivas licenças. O programa deverá mostrar apenas opções realmente suportadas.

## 🧭 Como o Axolote Suite deverá funcionar?

A experiência planejada é simples:

1. O usuário abre o aplicativo e encontra um painel com as categorias de ferramentas.
2. Escolhe uma ferramenta, como PDF, imagens ou vídeo.
3. Seleciona arquivos do próprio computador.
4. Configura a operação desejada.
5. Acompanha o processamento e o resultado.
6. Escolhe onde salvar o arquivo gerado.

As operações de conversão e edição local deverão preservar o arquivo original. O aplicativo deverá informar erros, limitações e formatos não suportados de maneira clara.

## 🛠️ Tecnologias escolhidas

A base planejada para a primeira versão utiliza:

| Tecnologia | Finalidade |
|---|---|
| **TypeScript** | Linguagem principal do projeto |
| **Electron** | Aplicativo desktop para Windows |
| **Vite** | Ambiente de desenvolvimento e construção da interface |
| **HTML e CSS** | Estrutura e aparência da interface |
| **electron-builder** | Empacotamento e geração do instalador |
| **FFmpeg / ffprobe** | Processamento e análise de áudio e vídeo, após avaliação das licenças |

A escolha de TypeScript busca facilitar a organização do código e a identificação de erros durante o desenvolvimento. Electron será utilizado para criar a aplicação desktop.

O projeto não pretende utilizar .NET, Python ou inteligência artificial integrada.

## 🔐 Privacidade e funcionamento local

Privacidade e controle dos arquivos são diretrizes do Axolote Suite.

Os objetivos são:

- **Sem login obrigatório:** as ferramentas locais não deverão depender de uma conta.
- **Sem telemetria por padrão:** não planejar coleta automática de dados de uso.
- **Sem envio automático de arquivos:** operações locais deverão ocorrer no computador.
- **Uso offline como prioridade:** as funções principais deverão funcionar sem conexão sempre que tecnicamente possível.
- **Transparência:** qualquer recurso que dependa de internet deverá explicar sua finalidade.
- **Controle do usuário:** arquivos originais não deverão ser apagados ou substituídos sem autorização.

Esses pontos são requisitos de desenvolvimento, não uma declaração de que o aplicativo já foi auditado ou certificado. O comportamento real deverá ser verificado antes de cada lançamento.

## 🗺️ Etapas de desenvolvimento

O trabalho será dividido em etapas pequenas para que cada entrega possa ser testada antes de aumentar o escopo.

| Etapa | Objetivo |
|---|---|
| 1. Fundação | Configurar TypeScript, Electron e a estrutura do aplicativo |
| 2. Interface | Criar painel, navegação e identidade visual dentro do programa |
| 3. Produtividade | Implementar notas, tarefas e calculadora |
| 4. PDF e imagens | Criar as primeiras ferramentas de manipulação e conversão |
| 5. Áudio e vídeo | Integrar processamento local e opções de codificação |
| 6. Arquivos | Adicionar ZIP, renomeação em lote e fila de operações |
| 7. Validação | Testar, corrigir, revisar dependências e documentar limitações |
| 8. Distribuição | Preparar e testar o instalador para Windows |

Não há data de lançamento confirmada. A prioridade é concluir e validar uma etapa antes de apresentá-la como pronta.

## 🎨 Identidade visual

O projeto utiliza um mascote de axolote e uma identidade visual minimalista, com roxo, branco e tons escuros.

<div align="center">
  <img src="assets/branding/logo.svg" alt="Logotipo Axolote Suite" width="74%">
</div>

Arquivos da identidade:
- [Banner](assets/branding/banner.svg)
- [Logotipo](assets/branding/logo.svg)
- [Ícone](assets/branding/icon.svg)
- [Guia da identidade visual](assets/branding/README.md)

## 📚 Documentação

- [Visão geral do projeto](docs/VISAO-GERAL.md)
- [Módulos planejados](docs/MODULOS-PLANEJADOS.md)
- [Princípios do projeto](docs/PRINCIPIOS.md)
- [Estado atual do desenvolvimento](docs/ESTADO-ATUAL.md)
- [Identidade visual](assets/branding/README.md)

## ❓ Perguntas frequentes

**O Axolote Suite já está pronto?**  
Não. O projeto ainda está em desenvolvimento. A implementação está sendo organizada em uma versão inicial menor para que as ferramentas possam ser concluídas e testadas por etapas.

**Já posso baixar e instalar?**  
Ainda não existe um instalador público. Uma versão para download será anunciada quando houver um pacote real e testado.

**Qual linguagem será utilizada?**  
TypeScript. A aplicação desktop está planejada com Electron e Vite.

**Quais ferramentas estarão disponíveis?**  
O escopo planejado inclui produtividade, PDF, imagens, vídeo, áudio e utilitários para arquivos. A disponibilidade dependerá da implementação e dos testes.

**O programa funcionará sem internet?**  
O objetivo é priorizar operações locais e funcionamento offline. Downloads e atualizações poderão exigir conexão.

**Terá inteligência artificial, login ou telemetria?**  
IA integrada está fora do escopo. Login obrigatório e telemetria por padrão também não fazem parte dos objetivos do projeto.

**Todos os formatos e codecs serão aceitos?**  
Não necessariamente. O suporte dependerá das bibliotecas, codecs e ferramentas efetivamente integradas. A documentação deverá listar os formatos testados.

---

<div align="center">

<img src="assets/branding/icon.svg" alt="Mascote Axolote" width="84">

### Axolote Suite

**Um projeto em desenvolvimento, construído por etapas.**

<sub>Ideias claras, ferramentas úteis e evolução com testes.</sub>

</div>
