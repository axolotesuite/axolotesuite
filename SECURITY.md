# Segurança

A segurança do Axolote Suite é tratada como requisito de desenvolvimento.

## Reportar uma vulnerabilidade

Não publique detalhes de uma vulnerabilidade não corrigida em uma issue pública.

Use a opção **Report a vulnerability** na aba **Security** do GitHub, caso esteja habilitada para este repositório. Envie uma descrição clara, passos para reproduzir e o impacto observado.

Não inclua senhas, tokens, documentos pessoais ou arquivos privados no relato.

## Práticas esperadas

- Validar caminhos e entradas fornecidos pelo usuário.
- Evitar sobrescrita ou exclusão inesperada de arquivos.
- Proteger extração ZIP contra Zip Slip e entradas maliciosas.
- Não executar comandos por shell com dados não confiáveis.
- Verificar origem, versão e licença das dependências.
- Tratar falhas sem expor dados privados.
- Revisar o processo de atualização e os componentes baixados pelo instalador.

Este documento descreve práticas pretendidas. Não declara que o aplicativo passou por auditoria externa ou que todas as vulnerabilidades foram eliminadas.
