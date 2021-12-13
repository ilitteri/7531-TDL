# 7531-TDL
Trabajo Práctico Grupal de TDL (75.31)

# Usando el sistema

## Clone el repositorio

### Por SSH
```bash
git clone git@github.com:ilitteri/7531-TDL.git
```

### Por HTTPS
```bash
git clone https://github.com/ilitteri/7531-TDL.git
```

### Por GitHub CLI
```bash
gh repo clone ilitteri/7531-TDL
```

Para lo que sigue se requiere instancia de consolas distintas (una por cada parte a ejecutar, para la experiencia completa mínimo dos, una para el servidor y otra para el cliente).

## Servidor
Para ejecutar el servidor una vez clonado el repositorio debe acceder al directorio del mismo

```bash
cd 7531-TDL/server
```

y ejecutar el comando `cargo run` enviando como argumento el nombre del archivo de configuración, de la siguiente manera

```bash
cargo run condig.txt
```

## Cliente
Para ejecutar el cliente una vez clonado el repositorio debe acceder al directorio del mismo (debe haber un servidor corriendo en otra consola para que el cliente pueda conectarse al mismo, de otro modo no se va a poder usar)

```bash
cd 7531-TDL/client
```

y ejecutar el comando `cargo run` enviando como argumento la ip y el puerto en donde está corriendo el servidor, para desarrollo enviar de la siguiente forma
```bash
cargo run localhost <puerto al que se conectó el server>
```
