Le Bayes-O-Matic est une application web visant à faciliter les
inférence Bayésiennes sur toute question que vous pourriez considérer.

## « Inférence Bayésienne »  ?

L'inférence Bayésienne est un outil à l'intersection de la théorie des probabilités
et de l'epistémologie, qui est centré sur l'utilisation du
[Théorème de Bayes](https://fr.wikipedia.org/wiki/Th%C3%A9or%C3%A8me_de_Bayes) comme
un moyen de mettre à jour ses connaissances lors de l'obtention de nouvelles preuves.

Dans ce contexte, les probabilités ne sont plus limitées à la modélisation du hasard.
Au contraire, elles servent ici à mesurer nos degrés de croyance qu'une certaine
proposition peut être vraie.
Avoir \\(P(A) = 0.99\\) signifiera « Je pense que \\(A\\) est vrai », alors que
\\(P(A) = 0.01\\) signifiera plutôt « Je pense que \\(A\\) est faux ». De manière similaire
\\(P(A) = 0.5\\) veut dique « Je ne sais pas du tout si \\(A\\) est vrai ou faux ».

L'inférence Bayésienne repose énormément sur les probabilités conditionnelles :
\\(P(A|B)\\) mesure à quel point on croira que \\(A\\) est vrai si on sait que
\\(B\\) est vrai. Le Théorème de Bayes nous permet alors de retourner ces probabilités.
Considérons une hypothèse \\(H\\) à propos du monde (une théorie physique par exemple).
Cette théorie nous permet de faire des prédictions à propos de certaines données \\(D\\)
que l'on pourrait observer. On peut alors comparer ces prédictions à la réalité en
observant les données \\(D\\). Les prédictions sont une évaluation de \\(P(D|H)\\),
et avec le théorème de Bayes, on peut alors calculer \\(P(H|D)\\) : ceci permet
de mesurer à quel point ces observations sont une preuve en faveur ou à l'encontre
de l'hypothèse \\(H\\).

Il y a cependant quelques considérations supplémentaires à prendre en compte. Tout
d'abord, il n'est pas vraiment possible de déterminer si une hypothèse est plausible
dans l'absolu, il nsou faut toujours la comparer à d'autres. Dans le context Bayésien
on ne peut pas dire « \\(H\\) est vrai » or « \\(H\\) est faux » ; nous obtenons plutôt
des résultats comme « \\(H_1\\) est 100 fois plus plausible que \\(H_2\\) sachant les
observations ».

## Les Réseaux Bayésiens

De plus, appliquer le Théorème de Bayes est en général plutôt difficile. Prenez
par exemple l'hypothèse \\(H\\) : « Les lois de la gravitation sont telles que
Newton les a décrites », et la donnée \\(D\\) comme étant les orbites des différentes
planètes que l'on observe. Comment pourrait-on évaluer \\(P(D|H)\\) numériquement ?
Ça serait plutôt très compliqué.

C'est ici que les
[Réseaux Bayésiens](https://fr.wikipedia.org/wiki/R%C3%A9seau_bay%C3%A9sien) entrent
en scène : ils rendent possible de découper un raisonnement un plusieurs sous-hypothèses
et prédictions, toutes organisées ensemble sous la forme d'un graphe acyclique non-orienté.

Chaque nœud du graphe représente une variable, qui peut prendre un ensemble pré-détemriné
de valeurs. Celà peut être « vrai »/« faux » si la variable est une affirmation logique,
mais elles peuvent également être n'importe quel ensemble de valeurs mutuellement exclusives.
Par exemple, un nœud « Couleur de la voiture » pourrait prendre les valeurs « rouge »,
« vert », « bleu », « noir ».

Chaque arrête du graphe représente une dépendance logique de raisonnement. Une flèche
du nœud \\(A\\) au nœud \\(B\\) veut dire que les valeurs que l'on considèrera plausible
pour \\(B\\) dépendent de la valeur de \\(A\\). En conséquence, un réseau Bayésien complètement
définit requiert que l'on fournisse pour chaque nœud les valeurs \\(P(v | v_p)\\),
où \\(v\\) prend toutes les valeurs possibles du nœud courant, et \\(v_p\\) toutes les
combinaisons de valeurs possibles des parents de ce nœud.

Spécifier le graphe dans son intégralité doit être fait indépendamment de toute
observation, de manière déductive. À chaque nœud, il faut répondre à la question
« Quelle valeur prendrait probablement ce nœud en supposant que ses parents on
une certaine valeur ? ». Les observations arrivent dans un second temps : une fois
le graphe prêt, certains de ses nœuds correspondent à des affirmation que l'on peut
généralement comparer au monde réel. On peut ensuite utiliser le théorème de Bayes
pour calculer les probabilités de tous les autres nœuds du graphe sachant ceux que
l'on a observé.

Cette application implémente un algorithme nommé "Loopy Belief Propagation", qui
calcule une approximation de cette probabilité pour chaque nœud. Cette approximation
n'est pas toujours parfaitement bonne, mais elle est suffisante pour l'inférence
Bayésienne dans de nombreux cas.

## Log-cotes et Crédences

En général, les humains ont tendance à percevoir le monde de manière logarithmique,
et nos croyances ne font pas exception. C'est pourquoi it est généralement plus
naturel de parler de probabilités en termes de log-cote, également appelées logits :
\\(logit(A) = \log_{10}\frac{P(A)}{P(\neg A)}\\). Celà donne une idée d'à quel point
\\(A\\) est probablement vrai ou faux : une log-cote de 0 signifie que l'on ne peut pas
décider, une log-cote de 1 que \\(A\\) a 10 fois plus de chances d'être vrai que faux,
une log-cote de 2 qu'il est 100 plus probable d'être vrai, etc. De manière similaire,
une log-cote négative est en faveur du fait que \\(A\\) soit faux plutôt que vrai.

Quand on considère un nœud à plus de deux valeurs (par exemple la couleur de la voiture),
il peut être plus pratique de considérer des log-cotes relatives entre deux valeurs.
Ici, plutôt que de considérer la log-cote de « rouge »
\\(\log_{10}\frac{P(Rouge)}{P(pas Rouge)}\\) on va plutôt considérer le log-ratio des
probabilités d'une couleur donnée comparée à une autre, comme par exemple
\\(\log_{10}\frac{P(Rouge)}{P(Bleu)}\\). Une valeur de 2 voudrait dire que la voiture a
100 fois plus de chances d'être rouge que bleue. On peut noter que grâce aux propriétés
du logarithme, on peut plus généralement écrire les choses ainsi :

\\(\log_{10}\frac{P(A = a_i)}{P(A = a_j)} = \log_{10} P(A = a_i) - \log_{10} P(A = a_j)\\)

Ainsi, décrire notre état de croyance pour les valeurs possibles \\(a_1, ... a_k\\)
pour un nœud \\(A\\) peut être fait en donnant uniquement les valeurs
\\(\log_{10} P(A = a_i)\\) pour chaque \\(i\\), et les log-cotes relatives peuvent être
aisément calculées avec les différences entre ces log-probabilités.

Cette représentation a également l'avantage de ne pas nécessiter de normalization (en
générale des probabilités doivent sommer à 1) : comme seules les différences entre
deux log-probabilités ont de l'importance, ajouter une constante donnée à toutes ne
change rien. Le Bayes-O-Matic utilise ça à son avantage, et travaille en utilisant des
log-probabilités non-normalisées. Pour marquer cette différence, nous utilisons le
terme « crédence » pour les représenter, et on les note \\(C(A = a_i)\\).

Il est important de noter que comparer des log-probabilités non-normalisées n'a de sens
que si on compare les différentes valeurs d'un même nœud. Donc  \\(C(A = a_i)\\) peut
être comparé à \\(C(A = a_j)\\), mais \\(C(A = a_i)\\) ne peut pas être comparé
à \\(C(B = b_j)\\).

## Comment utiliser cette application ?

#### Conception du graphe

Pour utiliser le Bayes-O-Matic, vous devez tout d'abord décrire le graphe de votre
modèle. Vous pouvez créer des nœuds à l'ide du bouton « Ajouter nœud », et ensuite
choisir le nœud que vous voulez modifier en cliquant dessus dans la liste des nœuds.

Lors de l'édition d'un nœud, vous pouvez changer son nom pour mieux le reconnaître.
Vous pouvez également changer les valeurs possible qu'il peut prendre, ainsi que
modifier la liste de ses parents. À gauche de l'écran, une représentation en direct
de votre graphe est faite, vous permettant de garder un œil sur votre modèle d'ensemble.
Les nœuds où vous n'avez pas encore rentré de valeur possible apparaissent en rouge
sur cette représentation, et l'inférence ne peut pas être faite si au moins un
nœud est dans cet état.

Vous pouvez ensuite définir les crédences des différentes valeurs du nœud sachant
ses parents. La tableau contient une ligne pour chaque combinaison possible des
valeurs des parents du nœud, et chaque colonne représente une valeur possible du nœud
en cours d'édition. Remplir ce tableau vous permet de spécifier la probabilité de
chaque valeur du nœud sachant les valeurs de ses parents.

Les crédences que vous rentrez, étant des log-probabilités non-normalisées, peuvent
seulement être comparées ensemble au sein d'une même ligne. Et de manière similaire,
seules les différences entre les valeurs que vous rentrez sont pertinentes. Pour vous
aider à les remplir, vous pouvez par exemple choisir une valeur comme référence à 0 et
décrire les autres relativement à elle. Ou bient vous pouvez décider de toujours
mettre 0 pour la valeur la moins probable et remplir les autres valeurs relativement
à elle.

#### Observations et croyances

Une fois définies les valeurs et les crédences pour tous vos nœuds, votre modèle
est en place. Vous pouvez maintenant vous rendre sur l'onglet « Fixer les
observations » et remplir les valeurs des nœuds que vous avez observé, et donc
pour lesquels vous connaissez les valeurs. Les nœuds observés apparaissent en
gras dans la représentation graphique de votre modèle.

Finalement, vous pouvez exécuter l'algorithme pour cacluler les croyances, en
cliquant sur le bouton « Calculer les croyances ». Les croyances sont
mathématiquement la même chose que les crédences (des log-probabilités
non-normalisées), mais nous utilisons un terme différent pour mettre en évidence
leur différent rôle (les crédences son des entrées de l'algorithme, les croyances
sont son résultat). Pour chaque nœud non-observé, le Bayes-O-Matic va calculer
un vecteur de croyances pour ses différentes valeurs. Comme pour les crédences,
seule la différence entre deux croyances a du sens, et seulement au sein
d'un même nœud.

Pour donner une échelle, une différence de 1 entre deux croyances ou crédences est
considéré comme une légère préférence pour une valeur, alors qu'une différence de 5
est une forte croyance qu'une valeur est vraie au détriment de l'autre.

Lors de l'affichage du résultat, vous pouvez choisir de voir les « croyances brutes »
comme expliqué à instance, out de plutôt afficher les log-cotes. Lors de l'affichage
des log-cotes, l'application va calculer \\(\log_{10}\frac{P(A = a_i)}{P(A \neq a_i)}\\)
pour chaque valeur, plutôt que de simplement afficher \\(\log_{10}P(A = a_i)\\).

#### Information mutuelle

une autre fonctionnalité proposée est le calcul des
[informations mutuelles](https://fr.wikipedia.org/wiki/Information_mutuelle) entre
des nœuds non observés du graphe. Supponsons que vous ayez conçu votre graphe,
êtes particulièrement intéressés par la valeur d'un nœud en particulier, et n'avez
pas encore fait d'expérimentations. Si vous avez imaginé des observations potentielles
et les avez intégrés à votre graphe sous forme de nœuds, cet onglet va calculer pour vous
la quantité d'information qu'observer chacun de ces nœuds apporterait à propos de
votre nœud d'intérêt. Ainsi vous pouvez cibler en priorité les observations qui
apporteraient le plus d'information.

L'information est exprimée dans le bayes-O-matic en bits (donc en utilisant un logarithme
de base 2, à la différence des crédences qui sont en base 10) car elle est plus explicite
dans cette base : un bit correspond à la quantité d'information nécéssaire pour discriminer
deux valeurs avec une certitude absolue.