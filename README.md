# Polygon Fill Lab — Polígono 1

## Propósito de la rama

Esta rama demuestra de forma independiente el relleno y trazado del polígono 1, utilizando la infraestructura compartida (framebuffer, línea y algoritmo de relleno) creada en `main`. `src/main.rs` en esta rama construye únicamente el polígono 1 y genera su evidencia visual por separado.

## Polígono implementado

Definido en `src/polygons/polygon_1.rs`, con los siguientes vértices:

```
(165.0, 380.0) (185.0, 360.0) (180.0, 330.0) (207.0, 345.0) (233.0, 330.0)
(230.0, 360.0) (250.0, 380.0) (220.0, 385.0) (205.0, 410.0) (193.0, 383.0)
```

- Color de relleno: `Color::GREEN`
- Color de borde: `Color::BLACK`

## Algoritmos utilizados

- **Point-in-Polygon** con regla par-impar: determina si el centro de cada píxel candidato está dentro del contorno del polígono.
- **Bounding box**: calculado a partir de los valores mínimos y máximos de `x` e `y` de los vértices, para limitar los píxeles evaluados durante el relleno.
- **Bresenham**: traza los segmentos de línea entre vértices consecutivos para dibujar el borde del polígono.
- **Validación de límites**: el framebuffer descarta cualquier coordenada fuera de las dimensiones de la imagen antes de pintar un píxel.

## Archivos relevantes

- `src/main.rs`: punto de entrada de esta rama. Crea el framebuffer, construye el polígono 1, ejecuta su relleno y el trazado de su borde, y exporta el resultado a `evidence/polygon-1.png`.
- `src/polygons/polygon_1.rs`: define la función que construye el `Polygon` del polígono 1 con sus vértices y colores.
- `src/polygon.rs`: define la estructura `Polygon` (vértices, color de relleno, color de borde, agujeros) y el método `draw_border`, que traza el contorno mediante `line()`.
- `src/polygon_fill.rs`: implementa `fill_polygon`, que calcula el bounding box y aplica la prueba Point-in-Polygon sobre cada píxel candidato.
- `src/line.rs`: implementa el algoritmo de Bresenham utilizado para dibujar los bordes.
- `src/framebuffer.rs`: encapsula la imagen de raylib; expone `point()` con validación de límites, `clear()` y `render_to_file()`.

## Ejecución

```bash
cargo run
```

Esta rama genera el archivo de evidencia:

```text
evidence/polygon-1.png
```

## Evidencia

![Evidencia del polígono 1](evidence/polygon-1.png)

## Relación con el proyecto

Esta rama conserva una prueba aislada del polígono 1. La integración de todos los polígonos en una sola escena, junto con la generación de `out.png`, se encuentra en `main`.
