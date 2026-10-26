# Costo botones Si/No (ask_question)
- Pregunta normal texto: ~30-60 tokens (pregunta + respuesta libre larga del usuario).
- Con botones Si/No: ~80-120 tokens por turno (+50 aprox vs texto).
  Desglose: question string ~20 + options 2x~10 + overhead tool JSON ~40.
- Ahorro real: respuesta usuario pasa de ~20-50 tokens libres a 1 token (choice), menos idas/vueltas por ambiguedad.
- Neto: +50 tokens turno, pero -1-2 turnos aclaratorios (~200-400 tokens ahorrados).
- Recomendacion: usar Si/No solo en decisiones binarias (seguir Txx? commit? instalar?). No en preguntas abiertas.
- Regla DittoX: max 1 ask_question por micro-tarea, 2 opciones max, texto corto <15 palabras.
