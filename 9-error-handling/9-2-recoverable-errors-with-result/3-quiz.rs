Pergunta 1
Qual dessas afirmações descreve melhor por que File::open retorna um Result e não um Option?

Você respondeu:
Porque Result pode representar por que uma operação falhou, e a abertura de arquivo pode falhar por muitos motivos

Contexto: Option pode apenas representar que uma operação falhou, mas Result pode explicar por que a operação falhou.