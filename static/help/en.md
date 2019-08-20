The Bayes-O-Matic is a webapp meaning to help you do Bayesian inference on
various questions you may consider.

## What is this about?

Bayesian inference is a tool at the interesction of probability theory
and epistemology, and revolves around the use of
[Bayes' Theorem](https://en.wikipedia.org/wiki/Bayes%27_theorem)
as a mean to update one's knownledge about the world given new evidence.

In this context, propabilities are no longer linked to randomness, but
rather reflect one's degree of belief that some proposition is true.
Having \\(P(A) = 0.99\\) means "I think that \\(A\\) is true", while
\\(P(A) = 0.01\\) means "I think that \\(A\\) is false". Similarly
\\(P(A) = 0.5\\) means "I don't know at all if \\(A\\) is true or false".

Bayesian inference makes heavy use of conditional probability: \\(P(A|B)\\)
measures how much you would believe that \\(A\\) is true if you knew that
\\(B\\) was true. And Baye's Theorem allows us to reverse these probabilities.
Consider some hypothesis \\(H\\) about the world (a physics heory for example).
This hypothesis can allow you to make some predictions about some data \\(D\\)
you may observe. You can then compare these predictions to the
reality, and observe some data \\(D\\). Your predictions are an evaluation of
\\(P(D|H)\\), and using Bayes' Theorem you can then compute \\(P(H|D)\\): this allows
you to measure how much the data you observed is evidence in favour or
against your hypothesis \\(H\\).

We have some more considerations to take into account though. First, it
is not really possible to determine how plausible an hypothesis is alone.
We always need to compare it to other hypotheses. In the Bayesian framework
one cannot say that "\\(H\\) is true" or "\\(H\\) is false", but rather we can obtain
results like "\\(H_1\\) is 100 times more plausible than \\(H_2\\) given the
observed data".

## Bayesian Networks

Second, applying Bayes' Theorem is in general actually pretty hard. Take
for example the hypothesis \\(H\\): "The laws of gravitation are as Newton
described them", and the data \\(D\\) as being the various orbits of the planets
we observe. How would one evaluate \\(P(D|H)\\)? That would be pretty difficult.

This is where
[Bayesian Networks](https://en.wikipedia.org/wiki/Bayesian_network) take place:
they allow you to split your reasonning into various sub-hypotheses or predictions,
all of them assembled as the nodes of a directed acyclic graph.

Each node represents some variable, and can take some pre-determined set of value.
It can be "true"/"false" if the variable is a logical assertion, but is can also be
any set or mutually-exclusive values. For example a node "color of the car" could
take the values "red", "green", "blue", "black".

Each edge of the graph represents a logical dependency of the reasonning. An
arrow from node \\(A\\) to node \\(B\\) means that what we consider as plausible values
for \\(B\\) depends on the value of \\(A\\). As such, fully specifying a Bayesian Network
requires providing for each node the values of \\(P(v | v_p)\\), where \\(v\\) spans over
all the possible values of the given node, and \\(v_p\\) all possible values of all
parents of this node.

Specifying the whole graph should be done independently of any observations,
in a deductive maneer. At every node, one needs to answer the question "what would
this node likely be assuming its parents are some value". Observed evidence come
in a second time: once the graph is ready, some of its node correspond to assertions
we can actually check in the real world. We can then use Bayes' Theorem to compute
the probabilities of all other nodes in the graph given the ones that are observed.

This app implements an algorithm named "Loopy Belief Propagation" which computes
an approximation of this last probability for each node. This approximation is not
necessarily good in all cases, but it is good enough for Bayesian inference in many
practical cases.

## Log-odds and Credencies

In general, humans often tend to perceive the world in logarithmic scales, and
our beliefs are no exception. That is why it is generally more natural to talk
about probabilities in terms of log-odds, also called logits:
\\(logit(A) = \log_{10}\frac{P(A)}{P(\neg A)}\\). It gives a rought scale of how
\\(A\\) is likely to be true or false: a logit of 0 means we cannot decide, a logit
of 1 that \\(A\\) is 10 times more likely to be true than false, a logit of 2 that it is
100 times more likely to be true, etc. Similarly, negative values for the logit are
in favor of \\(A\\) being false rather than true.

When considering a multi-valued node (for example the color of the car), it can be
more practical to consider relative log odds from a value to an other. There, rather
than the log-odds of "red" \\(\log_{10}\frac{P(Red)}{P(not Red)}\\), we would consider the
log ratio of the probabilities of a given color compared to an other, such as
\\(\log_{10}\frac{P(Red)}{P(Blue)}\\). A value of 2 would mean that the car is 100 times
more likely to be red than blue. Note that due to the properties of the logarithm, we
can also write more generally things like so:

\\(\log_{10}\frac{P(A = a_i)}{P(A = a_j)} = \log_{10} P(A = a_i) - \log_{10} P(A = a_j)\\)

So describing our belief state on the possible values \\(a_1, ... a_k\\) for a node \\(A\\)
can be done by only giving the value of \\(\log_{10} P(A = a_i)\\) for all \\(i\\), and the
relative log odds can easily be computed by computing the difference between two
log-probabilities.

This representation also has the advantage of not requiring normalization (in general
probabilities should sum to 1): as only the difference between two log-probabilities
matter, adding a single constant to everything does not change anything. The
Bayes-O-Matic takes advantage of this and uses unnormalized log-probabilities. To
mark this difference, the Bayes-O-Matic uses the term "credency" to represent them,
and it is noted \\(C(A = a_i)\\).

Note however that comparing unnormalized log-probabilities only makes sense for comparing
the different values of a given node. So \\(C(A = a_i)\\) can be compared to \\(C(A = a_j)\\),
but \\(C(A = a_i)\\) cannot be compared to \\(C(B = b_j)\\).

## How do I use this app?

#### Designing the graph

To use the Bayes-O-Matic, you first need to describe the graph of your model. You
can add several nodes using the "Add node" button, and then select the node you
wish to edit in the node list.

When editing a node, you can change its name to make it more recognizable. You
can also change the possible values it can take as well as edit the list of its
parents. On the left of the screen, a live representation of your graph is displayed
to allow you to keep an eye on your model as a whole. Nodes without any possible value
will appear in red on this representation, and the computation cannot be done if any
node is in that state.

You can then set the credencies of the different values of your node given its parents.
The table contains a row for each possible combination of values of the parents of
your node, and each column represents a possible value of the current node. Filling
this table allows you to specify how likely each value of the node is depending on
its parents.

The credencies you input, being unnormalized log-probabilities, can only
be compared to each other within a row. And similarly, only the difference between each
value withing a row matters. To help filling them, you can for example choose a value
as a reference and describe all other values relative to it. Or you can decide to always
put 0 for the least likely value of the row and fill the other values relative to it.

#### Observations and beliefs

Once your have defined the values and credencies for all your nodes, your model is
in place. You can then go to the "Set observations" tab and set the values for the
nodes that are observed, and thus for which you know their values. Nodes that are
observed will appear in bold in the graphical representation of your model.

Finally, you can run the algorithm to compute the beliefs, by clicking the
"Compute beliefs" button. Beliefs are the same as credencies mathematically
speaking (unnormalized log-probabilities), but we use a different name to
emphasise their different role (credencies are input to the algorithm, beliefs are
its output). For each non-observed node, the Bayes-O-Matic will compute a vector
of beliefs for its different values. As for credencies, only the difference between
two beliefs is meaningful, and only within the same node.

To give some scale, a difference of 1 in belief or credency value is considered as
a slight preference for a value, while a difference of 5 will be a strong belief
that one value must be chosen rather than the other.

When displaying the inference result, you can choose to see the "raw beliefs" as
explained just before, or to dsplay them as log-odds. When choosing to display
log-odds, the app will compute \\(\log_{10}\frac{P(A = a_i)}{P(A \neq a_i)}\\) for
each value \\(a_i\\), rather than just displaying \\(\log_{10}P(A = a_i)\\).

#### Mutual information

An other capability provided is the ability to compute the
[mutual information](https://en.wikipedia.org/wiki/Mutual_information) between
unobserved nodes. Suppose you have designed your graph, are particularly interested
in the value of one particular node, and have not run any experiment yet. If you have
designed a few potential observations as nodes in your graph, this tab will compute
for you the amount of information observing each of them would give you about your node
of interest. This way you can target the more informative observations first.

The information is expressed in the Bayes-O-Matic in bits (so using a logarithm in base 2,
as opposed to credencies which are in base 10) as they are more explicit in this base:
one bit is the amount of information required to discriminate with full certitude between
two values.