# Aprenent Rust

Aquest és el meu segon programa en Rust. Fins ara no n'havia fet mai cap i per tant segurament hi faig algunes coses que poden ser fetes d'una forma més òptima.

## Compilar

1.  Primer s'ha d'instal·lar Rust. La forma més senzilla sol ser seguir les instruccions de [la seva web](https://www.rust-lang.org/es-ES/) ;-). també estan en castellà però en resum consisteixen en executar:

    curl https://sh.rustup.rs -sSf | sh

2.  clonar el repositori

3.  compilar i executar el programa (per defecte agafa in.txt com a paràmetre)

    cargo run --release

Un cop compilat l'executable es trobarà disponible a **target/release/** i es pot executar sense fer servir cargo.

A més se li pot passar un fitxer amb les dades a provar com a paràmetre:

    target/release/rebenton in.txt

# Exercici: Buscando el pinchazo

El programa és un dels exercicis del ProgramaMe de l'any 2018: Programa de la Nacional del año 2018, fet a la Universidad Complutense de Madrid.

> Tiempo máximo: 1,000 s Memoria máxima: 4096 KiB

Quitando la cubierta de una rueda de bici para arreglar un pinchazo

Hacer rutas en bici está muy bien… hasta que pinchas. No es solo el engorro de tener que cambiar la cámara de la rueda en mitad de cualquier camino, es que luego al llegar a casa toca arreglar el pinchazo.

Eso supone llenar un barreño con agua, hinchar la cámara pinchada y sumergirla en agua, sección a sección, hasta ver aparecer las burbujillas delatoras que te dicen el punto que ha amargado tu paseo.

Y, por si pinchar no es suficiente mala suerte, la búsqueda del pinchazo en la cámara vuelve a poner a prueba tu relación con la diosa Fortuna. El primer punto de la cámara que sumerges en agua con la esperanza de encontrar el pinchazo y el sentido en el que vayas girando la cámara para sumergir los tramos siguientes afectan, y mucho, al tiempo que tardarás en ver las esperadas burbujas. Si todo va bien, encontrarás el pinchazo antes de recorrer la mitad de la cámara. Si la mala suerte te ha acompañado hasta casa, las burbujas aparecerán después de haber sumergido la cámara prácticamente entera.

## Entrada

La entrada comienza con un número que indica cuántos casos de prueba habrá que procesar.

Cada caso de prueba son dos números 0 ≤ i, p < 360 indicando el punto inicial de la cámara donde empiezas a buscar el pinchazo (el primero que sumerges) y la posición del pinchazo, respectivamente. Ambos son números enteros indicando el grado en la circunferencia de la cámara.

## Salida

Por cada caso de prueba se escribirá "ASCENDENTE" si es mejor realizar la búsqueda del pinchazo girando de manera ascendente en grados desde el punto de partida, y "DESCENDENTE" si es mejor ir en sentido opuesto. Si el pinchazo está en el punto de inicio o en el opuesto de la circunferencia se escribirá "DA IGUAL".

Ten en cuenta que, al ser grados de una circunferencia, la posición 0 es la misma que la 360.

## Entrada de ejemplo

    3
    90 91
    0 359
    0 180

Salida de ejemplo

    ASCENDENTE
    DESCENDENTE
    DA IGUAL
