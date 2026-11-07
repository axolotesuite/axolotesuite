# Checklist de validação

Execute os itens aplicáveis quando o código-fonte estiver presente.

## Compilação

- [ ] cargo fmt --all -- --check
- [ ] cargo check --all-targets
- [ ] cargo test --all-targets
- [ ] cargo clippy --all-targets -- -D warnings
- [ ] cargo build --release

## Aplicativo

- [ ] Inicialização no Windows 10.
- [ ] Inicialização no Windows 11.
- [ ] Navegação entre módulos.
- [ ] Interface não congela durante operações demoradas.
- [ ] Erros são apresentados de forma compreensível.

## Arquivos

- [ ] Arquivos originais são preservados.
- [ ] Destino de saída é respeitado.
- [ ] Nomes duplicados são tratados.
- [ ] Falta de permissão é tratada.
- [ ] Arquivos inválidos são rejeitados.

## ZIP

- [ ] Extração normal.
- [ ] Rejeição de caminhos com traversal.
- [ ] Tratamento de entradas duplicadas e arquivos muito grandes.

## Instalador

- [ ] Instalação em perfil sem privilégios administrativos, se esse for o modo suportado.
- [ ] Atalhos corretos.
- [ ] Desinstalação.
- [ ] Opção de preservar dados pessoais.
- [ ] Download e verificação de dependências, quando aplicável.

Registre versão do Windows, versão do aplicativo, comandos executados, resultados e problemas encontrados em um relatório datado.
