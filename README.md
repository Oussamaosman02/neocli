# NeoCli

> Tu asistente personal de línea de comandos

¿Te gustaría tener a tu disposición un asistente personal que pueda responder tus preguntas y ayudarte en tus tareas diarias desde la línea de comandos? ¡**neocli** es la solución que estabas buscando!

Esta CLI (Command Line Interface) hecha con Rust hace uso de la poderosa inteligencia artificial GPT-3 para ofrecerte respuestas precisas y útiles a cualquier consulta que le hagas. Ya sea para obtener información sobre un tema específico, realizar conversiones, resolverte dudas sobre comandos,etc... neocli estará a tu disposición para lo que necesites.

## Instalación

> La versión 1.1 está actualmente solo para Linux

Puedes ir al apartado **Releases** de este repositorio y descargarlo manualmente.
También puedes ejecutar el siguiente comando:

```bash
wget https://github.com/Oussamaosman02/neocli/releases/download/v1.1/neocli
```

Una vez descargado, habrá que darle permisos y moverlo a los binarios del usuario para poder usarlo en cualquier lado de la terminal:

```bash
chmod +x /carpeta/de/descarga/neocli
sudo mv /carpeta/de/descarga/neocli /usr/bin
```

Ahora es accesible en cualquier parte de la terminal, sin necesidad de root.

EL último paso es configurar la variable de entorno llamada `AI_TOKEN`, que tendrá el valor de nuestra api de `OpenAi`. Puedes solicitar la tuya [aquí](https://beta.openai.com/account/api-keys)

```bash
sudo nano ~/.bashrc
```

Ahí, añade la siguiente línea:

```bash
export AI_TOKEN="VALOR"
```

Recuerda cambiar "VALOR" por el valor real de tu API key.

AHora ya puedes usar `neocli`:

```bash
neocli Hola
```

## `v1.1`

En esta versión, añadimos un nuevo crate y quitamos las otras dependencias.

¡Ahora usamos [davinci](https://crates.io/crates/davinci)!

## Uso

Una vez instalado y configurado, puedes invocar a neocli desde cualquier terminal con el comando `neocli`. A partir de ahí, solo tienes que seguir las instrucciones del asistente para empezar a hacerle preguntas.

Algunos ejemplos de cosas que puedes preguntarle a neocli:

- ¿Cómo salgo de vim?
- ¿Cómo se centra un div?
- ¿Cómo hago un comentario en bash?

¡Prueba a hacerle cualquier pregunta que se te ocurra y descubre todo lo que neocli puede hacer por ti!

## Créditos

`neocli` es un proyecto desarrollado por [Oussama Osman](https://github.com/Oussamaosman02) con la ayuda de la API de GPT-3 de OpenAI.

No pretende ser un proyecto avanzado, ni mucho menos. Es solo un proyecto mientras se produce el aprendizaje del lenguaje Rust.

## Contribuir

Eres libre para abrir una issue si piensas que se podría mejorar el código de alguna manera, después se te asignará la PR si es necesario. Cualquier contribución es bienvenida.
