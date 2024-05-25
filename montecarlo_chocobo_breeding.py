from random import random, choice
from operator import itemgetter

class Gender:
    FEMALE = 0
    MALE   = 1

class Dna:
    def __init__(self, label: str, gender: Gender, mother_strands: list[bool], father_strands: list[bool]):
        self.label = label
        self.gender = gender
        self.mother_strands = mother_strands
        self.father_strands = father_strands
        self.nb_reproductions_left = 10

    def count_good_strands(self):
        return len([strand for strand in self.mother_strands + self.father_strands if strand])

    def count_adam_and_eva_score(self):
        return self.label.count("+")

def get_best(chocobos: list[Dna]) -> Dna:
    best_score = max([chocobo.count_good_strands() for chocobo in chocobos])
    for chocobo in chocobos:
        if chocobo.count_good_strands() == best_score:
            return chocobo
    return None

def reproduce(chocobo_a: Dna, chocobo_b: Dna) -> Dna:
    gender = Gender.FEMALE if random() >= 0.5 else Gender.MALE

    offspring_mother_strands = [
        strand_a if random() >= 0.5 else strand_b
        for strand_a, strand_b in zip(chocobo_a.mother_strands, chocobo_b.mother_strands)
    ]
    
    offspring_father_strands = [
        strand_a if random() >= 0.5 else strand_b
        for strand_a, strand_b in zip(chocobo_a.father_strands, chocobo_b.father_strands)
    ]
    
    chocobo_a.nb_reproductions_left -= 1
    chocobo_b.nb_reproductions_left -= 1

    return Dna(f"({chocobo_a.label} + {chocobo_b.label})", gender, offspring_mother_strands, offspring_father_strands)

def rate_chocobos(chocobos: list[Dna], nb_generations, nb_reproductions_per_generation: int) -> list[tuple[tuple[Dna, Dna], float]]:
    ratings = []

    for _ in range(nb_generations):
        chocobos = [chocobo for chocobo in chocobos if chocobo.nb_reproductions_left > 0]

        females = [chocobo for chocobo in chocobos if chocobo.gender == Gender.FEMALE]
        males   = [chocobo for chocobo in chocobos if chocobo.gender == Gender.MALE]
        
        female, male = get_best(females), get_best(males)
        offsprings = [
            reproduce(female, male)
            for _ in range(nb_reproductions_per_generation)
        ]

        average_good_strands = sum([offspring.count_good_strands() for offspring in offsprings]) / nb_reproductions_per_generation
        if not ((female, male), average_good_strands) in chocobos:
            ratings.append(((female, male), average_good_strands))
            chocobos.extend(offsprings)

    return ratings

if __name__ == "__main__":
    chocobos = [
        Dna("Jacynthe",       Gender.FEMALE, [0, 0, 1, 0, 0], [0, 0, 1, 1, 0]),
        Dna("Géoffroy",       Gender.MALE,   [0, 0, 0, 0, 1], [0, 1, 1, 0, 0]),
        Dna("Stephanie",      Gender.FEMALE, [1, 0, 1, 0, 0], [0, 0, 1, 1, 0]),
        Dna("Jean-Hyves",     Gender.MALE,   [0, 1, 0, 1, 0], [1, 0, 0, 1, 1]),
        Dna("Marie-Josianne", Gender.FEMALE, [1, 0, 0, 1, 1], [1, 0, 1, 0, 0]),
        Dna("Jean-Steve",     Gender.MALE,   [1, 0, 1, 0, 1], [0, 0, 1, 1, 1]),
    ]

    ratings = rate_chocobos(chocobos, nb_generations=5, nb_reproductions_per_generation=10)

    best_score = max([average_good_strands for ((female, male), average_good_strands) in ratings])
    best_couples = [
        ((female, male), average_good_strands)
        for ((female, male), average_good_strands) in ratings
        if average_good_strands == best_score
    ]

    print("Couple scores:\n")
    for ((female, male), average_good_strands) in ratings:
        suffix = "✨" if ((female, male), average_good_strands) in best_couples else ""
        print(f"\t{female.label} + {male.label} ➡ {average_good_strands}\t{suffix}")

    print("\nNote: ✨ is the highest average number of good strands.")

    print(f"\nBest combinaison(s) found:")
    for ((female, male), average_good_strands) in best_couples:
        print(f"\t{female.label} + {male.label} ➡ {average_good_strands}")

    adam_and_eva_scores = [female.count_adam_and_eva_score() + male.count_adam_and_eva_score() for ((female, male), average_good_strands) in best_couples]
    best_couple_index = adam_and_eva_scores.index(min(adam_and_eva_scores))
    ((best_couple_female, best_couple_male), best_couple_average_good_strands) = best_couples[best_couple_index]
    
    print("\nThe best combinaison (max score and lowest a&e score):")
    print(f"\t{best_couple_female.label} + {best_couple_male.label} ➡ {best_couple_average_good_strands}")
