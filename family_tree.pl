% -----------------------------
% Family Tree Program
% -----------------------------

% define writeln/1 for environments that don't have it built in
writeln(X) :- write(X), nl.

% -----------------------------
% Gender facts (grouped)
% -----------------------------
male(robert).
male(peter).
male(john).
male(mark).
male(alex).
male(noah).

female(linda).
female(susan).
female(emma).
female(kate).
female(olivia).
female(mia).

% -----------------------------
% Parent facts
% -----------------------------

% Generation 1 -> 2
parent(robert, john).
parent(linda, john).
parent(robert, kate).
parent(linda, kate).

parent(peter, emma).
parent(susan, emma).
parent(peter, mark).
parent(susan, mark).

% Generation 2 -> 3
parent(john, alex).
parent(emma, alex).
parent(john, olivia).
parent(emma, olivia).

parent(kate, noah).
parent(mark, noah).
parent(kate, mia).
parent(mark, mia).

% -----------------------------
% Derived relationship rules
% -----------------------------

% child(X, Y): X is child of Y
child(X, Y) :- parent(Y, X).

% grandparent(X, Y): X is grandparent of Y
grandparent(X, Y) :-
    parent(X, Z),
    parent(Z, Y).

% sibling(X, Y): share a parent, not same person
sibling(X, Y) :-
    parent(P, X),
    parent(P, Y),
    X \= Y.

% cousin(X, Y): parents are siblings
cousin(X, Y) :-
    parent(P1, X),
    parent(P2, Y),
    sibling(P1, P2),
    X \= Y.

% -----------------------------
% Recursive logic
% -----------------------------

% descendant(D, A): D is descendant of A
descendant(D, A) :-
    parent(A, D).
descendant(D, A) :-
    parent(A, X),
    descendant(D, X).

% ancestor(A, D): A is ancestor of D
ancestor(A, D) :-
    descendant(D, A).

% convenience rules
children_of(P, C) :- parent(P, C).
siblings_of(X, S) :- sibling(X, S).

% -----------------------------
% Main for online compilers
% -----------------------------

main :-
    writeln("Family Tree Loaded."),
    writeln("Sample outputs:"),

    writeln("Children of john:"),
    findall(X, children_of(john, X), KidsJohn),
    writeln(KidsJohn),

    writeln("Siblings of john:"),
    findall(X, siblings_of(john, X), SibJohn),
    writeln(SibJohn),

    writeln("Grandchildren of robert:"),
    findall(X, grandparent(robert, X), GcRob),
    writeln(GcRob),

    writeln("Cousins of alex:"),
    findall(X, cousin(alex, X), CousAlex),
    writeln(CousAlex),

    writeln("Descendants of robert:"),
    findall(X, descendant(X, robert), DescRob),
    writeln(DescRob).

:- initialization(main).
