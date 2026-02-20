# Réponses — Mini-projet d'algorithmique avancée

---

## A. Questions préliminaires

### A.1 — Nombre de cordes distinctes dans un polygone à n sommets

Une corde relie deux sommets non adjacents. Le nombre total de paires de sommets est
C(n, 2) = n(n−1)/2. Parmi ces paires, n sont des arêtes (côtés adjacents du polygone).
Le nombre de cordes est donc :

```
C(n, 2) − n = n(n−1)/2 − n = n(n−3)/2
```

*Exemple : heptagone (n=7) → 7×4/2 = 14 cordes possibles.*

### A.2 — Toutes les triangulations comportent le même nombre de cordes

**Preuve par récurrence sur n (n ≥ 3).**

- **Base (n=3) :** Un triangle n'est pas subdivisible. La triangulation contient 0 = 3−3
  cordes. ✓

- **Hérédité :** Supposons que toute triangulation d'un polygone convexe à k sommets
  (3 ≤ k < n) comporte exactement k−3 cordes. Soit P un polygone convexe à n sommets.

  Soit T une triangulation quelconque de P. Considérons le triangle de T qui contient
  l'arête (s₀, s₁). Ce triangle a nécessairement la forme (s₀, s₁, sₖ) pour un certain
  k ≥ 2. Si k ≥ 3, la corde (s₀, sₖ) (ou (s₁, sₖ)) est dans T. Dans tous les cas, le
  triangle (s₀, s₁, sₖ) est un « oreille » : retirer le sommet s₁ donne un polygone
  convexe P' à n−1 sommets (convexité préservée car on supprime un sommet de la
  frontière convexe). La triangulation T restreinte à P' est une triangulation de P'
  avec exactement une corde de moins que T (la corde qui séparait le triangle oreille
  du reste). Par hypothèse de récurrence, T∩P' comporte (n−1)−3 = n−4 cordes.
  En ajoutant la corde séparatrice, T comporte **n−3** cordes au total.

Ainsi, toute triangulation d'un polygone convexe à n sommets comporte exactement
**n − 3 cordes**.

*Vérification : heptagone (n=7) → 4 cordes, cohérent avec la figure de l'énoncé.*

---

## B. Essais successifs

### B.2 — Stratégie naïve par sommets

