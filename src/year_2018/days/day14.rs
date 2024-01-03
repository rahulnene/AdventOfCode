use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let recipe_count = 409551;
    match part {
        1 => solve01(recipe_count),
        2 => solve02(recipe_count),
        _ => 1,
    }
}

fn solve01(recipe_count: usize) -> usize {
    let mut recipelist = RecipeList::new();
    while recipelist.recipes.len() < recipe_count + 10 {
        recipelist.step();
    }
    recipelist
        .recipes
        .iter()
        .skip(recipe_count)
        .take(10)
        .map(|i| i.to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn solve02(recipe_count: usize) -> usize {
    let mut recipelist = RecipeList::new();
    while recipelist
        .recipes
        .iter()
        .skip(recipelist.recipes.len() - recipe_count.to_string().len())
        .map(|i| i.to_string())
        .collect::<String>()
        != recipe_count.to_string()
    {
        recipelist.step();
    }
    list_to_string(&recipelist.recipes)
        .find(&recipe_count.to_string())
        .unwrap()
}

fn list_to_string(list: &[usize]) -> String {
    list.iter().map(|i| i.to_string()).collect::<String>()
}

#[derive(Debug, Clone)]
struct RecipeList {
    recipes: Vec<usize>,
    elf_ind: (usize, usize),
}

impl RecipeList {
    fn new() -> Self {
        RecipeList {
            recipes: vec![3, 7],
            elf_ind: (0, 1),
        }
    }

    fn read_values_at_elf(&self) -> (usize, usize) {
        (self.recipes[self.elf_ind.0], self.recipes[self.elf_ind.1])
    }

    fn increment_elves(&mut self) {
        let (val_a, val_b) = self.read_values_at_elf();
        self.elf_ind = (
            (self.elf_ind.0 + val_a + 1) % self.recipes.len(),
            (self.elf_ind.1 + val_b + 1) % self.recipes.len(),
        );
    }

    fn step(&mut self) {
        let vals = self.read_values_at_elf();
        let recipes_to_add = (vals.0 + vals.1)
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect_vec();
        self.recipes.extend(recipes_to_add);
        self.increment_elves();
    }

    fn print(&self) {
        println!(
            "{:?} <=> {} {}",
            self.recipes, self.elf_ind.0, self.elf_ind.1
        );
    }
}
