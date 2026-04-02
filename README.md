# Documentation du Moteur 3D : `three_rs`

Ce module est un moteur de rendu 3D purement logiciel (Software Rasterizer) conçu de zéro pour fonctionner spécifiquement dans un terminal via la bibliothèque `ratatui`. Ce document explique l'architecture, les choix de conception et le fonctionnement de chaque fichier.

## Architecture Globale

Le moteur utilise un pipeline de rendu mathématique classique (`Local -> World -> View -> Clip/Screen`), mais son rasterizer affiche manuellement les pixels sur un buffer qui est ensuite "traduit" en caractères ANSI pour le terminal.

### 1. `matrix.rs` et `vector.rs`
**Rôle** : Fondations mathématiques haute performance.
- **Vecteurs (`YVec2`, `YVec3`, `YVec4`)** : Implémentent les opérations standards (Dot product, Cross product, Normalisation).
  - **Ergonomie** : Support complet des opérateurs (`+`, `-`, `*`, `/`) entre types et références.
  - **Avancé** : Fonctions `lerp` (interpolation linéaire), `reflect` et `distance`.
- **Matrices (`YMat4`)** : Convention **Row-Major**. 
  - **Opérateurs** : Multiplication de matrices (`Mat * Mat`) et transformation directe de vecteurs (`Mat * Vec3`).
  - **`transform_vec3`** : Gère la projection perspective avec division par la composante `w`.

### 2. `core.rs` et `camera.rs`
**Rôle** : Structure et vision.
- **`Transform`** : Gère position, rotation et mise à l'échelle. Calcule la `world_matrix` dans l'ordre `S * R * T`.
- **`Camera`** : Fournit les matrices de Vue (`LookAt`) et de Projection (`Perspective`).
  - **Orbit** : Méthode `orbit(radius, angle, height)` pour faire tourner la caméra autour d'un point cible dynamiquement.

### 3. `scene.rs`
**Rôle** : Le gestionnaire d'entités (Scenegraph simplifié).
- **`Model`** : Un Mesh 3D doté d'un `Transform`, d'une couleur de base et d'une texture optionnelle.
- **`LightPoint`** : Source de lumière ponctuelle avec intensité et couleur.
- **Primitrices Filaires** : Support des points (`Point3D`) et lignes (`Line3D`).
- **Gizmos et Aides** : 
  - `add_grid` : Génère une grille au sol.
  - `add_axis` : Affiche les axes X, Y, Z.
  - `add_box` : Dessine une boîte filaire (bounding box).
  - `add_circle` : Trace des cercles (ex: orbites de planètes).

### 4. `rasterizer.rs`
**Rôle** : Le moteur de dessin (Rasterizer).
- **Z-Buffering** : Un `z_buffer` gère la profondeur pour occulter correctement les surfaces.
- **Gouraud Shading** : Interpolation des couleurs par sommet via coordonnées barycentriques.
- **Lumière Ambiante** : Support d'un niveau de gris global (`ambient_light`) pour éclairer les zones d'ombre.
- **Texture Mapping** : Application de textures répétées ou clampées sur les triangles.
- **Point & Line Drawing** : Algorithme DDA pour tracer des lignes avec interpolation de profondeur et de couleur.

### 5. `widget.rs`
**Rôle** : L'interface avec `ratatui`.
Le widget convertit le `frame_buffer` du Rasterizer en cellules de terminal.

#### Les Modes de Rendu :
1. **`Ascii`** : Utilise une rampe de 10 caractères (` .:-=+*#%@`) basée sur la luminance pour un look rétro "terminal".
2. **`BackgroundColors`** : Utilise les couleurs de fond ANSI (`24-bit RGB`) pour un rendu "pixel art" ultra-coloré (1 pixel = 1 cellule).

---

## Utilisation Rapide

```rust
let mut camera = Camera::new(YVec3::new(0.0, 5.0, -10.0), YVec3::new(0.0, 0.0, 0.0), 1.0);
let mut scene = Scene::new(width, height, camera, Some(0.1));

// Ajouter des objets
scene.add_model(R3DModels::get_drill_model(None).unwrap());
scene.add_grid(10.0, 1.0, YVec3::new(0.5, 0.5, 0.5));
scene.add_circle(YVec3::new(0.0, 0.0, 0.0), 5.0, YVec3::new(0.0, 1.0, 0.0), 32);

// Rendu et Affichage
scene.render(width, height, RenderMode::BackgroundColors);
frame.render_widget(scene.to_widget(RenderMode::BackgroundColors), area);
```
