fn main(){
    // Ce programme utilise les macros println! et print! qui permettent d'afficher du texte dans la console,
    // Respectivement avec et sans retour à la ligne
    println!("My alias is AstralNether");
    print!("Let's get ");
    println!("Rusty !");
}

/*
Note : 

Ce code est le résultat de l'apprentissage du chapitre 1.1-3 de la documentation officielle du langage Rust : 
"https://doc.rust-lang.org/book/"

J'ai créé ce projet Rust grâce à la commande :
cargo new nom_du_projet

Cette commande m'a généré le dossier racine du projet dans lequel se trouvaient deux éléments :
- un dossier src pour le code source du projet dans lequel se trouve le fichier main.rs qui est un programme 'helloworld' classique 
- un fichier Cargo.toml du type TOML (Tom’s Obvious, Minimal Language) qui est le format de configuration de cargo

J'aurais eu un troisième élément dans le projet, c'est-à-dire un fichier .gitignore si je n'avais pas créé ce projet dans un repo github
Il est néanmoins possible d'en avoir un en utilisant la commande :
cargo new nom_du_projet --vcs=git

Dans ce programme, en plus de la macro println! qui permet d'écrire du texte dans le terminal et d'ajouter un retour à la ligne, 
J'ai utilisé la macro print! qui permet aussi de saisir du texte dans le terminal, mais sans ajouter un retour à la ligne

Pour exécuter ce programme, nous pouvons d'abord le compiler grace à la commande :
cargo build

Cette commande crée au niveau du dossier racine un fichier Cargo.lock qui fait état des différentes versions des dépendances du projet,
Et le dossier target qui contient le répertoire debug dans lequel se trouve l'exécutable créé.

On peut ensuite lancer l'exécution du programme grace à la commande :
./chemin_relatif_de_l'exécutable

Pour faire plus simple on peut à la fois compiler le programme et l'exécuter grâce à la commande :
cargo run

La commande run crée un fichier exécutable dans le répertoire target/debug (tout comme la commande build de cargo), mais l'exécute automatiquement

Pour vérifier que le programme peut être compiler sans avoir à le faire (dans l'optique de gagner du temps), on utilise la commande :
cargo check

Lorsqu'on a atteint une version du projet assez stable pour une sortie, on la compile grâce à la commande :
cargo build --release

Cette commande crée un exécutable dans le répertoire target/release plutôt que dans le répertoire target/debug qui est une version avec une compilation optimisée
L'avantage de cette compilation est que le programme Rust s'exécute plus vite, mais elle prends plus de temps à effectuer qu'une compilation classique
*/