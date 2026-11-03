fn main() {
    exo1();
    exo2();
    exo3tableau();
    exo4tableau();
    let vivante = true;
    let voisins = 3;
    if ((vivante) && (voisins == 3 || voisins == 2)) || ((!vivante)&&(voisins==3)) {
        println!("survie!");

    }
    else {
        println!("Mort");
    }
}
fn exo1() {
    let cellules = [true, false, true, true, false];

    let mut compteur = 0;
    let mut cmpt2= 0 ;

    loop {
       if cmpt2 == cellules.len() {
        break;
       }
       if cellules[cmpt2] {
            compteur +=1; 
        }
       cmpt2 +=1;
    }
    println!("{}", compteur)
}
fn exo2() {
        let cellules = [true, false, true, true, false];
        let mut compteur =0;
        for x in cellules {
            if x {
                compteur +=1;
            }
        }
        println!("{}",compteur)
}
//on vas maintenant travailler sur une grille

fn exo3tableau() {
    let grille = [
    [true, false, true, false],
    [false, true, true, false],
    [true, false, false, true],
];
    let mut nbrcel = 0;
    for ligne in grille {
        for x in ligne{
            if x {
                nbrcel +=1;

            }
        }
    }
    println!("{}",nbrcel)
}

fn exo4tableau(){
        let grille = [
        [false, false, false, false, false, false, false],
        [false, true,  false, true,  false, true,  false],
        [false, false, true,  true,  false, false, false],
        [false, true,  true,  false, true,  false, false],
        [false, false, true,  false, true,  true,  false],
        [false, true,  false, false, true,  false, false],
        [false, false, false, false, false, false, false],
    ];
   cmptvoisines(3,4,grille);
}
fn cmptvoisines(i:usize,j:usize,grille:[[bool;7];7]){
    let mut cmpt=0;
    if grille[i-1][j-1] {
        cmpt+=1
    }
    if grille[i-1][j] {
        cmpt+=1
    }
if grille[i-1][j+1] {
        cmpt+=1
    }
if grille[i+1][j-1] {
        cmpt+=1
    }
if grille[i+1][j] {
        cmpt+=1
    }
if grille[i+1][j+1] {
        cmpt+=1
    }
    if grille[i][j+1] {
        cmpt+=1
    }
    if grille[i][j-1] {
        cmpt+=1
    }
    println!("{}",cmpt)
}

//prchaine étape fonction qui prend une grille et qui renvoie la grille au temps +1

