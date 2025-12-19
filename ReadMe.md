# Proiect AC - Tema 9 - calibrarea unei camere Lidar si a uneia RGB, intr-o singura sursa RGBD
### Balas Vlad-George 333AB, Radu Codreanu 334AB, David Grapa 332AB

---

## Introducere

Abordarea noastra se bazeaza pe simplificarea problemei pentru a extrage datelele
usor din imagini. In loc de tinta mare checkerboard, vom folosi o tinta sintetica
de 2x2 alb-negru cu singurele transformari permise fiind rotatia si translatia.

Codul returneaza in functie de parametrul "graphs" din main fie waveform-uri, fie
datele de translatie calculate. SVG-ul se poate gasi in /doc/reg.svg. Este destul
de mare (20MB) daca ii dam toate ciclurile de ceas ( 1024 ) si de aceea contine doar
100 de cicluri de ceas. Aceasta poate fi schimbata cu variabila 'clocks' din main.

Tot codul important este documentat prin 'cargo doc', anume modulul important este
circ_sec.rs

## Comenzi

```bash
# pentru a rula codul
cargo run

# pentru documentatie
cargo doc --open
```
