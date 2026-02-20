fn main() {
    // Cette commande utilise la macro println! pour afficher du texte dans la console
    println!("Hello, world!");
}

/*
Note : 

Ce code est le résultat de l'apprentissage du chapitre 1.1-2 de la documentation officielle du langage Rust : 
"https://doc.rust-lang.org/book/"

J'ai pu installer Rust depuis le terminal (Linux) de mon codespace grâce à la commande suivante : 
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

Pour une installation sur Windows il faudra téléchager l'installeur de Rust sur le site officiel : 
"https://rust-lang.org/tools/install/"

Dans ce projet Rust j'ai moi-même créé le dossier helloworld et son contenu (le fichier helloworld.rs que vous lisez actuellement) 
Et j'ai écris le premier programme de beaucoup de dévellopeurs en Rust, pour me familiariser avec la syntax

La fonctin main est le premier bout de code exécuté par un programme Rust,
Et println! (notez bien le '!') est une macro du langage. C'est similaire aux fonctions, d'après mon niveau actuel de compréhension, mais ce ne serait pas tout à fait ça ?

Pour exécuter ce programme il faudra d'abord le compiler grace à la commande :
rustc nom_du_programme.rs

Et l'exécution se fait grace à la commande :
./nom_du_programme
*/