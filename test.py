from dataclasses import dataclass
import itertools

@dataclass
class Pedigree:
    speed: tuple
    accel: tuple
    endur: tuple
    stamn: tuple
    cunni: tuple

@dataclass
class UChoco:
    count: int
    pedig: Pedigree

def generate_children(parent1, parent2):
    """
    Generates all possible combinations of child pedigrees from parents.
    """
    children: list[Pedigree] = []
    combinations = itertools.product([0, 1], repeat=10)  # 5 attributes, 0 or 1 for each
    for c in combinations:
        children.append(
            Pedigree(
                (parent1.speed[c[0]], parent2.speed[c[1]]),
                (parent1.accel[c[2]], parent2.accel[c[3]]),
                (parent1.endur[c[4]], parent2.endur[c[5]]),
                (parent1.stamn[c[6]], parent2.stamn[c[7]]),
                (parent1.cunni[c[8]], parent2.cunni[c[9]])
            )
        )
    return children

def star_score(parent1: Pedigree, parent2: Pedigree){
    
}


parent1 = Pedigree(
    (3, 2),
    (1, 1),
    (4, 4),
    (4, 1),
    (3, 2)
)

parent2 = Pedigree(
    (1, 2),
    (4, 2), 
    (4, 4),
    (1, 2),
    (1, 2)
)

children: list[Pedigree] = generate_children(parent1, parent2)
unique_children: list[UChoco] = []

for child in children:
    unique = True
    for uchild in unique_children:
        if uchild.pedig == child:
            unique = False
            uchild.count += 1
            break
    if unique:
        unique_children.append(UChoco(1, child))

best_children: list[UChoco] = []
best_stars = 0
for uchild in unique_children:
    vals = [a for b in uchild.pedig.__dict__.values() for a in b]
    stars = vals.count(4)
    if stars > best_stars:
        best_stars = stars

best_chance = 0
for uchild in unique_children:
    vals = [a for b in uchild.pedig.__dict__.values() for a in b]
    stars = vals.count(4)
    if stars == best_stars:
        best_children.append(uchild)
        best_chance += uchild.count/len(children)*100

stars_count = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
stars_count_children: list[list[UChoco]] = [[], [], [], [], [], [], [], [], [], []]
for uchild in unique_children:
    vals = [a for b in uchild.pedig.__dict__.values() for a in b]
    stars = vals.count(4)
    stars_count[stars] += uchild.count
    stars_count_children[stars].append(uchild)

most_stars_chance = (max(stars_count)/len(children))*100
most_stars = stars_count.index(max(stars_count))

input("Press enter to end")