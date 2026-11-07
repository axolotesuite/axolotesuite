# Visão geral do Axolote Suite

## Objetivo

O Axolote Suite é uma aplicação desktop para Windows 10 e Windows 11 que reúne ferramentas de produtividade, documentos, imagens, vídeo, áudio e organização de arquivos.

A proposta é oferecer uma interface única, com processamento local e controle claro sobre os arquivos do usuário.

## Base técnica

A versão descrita pelo desenvolvedor utiliza:

| Tecnologia | Finalidade |
|---|---|
| Rust | Linguagem principal |
| Iced | Interface desktop |
| Cargo | Dependências e compilação |
| FFmpeg / ffprobe | Processamento e inspeção de mídia |

A configuração real, as versões e os recursos habilitados devem ser confirmados no Cargo.toml e no código-fonte enviado ao repositório.

## Áreas da aplicação

- **Produtividade:** notas, tarefas e calculadora.
- **Imagens:** edição, filtros, redimensionamento, conversão e compressão.
- **Vídeo:** cortes, velocidade, filtros, conversão e compressão.
- **PDF:** união, divisão e inspeção técnica.
- **Áudio:** extração, conversão e processamento.
- **Arquivos:** organização, renomeação em lote e ZIP.
- **Sistema:** fila de operações, histórico, configurações e instalador.

## Privacidade e funcionamento local

O objetivo é processar arquivos no computador, sem login obrigatório, sem telemetria por padrão e sem envio automático de documentos.

O primeiro setup poderá precisar de conexão para baixar dependências opcionais, como FFmpeg. Depois de instalados os componentes necessários, as funções locais devem operar offline sempre que tecnicamente possível.

Esses pontos são requisitos do produto e precisam ser verificados por inspeção do código e testes.

## Desenvolvimento

A implementação deve avançar por módulos, mantendo responsabilidades separadas, dependências justificadas, documentação atualizada e testes reproduzíveis.
