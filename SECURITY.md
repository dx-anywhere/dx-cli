# Segurança no dx-cli

Obrigado por ajudar a manter o ecossistema seguro. Este documento descreve como reportar vulnerabilidades e como tratamos divulgações responsáveis.

> EN: This policy is primarily in Portuguese. A concise English version can be added on request.

---

## Tabela de Conteúdos
- [Política de versões suportadas](#política-de-versões-suportadas)
- [Como reportar uma vulnerabilidade](#como-reportar-uma-vulnerabilidade)
- [Conteúdo mínimo do relatório](#conteúdo-mínimo-do-relatório)
- [Processo e SLAs](#processo-e-slas)
- [Embargo e comunicados](#embargo-e-comunicados)
- [Escopo e fora de escopo](#escopo-e-fora-de-escopo)
- [Safe Harbor](#safe-harbor)
- [Agradecimentos](#agradecimentos)

## Política de versões suportadas
- Projeto em estágio inicial (0.x). Suporte ativo à última versão publicada.
- Correções de segurança podem ser lançadas como patch/minor a critério dos mantenedores.

## Como reportar uma vulnerabilidade
- Preferencial: GitHub Security Advisory (aba "Security" do repositório) para enviar um relatório privado.
- Se indisponível: abra uma Issue sucinta solicitando canal privado. Não publique detalhes técnicos nem PoCs.
- Evite abrir Pull Requests públicos com correções antes de coordenação privada.

## Conteúdo mínimo do relatório
- Descrição clara do problema e impacto potencial.
- Passos para reproduzir (PoC), preferencialmente em anexo privado.
- Versão/commit afetado e ambiente (SO, versão do Rust/toolchain).
- Mitigações conhecidas, se houver.

## Processo e SLAs
- Reconhecimento inicial: até 7 dias corridos.
- Avaliação e priorização: assim que possível, considerando severidade/impacto.
- Correção: trabalharemos em um patch e, quando aplicável, lançaremos nova versão e notas de segurança.

## Embargo e comunicados
- Durante análise/correção, pedimos confidencialidade (coordinated disclosure).
- Após o release do patch, publicaremos detalhes suficientes para usuários se protegerem.

## Escopo e fora de escopo
- Escopo: código do repositório `dx-cli` e artefatos publicados associados.
- Fora de escopo: problemas em dependências de terceiros fora do nosso controle; ataques que exigem acesso físico ou configuração não padrão.

## Safe Harbor
- Pesquisadores que seguirem esta política, agirem de boa-fé e evitarem interrupções terão proteção de boa-fé por parte dos mantenedores.

## Agradecimentos
- Podemos creditar (com seu consentimento) contribuidores de segurança em notas de versão.

Obrigado por seguir práticas de divulgação responsável e por apoiar a comunidade open source.
