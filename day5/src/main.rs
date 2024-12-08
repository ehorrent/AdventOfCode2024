use std::collections::{HashMap, HashSet};

struct PageRules {
    after_pages: HashSet<usize>,
}

impl PageRules {
    fn new(after_page: usize) -> Self {
        PageRules {
            after_pages: [after_page].into(),
        }
    }

    fn validate<'a, I>(&self, before_pages: I) -> bool
    where
        I: Iterator<Item = &'a usize>,
    {
        for before_page in before_pages {
            if self.after_pages.contains(&before_page) {
                return false;
            }
        }

        true
    }
}

type Rules = HashMap<usize, PageRules>;
type Update = Vec<usize>;

fn validate_update(rules: &Rules, update: &Update) -> bool {
    for (page_index, page) in update.iter().enumerate() {
        if let Some(page_rules) = rules.get(page) {
            if false == page_rules.validate(update.iter().take(page_index)) {
                return false;
            }
        }
    }

    true
}

fn sum_valid_updates(updates: &Vec<Update>, rules: &Rules) -> usize {
    let mut result = 0;

    for update in updates {
        if validate_update(rules, &update) {
            let middle_index = (update.len() - 1) / 2;
            result += update[middle_index];
        }
    }

    result
}

fn sort_invalid_update(update: &Update, rules: &Rules) -> Update {
    let mut new_update = update.clone();
    new_update.sort_by(|lpage, rpage| {
        if let Some(left_page_rules) = rules.get(lpage) {
            if left_page_rules.after_pages.contains(rpage) {
                return std::cmp::Ordering::Less;
            }
        }

        if let Some(right_page_rules) = rules.get(rpage) {
            if right_page_rules.after_pages.contains(lpage) {
                return std::cmp::Ordering::Greater;
            }
        }

        std::cmp::Ordering::Equal
    });

    new_update
}

fn sum_invalid_updates(updates: &Vec<Update>, rules: &Rules) -> usize {
    let mut result = 0;
    for update in updates {
        if !validate_update(rules, &update) {
            let ordered_update = sort_invalid_update(&update, &rules);
            let middle_index = (update.len() - 1) / 2;
            result += ordered_update[middle_index];
        }
    }

    result
}

fn parse_rules(raw_data: &str) -> Rules {
    raw_data.lines().fold(HashMap::new(), |mut rules, line| {
        let parts: Vec<&str> = line.split("|").collect();
        let lpage = parts[0].parse::<usize>().unwrap();
        let rpage = parts[1].parse::<usize>().unwrap();
        match rules.get_mut(&lpage) {
            Some(page_rules) => {
                page_rules.after_pages.insert(rpage);
            }
            None => {
                let new_page_rules = PageRules::new(rpage);
                rules.insert(lpage, new_page_rules);
            }
        }

        rules
    })
}

fn parse_updates(raw_data: &str) -> Vec<Update> {
    raw_data
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.split(",")
                .map(|page| page.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let raw_data = include_str!("./input.txt");
    let parts: Vec<&str> = raw_data.split(" ").collect();
    let rules = parse_rules(parts[0]);
    let updates = parse_updates(parts[1]);

    let result = sum_valid_updates(&updates, &rules);
    println!("Result = {}", result);

    let invalid_result = sum_invalid_updates(&updates, &rules);
    println!("Reordered result = {}", invalid_result);
}
