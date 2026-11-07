# Como contribuir

Obrigado por contribuir com o Axolote Suite.

## Antes de começar

- Consulte o README e a documentação em docs/.
- Verifique a compatibilidade pretendida com Windows 10 e Windows 11.
- Não adicione telemetria, login obrigatório, nuvem ou IA integrada.
- Não inclua arquivos pessoais, tokens, senhas ou dados privados.

## Organização

- Mantenha cada módulo dentro de sua pasta em src/modules/.
- Coloque código compartilhado em src/shared/.
- Deixe serviços centrais em src/core/.
- Separe interface, processamento e testes.
- Evite dependências circulares.
- Documente as dependências e suas licenças.

## Verificações Rust

Quando o código estiver presente na raiz, execute:

~~~powershell
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
~~~

Não marque verificações como aprovadas sem executá-las.

## Fluxo sugerido

1. Crie uma branch com prefixo feat/, fix/, docs/ ou refactor/.
2. Faça alterações pequenas e focadas.
3. Execute as verificações relevantes.
4. Atualize a documentação afetada.
5. Abra um pull request explicando o que mudou e como foi testado.

## Dependências

Antes de adicionar uma dependência, avalie manutenção, origem, versão, licença, tamanho e necessidade.
