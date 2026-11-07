# Visão geral do Axolote Suite

<blockquote>
**Estado:** em desenvolvimento. Este documento descreve a visão e os objetivos do projeto. Não é uma declaração de funcionalidades já implementadas.
</blockquote>

## 1. Resumo

O Axolote Suite é um projeto de aplicativo desktop para Windows 10 e Windows 11. A proposta é reunir ferramentas de produtividade, manipulação de documentos e conversão de arquivos em uma única interface.

O projeto está sendo reorganizado em uma versão inicial menor, chamada provisoriamente de Axolote Suite Mini. A intenção é desenvolver e testar ferramentas úteis por etapas, evitando um escopo grande demais antes de existir uma versão funcional.

## 2. Problema que pretende resolver

Tarefas simples podem exigir programas diferentes: escrever notas, gerenciar atividades, juntar PDFs, converter imagens ou preparar arquivos de áudio e vídeo. O Axolote Suite pretende reduzir essa fragmentação, reunindo ferramentas locais em um ambiente organizado.

O objetivo não é prometer substituir todos os programas especializados. O foco é oferecer ferramentas práticas para necessidades comuns.

## 3. Público pretendido

Pessoas que usam Windows, precisam trabalhar com arquivos locais e valorizam uma experiência simples, organizada e transparente sobre o tratamento dos seus dados.

## 4. Áreas planejadas

- Produtividade: notas, tarefas e calculadora.
- PDF: juntar, dividir, extrair e organizar páginas.
- Imagens: converter, comprimir e redimensionar.
- Vídeo: converter, comprimir, cortar e consultar informações.
- Áudio: converter e extrair áudio de vídeos.
- Arquivos: compactar, descompactar e renomear em lote.

Todas essas áreas estão planejadas. A disponibilidade real dependerá da implementação e dos testes.

## 5. Tecnologias escolhidas

A linguagem principal será TypeScript. A aplicação desktop está planejada com Electron, Vite, HTML e CSS. O empacotamento deverá utilizar electron-builder.

Para áudio e vídeo, o projeto pretende avaliar FFmpeg e ffprobe. Antes de distribuir qualquer dependência, será necessário revisar sua origem, configuração e licença.

## 6. Princípios do produto

- Prioridade para processamento local.
- Funcionamento offline sempre que tecnicamente possível.
- Sem login obrigatório para as funções locais.
- Sem telemetria por padrão.
- Sem IA integrada.
- Preservação dos arquivos originais.
- Transparência sobre formatos, limitações e dependências.

Esses pontos são objetivos a implementar e verificar, não garantias já auditadas.

## 7. Experiência imaginada

1. O usuário abre o aplicativo.
2. Encontra um painel com categorias de ferramentas.
3. Escolhe uma operação.
4. Seleciona arquivos locais.
5. Configura a operação e o destino.
6. Acompanha o progresso.
7. Recebe o resultado e pode abrir a pasta de saída.

## 8. Etapas de desenvolvimento

1. Configuração da base TypeScript/Electron.
2. Interface e navegação.
3. Notas, tarefas e calculadora.
4. Ferramentas de PDF e imagens.
5. Ferramentas de áudio e vídeo.
6. Utilitários para arquivos.
7. Testes, correções e revisão de dependências.
8. Empacotamento e validação do instalador.

## 9. Como medir progresso

O progresso deve ser demonstrado por código publicado, funcionalidades que possam ser executadas, testes, relatórios e builds verificáveis. Uma funcionalidade só deve ser apresentada como concluída quando houver evidência.

## 10. Estado atual do repositório

O repositório publica a apresentação, a identidade visual e a documentação do projeto. O código-fonte do aplicativo e os instaladores ainda não estão publicados aqui. A implementação está em desenvolvimento e será documentada conforme as entregas forem verificadas.
