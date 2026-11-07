# Estrutura do código

A estrutura abaixo é a organização prevista para o código Rust + Iced. Os diretórios serão preenchidos com os arquivos reais do aplicativo.

~~~text
AxoloteSuite/
├── .github/
│   ├── ISSUE_TEMPLATE/
│   └── PULL_REQUEST_TEMPLATE.md
├── assets/
│   └── branding/
├── docs/
│   ├── architecture/
│   ├── modules/
│   ├── security/
│   └── testing/
├── installer/
├── scripts/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── app.rs
│   ├── core/
│   │   ├── configuration/
│   │   ├── events/
│   │   ├── paths/
│   │   └── services/
│   ├── shared/
│   │   ├── components/
│   │   ├── theme/
│   │   └── utilities/
│   └── modules/
│       ├── productivity/
│       ├── images/
│       ├── video/
│       ├── audio/
│       ├── pdf/
│       ├── zip/
│       ├── batch_rename/
│       ├── installer/
│       ├── queue/
│       ├── history/
│       └── settings/
├── tests/
├── Cargo.toml
├── Cargo.lock
├── LICENSE
├── README.md
└── .gitignore
~~~

## Regras

- core contém serviços centrais e coordenação.
- modules contém funcionalidades específicas, cada uma isolada em sua pasta.
- shared contém componentes reutilizáveis sem lógica específica de um módulo.
- tests contém testes de integração e cenários compartilhados.
- installer contém recursos do instalador.
- scripts contém automações de build e manutenção.

Não crie módulos vazios só para preencher a árvore. Adicione arquivos quando houver implementação ou necessidade concreta.
