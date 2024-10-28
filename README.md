# Lab4-Shaders
Michelle Mejía 22596

Sistema Solar Renderizado con Shaders
Este proyecto es parte de un laboratorio en el cual se implementa un sistema solar básico en un renderizador de software, usando únicamente shaders y sin texturas o materiales. El objetivo es practicar la creación de shaders interesantes variando colores y utilizando parámetros disponibles.

Objetivos
Crear y renderizar 7 cuerpos celestes usando únicamente shaders:
Una estrella (sol).
Al menos un planeta rocoso.
Al menos un gigante gaseoso.
Un planeta con anillos y una luna en órbita.
Un cuerpo celeste adicional (meteorito).
Implementar shaders complejos para simular características atmosféricas y de superficie en los planetas.
Demostrar creatividad en el diseño y complejidad de los shaders aplicados.
Instrucciones para Ejecutar el Proyecto
El proyecto está estructurado en dos ramas principales:

Rama main: Contiene el sistema solar con la mayoría de los cuerpos celestes básicos.
Rama moonOrbiting: Implementa un planeta con una luna en órbita, que puede ser probado de manera independiente.
Para compilar y ejecutar el proyecto, asegúrate de tener el compilador de Rust instalado, y ejecuta el siguiente comando en el directorio del proyecto:

cargo run --release
Una vez en ejecución, puedes utilizar el teclado para seleccionar y visualizar los diferentes cuerpos celestes:

Z: Renderiza la estrella (sol) con efecto de lava y material emisivo.
R: Renderiza un planeta rocoso con texturas de superficie.
G: Renderiza un gigante gaseoso con capas de colores simulando nubes.
X: Renderiza un planeta similar a la Tierra, con océanos y continentes.
C: Renderiza un planeta con anillos y su luna en órbita (en la rama moonOrbiting).
N: Renderiza una nave espacial.
M: Renderiza una luna en órbita de un planeta.
Implementación de Shaders y Efectos
Sol (Estrella)
Shader de lava y material emisivo: La estrella utiliza un shader que simula patrones de lava mediante una mezcla de colores cálidos (rojos y naranjas) y un efecto de emisión que la hace brillar. Para lograr este efecto, se añadió un buffer de emisión y una etapa de postprocesamiento en el pipeline.
Planeta Rocoso
Superficie rocosa con efectos de textura: Utiliza un shader con patrones de ruido para simular una superficie rocosa. La textura simula variaciones en la topografía del planeta, con detalles de sombras y luces que cambian con la posición del "sol".
Gigante Gaseoso
Capas de nubes en movimiento: Este planeta gigante gaseoso cuenta con un shader que aplica varias capas de colores y ruido para simular un efecto atmosférico similar al de Júpiter o Saturno. Las capas de colores dan la impresión de nubes en constante movimiento, cambiando a lo largo del tiempo para simular patrones de viento en la atmósfera.
Planeta con Anillos
Sistema de anillos: El planeta tiene un shader que simula anillos rodeándolo. Los anillos están diseñados con ruido y una función de textura circular que los hace ver más realistas. En la rama moonOrbiting, este planeta tiene una luna en órbita.
Luna en órbita: Implementada en una órbita que simula su movimiento alrededor del planeta. Utiliza la función render_orbiting_moon en combinación con create_model_matrix para actualizar la posición en cada cuadro.
Meteorito
Textura rugosa y variaciones de superficie: El meteorito tiene un shader que aplica ruido perlin para simular una superficie irregular con cráteres. Esto le da un aspecto desgastado, característico de un objeto que ha viajado por el espacio.
Puntos de Evaluación
Este proyecto cumple con los requisitos de la rúbrica en los siguientes aspectos:

Creatividad en el diseño:

Se diseñaron shaders personalizados para cada cuerpo celeste, con patrones de color y texturas únicos.
Complejidad de shaders:

El shader de la Tierra simula océanos y continentes en capas separadas.
La estrella incluye un material emisivo que utiliza un buffer de emisión.
Los planetas con atmósfera utilizan múltiples capas de colores para simular nubes y efectos atmosféricos.
Efectos atmosféricos y de superficie:

El gigante gaseoso cuenta con una atmósfera dinámica, y el planeta similar a la Tierra tiene continentes en movimiento.
La estrella simula un efecto de lava en su superficie.
Implementación de anillos y luna:

Se implementaron anillos en el planeta gaseoso y una luna en órbita usando modelos y cálculos de transformación.
Cuerpo celeste adicional:

Se creó un meteorito con una superficie rugosa para simular la textura irregular de un objeto espacial.
Control de Cámara
Para mejorar la visualización, se implementaron controles de cámara:

Rotación de la cámara: Controla la rotación en torno a los cuerpos celestes usando las teclas de flecha y W y S.
Traslación y acercamiento: Las teclas A, D, Q, E, Up, y Down permiten trasladar y hacer zoom.
Capturas de Pantalla

Conclusión
Este proyecto muestra la flexibilidad de los shaders para crear efectos visuales variados y complejos sin el uso de texturas o materiales, solo mediante el control de colores y patrones en el pipeline de renderizado. La implementación de anillos, luna en órbita y efectos atmosféricos en el sistema solar enriquecen la simulación, cumpliendo con los requisitos del laboratorio y alcanzando un alto nivel de creatividad y realismo.