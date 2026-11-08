# Política de Segurança

## Versões com suporte

| Versão | Suporte |
|---|---|
| 1.3.x | ✅ Sim |
| < 1.3 | ❌ Não, atualize para a mais recente |

## Como relatar uma vulnerabilidade

**Não abra uma issue pública para falhas de segurança.**

Use o recurso privado do GitHub:
**[Security → Report a vulnerability](https://github.com/axolotesuite/axolotesuite/security/advisories/new)**.

Inclua, se puder:

- versão do Axolote Suite e do Windows;
- passo a passo para reproduzir;
- um arquivo de exemplo (ZIP, PDF, imagem...) que dispare o problema;
- o impacto que você imagina (travamento, leitura/escrita fora da pasta, execução de código...).

Respondemos em até 7 dias. Quando a correção sair, damos o crédito a você nas notas da versão (se quiser).

## O que o Axolote já faz para te proteger

- **100% offline:** nenhum arquivo seu sai do PC. A única conexão de rede é o download do FFmpeg, uma vez, no primeiro uso.
- **Original intocado:** arquivos de entrada são abertos só para leitura.
- **Gravação atômica:** a saída vai para um `.axolote_tmp` e só é renomeada no final, então um erro não deixa arquivo pela metade.
- **Anti Zip-Slip:** a extração de ZIP rejeita caminhos que tentem sair da pasta de destino (`../`, caminhos absolutos).
- **Sem administrador:** o instalador trabalha só dentro do seu perfil de usuário (`%LOCALAPPDATA%`).

## Verificando o executável

Todo lançamento traz o hash SHA-256 do `axolote_suite.exe` nas notas da versão e no README.

```powershell
Get-FileHash .\axolote_suite.exe -Algorithm SHA256
```

Compare com o hash publicado e, se quiser, consulte o mesmo hash no [VirusTotal](https://www.virustotal.com/).
