# Guide Technique du Moteur `three_rs` : Les 8 Piliers

Ce document explique les concepts fondamentaux du moteur de rendu 3D logiciel.

---

### 1. Software Rasterizer (Rendu Logiciel)
Le moteur n'utilise pas le GPU. Il écrit directement des couleurs dans un buffer de pixels CPU (`frame_buffer`).

```rust
// Dans rasterizer.rs
pub struct Rasterizer {
    pub frame_buffer: Vec<YVec3>, // Stocke les couleurs RGB (0.0 à 1.0)
    pub z_buffer: Vec<f32>,       // Gère la superposition des objets
}
```

### 2. Row-Major Math (Mathématiques)
Le moteur utilise la convention **Row-Major** pour ses matrices 4x4. Les transformations (`Scale`, `Rotation`, `Translation`) sont multipliées dans cet ordre spécifique pour obtenir la matrice "World".

```rust
// Dans matrix.rs : Application d'un vecteur sur une matrice (v * M)
let x = v.x * self.x[0] + v.y * self.y[0] + v.z * self.z[0] + 1.0 * self.w[0];
```

### 3. Coordinate Pipeline (Pipeline de Rendu)
Chaque point 3D passe par 4 espaces avant de devenir un pixel :
- **Local Space** : Coordonnées de l'objet lui-même.
- **World Space** : Position finale dans l'univers (`transform.world_matrix`).
- **View Space** : Position relative à la caméra (`camera.view_matrix`).
- **Clip/Screen Space** : Projection perspective et conversion en pixels (`camera.projection_matrix`).

### 4. Barycentric Rasterization (Rastérisation)
Pour remplir un triangle, le moteur calcule les coordonnées barycentriques de chaque pixel. Cela permet de savoir si le pixel est "dedans" et d'interpoler les valeurs (UV, Couleurs, Z).

```rust
// Dans rasterizer.rs : Test d'appartenance au triangle
if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
    // Le pixel est à l'intérieur du triangle !
}
```

### 5. Z-Buffering (Gestion de la Profondeur)
Pour éviter que les objets de derrière s'affichent devant, on compare la profondeur `z` de chaque nouveau pixel avec celle stockée dans le `z_buffer`.

```rust
// Dans rasterizer.rs
if z < self.z_buffer[index] {
    self.z_buffer[index] = z;
    self.frame_buffer[index] = color;
}
```

### 6. Gouraud Shading & Textures (Ombrage)
- **Gouraud** : La lumière est calculée aux sommets (v0, v1, v2) puis mélangée sur toute la face.
- **Textures** : On utilise les coordonnées UV pour "coller" une image (JPG/PNG) sur les polygones.

### 7. Primitive Support (Dessin de Lignes et Points)
En plus des triangles, le moteur trace des points et des lignes (utilisés pour les orbites, les grilles et les sélections). 
- **Algorithme DDA (Digital Differential Analyzer)** : Pour les lignes, on calcule le nombre de pixels entre A et B et on avance par "pas". 
- **Z-Buffer pour les LIGNES** : Chaque pixel d'une ligne est testé contre le Z-buffer, ce qui permet aux orbites de passer physiquement derrière les planètes.
- **Dégradés** : La couleur est interpolée le long de la ligne en même temps que la position.

```rust
// Dans scene.rs : Ajout d'une orbite (cercle)
scene.add_circle(center, radius, color, segments);
```

### 8. Ratatui Integration (Affichage Terminal)
L'étape finale : le `Three3DWidget` convertit les pixels du buffer en caractères :
- **Mode Ascii** : Map l'intensité lumineuse sur une rampe de 10 caractères (` .:-=+*#%@`).
- **Mode BackgroundColors** : Utilise les couleurs de fond RGB (`24-bit`) pour un rendu "Pixel Art" fidèle.
- **Transparence** : Le widget vérifie le Z-buffer. Si une case est vide (`INFINITY`), il ne dessine rien, ce qui permet de superposer la 3D par-dessus l'interface du jeu.

---

## Résumé de l'utilisation utile
Pour ton collègue, l'essentiel se passe dans `Scene::render`, qui boucle sur les modèles, applique les matrices, et demande au `Rasterizer` de dessiner les formes géométriques finales.