La stratégie proposée consiste, à l'étape i, à tracer **une** des cordes valides issues de
sᵢ (c'est-à-dire ayant sᵢ pour extrémité) ou à ne rien tracer.

**Calcule-t-on plusieurs fois la même triangulation ?**

Oui. La corde (sᵢ, sⱼ) peut être tracée lors de l'étape i (depuis sᵢ) **ou** lors de
l'étape j (depuis sⱼ). Deux branches différentes de l'arbre de recherche peuvent donc
produire la même triangulation via des ordres de tracé différents.

*Exemple sur le pentagone (n=5) :* la triangulation {(s₀,s₂), (s₂,s₄)} peut être
produite par :
- Étape 0 : tracer (s₀,s₂) ; étape 2 : tracer (s₂,s₄). ✓
- Étape 0 : rien ; étape 2 : tracer (s₀,s₂) (depuis s₂) ; étape 4 : tracer (s₂,s₄) (depuis s₄). ✓

Ces deux chemins différents génèrent la même triangulation.

**Cette méthode permet-elle d'obtenir toutes les triangulations ?**

Non. La stratégie trace **au plus une corde par étape**. Or certaines triangulations
requièrent plusieurs cordes dont la seule étape disponible est la même.

*Contre-exemple :* Pour un hexagone (n=6), considérons la triangulation « en éventail »
depuis s₀ : T = {(s₀,s₂), (s₀,s₃), (s₀,s₄)}. Les trois cordes ont s₀ pour extrémité
de plus petit indice. Si l'on interprète la stratégie de manière **canonique** (une corde
(sᵢ,sⱼ) avec i < j n'est disponible qu'à l'étape i), alors les trois doivent être tracées
à l'étape 0 — mais on ne peut en tracer qu'une. La triangulation en éventail est alors
inaccessible.

*Remarque :* si la stratégie est interprétée de façon bidirectionnelle (une corde (sᵢ,sⱼ)
est disponible à l'étape i ET à l'étape j), alors la méthode est complète mais génère des
doublons. La question B.3a résout ce problème en itérant sur les cordes dans un ordre
fixé, garantissant que chaque triangulation est construite une seule fois.

### B.3b — Complexité de l'algorithme par essais successifs (sur les cordes)

Soit m = n(n−3)/2 le nombre de cordes possibles. L'algorithme considère chaque corde
et décide de l'inclure ou non dans la triangulation. Le nombre total d'appels récursifs
est **O(2^m)** dans le pire cas, soit **O(2^(n(n-3)/2))**.

En pratique, l'élagage réduit drastiquement ce nombre (voir ci-dessous).

### B.3c — Conditions d'élagage

Plusieurs conditions permettent de couper des branches stériles :

1. **Triangulation complète :** Si on possède déjà n−3 cordes non-croisées, la
   triangulation est complète (propriété des polygones convexes). On enregistre le
   résultat et on s'arrête.

2. **Nombre de cordes insuffisant :** Si le nombre de cordes restantes à explorer est
   strictement inférieur au nombre de cordes encore nécessaires (n−3 − |sélection|),
   il est impossible de compléter la triangulation. On élagage.

3. **Borne supérieure dépassée (branch-and-bound) :** Si le coût courant dépasse déjà
   la meilleure solution connue, on élagage.

4. **Borne inférieure par longueur minimale :** Les cordes sont triées par longueur
   croissante. Si `coût_courant + somme_des_k_prochaines_cordes ≥ meilleure_solution`
   (avec k = cordes encore nécessaires), toute extension ne peut qu'être pire. On peut
   alors **arrêter la boucle** (break), car toutes les cordes suivantes sont au moins aussi
   longues. C'est la condition la plus puissante.

**Mesures expérimentales** (polygone régulier, avec les quatre conditions actives) :

| n  | Appels récursifs | Temps (ms) | (sans cond. 4) |
|----|-----------------|-----------|----------------|
|  4 |               2 |         0 |              3 |
|  5 |               3 |         0 |             11 |
|  6 |               5 |         0 |             45 |
|  7 |              17 |         0 |            191 |
|  8 |              29 |         0 |            903 |
|  9 |              69 |         0 |          4 221 |
| 10 |             207 |         0 |         20 791 |
| 11 |             628 |         0 |         99 846 |
| 12 |           1 698 |         0 |        512 831 |
| 13 |           4 516 |         1 |      2 596 797 |
| 14 |          14 636 |         3 |            ... |
| 15 |          38 014 |         9 |            ... |
| 16 |          87 451 |        22 |            ... |
| 17 |         274 976 |        77 |            ... |
| 18 |         927 798 |       296 |            ... |
| 19 |       3 173 616 |     1 012 |            ... |
| 20 |      10 338 656 |     3 241 |            ... |

La condition 4 réduit le nombre d'appels de façon spectaculaire : de 2,6 millions (sans
cette borne, avec les conditions 1–3 seulement) à 4 516 pour n=13 (facteur ~575).

### B.3d — Limite pratique avec élagage

Le temps croît d'un facteur ~3.2 par incrément de n. En extrapolant :
- n=23 : ~96 s → accessible en moins de 2 minutes.
- n=24 : ~310 s → dépasse la limite.

Avec les quatre conditions d'élagage implémentées, la limite pratique est donc **n ≈ 23**
en moins de 2 minutes sur une machine moderne.

---

## C. Programmation dynamique

### C.1 — Formule de récurrence

On note T(i, t) le coût minimal en termes de **cordes** (segments non-arêtes) de la
triangulation du sous-polygone formé par les sommets sᵢ, sᵢ₊₁, …, sᵢ₊ₜ₋₁
(indices modulo n). Le segment de base de ce sous-problème est (sᵢ, sᵢ₊ₜ₋₁), qui est
soit une arête du polygone (au niveau racine), soit une corde engagée par le père.
Dans les deux cas, son coût n'est pas à imputer à T(i, t).

