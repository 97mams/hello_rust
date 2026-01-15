# 🦀 Rust – Variables, types et réutilisation

Ce document est une **introduction pratique** à la gestion des variables en Rust, pensée pour un **développeur venant de PHP / JS / TS**.

---

## 1️⃣ Déclarer une variable en Rust

En Rust, une variable se déclare avec le mot-clé `let`.

```rust
let x = 10;
```

➡️ Ici :

- `x` est une variable
- `10` est une valeur entière
- **Rust infère automatiquement le type** (`i32` par défaut)

💡 Contrairement à JS ou PHP, une variable est **immuable par défaut**.

---

## 2️⃣ Immutabilité (différence clé avec JS / PHP)

Ce code ❌ ne compile pas :

```rust
let x = 10;
x = 20; // erreur
```

Erreur :

> cannot assign twice to immutable variable

### ✅ Rendre une variable modifiable

```rust
let mut x = 10;
x = 20;
```

➡️ `mut` = mutable

🧠 Pourquoi ?

- code plus prévisible
- moins de bugs
- meilleur raisonnement mémoire

---

## 3️⃣ Typer une variable explicitement

Même si Rust infère, tu peux **forcer le type** :

```rust
let age: i32 = 30;
let price: f64 = 19.99;
let is_admin: bool = true;
let name: &str = "Mamisoa";
```

### Types courants

| Type     | Description      |
| -------- | ---------------- |
| `i32`    | entier signé     |
| `u32`    | entier non signé |
| `f64`    | nombre décimal   |
| `bool`   | true / false     |
| `&str`   | string statique  |
| `String` | string dynamique |

---

## 4️⃣ `String` vs `&str` (très important)

### `&str` (référence, non possédée)

```rust
let name: &str = "Rust";
```

- léger
- souvent utilisé pour les constantes

### `String` (possède la donnée)

```rust
let name: String = String::from("Rust");
```

- alloué en mémoire
- modifiable

```rust
let mut name = String::from("Rust");
name.push_str(" Lang");
```

---

## 5️⃣ Réutiliser une variable (shadowing)

Rust permet le **shadowing** (redéclaration propre).

```rust
let x = 10;
let x = x + 5;
let x = x * 2;
```

➡️ `x` est recréée à chaque fois

Avantage :

- pas besoin de `mut`
- types peuvent changer

```rust
let value = "42";
let value: i32 = value.parse().unwrap();
```

🔥 Très utilisé en Rust réel.

---

## 6️⃣ Variables et fonctions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(2, 3);
    println!("Résultat: {}", result);
}
```

➡️ Pas de `return` obligatoire
➡️ La dernière expression est retournée

---

## 7️⃣ Comparaison rapide avec JS / PHP

### JavaScript

```js
let x = 10;
x = "hello"; // OK
```

### Rust

```rust
let x = 10;
let x = "hello"; // OK (shadowing)
```

❌ MAIS :

```rust
let mut x = 10;
x = "hello"; // interdit
```

---

## 8️⃣ Bonnes pratiques Rust

✅ Utiliser `let` sans `mut` par défaut
✅ Typer quand c'est ambigu
✅ Utiliser le shadowing intelligemment
❌ Éviter `mut` partout (anti-pattern)

---

## 🎯 Exercice pratique

1. Déclare une variable `name` de type `String`
2. Déclare une variable `age` de type `u32`
3. Affiche :

```text
Bonjour NAME, tu as AGE ans
```

💡 Utilise `println!`
