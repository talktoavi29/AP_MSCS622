% -----------------------------
% Family Tree Program
% -----------------------------

% define writeln/1 for environments that don't have it built in
writeln(X) :- write(X), nl.

% -----------------------------
% Facts: Gender
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
% Facts: Parent Relationships
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
% Rules: Derived Relationships
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
% Recursive Logic
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

% -----------------------------
% Convenience Rules (for specific queries)
% -----------------------------
children_of(P, C) :- parent(P, C).
siblings_of(X, S) :- sibling(X, S).

% -----------------------------
% Main Entry Point (for non-interactive compilers)
% -----------------------------
main :-
    writeln('--- Family Tree Execution ---'),
    
    writeln('1. Children of John:'),
    findall(X, children_of(john, X), KidsJohn),
    writeln(KidsJohn),
    
    writeln('2. Siblings of John:'),
    findall(X, siblings_of(john, X), SibJohn),
    writeln(SibJohn),
    
    writeln('3. Grandchildren of Robert:'),
    findall(X, grandparent(robert, X), GcRob),
    writeln(GcRob),
    
    writeln('4. Cousins of Alex:'),
    findall(X, cousin(alex, X), CousAlex),
    writeln(CousAlex),
    
    writeln('5. Descendants of Robert:'),
    findall(X, descendant(X, robert), DescRob),
    writeln(DescRob).

:- initialization(main).