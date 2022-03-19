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
\\(P(A) = 0.5\\) veut dire « Je ne sais pas du tout si \\(A\\) est vrai ou faux ».

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
dans l'absolu, il nous faut toujours la comparer à d'autres. Dans le context Bayésien
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

Chaque nœud du graphe représente une variable, qui peut prendre un ensemble pré-déterminé
de valeurs. Celà peut être « vrai »/« faux » si la variable est une affirmation logique,
mais elles peuvent également être n'importe quel ensemble de valeurs mutuellement exclusives.
Par exemple, un nœud « Couleur de la voiture » pourrait prendre les valeurs « rouge »,
« vert », « bleu » ou « noir ».

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

## Cotes et Probabilités non-normalisées

Les probabilités très proches de 0 ou 1 sont souvent assez difficiles à se représenter
et il est souvent plus simple de les exprimer en termes de ratios, que l'on nomme des
cotes : \\(cote(A) = \frac{P(A)}{P(\neg A)}\\). Cette cote représentent à quel point
\\(A\\) est plus probablement vraie que fausse. Une cote de 10 veut dire qu'on pense
qu'il est 10 fois plus probable que \\(A\\) soit vraie, plutôt que fausse. Une cote
de 0.1 veut dire au contraire qu'on pense que \\(A\\) a 10 fois plus de chances d'être
fausse que vraie.

Quand on considère un nœud à plus de deux valeurs (par exemple la couleur de la voiture),
il peut être plus pratique de considérer des cotes relatives entre deux valeurs.
Ici, plutôt que de considérer la cote de « rouge »
\\(\frac{P(Rouge)}{P(pas Rouge)}\\) on va plutôt considérer le ratio des
probabilités d'une couleur donnée comparée à une autre, comme par exemple
\\(\frac{P(Rouge)}{P(Bleu)}\\). Une valeur de 100 voudrait dire que la voiture a
100 fois plus de chances d'être rouge que bleue.

Ainsi, décrire notre état de croyance pour les valeurs possibles \\(a_1, ... a_k\\)
pour un nœud \\(A\\) peut être fait en donnant des valeurs de probabilité non normalisées
(il n'est pas nécessaire que leur somme soit 1) pour chaque \\(i\\), et cotes relatives
peuvent être aisément calculées avec les ratios entre ces probabilités non normalisées.
Le Bayes-O-Matic utilise ça à son avantage, et travaille en utilisant de telles
probabilités non-normalisées. Pour marquer cette non-normalisation, nous les notons
\\(\mathcal{P}(A = a_i)\\).

Il est important de noter que comparer des probabilités non-normalisées n'a de sens
que si on compare les différentes valeurs d'un même nœud. Donc
\\(\mathcal{P}(A = a_i)\\) peut être comparé à \\(\mathcal{P}(A = a_j)\\),
mais \\(\mathcal{P}(A = a_i)\\) ne peut pas être comparé à \\(\mathcal{P}(B = b_j)\\).

## Comment utiliser cette application ?

#### Conception du graphe

Pour utiliser le Bayes-O-Matic, vous devez tout d'abord décrire le graphe de votre
modèle. Vous pouvez créer des nœuds à l'aide du bouton « Ajouter nœud », et ensuite
choisir le nœud que vous voulez modifier en cliquant dessus dans la liste des nœuds.

Lors de l'édition d'un nœud, vous pouvez changer son nom pour mieux le reconnaître.
Vous pouvez également changer les valeurs possible qu'il peut prendre, ainsi que
modifier la liste de ses parents. À gauche de l'écran, une représentation en direct
de votre graphe est faite, vous permettant de garder un œil sur votre modèle d'ensemble.
Les nœuds où vous n'avez pas encore rentré de valeur possible apparaissent en rouge
sur cette représentation, et l'inférence ne peut pas être faite si au moins un
nœud est dans cet état.

Vous pouvez ensuite définir les probabilités des différentes valeurs du nœud sachant
ses parents. La tableau contient une ligne pour chaque combinaison possible des
valeurs des parents du nœud, et chaque colonne représente une valeur possible du nœud
en cours d'édition. Remplir ce tableau vous permet de spécifier la probabilité
non-normalisée de chaque valeur du nœud sachant les valeurs de ses parents.

Ces cotes que vous définisses, étant des probabilités non-normalisées, peuvent
seulement être comparées ensemble au sein d'une même ligne. Et de manière similaire,
seules les ratios entre les valeurs que vous rentrez sont pertinentes. Pour vous
aider à les remplir, vous pouvez par exemple choisir une valeur comme référence à 1 et
décrire les autres relativement à elle. Ou bient vous pouvez décider de toujours
mettre 1 pour la valeur la moins probable et remplir les autres valeurs relativement
à elle.

#### Observations et croyances

Une fois définies les valeurs et les probabilités pour tous vos nœuds, votre modèle
est en place. Vous pouvez maintenant vous rendre sur l'onglet « Fixer les
observations » et remplir les valeurs des nœuds que vous avez observé, et donc
pour lesquels vous connaissez les valeurs. Les nœuds observés apparaissent en
gras dans la représentation graphique de votre modèle.

Finalement, vous pouvez exécuter l'algorithme pour cacluler les croyances, en
cliquant sur le bouton « Calculer les croyances ». Pour chaque nœud non-observé,
le Bayes-O-Matic va calculer une liste de croyances pour ses différentes valeurs.
Il s'agit ncore ici de probabilités non normalisées, seul le ratio entre deux
croyances a du sens, et seulement au sein d'un même nœud.

Lors de l'affichage du résultat, vous pouvez choisir de voir les « croyances brutes »
comme expliqué à l'instant, ou de plutôt afficher les cotes. Lors de l'affichage
des log-cotes, l'application va calculer
\\(\frac{\mathcal{P}(A = a_i)}{\mathcal{P}(A \neq a_i)}\\) pour chaque valeur,
plutôt que de simplement afficher \\(\mathcal{P}(A = a_i)\\).

L'affichage en « probabilités » normalise les probabilités pour les afficher, il peut
être plus parlant pour les cas très incertains, mais peut facilement saturer pour les
probabilités très proches de 0 ou de 1.

#### Information mutuelle

Une autre fonctionnalité proposée est le calcul des
[informations mutuelles](https://fr.wikipedia.org/wiki/Information_mutuelle) entre
des nœuds non observés du graphe. Supponsons que vous ayez conçu votre graphe,
êtes particulièrement intéressés par la valeur d'un nœud en particulier, et n'avez
pas encore fait d'expérimentations. Si vous avez imaginé des observations potentielles
et les avez intégrés à votre graphe sous forme de nœuds, cet onglet va calculer pour vous
la quantité d'information qu'observer chacun de ces nœuds apporterait à propos de
votre nœud d'intérêt. Ainsi vous pouvez cibler en priorité les observations qui
apporteraient le plus d'information.

L'information est exprimée dans le Bayes-O-Matic en bits (donc en utilisant un logarithme
de base 2, à la différence des crédences qui sont en base 10) car elle est plus explicite
dans cette base : un bit correspond à la quantité d'information nécéssaire pour discriminer
deux valeurs avec une certitude absolue.