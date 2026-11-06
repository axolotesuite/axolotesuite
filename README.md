<div align="center">

<img src="assets/branding/banner.svg" alt="Banner do Axolote Suite" width="100%">

# Axolote Suite

### Uma proposta de suíte de produtividade modular para Windows.

<p>
  <img src="https://img.shields.io/badge/Windows-10%20%2F%2011-5865F2?logo=windows&logoColor=white" alt="Windows 10 e 11">
  <img src="https://img.shields.io/badge/Abordagem-Offline--first-64748b" alt="Offline first">
  <img src="https://img.shields.io/badge/IA-Fora%20do%20escopo-7c3aed" alt="Sem IA integrada">
  <img src="https://img.shields.io/badge/Estado-Planejamento-f59e0b" alt="Em planejamento">
</p>

**Produtividade • Modularidade • Privacidade • Controle local**

</div>

---

## 🦎 Conheça o projeto

O **Axolote Suite** é um projeto que pretende reunir ferramentas de produtividade em um único ambiente para computadores com Windows 10 e Windows 11.

Em vez de obrigar o usuário a procurar e organizar vários programas separados para tarefas cotidianas, a proposta é oferecer uma suíte com áreas diferentes, chamadas de **módulos**. Cada módulo teria uma finalidade própria, mas todos fariam parte da mesma experiência.

O projeto nasce com uma prioridade clara: o usuário deve conseguir trabalhar com seus arquivos localmente, sem depender de uma conta online ou de uma conexão permanente com a internet para as funções principais.

> **Importante: este repositório é uma apresentação do conceito e da identidade visual. O aplicativo ainda não está disponível como produto pronto. Os módulos, telas e recursos descritos nesta página são objetivos planejados, não funcionalidades implementadas ou testadas.**

## 🎯 Qual problema o projeto pretende resolver?

No uso diário, é comum alternar entre diversos aplicativos para escrever, organizar imagens, converter arquivos e realizar outras tarefas. Isso pode deixar o fluxo de trabalho fragmentado.

O Axolote Suite pretende estudar uma forma de reunir essas atividades em um só lugar, com uma interface consistente e ferramentas organizadas por finalidade.

A intenção não é prometer que todos os recursos de programas especializados serão reproduzidos. O objetivo é construir, passo a passo, ferramentas úteis, compreensíveis e adequadas ao escopo do projeto.

## 👥 Para quem está sendo pensado?

A proposta é voltada principalmente a pessoas que:

- usam Windows 10 ou Windows 11;
- preferem ferramentas organizadas em um único ambiente;
- trabalham com documentos e arquivos no próprio computador;
- valorizam a privacidade e querem entender o que um programa faz com seus dados;
- preferem não depender de login, assinatura ou conexão constante para as funções principais;
- procuram uma experiência simples, modular e fácil de acompanhar.

## 🧩 Como a suíte poderá funcionar?

A experiência ainda será definida por protótipos. A visão inicial é a seguinte:

1. **Abrir o Axolote Suite:** o usuário encontra uma tela inicial com as áreas disponíveis.
2. **Escolher uma ferramenta:** cada módulo apresenta sua finalidade e seus recursos.
3. **Trabalhar com arquivos locais:** o usuário abre ou salva arquivos no próprio computador.
4. **Organizar o ambiente:** módulos e configurações ficam apresentados de forma consistente.
5. **Gerenciar a instalação:** a proposta inclui meios claros para instalar, atualizar e remover componentes.

Esse fluxo é uma descrição conceitual. A interface e o comportamento final dependerão dos testes e das decisões técnicas.

## 🧰 Módulos planejados

A suíte deverá crescer por partes. Cada módulo precisará ter uma finalidade bem definida, documentação própria e testes antes de ser apresentado como pronto.

| Módulo | Objetivo explicado |
|---|---|
| 📝 **Documentos** | Criar, abrir, editar e organizar documentos de texto. |
| 🖼️ **Imagens** | Oferecer ferramentas para visualizar, organizar e realizar edições de imagem compatíveis com o escopo. |
| 🎬 **Vídeo** | Reunir ferramentas para tarefas básicas com arquivos de vídeo, conforme a viabilidade técnica. |
| 🎧 **Áudio** | Permitir trabalhar com arquivos de áudio por meio de ferramentas específicas. |
| 🔄 **Conversão** | Converter arquivos entre formatos que sejam efetivamente suportados. |
| 🧰 **Utilitários** | Disponibilizar pequenas ferramentas para tarefas cotidianas no computador. |
| 📦 **Gerenciador** | Ajudar a visualizar, instalar, atualizar ou remover módulos da suíte. |

### O que significa ser modular?

Significa que a suíte não precisa ser construída como um bloco único. Cada área poderá ser desenvolvida, testada e documentada separadamente. Isso facilita priorizar recursos menores, corrigir problemas e evitar que uma mudança em uma ferramenta afete desnecessariamente as outras.

A forma técnica de implementar essa separação ainda será estudada.

## 🔐 Privacidade e funcionamento offline

Privacidade é uma diretriz de projeto, não apenas uma frase de divulgação.

### O que queremos dizer com “offline-first”?

Significa planejar as funções principais para trabalhar no computador do usuário. Abrir ou editar um arquivo local, por exemplo, não deverá depender de enviar esse arquivo para um serviço na internet.