**Cas de base :**
```
T(i, 1) = 0   (un sommet : aucun triangle)
T(i, 2) = 0   (deux sommets : aucun triangle)
```

**Cas récursif (t ≥ 3) :**

Pour chaque k ∈ {1, …, t−2}, on forme le triangle (sᵢ, sᵢ₊ₖ, sᵢ₊ₜ₋₁) et on résout
les sous-problèmes T(i, k+1) et T(i+k, t−k). Les nouveaux segments créés sont :
- (sᵢ, sᵢ₊ₖ) : c'est une **corde** si et seulement si k ≥ 2 (sinon c'est l'arête sᵢ→sᵢ₊₁).
- (sᵢ₊ₖ, sᵢ₊ₜ₋₁) : c'est une **corde** si et seulement si k ≤ t−3 (sinon c'est l'arête sᵢ₊ₜ₋₂→sᵢ₊ₜ₋₁).

La formule est donc :

```
T(i, t) = min_{k=1}^{t-2} [
    [k≥2]   · d(sᵢ, sᵢ₊ₖ)
  + [k≤t-3] · d(sᵢ₊ₖ, sᵢ₊ₜ₋₁)
  + T(i, k+1)
  + T(i+k, t-k)
]
```

où [·] désigne l'indicateur booléen (1 si vrai, 0 si faux), et d(·,·) est la distance
euclidienne.

La réponse optimale est T(0, n).

### C.2 — Algorithme de programmation dynamique

```
Algorithme DP_Triangulation(S[0..n-1], d[n][n]) :
  Pour i = 0 à n-1 :
    dp[i][1] ← 0
    dp[i][2] ← 0

  Pour t = 3 à n :
    Pour i = 0 à n-1 :
      dp[i][t] ← +∞
      Pour k = 1 à t-2 :
        ik  ← (i + k) mod n
        it1 ← (i + t - 1) mod n
        coût_corde_ik  ← d[i][ik]   si k ≥ 2, sinon 0
        coût_corde_it1 ← d[ik][it1] si k ≤ t-3, sinon 0
        coût ← coût_corde_ik + coût_corde_it1 + dp[i][k+1] + dp[ik][t-k]
        Si coût < dp[i][t] :
          dp[i][t] ← coût
          choix[i][t] ← k

  Retourner dp[0][n]
```

La reconstruction des cordes se fait en parcourant `choix` de façon récursive depuis
`choix[0][n]`.

### C.3 — Complexité

**Spatiale :** On stocke dp[i][t] et choix[i][t] pour i ∈ [0, n−1] et t ∈ [1, n].
Cela représente 2n² entrées → complexité spatiale **O(n²)**.

**Temporelle :** Pour chaque sous-problème dp[i][t], la boucle interne en k effectue
t−2 itérations (chacune en O(1)). Le nombre total de comparaisons est :

```
∑_{i=0}^{n-1} ∑_{t=3}^{n} (t − 2)
= n × ∑_{t=3}^{n} (t − 2)
= n × ∑_{j=1}^{n-2} j
= n × (n−2)(n−1)/2
= O(n³)
```

Complexité temporelle : **O(n³)**.

### C.4 — Intérêt du sommet commun

Dans la décomposition choisie, les deux sous-problèmes T(i, k+1) et T(i+k, t−k)
partagent toujours le sommet sᵢ₊ₖ. Cela signifie que chaque sous-problème correspond
à un **sous-polygone contigu** délimité par un arc du contour et une corde.

**Intérêt :** Les deux zones sont disjointes (elles ne partagent que la corde frontière),
donc la réunion de leurs triangulations est automatiquement une triangulation valide du
polygone d'origine. Il n'y a pas besoin de vérifier l'absence d'intersection entre les
cordes des deux sous-triangulations.

De plus, cette décomposition est **minimale** : chaque triangulation possible est examinée
exactement une fois. On peut le montrer par bijection : toute triangulation de T(i, t)
correspond à exactement un choix de k (le sommet du triangle qui contient la base
(sᵢ, sᵢ₊ₜ₋₁)), et ce k détermine uniquement les deux sous-problèmes.

