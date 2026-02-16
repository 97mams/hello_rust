# 🦀 Exercice 2 — Gestion d’une liste de tâches en Rust

## 🎯 Objectif

Créer un petit programme en Rust qui permet de gérer une liste de tâches.

Le programme devra :

- Définir une structure `Task`
- Stocker plusieurs tâches dans un `Vec`
- Ajouter une tâche
- Afficher toutes les tâches
- Marquer une tâche comme terminée

---

## 📝 Étapes à suivre

### 1️⃣ Créer une structure

Créer une structure `Task` avec les champs suivants :

- `title: String`
- `completed: bool`

---

### 2️⃣ Créer une liste de tâches

Créer un `Vec<Task>` vide pour stocker les tâches.

---

### 3️⃣ Implémenter les fonctions suivantes

- `add_task(tasks: &mut Vec<Task>, title: String)`
- `list_tasks(tasks: &Vec<Task>)`
- `complete_task(tasks: &mut Vec<Task>, index: usize)`

---

## 💡 Exemple d'affichage attendu

- `[ ]` → tâche non terminée
- `[X]` → tâche terminée

---

## 🔥 Bonus (niveau supérieur)

- Utiliser `impl` pour créer des méthodes sur `Task`
- Empêcher un crash si l’index n’existe pas (utiliser `get_mut`)
- Ajouter une boucle pour créer un petit menu interactif

Exemple :
1 - Ajouter une tâche
2 - Lister les tâches
3 - Terminer une tâche
4 - Quitter

---

## 🧠 Compétences travaillées

- `struct`
- `Vec`
- Ownership
- Mutable references
- `Option`
- `match`
- `impl`

---

Bon courage 🦀🔥  
Quand tu as fini, envoie ton code pour correction !