Algumas atividades futuras, como baixar um instalador ou obter uma atualização, podem exigir conexão. Qualquer comportamento desse tipo deverá ser explicado claramente, e não deve ser confundido com a operação local das ferramentas.

### Compromissos pretendidos

- **Sem login obrigatório:** a proposta é permitir o uso local sem criar uma conta.
- **Sem telemetria por padrão:** não planejar coleta automática de dados de uso.
- **Sem envio automático de documentos:** os arquivos do usuário devem permanecer locais durante as operações locais.
- **Sem IA integrada:** inteligência artificial não faz parte do escopo definido para o produto.
- **Transparência de rede:** qualquer recurso que precise se conectar deverá explicar o motivo.
- **Controle do usuário:** instalação, atualização e remoção devem ser documentadas.

Esses pontos são requisitos que deverão ser verificados na implementação. Eles ainda não representam auditoria, certificação ou garantia de segurança.

## 🖥️ Plataforma e decisões técnicas

| Tema | Direção atual |
|---|---|
| Sistema operacional | Windows 10 e Windows 11 |
| Modelo de uso | Prioridade para funcionamento local |
| Conta de usuário | Sem login obrigatório |
| Telemetria | Não prevista por padrão |
| Inteligência artificial | Fora do escopo |
| Organização | Suíte dividida em módulos |
| Linguagem e ferramentas | Ainda serão avaliadas |
| Tecnologia .NET | Não faz parte das diretrizes do projeto |

A linguagem de programação e as ferramentas de desenvolvimento não foram definidas nesta apresentação. A escolha deverá considerar compatibilidade com Windows, desempenho, acessibilidade, manutenção, segurança e licenças das dependências.

## 🎨 Identidade visual

O mascote do Axolote Suite é um axolote estilizado. A identidade utiliza roxo, branco e tons escuros, com uma proposta minimalista e tecnológica.

Os elementos visuais foram preparados em formato SVG, que permite redimensionar as imagens sem perda de qualidade.

<div align="center">
  <img src="assets/branding/logo.svg" alt="Logotipo Axolote Suite" width="74%">
</div>

Arquivos da identidade:
- [Banner](assets/branding/banner.svg)
- [Logotipo](assets/branding/logo.svg)
- [Ícone](assets/branding/icon.svg)
- [Guia da identidade visual](assets/branding/README.md)

## 🗺️ Como o projeto pretende evoluir?

O desenvolvimento será dividido em etapas. Isso ajuda a evitar promessas exageradas e permite mostrar o que realmente foi concluído.

| Etapa | O que precisa acontecer |
|---|---|
| 1. Planejamento | Definir o escopo, as necessidades e os limites do produto. |
| 2. Pesquisa técnica | Comparar tecnologias, dependências, licenças e compatibilidade. |
| 3. Protótipos | Testar ideias de interface e organização dos módulos. |
| 4. Núcleo da suíte | Construir a base que poderá integrar os módulos. |
| 5. Módulos | Desenvolver e testar cada ferramenta por etapas. |
| 6. Instalação | Preparar instalação, atualização e desinstalação. |
| 7. Validação | Testar em ambientes compatíveis e documentar resultados. |

Não há datas de lançamento confirmadas. Uma etapa só deverá ser marcada como concluída quando houver implementação e evidência verificável.

## 📚 Documentação

Os documentos explicam o conceito e as diretrizes do projeto:

- [Visão geral do Axolote Suite](docs/VISAO-GERAL.md)
- [Módulos planejados](docs/MODULOS-PLANEJADOS.md)
- [Princípios do projeto](docs/PRINCIPIOS.md)
- [Identidade visual](assets/branding/README.md)

## 🤝 Sugestões e participação

Ideias, perguntas e sugestões podem ajudar a melhorar o planejamento. Ao apresentar uma proposta, explique qual problema ela resolveria, quem seria beneficiado e como ela respeitaria os princípios do projeto.

Sugestões não significam que uma funcionalidade já foi aprovada ou que será implementada. Cada ideia deverá ser avaliada quanto à utilidade, viabilidade, segurança, manutenção e compatibilidade.

## ❓ Perguntas frequentes

**O Axolote Suite já pode ser baixado?**  
Ainda não. Esta página apresenta o projeto e seus objetivos. Uma versão para download só deverá ser anunciada quando existir um pacote real, testado e documentado.

**O projeto vai funcionar sem internet?**  
A prioridade é que as funções principais trabalhem localmente. Download de programas e atualizações poderá exigir conexão, e isso deverá ser informado.

**Vai exigir cadastro ou assinatura?**  
A proposta é não exigir login nem assinatura para as funções locais principais.

**Terá inteligência artificial?**  
Não. IA integrada está fora do escopo definido para o Axolote Suite.

**Quais linguagens serão usadas?**  
Ainda não foi escolhida uma linguagem. A decisão será tomada depois de pesquisa e protótipos.

**Todos os módulos já estão prontos?**  
Não. Os módulos apresentados são ideias e objetivos de desenvolvimento.

**O projeto já possui certificação de segurança ou conformidade legal?**  
Não há certificação declarada. Segurança, privacidade, licenças e obrigações legais deverão ser analisadas e documentadas durante o desenvolvimento.

---

<div align="center">

<img src="assets/branding/icon.svg" alt="Mascote Axolote" width="84">

### Axolote Suite

**Ferramentas organizadas. Arquivos sob controle. Produtividade local.**

<sub>Projeto em planejamento. Acompanhe este repositório para ver a evolução.</sub>

</div>
