## DOCUMENTATION GROUPE 1

Aristide Fumo a fait le serveur, client et a fait la fractale julia et mandelbrot
ARBAOUI Nourdine a fait les autres fractales et des optimisations du code et l'intégration continue.
EL MATROR Yassine a fait les tests unitaires et la documentation.

## Lancer le serveur et le client
Pour lancer le serveur de référence, vous pouvez utiliser la commande suivante
à télécharger et dézippé, sur myges

```bash
$ ./server
```

```bash
cargo run --bin client ari localhost:8787  
```

Nous avons aussi un serveur de référence qui peut être utilisé pour tester votre travailleur.
Vous pouvez le lancer avec

```bash
    cargo run --bin server
```

## Bonus en plus

# Bonus possibles :
Fait un serveur et un client en rust

Ajouter une intégration continue qui permette de tester votre code client et serveur (sous GitHub ou GitLab)

Nous avons un fichier .yml qui permet de faire les tests unitaires et de compiler le projet qui est dans .github/workflows/rust.yml

Déployer des techniques avancées pour optimiser la performance de résolution. Utilisation de la programmation parallèle, de la programmation asynchrone, avec un thread par client, etc.

Nous avons récrit les fractales, car pas optimisé, parce qu'il n'était pas optimisé.

Nous avons utilisé des threads pour les clients, et pour les fractales.

Nous avons presque supprimé les warnings de compilation et les erreurs de panic et les mut.

Nous avons créé des tests unitaires pour les complex,

Nous avons utilisé une implémentation d'un closure pour le Tcpstream, pour le serveur et le client.

les unwrap(), les expect(), les panic!()

Nous avons utilisé des match pour les erreurs.
