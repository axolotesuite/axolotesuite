# 🤝 Como Contribuir com o Axolote Suite

Ficamos muito felizes com o seu interesse em contribuir para o **Axolote Suite**! Qualquer desenvolvedor pode enviar melhorias, novas ferramentas ou correções de bugs via **Pull Request (PR)**.

---

## 🚀 Passo a Passo para Enviar um Pull Request

1. **Faça um Fork** deste repositório clicando no botão **Fork** no topo da página do GitHub.
2. **Clone o seu Fork** para o seu computador:
   ```powershell
   git clone https://github.com/SEU-USUARIO/axolotesuite.git
   cd axolotesuite
   ```
3. **Crie uma branch** para a sua melhoria ou correção:
   ```powershell
   git checkout -b minha-melhoria
   ```
4. **Compile e teste localmente** (requer Rust instalado):
   ```powershell
   cargo run
   # Ou para compilar a versão final otimizada:
   cargo build --release
   ```
5. **Faça o commit e envie para o seu Fork**:
   ```powershell
   git add .
   git commit -m "feat: descricao clara da melhoria"
   git push origin minha-melhoria
   ```
6. **Abra o Pull Request**:
   - Acesse https://github.com/axolotesuite/axolotesuite/pulls e clique em **New Pull Request**.
   - Descreva o que foi adicionado ou corrigido. Assim que revisarmos, faremos o *merge* na branch principal!

---

## 📐 Regras e Padrões do Projeto

- **100% Offline e Privado**: nenhuma ferramenta deve depender de servidores externos ou telemetria para processar os arquivos do usuário.
- **Preservação de Arquivos**: nunca sobrescreva nem apague os arquivos originais do usuário sem confirmação explícita.
- **Interface em Português (PT-BR)**: mantenha todos os textos, botões e dicas ("💡 Como usar") claros e em português.
- **Zero Warnings**: certifique-se de que `cargo check` e `cargo build --release` compilam sem erros ou avisos.