**Si la décomposition utilisait deux cordes quelconques**, les sous-polygones pourraient
ne pas être contigus, voire se chevaucher. L'état d'un sous-problème ne pourrait plus
être décrit par le seul couple (i, t), mais nécessiterait de spécifier un sous-ensemble
quelconque de sommets — soit 2^n états possibles au lieu de n². La mémoïsation
deviendrait exponentiellement plus coûteuse, et la vérification de la validité des cordes
(non-croisement) devrait être refaite à chaque décomposition.

---

## D. Algorithme glouton

### D.1 — Mise en œuvre

L'algorithme maintient un anneau de sommets restants. À chaque étape :
1. Calculer toutes les cordes extérieures : pour chaque sommet sᵢ de l'anneau, la corde
   extérieure est (sᵢ₋₁, sᵢ₊₁), qui forme un triangle avec les arêtes (sᵢ₋₁,sᵢ) et (sᵢ,sᵢ₊₁).
2. Choisir la corde extérieure la plus courte.
3. Enregistrer cette corde, retirer sᵢ de l'anneau.
4. Répéter jusqu'à ce qu'il ne reste que 3 sommets.

Complexité : O(n²) — n−3 itérations, chaque itération parcourt O(n) sommets et
effectue une suppression O(n) dans un tableau.

### D.2 — Caractère exact de la stratégie gloutonne

La stratégie gloutonne **n'est pas exacte** : elle ne garantit pas la triangulation minimale.

**Contre-exemple concret — l'heptagone de l'énoncé :**

Le glouton produit les cordes {(s₃,s₅), (s₁,s₃), (s₀,s₃), (s₀,s₅)} pour un coût de
**75.8304**.

La triangulation optimale (trouvée par DP et essais successifs) est {(s₀,s₂), (s₀,s₅),
(s₂,s₅), (s₃,s₅)} pour un coût de **75.4307**.

Le glouton commence par sélectionner (s₃,s₅) (longueur ≈ 15.65, la plus courte), puis
(s₁,s₃) (≈ 16.16). Ces choix localement optimaux forcent ensuite (s₀,s₃) ≈ 21.93 et
(s₀,s₅) ≈ 22.09. La solution optimale évite cette situation en choisissant des cordes
d'abord moins évidentes mais globalement plus avantageuses.

**Résultats expérimentaux (polygone régulier) :**

| n   | Coût DP    | Coût Glouton | Ratio glouton/DP |
|-----|------------|-------------|-----------------|
|  10 |  968.21    |  984.05     | 1.016           |
|  20 | 1 586.24   | 1 626.43    | 1.025           |
|  50 | 2 415.15   | 2 463.41    | 1.020           |
| 100 | 3 043.06   | 3 094.65    | 1.017           |

Le glouton s'écarte d'environ 2 % de l'optimal sur des polygones réguliers. En pratique,
la qualité de l'approximation dépend fortement de la géométrie du polygone.

---

## E. Recommandation argumentée

| Méthode | Complexité temporelle | Exacte | Praticable (grand n) |
|---|---|---|---|
| Essais successifs (avec élagage) | O(2^(n(n-3)/2)) réduit en pratique | Oui | Jusqu'à n ≈ 20 |
| Programmation dynamique | O(n³) | Oui | Excellente (n jusqu'à plusieurs milliers) |
| Algorithme glouton | O(n²) | Non (~2 % d'écart) | Excellente |

**Recommandation : la programmation dynamique.**

- **Exactitude garantie** : la DP donne toujours la triangulation minimale.
- **Complexité polynomiale** : O(n³) en temps et O(n²) en espace sont parfaitement
  raisonnables pour tout polygone de taille pratique.
- **Robustesse** : contrairement aux essais successifs, la complexité est garantie dans
  tous les cas, sans dépendre de la qualité de l'élagage ni de la géométrie du polygone.
- **Reconstruction** : la table `choix[i][t]` permet de reconstruire les cordes en O(n).

Le glouton peut être utilisé comme **borne supérieure initiale** pour l'élagage des essais
successifs, améliorant leur efficacité pratique. Les essais successifs restent utiles pour
**valider** les résultats de la DP sur de petits polygones.
