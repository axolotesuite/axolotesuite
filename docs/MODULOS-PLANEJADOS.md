# Módulos planejados

**Estado geral: em desenvolvimento.** As ferramentas descritas abaixo fazem parte do escopo pretendido do Axolote Suite Mini. Não significa que estejam implementadas, testadas ou disponíveis para download.

O objetivo é desenvolver por etapas, priorizando ferramentas úteis e verificáveis.

## 📝 1. Produtividade

### Bloco de notas
- Criar, editar e excluir notas.
- Pesquisar notas.
- Salvar automaticamente.
- Manter os dados localmente.

### Gerenciador de tarefas
- Criar, editar e excluir tarefas.
- Marcar tarefas como concluídas.
- Filtrar pendências e tarefas concluídas.
- Salvar os dados localmente.

### Calculadora
- Operações matemáticas básicas.
- Números decimais.
- Suporte ao teclado.
- Tratamento de divisão por zero.

## 📄 2. PDF

Ferramentas a avaliar e desenvolver:
- Juntar arquivos PDF.
- Dividir um documento.
- Extrair páginas selecionadas.
- Reorganizar, girar e remover páginas.
- Criar PDF a partir de imagens.
- Avaliar compressão de PDF.

O suporte dependerá das bibliotecas escolhidas e de testes com documentos reais. Conversão de PDF para Word não faz parte do primeiro escopo.

## 🖼️ 3. Imagens

- Converter PNG, JPG/JPEG e WebP, conforme suporte real.
- Comprimir imagens.
- Redimensionar mantendo proporção.
- Processar várias imagens em lote.
- Consultar dimensões.
- Avaliar remoção de metadados na exportação.

## 🎬 4. Vídeo

Com FFmpeg e ffprobe, após revisão de licença e distribuição:
- Converter formatos suportados.
- Selecionar opções de codificação.
- Ajustar resolução, FPS e qualidade.
- Comprimir vídeo.
- Cortar trechos.
- Consultar duração, resolução e codecs.
- Acompanhar progresso e cancelar operações.

Os codecs disponíveis dependerão da compilação e dos componentes efetivamente integrados.

## 🎧 5. Áudio

- Converter formatos compatíveis.
- Extrair áudio de vídeos.
- Cortar trechos.
- Ajustar bitrate e qualidade, quando suportado.
- Consultar informações do arquivo.

## 🗂️ 6. Arquivos e utilitários

- Criar e extrair arquivos ZIP.
- Renomear vários arquivos em lote.
- Visualizar alterações antes de renomear.
- Detectar nomes duplicados.
- Organizar uma fila de operações.
- Consultar histórico de conversões.

## Ordem de implementação pretendida

| Etapa | Área |
|---|---|
| 1 | Base do aplicativo e painel |
| 2 | Notas, tarefas e calculadora |
| 3 | PDF e imagens |
| 4 | Vídeo e áudio |
| 5 | ZIP, renomeação e fila de operações |
| 6 | Testes e instalador |

A ordem poderá ser ajustada conforme a viabilidade e os resultados dos testes.

## Critério para marcar uma ferramenta como concluída

Uma ferramenta só deverá ser marcada como concluída quando:
1. Estiver implementada no aplicativo.
2. Tiver sido testada com arquivos ou dados reais.
3. Tratar erros e cancelamentos relevantes.
4. Preservar os arquivos originais quando aplicável.
5. Tiver limitações e formatos suportados documentados.

**Estado atual:** módulos planejados; implementação e validação ainda em andamento.
