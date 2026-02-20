# Compte Rendu
# Mini-projet d'algorithmique avancée
## Triangulation minimale d'un polygone convexe

**ENSSAT INFO 2 — 2025-2026**

---

## Table des matières

1. [Introduction](#introduction)
2. [A. Questions préliminaires](#a-questions-préliminaires)
3. [B. Essais successifs](#b-essais-successifs)
4. [C. Programmation dynamique](#c-programmation-dynamique)
5. [D. Algorithme glouton](#d-algorithme-glouton)
6. [E. Recommandation argumentée](#e-recommandation-argumentée)
7. [Conclusion](#conclusion)

---

## Introduction

Ce projet étudie le problème de la **triangulation minimale d'un polygone convexe**.
Étant donné un polygone plan à n sommets s₀, s₁, …, sₙ₋₁ (classés en ordre
rétrograde), il s'agit de sélectionner un ensemble de **cordes** — segments joignant
deux sommets non adjacents — tels que :

1. les cordes ne se croisent pas,
2. le polygone est entièrement subdivisé en triangles,
3. la somme des longueurs des cordes choisies est **minimale**.

Nous étudions trois approches algorithmiques : les essais successifs (backtracking),
la programmation dynamique et l'algorithme glouton. Le code est implémenté en **Rust**
pour ses garanties de performance et de correction mémoire.

**Exemple de référence :** l'heptagone de l'énoncé dont les sommets sont donnés
figure 1. La triangulation non minimale de l'énoncé a un poids de 77,56 ; la
triangulation optimale a un poids de **75,43** (chords (s₀,s₂), (s₀,s₅), (s₂,s₅), (s₃,s₅)).

---

## A. Questions préliminaires

### A.1 — Nombre de cordes distinctes dans un polygone à n sommets

Le nombre total de paires de sommets est C(n, 2) = n(n−1)/2.
Parmi ces paires, n sont des **arêtes** du polygone (sommets adjacents). Le nombre de
cordes (paires non adjacentes) est donc :

```
nombre de cordes = C(n, 2) − n = n(n−1)/2 − n = n(n−3)/2
```

| n  | Cordes possibles | Cordes par triangulation |
|----|-----------------|--------------------------|
|  4 |               2 |                        1 |
|  5 |               5 |                        2 |
|  6 |               9 |                        3 |
|  7 |              14 |                        4 |
| 10 |              35 |                        7 |

### A.2 — Toutes les triangulations comportent le même nombre de cordes

**Théorème :** Toute triangulation d'un polygone convexe à n sommets comporte exactement
n − 3 cordes.

**Preuve par récurrence sur n.**

- **Base (n = 3) :** Le triangle est déjà triangulé sans aucune corde. 0 = 3 − 3. ✓

- **Hérédité :** Supposons le résultat établi pour tout polygone convexe à k < n sommets.
  Soit P un polygone convexe à n sommets et T une triangulation de P.

  Considérons le triangle de T qui contient l'arête (s₀, s₁). Il a la forme (s₀, s₁, sₖ)
  pour un certain k ≥ 2. Le sommet s₁ est alors une « oreille » de P : supprimer s₁
  produit un polygone convexe P' à n−1 sommets (la convexité est préservée car on
  retire un sommet de la frontière convexe). La restriction de T à P' est une triangulation
  de P' comportant, par hypothèse de récurrence, (n−1)−3 = n−4 cordes. En ajoutant
  la corde qui séparait le triangle (s₀, s₁, sₖ) du reste (la corde (s₀, sₖ) si k ≥ 3,
  ou aucune corde supplémentaire si k = 2), on retrouve exactement **n − 3** cordes
  pour la triangulation T de P. ✓

---

## B. Essais successifs

### B.1 — Fonction `validecorde(i, j)`

La fonction `valid_chord(i, j, drawn, vertices)` retourne `vrai` si et seulement si :

1. La corde (sᵢ, sⱼ) n'a pas déjà été tracée (vérification dans `drawn`).
2. La corde (sᵢ, sⱼ) ne coupe aucune corde déjà tracée (test d'intersection par
   produits vectoriels).
3. *(Pour généralité)* La corde ne coupe aucune arête du polygone — condition
   automatiquement vérifiée pour un polygone strictement convexe.

L'intersection de deux segments est détectée par la méthode des produits vectoriels
signés : deux segments (A,B) et (C,D) se coupent proprement si et seulement si C et D
sont de part et d'autre de la droite AB, et réciproquement A et B sont de part et d'autre
de la droite CD.

**Pseudo-code :**

```
Fonction validecorde(i, j, tracées, sommets) :
  Pour chaque corde (a, b) dans tracées :
    Si (a = i et b = j) ou (a = j et b = i) : retourner FAUX  // déjà tracée

  Pour chaque corde (a, b) dans tracées :
    Si a ≠ i et a ≠ j et b ≠ i et b ≠ j :
      Si segments_croisent(sommets[i], sommets[j], sommets[a], sommets[b]) :
        retourner FAUX

  retourner VRAI
```

### B.2 — Analyse de la stratégie naïve par sommets

La stratégie naïve traite les sommets en ordre croissant : à l'étape i, elle trace une
corde valide depuis sᵢ ou ne trace rien.

**Doublons :** Oui. La corde (sᵢ, sⱼ) peut être tracée à l'étape i (depuis sᵢ) ou à
l'étape j (depuis sⱼ). Deux branches différentes de l'arbre de recherche peuvent donc
produire la même triangulation via des ordres de tracé différents.

*Exemple :* Sur le pentagone, {(s₀,s₂), (s₂,s₄)} peut être trouvée par :
(étape 0 : (s₀,s₂), étape 2 : (s₂,s₄)) ou (étape 0 : rien, étape 2 : (s₀,s₂) vu depuis s₂, étape 4 : (s₂,s₄)).

**Incomplétude :** Sous l'interprétation canonique où la corde (sᵢ, sⱼ) (i < j) n'est
disponible qu'à l'étape i, la stratégie ne peut tracer qu'**une seule corde par étape**.
Or certaines triangulations requièrent plusieurs cordes issues du même sommet sᵢ avec i
comme plus petit indice.

*Contre-exemple :* Pour l'hexagone (n=6), la triangulation en éventail depuis s₀ :
T = {(s₀,s₂), (s₀,s₃), (s₀,s₄)} comporte 3 cordes dont s₀ est la plus petite extrémité.
Elles ne peuvent être tracées qu'à l'étape 0, mais la stratégie n'en trace qu'une seule.
Cette triangulation est donc inaccessible.

### B.3 — Stratégie correcte : itération sur les cordes

#### B.3a — Algorithme

Pour garantir que chaque triangulation est construite **une fois et une seule**, on
énumère les cordes dans un ordre fixé (par exemple, triées par longueur croissante) et
on décide pour chacune de l'inclure ou non. Cela forme un arbre binaire de décision
sans ambiguïté.

**Pseudo-code :**

```
Algorithme Backtrack(cordes, start, sélection, meilleuresCords, meilleuresCost) :
  Si |sélection| = n − 3 :
    // Propriété : n-3 cordes non-croisées ⟹ triangulation complète (conv. Euler)
    coût ← somme des longueurs dans sélection
    Si coût < meilleuresCost :
      meilleuresCost ← coût
      meilleuresCords ← sélection
    Retourner

  restantes ← |cordes| − start
  nécessaires ← (n − 3) − |sélection|

  // Élagage 1 : pas assez de cordes restantes
  Si restantes < nécessaires : Retourner

  // Élagage 2 : coût déjà trop élevé
  Si somme(sélection) ≥ meilleuresCost : Retourner

  Pour idx = start à |cordes| − 1 :
    // Élagage 3 (borne inférieure) : les k prochaines cordes sont au minimum
    //   cordes[idx].longueur (triées croissant)
    Si somme(sélection) + nécessaires × cordes[idx].longueur ≥ meilleuresCost :
      Arrêter la boucle (break)  // toutes les suivantes sont pires

    Si validecorde(cordes[idx], sélection, sommets) :
      sélection.ajouter(cordes[idx])
      Backtrack(cordes, idx+1, sélection, meilleuresCords, meilleuresCost)
      sélection.retirer(cordes[idx])
```

#### B.3b — Complexité

Sans élagage, l'algorithme explore tous les sous-ensembles de taille n−3 parmi m cordes,
soit C(m, n−3) appels — mais l'arbre binaire donne O(2^m) appels en pire cas, avec
m = n(n−3)/2. La complexité est donc **O(2^(n(n-3)/2))**.

#### B.3c — Conditions d'élagage et gains

Quatre conditions sont implémentées :

| Condition | Description | Impact |
|-----------|-------------|--------|
| 1 — Complétude | n−3 cordes → triangulation finie | Coupe les branches trop longues |
| 2 — Insuffisance | Cordes restantes < cordes nécessaires | Coupe les branches courtes |
| 3 — Branch-and-bound | Coût partiel ≥ meilleur connu | Coupe les branches sous-optimales |
| 4 — Borne inférieure | Coût partiel + k×min_corde ≥ meilleur | **Coupe la boucle entière** |

La condition 4 est de loin la plus efficace : elle exploite le tri des cordes par longueur
croissante pour arrêter la boucle dès que toute continuation est sous-optimale.

**Tableau comparatif (polygone régulier, conditions 1-3 vs conditions 1-4) :**

| n  | Appels (cond. 1-3) | Appels (cond. 1-4) | Gain    |
|----|-------------------|-------------------|---------|
|  7 |             191   |              17   | ×11     |
| 10 |          20 791   |             207   | ×100    |
| 13 |       2 596 797   |           4 516   | ×575    |
| 20 |            > 10⁹  |      10 338 656   |   —     |

#### B.3d — Limite pratique

Avec les quatre conditions, le temps croît d'un facteur ~3,2 par incrément de n.

| n  | Appels      | Temps (ms) |
|----|-------------|------------|
| 10 |         207 |          0 |
| 13 |       4 516 |          1 |
| 15 |      38 014 |          9 |
| 17 |     274 976 |         77 |
| 18 |     927 798 |        296 |
| 19 |   3 173 616 |      1 012 |
| 20 |  10 338 656 |      3 241 |

Par extrapolation, n = 23 est atteignable en ~96 s < 2 min, n = 24 en ~310 s > 2 min.
**La limite pratique est donc n ≈ 23 en moins de 2 minutes.**

---

## C. Programmation dynamique

### C.1 — Formule de récurrence

On définit T(i, t) comme le coût minimal en cordes de la triangulation du sous-polygone
sᵢ, sᵢ₊₁, …, sᵢ₊ₜ₋₁ (indices modulo n). Le segment de base (sᵢ, sᵢ₊ₜ₋₁) est exclu
du coût car il est soit une arête du polygone, soit une corde déjà comptabilisée par
le problème père.

**Cas de base :**
```
T(i, 1) = 0
T(i, 2) = 0
```

**Cas récursif (t ≥ 3) :** On choisit un sommet sᵢ₊ₖ (k ∈ {1, …, t−2}) pour former le
triangle (sᵢ, sᵢ₊ₖ, sᵢ₊ₜ₋₁). Les nouvelles cordes introduites sont :

- (sᵢ, sᵢ₊ₖ) si k ≥ 2 (sinon c'est l'arête sᵢ→sᵢ₊₁)
- (sᵢ₊ₖ, sᵢ₊ₜ₋₁) si k ≤ t−3 (sinon c'est l'arête sᵢ₊ₜ₋₂→sᵢ₊ₜ₋₁)

```
T(i, t) = min_{k=1}^{t-2} [
    [k ≥ 2]   · d(sᵢ, sᵢ₊ₖ)
  + [k ≤ t-3] · d(sᵢ₊ₖ, sᵢ₊ₜ₋₁)
  + T(i, k+1)
  + T(i+k, t-k)
]
```

La réponse est T(0, n).

### C.2 — Algorithme

```
Algorithme DP_Triangulation(sommets[0..n-1]) :
  // Précalcul des distances
  Pour i, j = 0 à n-1 : d[i][j] ← distance(sommets[i], sommets[j])

  // Initialisation
  Pour i = 0 à n-1 :
    dp[i][1] ← 0  ;  dp[i][2] ← 0

  // Remplissage par taille de sous-problème croissante
  Pour t = 3 à n :
    Pour i = 0 à n-1 :
      dp[i][t] ← +∞
      Pour k = 1 à t-2 :
        ik  ← (i + k) mod n
        it1 ← (i + t − 1) mod n
        c₁ ← d[i][ik]   si k ≥ 2, sinon 0
        c₂ ← d[ik][it1] si k ≤ t−3, sinon 0
        coût ← c₁ + c₂ + dp[i][k+1] + dp[ik][t-k]
        Si coût < dp[i][t] :
          dp[i][t] ← coût  ;  choix[i][t] ← k

  Retourner dp[0][n]
```

La reconstruction des cordes parcourt `choix` récursivement depuis (0, n).

### C.3 — Complexité

**Spatiale :** O(n²) — tableau dp[n][n+1] et choix[n][n+1].

**Temporelle :**

```
∑_{i=0}^{n-1} ∑_{t=3}^{n} (t − 2) = n × ∑_{j=1}^{n-2} j = n × (n−2)(n−1)/2 = O(n³)
```

Complexité temporelle : **O(n³)** comparaisons.

### C.4 — Intérêt du sommet commun

Les sous-problèmes T(i, k+1) et T(i+k, t−k) partagent le sommet sᵢ₊ₖ. Cela garantit
que leurs zones respectives sont disjointes et contiguës, formant ensemble une
triangulation valide sans vérification d'intersection supplémentaire. De plus, chaque
triangulation est examinée **exactement une fois** : il y a une bijection entre les
arbres de décomposition et les triangulations.

Si la décomposition utilisait des cordes arbitraires, l'état d'un sous-problème ne
pourrait plus être caractérisé par un simple couple (i, t) mais nécessiterait un
sous-ensemble quelconque de sommets (2ⁿ états possibles), rendant la mémoïsation
exponentiellement plus coûteuse.

**Résultats sur l'heptagone :**
```
DP : coût = 75.4307, cordes = {(0,2), (0,5), (2,5), (3,5)}
     temps = 6 µs
```

---

## D. Algorithme glouton

### D.1 — Mise en œuvre

L'algorithme maintient un anneau `ring` de sommets restants (initialement 0, 1, …, n−1).
À chaque étape :
1. Pour chaque sommet sᵢ de l'anneau, calculer la longueur de la corde extérieure
   (sᵢ₋₁, sᵢ₊₁).
2. Sélectionner l'oreille de longueur minimale.
3. Enregistrer la corde (sᵢ₋₁, sᵢ₊₁), retirer sᵢ de l'anneau.
4. Répéter jusqu'à ce qu'il reste 3 sommets.

**Pseudo-code :**

```
Algorithme Glouton(sommets[0..n-1]) :
  ring ← [0, 1, …, n−1]
  cordes ← []  ;  coût_total ← 0

  Tant que |ring| > 3 :
    meilleure_longueur ← +∞
    meilleure_oreille ← -1

    Pour i = 0 à |ring| − 1 :
      prev ← ring[(i − 1) mod |ring|]
      next ← ring[(i + 1) mod |ring|]
      longueur ← d(sommets[prev], sommets[next])
      Si longueur < meilleure_longueur :
        meilleure_longueur ← longueur  ;  meilleure_oreille ← i

    prev ← ring[(meilleure_oreille − 1) mod |ring|]
    next ← ring[(meilleure_oreille + 1) mod |ring|]
    cordes.ajouter((prev, next))
    coût_total ← coût_total + meilleure_longueur
    ring.supprimer(meilleure_oreille)

  Retourner (coût_total, cordes)
```

Complexité : **O(n²)** — n−3 itérations, chacune en O(n).

### D.2 — Caractère exact

La stratégie gloutonne **n'est pas exacte** : elle ne garantit pas la triangulation minimale.

**Contre-exemple — l'heptagone de l'énoncé :**

| Méthode | Coût   | Cordes choisies                            |
|---------|--------|--------------------------------------------|
| Glouton | 75,83  | (s₃,s₅), (s₁,s₃), (s₀,s₃), (s₀,s₅)      |
| Optimal | 75,43  | (s₀,s₂), (s₀,s₅), (s₂,s₅), (s₃,s₅)      |

Le glouton commence par (s₃,s₅) ≈ 15,65, puis (s₁,s₃) ≈ 16,16. Ces choix localement
optimaux forcent ensuite (s₀,s₃) ≈ 21,93 et (s₀,s₅) ≈ 22,09, pour un total de 75,83.
La solution optimale, en choisissant (s₀,s₂) ≈ 17,89 et (s₂,s₅) ≈ 19,80 au lieu de
(s₁,s₃) et (s₀,s₃), obtient 75,43.

**Résultats expérimentaux (polygone régulier) :**

| n   | Coût DP   | Coût Glouton | Ratio |
|-----|-----------|-------------|-------|
|  10 |   968,21  |    984,05   | 1,016 |
|  20 | 1 586,24  |  1 626,43   | 1,025 |
|  50 | 2 415,15  |  2 463,41   | 1,020 |
| 100 | 3 043,06  |  3 094,65   | 1,017 |

Le glouton s'écarte d'environ **2 %** de l'optimal sur les polygones réguliers testés.
Cet écart peut être plus important pour des polygones de géométrie défavorable.

---

## E. Recommandation argumentée

### Synthèse comparative

| Critère | Essais successifs (élagage) | Programmation dynamique | Algorithme glouton |
|---|---|---|---|
| **Exactitude** | Oui | Oui | Non (~2 % d'écart) |
| **Complexité temporelle** | O(2^(n(n-3)/2)) (pratique : ~3.2^n) | **O(n³)** | O(n²) |
| **Complexité spatiale** | O(n) (pile) | O(n²) | O(n) |
| **Limite pratique** | n ≤ 23 (< 2 min) | n ≤ plusieurs milliers | Illimité |
| **Reconstruction** | Directe | Via table `choix` | Directe |

### Recommandation

**La programmation dynamique est l'algorithme à recommander** pour ce problème.

Ses atouts décisifs sont :
- **Exactitude garantie** dans tous les cas, quelle que soit la géométrie du polygone.
- **Complexité polynomiale O(n³)** garantie — non dépendante de l'élagage ni de la
  structure des données. Pour n = 1 000, la DP termine en une fraction de seconde.
- **Mémoïsation efficace** : les O(n²) sous-problèmes sont chacun résolus une seule fois.
- La reconstruction des cordes optimales est O(n) supplémentaire.

L'algorithme glouton, bien que très rapide (O(n²)), ne donne aucune garantie
d'optimalité. Pour une application où la précision est requise (robotique, maillage
numérique, etc.), ce 2 % d'écart peut être critique.

Les essais successifs avec élagage restent précieux comme **outil de validation** de la
DP sur de petits polygones, et pour explorer des variantes du problème où la
programmation dynamique ne s'applique pas directement (polygones non convexes,
distances non euclidiennes, contraintes additionnelles sur les cordes).

**Rôle de chaque algorithme dans la pratique :**
- **DP** → production (solution exacte, scalable)
- **Glouton** → initialisation de la borne supérieure pour l'élagage des essais successifs
- **Essais successifs** → validation et résolution de variantes contraintes

---

## Conclusion

Nous avons étudié trois approches pour la triangulation minimale d'un polygone convexe.

**La programmation dynamique** est la solution recommandée : exacte, polynomiale O(n³),
et adaptée à tout polygone de taille pratique. La clé de son efficacité est la
décomposition en sous-polygones contigus partageant un sommet, qui garantit la validité,
l'unicité et la mémoïsabilité des sous-problèmes.

**Les essais successifs** illustrent l'importance de l'élagage : sans la borne inférieure
par longueur minimale (condition 4), la limite pratique est n ≈ 13 ; avec elle, elle
monte à n ≈ 23. Le facteur de réduction pour n = 13 est de 575.

**L'algorithme glouton** est une heuristique rapide (O(n²)) qui s'écarte d'environ 2 %
de l'optimal sur les polygones réguliers. Il n'est pas exact : l'heptagone de l'énoncé
en est un contre-exemple concret (75,83 contre 75,43).

Les 24 tests unitaires passent avec succès et valident la cohérence des trois approches :
DP et backtracking donnent le même résultat optimal sur tous les polygones de n = 4
à n = 10 ; le glouton est toujours ≥ DP ; les cordes restituées sont non-croisées et
au nombre de n − 3.

---

## Annexe — Listings commentés

Les fichiers sources sont disponibles dans le dépôt. Architecture :

```
src/
  polygon.rs       — Structures de base (Point, distances, intersection de segments)
  backtracking.rs  — B.1 valid_chord, B.3a/b/c/d backtrack avec élagage
  dynamic.rs       — C.1/C.2 DP triangulation + reconstruction des cordes
  greedy.rs        — D.1 algorithme glouton par sélection d'oreilles
  main.rs          — Programme principal : benchmarks et comparaisons
  tests.rs         — 24 tests unitaires et d'intégration
```

**Compilation et exécution :**
```bash
cargo build --release
cargo run --release    # Affiche les résultats pour toutes les sections
cargo test             # Lance les 24 tests
```
